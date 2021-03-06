use http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};

pub use http::response::Response;
pub use http::request::Request;

use isahc::prelude::*;
use isahc::Body;
use isahc::RequestExt;

use crate::auth::Auth;
//use super::{Json, GitHubRequest, GitHubRequestBuilder, RequestAdapter, RequestBuilderExt, StatusExt};
use super::{Json, GitHubRequest, GitHubRequestBuilder, GitHubResponseExt};

use serde::Deserialize;

#[derive(thiserror::Error, Debug)]
pub enum AdapterError {
    #[error(transparent)]
    Http(#[from] http::Error),
    #[error(transparent)]
    Isahc(#[from] isahc::Error),
}

pub(crate) fn fetch(request: Request<()>) -> Result<Response<Body>, AdapterError> {
    Ok(request.send()?)
}

impl<T: std::io::Read> GitHubResponseExt for Response<T> {
    fn is_success(&self) -> bool {
        self.status().is_success()
    }

    fn status_code(&self) -> u16 {
        self.status().as_u16()
    }
}

impl<T, E> Json<E> for Response<T>
where
    T: std::io::Read,
    E: for<'de> Deserialize<'de>,
{
    fn to_json(mut self) -> Result<E, serde_json::Error> {
        self.json()
    }
}

impl GitHubRequestBuilder for Request<()> 
{
    fn build(req: GitHubRequest, auth: &Auth) -> Result<Self, AdapterError> {
        let mut builder = http::Request::builder();

        debug!("build request uri ({:?})", &req.uri);

        builder = builder
            .uri(req.uri)
            .header(ACCEPT, "application/vnd.github.v3+json")
            .header(CONTENT_TYPE, "application/json");

        builder = match auth {
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

        Ok(builder.body(req.body)?)
    }
}
