use http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};

use http::response::Response;
use http::request::Request;

use isahc::prelude::*;
use isahc::Body;
use isahc::RequestExt;

use crate::auth::Auth;
use super::{FromJson, GitHubRequest, GitHubRequestBuilder, GitHubResponseExt, ToJson};

use serde::{ser, Deserialize};

#[derive(thiserror::Error, Debug)]
pub enum AdapterError {
    #[error(transparent)]
    Http(#[from] http::Error),
    #[error(transparent)]
    Isahc(#[from] isahc::Error),
}

pub(crate) fn fetch(request: Request<Body>) -> Result<Response<Body>, AdapterError> {
    Ok(request.send()?)
}

pub(crate) async fn fetch_async(_request: Request<Body>) -> Result<Response<Body>, AdapterError> {
    todo!()
}

impl<T: std::io::Read> GitHubResponseExt for Response<T> {
    fn is_success(&self) -> bool {
        self.status().is_success()
    }

    fn status_code(&self) -> u16 {
        self.status().as_u16()
    }
}

impl<T, E> ToJson<E> for Response<T>
where
    T: std::io::Read,
    E: for<'de> Deserialize<'de>,
{
    fn to_json(mut self) -> Result<E, serde_json::Error> {
        self.json()
    }
}

pub(crate) type FromJsonType = Body;

impl<E> FromJson<E> for E
where
    E: ser::Serialize + std::fmt::Debug,
{
    fn from_json(model: E) -> Result<FromJsonType, serde_json::Error> {

        Ok(serde_json::to_vec(&model)?.into())
    }
}

impl GitHubRequestBuilder for Request<Body> 
{
    fn build(req: GitHubRequest, auth: &Auth) -> Result<Self, AdapterError> {
        let mut builder = http::Request::builder();

        builder = builder
            .uri(req.uri)
            .header(ACCEPT, "application/vnd.github.v3+json")
            .header(CONTENT_TYPE, "application/json");

        for header in req.headers.iter() {
            builder = builder.header(header.0, header.1);
        }

        builder = match auth {
            Auth::Basic { user, pass } => {
                let creds = format!("{}:{}", user, pass);
                builder.header(
                    AUTHORIZATION,
                    format!("Basic {}", base64::encode(creds.as_bytes())),
                )
            }
            Auth::OAuth { token } => builder.header(AUTHORIZATION, format!("token {}", token)),
            Auth::JWT { bearer } => builder.header(AUTHORIZATION, format!("Bearer {}", bearer)),
            Auth::None => builder,
        };

        if let Some(body) = req.body {
            Ok(builder.body(body)?)
        } else {
            Ok(builder.body(().into())?)
        }
    }
}
