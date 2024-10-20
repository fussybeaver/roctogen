extern crate cfg_if;
extern crate wasm_bindgen;

mod utils;

use cfg_if::cfg_if;
use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;

use roctogen::adapters::{wasm, Client};
use roctogen::api::{self, apps, issues, repos};
use roctogen::auth::Auth;
use roctogen::models;

use log::{debug, info, trace, Level};

use roctogen_common::jwt::{create_claim, create_token};

cfg_if! {
    if #[cfg(feature = "debug")] {
        fn init_log() {
            console_log::init_with_level(Level::Debug).expect("error initializing logger");
        }
    } else {
        fn init_log() {
            console_log::init_with_level(Level::Info).expect("error initializing logger");
        }
    }
}

#[wasm_bindgen]
extern "C" {
    type Error;

    #[wasm_bindgen(constructor)]
    fn new(message: &str) -> Error;
}

#[wasm_bindgen]
extern "C" {
    type KV;

    #[wasm_bindgen(static_method_of = KV)]
    fn get(key: &str) -> Promise;

    #[wasm_bindgen(static_method_of = KV)]
    fn put(key: &str, val: &str) -> Promise;

    #[wasm_bindgen(static_method_of = KV)]
    fn delete(key: &str) -> Promise;
}

async fn auth_from_jwt(jwt_js: JsValue, app_id_js: JsValue) -> Result<Auth, JsValue> {
    let jwt: String = jwt_js.dyn_into::<js_sys::JsString>()?.into();
    let app_id: f64 = js_sys::Number::from(app_id_js).value_of();

    let claims = create_claim(app_id.round() as u64);

    let token = create_token(claims, &jwt).await?;

    Ok(Auth::Bearer(token))
}

// Converts type to a JS `Error`.
fn to_js_error(e: impl std::fmt::Display) -> Error {
    Error::new(&e.to_string())
}

#[wasm_bindgen]
pub async fn run(jwt_js: JsValue, app_id_js: JsValue) -> Result<JsValue, JsValue> {
    init_log();

    info!("starting....");

    let auth = auth_from_jwt(jwt_js, app_id_js).await?;
    let client = wasm::Client::new(&auth).expect("Cannot create client");
    let apps = apps::new(&client);

    let installations = apps
        .list_installations_async(None::<apps::AppsListInstallationsParams>)
        .await
        .map_err(to_js_error)?;
    debug!("installations: {:?}", installations);

    let installation_id =
        if let Some(models::Installation { id: Some(id), .. }) = installations.get(0) {
            id
        } else {
            return Err(Error::new("Installation id not found").into());
        };

    let post_apps = models::PostAppsCreateInstallationAccessToken::default();

    let installation_token = apps
        .create_installation_access_token_async(*installation_id as i32, post_apps)
        .await
        .map_err(to_js_error)?;
    debug!("installation_token: {:?}", installation_token);

    let bearer = if let Some(token) = installation_token.token {
        token
    } else {
        return Err(Error::new("Token not found on InstallationToken").into());
    };

    let client = wasm::Client::new(&Auth::None).expect("Cannot create client");
    let repos = repos::new(&client);
    let per_page = api::PerPage::new(1);
    let commits = repos
        .list_commits_async("github", "rest-api-description", Some(&per_page))
        .await
        .map_err(to_js_error)?;

    trace!("commits: {:?}", commits);

    let sha = if commits.is_empty() {
        return Err(Error::new("No commits found on remote repo").into());
    } else {
        let head = commits.into_iter().next().unwrap();
        if let Some(sha) = head.id {
            sha
        } else {
            return Err(Error::new("No sha found for latest head commit").into());
        }
    };

    debug!("head: {}", &sha);

    let cached = JsFuture::from(KV::get("last_commit")).await?;
    let last_commit: String = cached.dyn_into::<js_sys::JsString>()?.into();

    debug!("last_commit: {}", &last_commit);

    if &sha == &last_commit {
        return Ok("No changes".into());
    }

    let commit_comparison = repos
        .compare_commits_async(
            "github",
            "rest-api-description",
            &format!("{}...{}", &last_commit, &sha),
            Some(&per_page),
        )
        .await
        .map_err(to_js_error)?;

    debug!("commit_comparison: {:?}", commit_comparison);

    let mut markdown = String::new();
    if let models::CommitComparison {
        commits: Some(commits),
        ..
    } = commit_comparison
    {
        markdown.push_str(&format!("Notify [rest-api-description](https://github.com/github/rest-api-description) changes [{}..{}](https://github.com/github/rest-api-description/compare/{}..{})\n", &last_commit[..8], &sha[..8], &last_commit, &sha));
        markdown.push_str("<details>\n");
        markdown.push_str("<summary>Commits</summary>\n\n");

        let pr_regex = regex::Regex::new(r"#(\d+)").map_err(to_js_error)?;
        let branch_regex =
            regex::Regex::new(r"(github/)(openapi-update-[a-z0-9]{64})").map_err(to_js_error)?;

        let mut iter = commits.into_iter();
        let mut cur = iter.next();

        debug!("commit: {:?}", &cur);

        while let Some(models::Commit {
            author: Some(models::Committer {
                name: Some(author), ..
            }),
            message: Some(message),
            ..
        }) = cur
        {
            debug!("author: {}, message: {}, sha: {}", &author, &message, &sha);

            let message = pr_regex.replace_all(
                &message,
                "[#$1](https://github.com/github/rest-api-description/pull/$1)",
            );
            let formatted_message = branch_regex.replace_all(
                &message,
                "[$2](https://github.com/github/rest-api-description/tree/$2)",
            );

            markdown.push_str(&format!(
                "  - [`{}`] *{}*: {}\n",
                &sha[..8],
                author,
                formatted_message
            ));

            cur = iter.next();
        }

        markdown.push_str("</details>\n");
    } else {
        return Err(Error::new("No commits found in comparison").into());
    };

    if !markdown.is_empty() {
        info!("Posting markdown: {}", &markdown);
        let post_issues_create = models::PostIssuesCreate {
            title: Some(
                format!(
                    "Notify rest-api-description changes {}..{}",
                    &last_commit[..8],
                    &sha[..8]
                )
                .into(),
            ),
            body: Some(markdown),
            labels: Some(vec!["openapi".to_owned().into()]),
            assignees: Some(vec!["fussybeaver".to_owned()]),
            ..Default::default()
        };

        let auth = Auth::Bearer(bearer);
        let client = wasm::Client::new(&auth).expect("Cannot create client");
        let issues = issues::new(&client);

        let issue = issues
            .create_async("fussybeaver", "roctogen", post_issues_create)
            .await
            .map_err(to_js_error)?;

        info!("Issue created: {:?}", issue);

        JsFuture::from(KV::put("last_commit", &sha)).await?;

        info!("Sha cached: {:?}", sha);
    } else {
        return Err(Error::new("Unable to generate text to create issue").into());
    }

    Ok("Ok".into())
}
