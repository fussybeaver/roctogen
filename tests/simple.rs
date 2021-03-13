#[cfg(target_arch = "wasm32")]
use {
    wasm_bindgen_test::*,
};

#[cfg(target_arch = "x86_64")]
use roctogen::api::{self, activity, gists, issues, licenses, meta, projects, reactions, repos, search, users};

#[cfg(target_arch = "wasm32")]
use roctogen::api::{self, repos};

use roctogen::auth::Auth;
use roctogen::models;

#[cfg(target_arch = "wasm32")]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[cfg(target_arch = "wasm32")]
// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen_test]
async fn get_wasm_fail() {

    let auth = Auth::None;

    let per_page = api::PerPage::new(1);
    
    let req = repos::new(&auth).list_commits_async("this-user-does-not-exist", "bollard", Some(&per_page)).await;
    match &req {
        Ok(_) => {},
        Err(repos::ReposListCommitsError::Status404(e)) => {
            log!("{}", e.message.as_ref().unwrap());
        }
        Err(_) => {
            assert!(false);
        }
    };

    assert!(req.is_err());
}

#[cfg(target_arch = "x86_64")]
#[test]
fn get_sync_fail() {

    let auth = Auth::None;

    let per_page = api::PerPage::new(1);

    let req = repos::new(&auth).list_commits("this-user-does-not-exist", "bollard", Some(&per_page));
    match &req {
        Ok(_) => {},
        Err(repos::ReposListCommitsError::Status404(e)) => {
            println!("{}", e.message.as_ref().unwrap());
        }
        Err(_) => {
            assert!(false);
        }
    };

    assert!(req.is_err());
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen_test]
async fn get_wasm_ok() {

    let auth = Auth::None;
    let per_page = api::PerPage::new(1);
    
    let mut params: repos::ReposListCommitsParams = per_page.as_ref().into();
    params = params.author("fussybeaver").page(2);

    let req = repos::new(&auth).list_commits_async("fussybeaver", "bollard", Some(params)).await;
    match req {
        Ok(ref repos) => {
            assert!(!&repos.is_empty());
        },
        Err(ref e) => log!("err: {}", e)
    };

    assert!(req.is_ok());
}

#[cfg(target_arch = "x86_64")]
#[test]
fn list_commits_sync_ok() {

    let auth = Auth::None;
    let per_page = api::PerPage::new(1);
    
    let mut params: repos::ReposListCommitsParams = per_page.as_ref().into();
    params = params.author("fussybeaver").page(2);

    let req = repos::new(&auth).list_commits("fussybeaver", "bollard", Some(params));

    assert!(req.is_ok());
}

#[cfg(target_arch = "x86_64")]
#[test]
fn search_sync_ok() {

    let auth = Auth::None;
    let search = search::new(&auth);
    let opts = search::SearchReposParams::new().q("bollard");
    let req = search.repos(opts);

    assert!(req.is_ok());
}

#[cfg(target_arch = "x86_64")]
#[test]
fn gists_sync_ok() {

    let auth = Auth::None;
    let gists = gists::new(&auth);
    let req = gists.list_public(Some(&api::PerPage::new(1)));

    assert!(req.is_ok());
}

#[cfg(target_arch = "x86_64")]
#[test]
fn users_sync_ok() {

    let auth = Auth::None;
    let users = users::new(&auth);

    let req = users.list(Some(users::UsersListParams::new().per_page(1)));

    assert!(req.is_ok());
}

#[cfg(target_arch = "x86_64")]
#[test]
fn meta_sync_ok() {

    let auth = Auth::None;
    let meta = meta::new(&auth);
    let req = meta.get();

    assert!(req.is_ok());
}

#[cfg(target_arch = "x86_64")]
#[test]
fn issues_sync_ok() {

    let auth = Auth::None;
    let issues = issues::new(&auth);
    let per_page = api::PerPage::new(1);
    let req = issues.list_for_repo("fussybeaver", "bollard", Some(&per_page));

    assert!(req.is_ok());

    let req = issues.list_milestones("fussybeaver", "bollard", Some(&per_page));

    assert!(req.is_ok());

    let req = issues.list_labels_for_repo("fussybeaver", "bollard", Some(&per_page));

    assert!(req.is_ok());

    let req = issues.list_events_for_repo("fussybeaver", "bollard", Some(&per_page));

    assert!(req.is_ok());

    let req = issues.list_comments_for_repo("fussybeaver", "bollard", Some(&per_page));

    assert!(req.is_ok());
}

#[cfg(target_arch = "x86_64")]
#[test]
fn license_sync_ok() {

    let auth = Auth::None;
    let license = licenses::new(&auth);
    let req = license.get_for_repo("fussybeaver", "bollard");

    assert!(req.is_ok());

    let req = license.get_all_commonly_used(None::<licenses::LicensesGetAllCommonlyUsedParams>);

    assert!(req.is_ok());

    let req = license.get("unlicense");

    assert!(req.is_ok());
}

#[cfg(target_arch = "x86_64")]
#[test]
fn reactions_sync_ok() {

    let auth = Auth::None;
    let per_page = api::PerPage::new(1);
    let reactions = reactions::new(&auth);
    let req = reactions.list_for_issue("fussybeaver", "bollard", 86, Some(&per_page));

    assert!(req.is_ok());
}

#[cfg(target_arch = "x86_64")]
#[test]
fn actions_sync_ok() {

    let auth = Auth::None;
    let per_page = api::PerPage::new(1);
    let activity = activity::new(&auth);
    let req = activity.list_watchers_for_repo("fussybeaver", "bollard", Some(&per_page));

    assert!(req.is_ok());

    let req = activity.list_repos_watched_by_user("fussybeaver", Some(&per_page));

    assert!(req.is_ok());

    let req = activity.list_repos_starred_by_user("fussybeaver", Some(&per_page));

    assert!(req.is_ok());

    let req = activity.list_repo_events("fussybeaver", "bollard", Some(&per_page));

    assert!(req.is_ok());
}

#[cfg(target_arch = "x86_64")]
#[test]
fn projects_sync_ok() {

    let auth = Auth::None;
    let per_page = api::PerPage::new(1);
    let projects = projects::new(&auth);
    let req = projects.list_for_user("fussybeaver", Some(&per_page));

    match &req {
        Ok(x) => {
            println!("{:#?}", x);
        },
        Err(x) => {
            println!("{:#?}", x);
            assert!(false);
        }
    };


    assert!(req.is_ok());
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen_test]
async fn post_wasm_fail() {

    let auth = Auth::None;

    let body = models::PostReposAddUserAccessRestrictions {
        users: Some(vec!["fussybeaver".to_string()])
    };

    let req = repos::new(&auth).add_user_access_restrictions_async("fussybeaver", "bollard", "master", body).await;
    match &req {
        Ok(_) => {},
        Err(repos::ReposAddUserAccessRestrictionsError::Generic { code }) => {
            assert_eq!(404, *code);
        }
        Err(_) => {
            assert!(false);
        }
    };

    assert!(req.is_err());
}

#[cfg(target_arch = "x86_64")]
#[test]
fn post_sync_fail() {
    let auth = Auth::None;

    let body = models::PostReposAddUserAccessRestrictions {
        users: Some(vec!["fussybeaver".to_string()])
    };

    let req = repos::new(&auth).add_user_access_restrictions("fussybeaver", "bollard", "master", body);
    match &req {
        Ok(_) => {},
        Err(repos::ReposAddUserAccessRestrictionsError::Generic { code }) => {
            assert_eq!(404, *code);
        }
        Err(_) => {
            assert!(false);
        }
    };

    assert!(req.is_err());
}
