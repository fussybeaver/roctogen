#[cfg(target_arch = "wasm32")]
use wasm_bindgen_test::*;

#[cfg(not(target_arch = "wasm32"))]
use roctogen::api::{
    self, activity, gists, issues, licenses, meta, projects, rate_limit, reactions, repos, search,
    users,
};

#[cfg(target_arch = "wasm32")]
use roctogen::api::{self, repos};

use roctogen::auth::Auth;
use roctogen::models;

use log::{debug, info};

#[cfg(target_arch = "wasm32")]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[cfg(target_arch = "wasm32")]
fn init_log() {
    console_log::init_with_level(log::Level::Info).unwrap_or(());
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen_test]
async fn get_wasm_fail() {
    init_log();

    let auth = Auth::None;

    let per_page = api::PerPage::new(1);

    let req = repos::new(&auth)
        .list_commits_async("this-user-does-not-exist", "bollard", Some(&per_page))
        .await;
    match &req {
        Ok(_) => {}
        Err(repos::ReposListCommitsError::Status404(e)) => {
            debug!("{}", e.message.as_ref().unwrap());
        }
        Err(_) => {
            assert!(false);
        }
    };

    assert!(req.is_err());
}

#[cfg(all(not(target_arch = "wasm32"), not(feature = "reqwest")))]
#[test]
fn get_sync_fail() {
    let auth = Auth::None;

    let per_page = api::PerPage::new(1);

    let req =
        repos::new(&auth).list_commits("this-user-does-not-exist", "bollard", Some(&per_page));
    match &req {
        Ok(_) => {}
        Err(repos::ReposListCommitsError::Status404(e)) => {
            debug!("{}", e.message.as_ref().unwrap());
        }
        Err(x) => {
            debug!("{:?}", x);
            assert!(false);
        }
    };

    assert!(req.is_err());
}

#[cfg(all(not(target_arch = "wasm32"), not(feature = "reqwest")))]
#[test]
fn get_sync_ok() {
    let auth = Auth::None;

    let per_page = api::PerPage::new(1);

    let req = repos::new(&auth).list_commits("fussybeaver", "bollard", Some(&per_page));
    match &req {
        Ok(ref commits) => {
            assert!(!&commits.is_empty());
        }
        Err(ref e) => debug!("err: {}", e),
    };

    assert!(req.is_ok());
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen_test]
async fn get_wasm_ok() {
    init_log();

    let auth = Auth::None;
    let per_page = api::PerPage::new(1);

    let mut params: repos::ReposListCommitsParams = per_page.as_ref().into();
    params = params.author("fussybeaver").page(2);

    let req = repos::new(&auth)
        .list_commits_async("fussybeaver", "bollard", Some(params))
        .await;
    match req {
        Ok(ref repos) => {
            assert!(!&repos.is_empty());
        }
        Err(ref e) => debug!("err: {}", e),
    };

    assert!(req.is_ok());
}

#[cfg(all(not(target_arch = "wasm32"), feature = "isahc"))]
#[test]
fn get_async_ok() {
    let req = futures_lite::future::block_on(async {
        let auth = Auth::None;
        let per_page = api::PerPage::new(1);

        let mut params: repos::ReposListCommitsParams = per_page.as_ref().into();
        params = params.author("fussybeaver").page(2);
        repos::new(&auth)
            .list_commits_async("fussybeaver", "bollard", Some(params))
            .await
    });

    match req {
        Ok(ref repos) => {
            assert!(!&repos.is_empty());
        }
        Err(ref e) => debug!("err: {}", e),
    };

    assert!(req.is_ok());
}

#[cfg(all(not(target_arch = "wasm32"), feature = "isahc"))]
#[test]
fn get_async_fail() {
    let req = futures_lite::future::block_on(async {
        let auth = Auth::None;

        let per_page = api::PerPage::new(1);

        repos::new(&auth)
            .list_commits_async("this-user-does-not-exist", "bollard", Some(&per_page))
            .await
    });

    match &req {
        Ok(_) => {}
        Err(repos::ReposListCommitsError::Status404(e)) => {
            debug!("{}", e.message.as_ref().unwrap());
        }
        Err(x) => {
            debug!("{:?}", x);
            assert!(false);
        }
    };

    assert!(req.is_err());
}

#[cfg(all(not(target_arch = "wasm32"), feature = "reqwest"))]
#[tokio::test]
async fn get_async_ok() {
    let auth = Auth::None;
    let per_page = api::PerPage::new(1);

    let mut params: repos::ReposListCommitsParams = per_page.as_ref().into();
    params = params.author("fussybeaver").page(2);
    let req = repos::new(&auth)
        .list_commits_async("fussybeaver", "bollard", Some(params))
        .await;

    match req {
        Ok(ref repos) => {
            assert!(!&repos.is_empty());
        }
        Err(ref e) => debug!("err: {}", e),
    };

    assert!(req.is_ok());
}

#[cfg(all(not(target_arch = "wasm32"), feature = "reqwest"))]
#[tokio::test]
async fn get_async_fail() {
    let auth = Auth::None;

    let per_page = api::PerPage::new(1);

    let req = repos::new(&auth)
        .list_commits_async("this-user-does-not-exist", "bollard", Some(&per_page))
        .await;
    match &req {
        Ok(_) => {}
        Err(repos::ReposListCommitsError::Status404(e)) => {
            debug!("{}", e.message.as_ref().unwrap());
        }
        Err(x) => {
            debug!("{:?}", x);
            assert!(false);
        }
    };

    assert!(req.is_err());
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen_test]
async fn post_wasm_fail() {
    init_log();

    let auth = Auth::None;

    let body = models::PostReposAddUserAccessRestrictions {
        users: Some(vec!["fussybeaver".to_string()]),
    };

    let req = repos::new(&auth)
        .add_user_access_restrictions_async("fussybeaver", "bollard", "master", body)
        .await;
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

#[cfg(all(not(target_arch = "wasm32"), not(feature = "reqwest")))]
#[test]
fn post_sync_fail() {
    let auth = Auth::None;

    let body: models::PostReposAddUserAccessRestrictions = vec!["fussybeaver".to_string()].into();

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

#[cfg(all(not(target_arch = "wasm32"), feature = "reqwest"))]
#[tokio::test]
async fn post_async_fail() {
    let auth = Auth::None;

    let body: models::PostReposAddUserAccessRestrictions = vec!["fussybeaver".to_string()].into();

    let req = repos::new(&auth)
        .add_user_access_restrictions_async("fussybeaver", "bollard", "master", body)
        .await;
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

#[cfg(all(not(target_arch = "wasm32"), feature = "isahc"))]
#[test]
fn post_async_fail() {
    let req = futures_lite::future::block_on(async {
        let auth = Auth::None;

        let body: models::PostReposAddUserAccessRestrictions =
            vec!["fussybeaver".to_string()].into();

        repos::new(&auth)
            .add_user_access_restrictions_async("fussybeaver", "bollard", "master", body)
            .await
    });

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
