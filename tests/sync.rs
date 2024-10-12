#[cfg(not(target_arch = "wasm32"))]
use roctogen::api::{
    self, activity, gists, issues, licenses, meta, rate_limit, reactions, repos, search, users,
};

use roctogen::auth::Auth;
use roctogen::models;

use log::{debug, info};

#[cfg(all(not(target_arch = "wasm32"), not(feature = "reqwest")))]
#[test]
fn list_commits_sync_ok() {
    let auth = Auth::None;
    let per_page = api::PerPage::new(1);

    let mut params: repos::ReposListCommitsParams = per_page.as_ref().into();
    params = params.author("fussybeaver").page(2);

    let req = repos::new(&auth).list_commits("fussybeaver", "bollard", Some(params));

    assert!(req.is_ok());
}

#[cfg(all(not(target_arch = "wasm32"), not(feature = "reqwest")))]
#[test]
fn search_sync_ok() {
    let auth = Auth::None;
    let search = search::new(&auth);
    let opts = search::SearchReposParams::new().q("bollard");
    let req = search.repos(opts);

    assert!(req.is_ok());
}

#[cfg(all(not(target_arch = "wasm32"), not(feature = "reqwest")))]
#[test]
fn gists_sync_ok() {
    let auth = Auth::None;
    let gists = gists::new(&auth);
    let req = gists.list_public(Some(&api::PerPage::new(1)));

    assert!(req.is_ok());
}

#[cfg(all(not(target_arch = "wasm32"), not(feature = "reqwest")))]
#[test]
fn users_sync_ok() {
    let auth = Auth::None;
    let users = users::new(&auth);

    let req = users.list(Some(users::UsersListParams::new().per_page(1)));

    assert!(req.is_ok());

    let req = users.list_gpg_keys_for_user("fussybeaver", Some(&api::PerPage::new(1)));

    assert!(req.is_ok());
}

#[cfg(all(not(target_arch = "wasm32"), not(feature = "reqwest")))]
#[test]
fn meta_sync_ok() {
    let auth = Auth::None;
    let meta = meta::new(&auth);
    let req = meta.get();

    assert!(req.is_ok());
}

#[cfg(all(not(target_arch = "wasm32"), not(feature = "reqwest")))]
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

#[cfg(all(not(target_arch = "wasm32"), not(feature = "reqwest")))]
#[test]
fn license_sync_ok() {
    use roctogen::api::licenses::LicensesGetForRepoParams;

    let auth = Auth::None;
    let license = licenses::new(&auth);
    let req = license.get_for_repo("fussybeaver", "bollard", None::<LicensesGetForRepoParams>);

    assert!(req.is_ok());

    let req = license.get_all_commonly_used(None::<licenses::LicensesGetAllCommonlyUsedParams>);

    assert!(req.is_ok());

    let req = license.get("unlicense");

    assert!(req.is_ok());
}

#[cfg(all(not(target_arch = "wasm32"), not(feature = "reqwest")))]
#[test]
fn reactions_sync_ok() {
    let auth = Auth::None;
    let per_page = api::PerPage::new(1);
    let reactions = reactions::new(&auth);
    let req = reactions.list_for_issue("fussybeaver", "bollard", 86, Some(&per_page));

    assert!(req.is_ok());
}

#[cfg(all(not(target_arch = "wasm32"), not(feature = "reqwest")))]
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

#[cfg(all(not(target_arch = "wasm32"), not(feature = "reqwest")))]
#[test]
fn rate_limit_sync_ok() {
    let auth = Auth::None;
    let rate_limit = rate_limit::new(&auth);
    let req = rate_limit.get();

    match &req {
        Ok(x) => {
            info!("{:#?}", x);
        }
        Err(x) => {
            debug!("{:#?}", x);
            assert!(false);
        }
    };

    assert!(req.is_ok());
}

#[cfg(all(not(target_arch = "wasm32"), not(feature = "reqwest")))]
#[test]
fn post_sync_fail() {
    let auth = Auth::None;

    let body = models::PostReposAddUserAccessRestrictions {
        users: vec!["fussybeaver".to_string()].into(),
    };

    let req =
        repos::new(&auth).add_user_access_restrictions("fussybeaver", "bollard", "master", body);
    match &req {
        Ok(_) => {}
        Err(repos::ReposAddUserAccessRestrictionsError::Generic { code }) => {
            assert_eq!(404, *code);
        }
        Err(_) => {
            assert!(false);
        }
    };

    assert!(req.is_err());
}
