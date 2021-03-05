//use roctogen::adapters::isahc;
use roctogen::adapters::RequestAdapter;
use roctogen::api::repos;
use roctogen::auth::Auth;
//use roctogen::models;

fn main() {
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
    let req = repos::new(&auth).list_for_user("fussybeaver", params).expect("http request");
    match req.retrieve() {
        Ok(repos)  => println!("ok: {:?}", repos),
        Err(e) => println!("err: {:?}", e)
    };
}
