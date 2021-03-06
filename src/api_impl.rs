//use http::header::HeaderValue;
//use http::header::{AUTHORIZATION, CONTENT_TYPE};
//use http::request::Request;
//use http::response::Response;

use serde::de::DeserializeOwned;
use serde::Deserialize;

//use async_trait::async_trait;

//use crate::adapters::{Json, RequestAdapter, RequestBuilderExt, StatusExt, GitHubRequest, GitHubRequestBuilder};
use crate::adapters::{AdapterError, Json, GitHubRequest, GitHubRequestBuilder, GitHubResponseExt};
use crate::auth::Auth;
use crate::models;

#[cfg(target_arch = "wasm32")]
use {
    cfg_if::cfg_if,
    js_sys::{Object, Promise},
    wasm_bindgen::prelude::*,
    wasm_bindgen::JsCast,
    wasm_bindgen_futures::JsFuture,
    web_sys::console,
    web_sys::{Request, RequestInit, Response, ServiceWorkerGlobalScope, Window},
};

#[cfg(target_arch = "x86_64")]
use {
    http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    http::request::Request,
    http::response::Response,
    isahc::prelude::*,
    isahc::Body,
    isahc::RequestExt,
};

pub const GITHUB_BASE_API_URL: &str = "https://api.github.com";

pub mod repos {
    use super::*;

    pub struct Repos<'api> {
        auth: &'api Auth,
    }

    pub fn new(auth: &Auth) -> Repos {
        Repos { auth }
    }

    //#[derive(Debug, thiserror::Error, Deserialize)]
    //pub enum ReposListForOrgsError {
    //    Status400(models::ValidationError),
    //    Status404(models::BasicError),
    //    Status401(models::BasicError),
    //    Generic { code: u16 },
    //}

    #[derive(Debug, Deserialize)]
    pub enum ReposFetchError {
        Status400(models::ValidationError),
        Status404(models::BasicError),
        Generic { code: u16 },
    }

    //impl<D, T> StatusExt<D, T> for ReposListForOrgsError
    //where
    //    D: for<'de> Deserialize<'de>,
    //    T: Json<D>,
    //{
    //    fn resolve(status: u16, res: impl Json<Box<dyn DeserializeOwned + 'static>>) -> Result<Self, serde_json::Error> {
    //        match status {
    //            400 => {
    //                Ok(ReposListForOrgsError::Status400(res.to_json()?))
    //            }
    //            404 => {
    //                Ok(ReposListForOrgsError::Status404(res.to_json()?))
    //            }
    //            401 => {
    //                Ok(ReposListForOrgsError::Status401(res.to_json()?))
    //            }
    //            code => Ok(ReposListForOrgsError::Generic { code }),
    //        }
    //    }
    //}

    //#[async_trait]
    //impl StatusExt for ReposFetchError
    //{
    //    async fn resolve(
    //        status: http::StatusCode,
    //        res: Response,
    //    ) -> Result<Self, Error> {
    //        match status.as_u16() {
    //            400 => {
    //                let e: models::ValidationError = res.to_json()?;
    //                Ok(ReposFetchError::Status400(e))
    //            }
    //            404 => {
    //                let e: models::BasicError = res.to_json()?;
    //                Ok(ReposFetchError::Status404(e))
    //            }
    //            code => Ok(ReposFetchError::Generic { code }),
    //        }
    //    }
    //}

    // impl std::error::Error for ReposListForOrgsError {
    //     fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
    //         None
    //     }
    // }

    //impl std::fmt::Display for ReposListForOrgsError {
    //    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    //        match self {
    //            ReposListForOrgsError::Status400(_) => write!(f, "{:?}", self),
    //            ReposListForOrgsError::Status401(_) => write!(f, "{:?}", self),
    //            ReposListForOrgsError::Status404(_) => write!(f, "{:?}", self),
    //            ReposListForOrgsError::Generic { code } => write!(f, "Status code: {}", code),
    //        }
    //    }
    //}

    #[derive(thiserror::Error, Debug)]
    pub enum ReposListForOrgsError {
        #[error(transparent)]
        AdapterError(#[from] AdapterError),

        // -- downstream errors

        #[cfg(target_arch = "x86_64")]
        #[error(transparent)]
        Isahc(#[from] isahc::Error),
        #[error(transparent)]
        Serde(#[from] serde_json::Error),

        // -- method errors

        #[error("status 400")]
        Status400(models::ValidationError),
        #[error("status 404")]
        Status404(models::BasicError),
        #[error("status 401")]
        Status401(models::BasicError),
        #[error("generic status error")]
        Generic { code: u16 },
    }

    impl<'api> Repos<'api> {
        // TODO: move this out
        #[cfg(target_arch = "wasm32")]
        pub async fn list_for_user(
            &self,
            user: &str,
            param: ReposListForUserParams,
        ) -> Result<Vec<models::Repository>, ReposListForOrgsError> {

            let request_uri = format!("{}/users/{}/repos", GITHUB_BASE_API_URL, user);

            let req = GitHubRequest {
                uri: request_uri,
                body: (),
                method: "GET"
            };

            let request = GitHubRequestBuilder::build(req, self.auth)?;

            // --

            let github_response = crate::adapters::fetch_async(request).await?;

            // --

            if github_response.is_success() {
                Ok(github_response.to_json()?)
            } else {
                match github_response.status_code() {
                    400 => Err(ReposListForOrgsError::Status400(github_response.to_json()?)),
                    404 => Err(ReposListForOrgsError::Status404(github_response.to_json()?)),
                    401 => Err(ReposListForOrgsError::Status401(github_response.to_json()?)),
                    code => Err(ReposListForOrgsError::Generic { code }),
                }
            }
        }

        #[cfg(not(target_arch = "wasm32"))]
        pub fn list_for_user(
            &self,
            user: &str,
            param: ReposListForUserParams,
        ) -> Result<Vec<models::Repository>, ReposListForOrgsError> {
            let request_uri = format!("{}/users/{}/repos", GITHUB_BASE_API_URL, user);

            let req = GitHubRequest {
                uri: request_uri,
                body: (),
                method: "GET"
            };

            let request: http::Request<()> = GitHubRequestBuilder::build(req, self.auth)?;

            // --
            
            let github_response = crate::adapters::fetch(request)?;
            
            // --

            if github_response.is_success() {
                Ok(github_response.to_json()?)
            } else {
                match github_response.status_code() {
                    400 => Err(ReposListForOrgsError::Status400(github_response.to_json()?)),
                    404 => Err(ReposListForOrgsError::Status404(github_response.to_json()?)),
                    401 => Err(ReposListForOrgsError::Status401(github_response.to_json()?)),
                    code => Err(ReposListForOrgsError::Generic { code }),
                }
            }
        }
    }

    pub struct ReposListForUserParams {
        pub _type: Option<String>,
        pub sort: Option<String>,
        pub direction: Option<String>,
        pub per_page: Option<i32>,
        pub page: Option<i32>,
    }
}
