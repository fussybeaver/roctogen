use github_stubs::adapters::isahc;
use github_stubs::adapters::RequestAdapter;
use github_stubs::api::repos;
use github_stubs::api::Auth;
use github_stubs::models;

fn main() {
    let auth = Auth::None;
    let req = repos::new(&auth).list_for_orgs("fussybeaver").unwrap();
    let res: Vec<models::Repository> = req.retrieve().unwrap();
    println!("{:?}", res);
}
