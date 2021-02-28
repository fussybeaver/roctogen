use http::header::HeaderValue;
use http::header::{AUTHORIZATION, CONTENT_TYPE};
use http::request::Request;
use http::response::Response;

use crate::adapters::RequestAdapter;
use crate::models;

pub enum Auth {
    Basic { user: String, pass: String },
    OAuth { token: String },
    None,
}

pub const GITHUB_BASE_API_URL: &str = "https://github.com/api";

pub mod repos {
    use super::*;

    pub struct Repos<'api> {
        auth: &'api Auth,
    }

    pub fn new(auth: &Auth) -> Repos {
        Repos { auth }
    }

    impl<'api> Repos<'api> {
        pub fn list_for_orgs(
            &self,
            org: &str,
        ) -> Result<impl RequestAdapter<Vec<models::Repository>>, http::Error> {
            env_logger::init();

            let mut builder = Request::builder();

            let request_uri = format!("{}/org/{}/repos", GITHUB_BASE_API_URL, org);

            debug!("build request uri ({:?})", &request_uri);

            builder = builder
                .uri(request_uri)
                .header(CONTENT_TYPE, "application/json");

            builder = match self.auth {
                Auth::Basic { user, pass } => {
                    let creds = format!("{}:{}", user, pass);
                    builder.header(
                        AUTHORIZATION,
                        format!("Basic {}", base64::encode(creds.as_bytes())),
                    )
                }
                Auth::OAuth { token } => builder.header(AUTHORIZATION, format!("token {}", token)),
                Auth::None => builder,
            };

            Ok(builder.body(())?)
        }
    }
}
