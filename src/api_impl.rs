use http::header::HeaderValue;
use http::header::{AUTHORIZATION, CONTENT_TYPE};
use http::request::Request;
use http::response::Response;

use serde::de::DeserializeOwned;
use serde::Deserialize;

use crate::adapters::{Json, RequestAdapter, RequestBuilderExt, StatusExt};
use crate::auth::Auth;
use crate::models;

pub const GITHUB_BASE_API_URL: &str = "https://github.com/api";

pub mod repos {
    use super::*;

    pub struct Repos<'api> {
        auth: &'api Auth,
    }

    pub fn new(auth: &Auth) -> Repos {
        Repos { auth }
    }

    #[derive(Debug, Deserialize)]
    pub enum ReposListForOrgsError {
        Status400(models::ValidationError),
        Status404(models::BasicError),
        Status401(models::BasicError),
        Generic { code: u16 },
    }

    #[derive(Debug, Deserialize)]
    pub enum ReposFetchError {
        Status400(models::ValidationError),
        Status404(models::BasicError),
        Generic { code: u16 },
    }

    impl<T> StatusExt<T> for ReposListForOrgsError
    where
        T: std::io::Read,
    {
        fn resolve(
            status: http::StatusCode,
            res: &mut Response<T>,
        ) -> Result<Self, serde_json::Error> {
            match status.as_u16() {
                400 => {
                    let e: models::ValidationError = res.to_json()?;
                    Ok(ReposListForOrgsError::Status400(e))
                }
                404 => {
                    let e: models::BasicError = res.to_json()?;
                    Ok(ReposListForOrgsError::Status404(e))
                }
                401 => {
                    let e: models::BasicError = res.to_json()?;
                    Ok(ReposListForOrgsError::Status401(e))
                }
                code => Ok(ReposListForOrgsError::Generic { code }),
            }
        }
    }

    impl<T> StatusExt<T> for ReposFetchError
    where
        T: std::io::Read,
    {
        fn resolve(
            status: http::StatusCode,
            res: &mut Response<T>,
        ) -> Result<Self, serde_json::Error> {
            match status.as_u16() {
                400 => {
                    let e: models::ValidationError = res.to_json()?;
                    Ok(ReposFetchError::Status400(e))
                }
                404 => {
                    let e: models::BasicError = res.to_json()?;
                    Ok(ReposFetchError::Status404(e))
                }
                code => Ok(ReposFetchError::Generic { code }),
            }
        }
    }

    impl std::error::Error for ReposListForOrgsError {
        fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
            None
        }
    }

    impl std::fmt::Display for ReposListForOrgsError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                ReposListForOrgsError::Status400(_) => write!(f, "{:?}", self),
                ReposListForOrgsError::Status401(_) => write!(f, "{:?}", self),
                ReposListForOrgsError::Status404(_) => write!(f, "{:?}", self),
                ReposListForOrgsError::Generic { code } => write!(f, "Status code: {}", code),
            }
        }
    }

    impl<'api> Repos<'api> {
        pub fn list_for_orgs(
            &self,
            org: &str,
        ) -> Result<impl RequestAdapter<Vec<models::Repository>, ReposListForOrgsError>, http::Error>
        {
            env_logger::init();

            let mut builder = Request::builder();

            let request_uri = format!("{}/org/{}/repos", GITHUB_BASE_API_URL, org);

            debug!("build request uri ({:?})", &request_uri);

            builder = builder
                .uri(request_uri)
                .header(CONTENT_TYPE, "application/json");

            builder = builder.authenticate(&self.auth);

            Ok(builder.body(())?)
        }
    }
}
