use base64::{prelude::BASE64_STANDARD, Engine};
use http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE, USER_AGENT};

use log::debug;
use reqwest::{Body, Client, Request, RequestBuilder, Response};

use super::{FromJson, GitHubRequest, GitHubRequestBuilder, GitHubResponseExt};
use crate::auth::Auth;

use serde::{ser, Deserialize};

use std::convert::TryFrom;

#[derive(thiserror::Error, Debug)]
pub enum AdapterError {
    #[error(transparent)]
    Http(#[from] http::Error),
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
}

pub(crate) fn fetch(_request: Request) -> Result<Response, AdapterError> {
    unimplemented!("Reqwest adapter only has async fetch implemented");
}

pub(crate) async fn fetch_async(request: Request) -> Result<Response, AdapterError> {
    let res = Client::new().execute(request).await?;

    debug!("Response: {:?}", &res);

    Ok(res)
}

impl GitHubResponseExt for Response {
    fn is_success(&self) -> bool {
        self.status().is_success()
    }

    fn status_code(&self) -> u16 {
        self.status().as_u16()
    }
}

pub(crate) fn to_json<E: for<'de> Deserialize<'de>>(_res: Response) -> Result<E, AdapterError> {
    unimplemented!("Reqwest adapter only has async json conversion implemented");
}

pub(crate) async fn to_json_async<E: for<'de> Deserialize<'de> + Unpin + std::fmt::Debug>(
    res: Response,
) -> Result<E, AdapterError> {
    let json = res.json().await?;

    debug!("Body: {:?}", json);

    Ok(json)
}

impl<E> FromJson<E, Body> for E
where
    E: ser::Serialize + std::fmt::Debug,
{
    fn from_json(model: E) -> Result<Body, serde_json::Error> {
        debug!("Error: {:?}", model);

        Ok(serde_json::to_vec(&model)?.into())
    }
}

impl GitHubRequestBuilder<Body> for Request {
    fn build(req: GitHubRequest<Body>, auth: &Auth) -> Result<Self, AdapterError> {
        let mut builder = http::Request::builder();

        builder = builder
            .uri(req.uri)
            .method(req.method)
            .header(ACCEPT, "application/vnd.github.v3+json")
            .header(USER_AGENT, "roctogen")
            .header(CONTENT_TYPE, "application/json");

        for header in req.headers.iter() {
            builder = builder.header(header.0, header.1);
        }

        builder = match auth {
            Auth::Basic { user, pass } => {
                let creds = format!("{}:{}", user, pass);
                builder.header(
                    AUTHORIZATION,
                    format!("Basic {}", BASE64_STANDARD.encode(creds.as_bytes())),
                )
            }
            Auth::Token(token) => builder.header(AUTHORIZATION, format!("token {}", token)),
            Auth::Bearer(bearer) => builder.header(AUTHORIZATION, format!("Bearer {}", bearer)),
            Auth::None => builder,
        };

        if let Some(body) = req.body {
            Ok(Request::try_from(builder.body(body)?)?)
        } else {
            Ok(Request::try_from(builder.body(Vec::<u8>::new())?)?)
        }
    }
}
