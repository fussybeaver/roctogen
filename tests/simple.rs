#[cfg(target_arch = "wasm32")]
use {
    wasm_bindgen_test::*,
};

use roctogen::api::{self, gists, repos, search, users};
//use roctogen::api::{self, repos};
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

    let per_page = api::PerPage::new(10);
    
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

    let per_page = api::PerPage::new(10);

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
    let per_page = api::PerPage::new(10);
    
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
    let per_page = api::PerPage::new(10);
    
    let mut params: repos::ReposListCommitsParams = per_page.as_ref().into();
    params = params.author("fussybeaver").page(2);

    let req = repos::new(&auth).list_commits("fussybeaver", "bollard", Some(params));
    match req {
        Ok(ref repos) => {
            assert!(!&repos.is_empty());
        },
        Err(ref e) => println!("err: {}", e)
    };

    assert!(req.is_ok());
}

#[cfg(target_arch = "x86_64")]
#[test]
fn search_sync_ok() {

    let auth = Auth::None;
    let search = search::new(&auth);
    let opts = search::SearchReposParams::new().q("bollard");
    let req = search.repos(opts);

    match req {
        Ok(ref results) => {
            assert!(results.total_count.unwrap() > 0);
        },
        Err(ref e) => println!("err: {}", e)
    };

    assert!(req.is_ok());
}

#[cfg(target_arch = "x86_64")]
#[test]
fn gists_sync_ok() {

    let auth = Auth::None;
    let gists = gists::new(&auth);
    let req = gists.list_public(Some(&api::PerPage::new(10)));

    match req {
        Ok(ref results) => {
            assert!(!results.is_empty());
        },
        Err(ref e) => println!("err: {}", e)
    };

    assert!(req.is_ok());
}

#[cfg(target_arch = "x86_64")]
#[test]
fn users_sync_ok() {

    let auth = Auth::None;
    let users = users::new(&auth);

    let req = users.list(Some(users::UsersListParams::new().per_page(10)));

    match req {
        Ok(ref results) => {
            assert!(!results.is_empty());
        },
        Err(ref e) => println!("err: {}", e)
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
