use http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};

use http::response::Response;
use http::request::Request;

use isahc::prelude::*;
use isahc::{AsyncBody, Body, RequestExt, ResponseFuture, AsyncReadResponseExt};

use crate::auth::Auth;
use super::{FromJson, GitHubRequest, GitHubRequestBuilder, GitHubResponseExt};

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

pub(crate) async fn fetch_async(request: Request<AsyncBody>) -> Result<Response<AsyncBody>, AdapterError> {
    Ok(request.send_async().await?)
}

impl<T> GitHubResponseExt for Response<T> {
    fn is_success(&self) -> bool {
        self.status().is_success()
    }

    fn status_code(&self) -> u16 {
        self.status().as_u16()
    }
}

pub(crate) fn to_json<E: for<'de> Deserialize<'de>>(mut res: Response<Body>) -> Result<E, serde_json::Error> {
    res.json()
}

pub(crate) async fn to_json_async<E: for<'de> Deserialize<'de> + Unpin>(mut res: Response<AsyncBody>) -> Result<E, serde_json::Error> {
    Ok(res.json().await?)
}

impl<E, T> FromJson<E, T> for E
where
    T: From<Vec<u8>>,
    E: ser::Serialize + std::fmt::Debug,
{
    fn from_json(model: E) -> Result<T, serde_json::Error> {

        Ok(serde_json::to_vec(&model)?.into())
    }
}

impl<T: From<()>> GitHubRequestBuilder<T> for Request<T> 
{
    fn build(req: GitHubRequest<T>, auth: &Auth) -> Result<Self, AdapterError> {
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
            Auth::Token(token) => builder.header(AUTHORIZATION, format!("token {}", token)),
            Auth::Bearer(bearer) => builder.header(AUTHORIZATION, format!("Bearer {}", bearer)),
            Auth::None => builder,
        };

        if let Some(body) = req.body {
            Ok(builder.body(body)?)
        } else {
            Ok(builder.body(().into())?)
        }
    }
}
