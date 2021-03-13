//! Method, error and parameter types for the Emojis endpoint.
#![allow(
    unused_imports,
)]
/* 
 * GitHub v3 REST API
 *
 * GitHub's v3 REST API.
 *
 * OpenAPI spec version: 1.1.4
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

use serde::Deserialize;

use crate::adapters::{AdapterError, FromJson, GitHubRequest, GitHubRequestBuilder, GitHubResponseExt, ToJson};
use crate::auth::Auth;
use crate::models::*;

use super::PerPage;

use std::collections::HashMap;
use serde_json::value::Value;

pub struct Emojis<'api> {
    auth: &'api Auth
}

pub fn new(auth: &Auth) -> Emojis {
    Emojis { auth }
}

/// Errors for the [Get emojis](Emojis::get_async()) endpoint.
#[derive(Debug, thiserror::Error)]
pub enum EmojisGetError {
    #[error(transparent)]
    AdapterError(#[from] AdapterError),
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
    #[error(transparent)]
    SerdeUrl(#[from] serde_urlencoded::ser::Error),


    // -- endpoint errors

    #[error("Not Modified")]
    Status304,
    #[error("Status code: {}", code)]
    Generic { code: u16 },
}



impl<'api> Emojis<'api> {
    /// ---
    ///
    /// # Get emojis
    ///
    /// Lists all the emojis available to use on GitHub.
    /// 
    /// [GitHub API docs for get](https://docs.github.com/rest/reference/emojis/#get-emojis)    
    /// ---
    pub async fn get_async(&self) -> Result<HashMap<String, String>, EmojisGetError> {

        let request_uri = format!("{}/emojis", super::GITHUB_BASE_API_URL);


        let req = GitHubRequest {
            uri: request_uri,
            body: None,
            method: "GET",
            headers: vec![]
        };

        let request = GitHubRequestBuilder::build(req, self.auth)?;

        // --

        let github_response = crate::adapters::fetch_async(request).await?;

        // --

        if github_response.is_success() {
            Ok(github_response.to_json()?)
        } else {
            match github_response.status_code() {
                304 => Err(EmojisGetError::Status304),
                code => Err(EmojisGetError::Generic { code }),
            }
        }
    }

    /// ---
    ///
    /// # Get emojis
    ///
    /// Lists all the emojis available to use on GitHub.
    /// 
    /// [GitHub API docs for get](https://docs.github.com/rest/reference/emojis/#get-emojis)    
    /// ---
    #[cfg(not(target_arch = "wasm32"))]
    pub fn get(&self) -> Result<HashMap<String, String>, EmojisGetError> {

        let request_uri = format!("{}/emojis", super::GITHUB_BASE_API_URL);


        let req = GitHubRequest {
            uri: request_uri,
            body: None,
            method: "GET",
            headers: vec![]
        };

        let request = GitHubRequestBuilder::build(req, self.auth)?;

        // --

        let github_response = crate::adapters::fetch(request)?;

        // --

        if github_response.is_success() {
            Ok(github_response.to_json()?)
        } else {
            match github_response.status_code() {
                304 => Err(EmojisGetError::Status304),
                code => Err(EmojisGetError::Generic { code }),
            }
        }
    }

}
