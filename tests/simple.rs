#[cfg(target_arch = "wasm32")]
use {
    wasm_bindgen_test::*,
};

use roctogen::api::repos;
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
    /*
    let per_page = api::PerPage::new(10);
    
    let params: repos::ReposListForUserParamsBuilder = per_page.into();
    params.sort("created").direction("asc");
    */

    let params = repos::ReposListForUserParams{
        _type: None,
        sort: None,
        direction: None,
        per_page: None,
        page: None
    };
    let req = repos::new(&auth).list_for_user("this-user-does-not-exist", params).await;
    match &req {
        Ok(_) => {},
        Err(repos::ReposListForOrgsError::Status404(e)) => {
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
    /*
    let per_page = api::PerPage::new(10);
    
    let params: repos::ReposListForUserParamsBuilder = per_page.into();
    params.sort("created").direction("asc");
    */

    let params = repos::ReposListForUserParams{
        _type: None,
        sort: None,
        direction: None,
        per_page: None,
        page: None
    };
    let req = repos::new(&auth).list_for_user("this-user-does-not-exist", params);
    match &req {
        Ok(_) => {},
        Err(repos::ReposListForOrgsError::Status404(e)) => {
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
    /*
    let per_page = api::PerPage::new(10);
    
    let params: repos::ReposListForUserParamsBuilder = per_page.into();
    params.sort("created").direction("asc");
    */

    let params = repos::ReposListForUserParams{
        _type: None,
        sort: None,
        direction: None,
        per_page: None,
        page: None
    };
    let req = repos::new(&auth).list_for_user("fussybeaver", params).await;
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
    /*
    let per_page = api::PerPage::new(10);
    
    let params: repos::ReposListForUserParamsBuilder = per_page.into();
    params.sort("created").direction("asc");
    */

    let params = repos::ReposListForUserParams{
        _type: None,
        sort: None,
        direction: None,
        per_page: None,
        page: None
    };
    let req = repos::new(&auth).list_for_user("fussybeaver", params);
    match req {
        Ok(ref repos) => {
            assert!(!&repos.is_empty());
        },
        Err(ref e) => println!("err: {}", e)
    };

    assert!(req.is_ok());
}
