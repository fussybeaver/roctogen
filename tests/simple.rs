#[cfg(target_arch = "wasm32")]
use {
    wasm_bindgen_test::*,
};

use roctogen::api::{self, repos};
use roctogen::auth::Auth;

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
async fn simple_fail() {

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
fn simple_fail() {

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
async fn simple_ok() {

    let auth = Auth::None;
    let per_page = api::PerPage::new(10);
    
    let mut params: repos::ReposListCommitsParams = per_page.as_ref().into();
    params.author("fussybeaver").page(2);

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
fn simple_ok() {

    let auth = Auth::None;
    let per_page = api::PerPage::new(10);
    
    let mut params: repos::ReposListCommitsParams = per_page.as_ref().into();
    params.author("fussybeaver").page(2);

    let req = repos::new(&auth).list_commits("fussybeaver", "bollard", Some(params));
    match req {
        Ok(ref repos) => {
            assert!(!&repos.is_empty());
        },
        Err(ref e) => println!("err: {}", e)
    };

    assert!(req.is_ok());
}
