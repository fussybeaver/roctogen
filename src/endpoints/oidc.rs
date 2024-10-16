//! Method, error and parameter types for the Oidc endpoint.
#![allow(
    clippy::all
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

use crate::adapters::{AdapterError, FromJson, GitHubRequest, GitHubRequestBuilder, GitHubResponseExt};
use crate::auth::Auth;
use crate::models::*;

use super::PerPage;

use std::collections::HashMap;
use serde_json::value::Value;

pub struct Oidc<'api> {
    auth: &'api Auth
}

pub fn new(auth: &Auth) -> Oidc {
    Oidc { auth }
}

/// Errors for the [Get the customization template for an OIDC subject claim for an organization](Oidc::get_oidc_custom_sub_template_for_org_async()) endpoint.
#[derive(Debug, thiserror::Error)]
pub enum OidcGetOidcCustomSubTemplateForOrgError {
    #[error(transparent)]
    AdapterError(#[from] AdapterError),
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
    #[error(transparent)]
    SerdeUrl(#[from] serde_urlencoded::ser::Error),


    // -- endpoint errors

    #[error("Status code: {}", code)]
    Generic { code: u16 },
}

/// Errors for the [Set the customization template for an OIDC subject claim for an organization](Oidc::update_oidc_custom_sub_template_for_org_async()) endpoint.
#[derive(Debug, thiserror::Error)]
pub enum OidcUpdateOidcCustomSubTemplateForOrgError {
    #[error(transparent)]
    AdapterError(#[from] AdapterError),
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
    #[error(transparent)]
    SerdeUrl(#[from] serde_urlencoded::ser::Error),


    // -- endpoint errors

    #[error("Resource not found")]
    Status404(BasicError),
    #[error("Forbidden")]
    Status403(BasicError),
    #[error("Status code: {}", code)]
    Generic { code: u16 },
}



impl<'api> Oidc<'api> {
    /// ---
    ///
    /// # Get the customization template for an OIDC subject claim for an organization
    ///
    /// Gets the customization template for an OpenID Connect (OIDC) subject claim.
    /// 
    /// OAuth app tokens and personal access tokens (classic) need the `read:org` scope to use this endpoint.
    ///
    /// [GitHub API docs for get_oidc_custom_sub_template_for_org](https://docs.github.com/rest/actions/oidc#get-the-customization-template-for-an-oidc-subject-claim-for-an-organization)
    ///
    /// ---
    pub async fn get_oidc_custom_sub_template_for_org_async(&self, org: &str) -> Result<OidcCustomSub, OidcGetOidcCustomSubTemplateForOrgError> {

        let request_uri = format!("{}/orgs/{}/actions/oidc/customization/sub", super::GITHUB_BASE_API_URL, org);


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
            Ok(crate::adapters::to_json_async(github_response).await?)
        } else {
            match github_response.status_code() {
                code => Err(OidcGetOidcCustomSubTemplateForOrgError::Generic { code }),
            }
        }
    }

    /// ---
    ///
    /// # Get the customization template for an OIDC subject claim for an organization
    ///
    /// Gets the customization template for an OpenID Connect (OIDC) subject claim.
    /// 
    /// OAuth app tokens and personal access tokens (classic) need the `read:org` scope to use this endpoint.
    ///
    /// [GitHub API docs for get_oidc_custom_sub_template_for_org](https://docs.github.com/rest/actions/oidc#get-the-customization-template-for-an-oidc-subject-claim-for-an-organization)
    ///
    /// ---
    #[cfg(not(target_arch = "wasm32"))]
    pub fn get_oidc_custom_sub_template_for_org(&self, org: &str) -> Result<OidcCustomSub, OidcGetOidcCustomSubTemplateForOrgError> {

        let request_uri = format!("{}/orgs/{}/actions/oidc/customization/sub", super::GITHUB_BASE_API_URL, org);


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
            Ok(crate::adapters::to_json(github_response)?)
        } else {
            match github_response.status_code() {
                code => Err(OidcGetOidcCustomSubTemplateForOrgError::Generic { code }),
            }
        }
    }

    /// ---
    ///
    /// # Set the customization template for an OIDC subject claim for an organization
    ///
    /// Creates or updates the customization template for an OpenID Connect (OIDC) subject claim.
    /// 
    /// OAuth app tokens and personal access tokens (classic) need the `write:org` scope to use this endpoint.
    ///
    /// [GitHub API docs for update_oidc_custom_sub_template_for_org](https://docs.github.com/rest/actions/oidc#set-the-customization-template-for-an-oidc-subject-claim-for-an-organization)
    ///
    /// ---
    pub async fn update_oidc_custom_sub_template_for_org_async(&self, org: &str, body: OidcCustomSub) -> Result<EmptyObject, OidcUpdateOidcCustomSubTemplateForOrgError> {

        let request_uri = format!("{}/orgs/{}/actions/oidc/customization/sub", super::GITHUB_BASE_API_URL, org);


        let req = GitHubRequest {
            uri: request_uri,
            body: Some(OidcCustomSub::from_json(body)?),
            method: "PUT",
            headers: vec![]
        };

        let request = GitHubRequestBuilder::build(req, self.auth)?;

        // --

        let github_response = crate::adapters::fetch_async(request).await?;

        // --

        if github_response.is_success() {
            Ok(crate::adapters::to_json_async(github_response).await?)
        } else {
            match github_response.status_code() {
                404 => Err(OidcUpdateOidcCustomSubTemplateForOrgError::Status404(crate::adapters::to_json_async(github_response).await?)),
                403 => Err(OidcUpdateOidcCustomSubTemplateForOrgError::Status403(crate::adapters::to_json_async(github_response).await?)),
                code => Err(OidcUpdateOidcCustomSubTemplateForOrgError::Generic { code }),
            }
        }
    }

    /// ---
    ///
    /// # Set the customization template for an OIDC subject claim for an organization
    ///
    /// Creates or updates the customization template for an OpenID Connect (OIDC) subject claim.
    /// 
    /// OAuth app tokens and personal access tokens (classic) need the `write:org` scope to use this endpoint.
    ///
    /// [GitHub API docs for update_oidc_custom_sub_template_for_org](https://docs.github.com/rest/actions/oidc#set-the-customization-template-for-an-oidc-subject-claim-for-an-organization)
    ///
    /// ---
    #[cfg(not(target_arch = "wasm32"))]
    pub fn update_oidc_custom_sub_template_for_org(&self, org: &str, body: OidcCustomSub) -> Result<EmptyObject, OidcUpdateOidcCustomSubTemplateForOrgError> {

        let request_uri = format!("{}/orgs/{}/actions/oidc/customization/sub", super::GITHUB_BASE_API_URL, org);


        let req = GitHubRequest {
            uri: request_uri,
            body: Some(OidcCustomSub::from_json(body)?),
            method: "PUT",
            headers: vec![]
        };

        let request = GitHubRequestBuilder::build(req, self.auth)?;

        // --

        let github_response = crate::adapters::fetch(request)?;

        // --

        if github_response.is_success() {
            Ok(crate::adapters::to_json(github_response)?)
        } else {
            match github_response.status_code() {
                404 => Err(OidcUpdateOidcCustomSubTemplateForOrgError::Status404(crate::adapters::to_json(github_response)?)),
                403 => Err(OidcUpdateOidcCustomSubTemplateForOrgError::Status403(crate::adapters::to_json(github_response)?)),
                code => Err(OidcUpdateOidcCustomSubTemplateForOrgError::Generic { code }),
            }
        }
    }

}
