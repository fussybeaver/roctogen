use base64::{prelude::BASE64_STANDARD, Engine};
use http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE, USER_AGENT};

use log::debug;
use reqwest::{Body, Client as ReqwestClient, Request, RequestBuilder};

use super::{GitHubRequest, GitHubResponseExt};
use crate::auth::Auth;

use serde::{ser, Deserialize};

use std::convert::TryFrom;

#[derive(thiserror::Error, Debug)]
pub enum AdapterError {
    #[error(transparent)]
    Http(#[from] http::Error),
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
    #[error(transparent)]
    IOError(#[from] std::io::Error),
    #[error("Reqwest adapter only has async fetch implemented")]
    UnimplementedSync,
}

impl From<AdapterError> for crate::adapters::AdapterError {
    fn from(err: AdapterError) -> Self {
        Self::Client {
            description: err.to_string(),
            source: Some(Box::new(err)),
        }
    }
}

struct Response {
    pub(crate) bytes: bytes::Bytes,
    pub(crate) status_code: u16,
}

impl super::Client for Client {
    type Req = ::reqwest::Request;
    type Body = Body;
    type Err = AdapterError where crate::adapters::AdapterError: From<Self::Err>;

    fn new(auth: &Auth) -> Result<Self, AdapterError> {
        Ok(Self {
            auth: auth.to_owned(),
            pool: reqwest::Client::new(),
        })
    }

    fn fetch(&self, _req: Self::Req) -> Result<impl GitHubResponseExt, Self::Err> {
        Err::<Response, _>(AdapterError::UnimplementedSync)
    }

    async fn fetch_async(&self, request: Self::Req) -> Result<impl GitHubResponseExt, Self::Err> {
        let res = self.pool.execute(request).await?;

        debug!("Response: {:?}", &res);

        let code = res.status();
        Ok(Response {
            bytes: res.bytes().await?,
            status_code: u16::from(code),
        })
    }

    fn build(&self, req: GitHubRequest<Self::Body>) -> Result<Self::Req, Self::Err> {
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

        builder = match &self.auth {
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

    fn from_json<E: ser::Serialize>(model: E) -> Result<Self::Body, Self::Err> {
        Ok(serde_json::to_vec(&model)?.into())
    }
}

pub struct Client {
    auth: Auth,
    pool: reqwest::Client,
}

impl GitHubResponseExt for Response {
    fn is_success(&self) -> bool {
        300 > self.status_code && self.status_code >= 200
    }

    fn status_code(&self) -> u16 {
        self.status_code
    }

    fn to_json<E: for<'de> Deserialize<'de> + std::fmt::Debug>(
        self,
    ) -> Result<E, serde_json::Error> {
        unimplemented!("Reqwest adapter only has async json conversion implemented");
    }

    async fn to_json_async<E: for<'de> Deserialize<'de> + Unpin + std::fmt::Debug>(
        self,
    ) -> Result<E, serde_json::Error> {
        let json = serde_json::from_slice(&self.bytes)?;

        debug!("Body: {:?}", json);

        Ok(json)
    }
}
