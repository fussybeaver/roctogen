use std::io::Read;

use base64::{prelude::BASE64_STANDARD, Engine};
use http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE, USER_AGENT},
    Request, Response,
};

use ureq::{config::Config, Agent, AsSendBody};

use log::debug;

use super::{GitHubRequest, GitHubResponseExt};
use crate::auth::Auth;

use serde::{de::DeserializeOwned, ser, Deserialize};
use serde_json::value::Value;

use std::error::Error;

#[derive(thiserror::Error, Debug)]
pub enum AdapterError {
    #[error(transparent)]
    Http(#[from] http::Error),
    #[error(transparent)]
    Ureq(#[from] ureq::Error),
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
    #[error(transparent)]
    IOError(#[from] std::io::Error),
    #[error("Ureq adapter only has sync fetch implemented")]
    UnimplementedAsync,
}

impl From<AdapterError> for crate::adapters::AdapterError {
    fn from(err: AdapterError) -> Self {
        Self::Client {
            description: err.to_string(),
            source: Some(Box::new(err)),
        }
    }
}

impl super::Client for Client {
    type Req = RequestWithBody;
    type Body = Vec<u8>;
    type Err = AdapterError where crate::adapters::AdapterError: From<Self::Err>;

    fn new(auth: &Auth) -> Result<Self, Self::Err> {
        Ok(Self {
            auth: auth.to_owned(),
            agent: Agent::new_with_config(Config::builder().http_status_as_error(false).build()),
        })
    }

    fn fetch(&self, req: Self::Req) -> Result<impl GitHubResponseExt, Self::Err> {
        let res = if let Some(body) = req.body {
            self.agent.run(req.req.body(body)?)
        } else {
            self.agent.run(req.req.body(())?)
        };

        match res {
            Ok(res) => Ok(res),
            Err(e) => Err(ureq::Error::into(e)),
        }
    }

    async fn fetch_async(&self, _request: Self::Req) -> Result<impl GitHubResponseExt, Self::Err> {
        Err::<Response<ureq::Body>, _>(AdapterError::UnimplementedAsync)
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

        Ok(RequestWithBody {
            req: builder,
            body: req.body,
        })
    }

    fn from_json<E: ser::Serialize>(model: E) -> Result<Self::Body, Self::Err> {
        Ok(serde_json::to_vec(&model)?)
    }
}

pub struct Client {
    auth: Auth,
    agent: Agent,
}

impl GitHubResponseExt for Response<ureq::Body> {
    fn is_success(&self) -> bool {
        300 > self.status().as_u16() && self.status().as_u16() >= 200
    }

    fn status_code(&self) -> u16 {
        self.status().as_u16()
    }

    fn to_json<E: for<'de> Deserialize<'de> + std::fmt::Debug>(
        self,
    ) -> Result<E, serde_json::Error> {
        Ok(serde_json::from_reader(self.into_body().as_reader())?)
    }

    async fn to_json_async<E: for<'de> Deserialize<'de> + Unpin + std::fmt::Debug>(
        self,
    ) -> Result<E, serde_json::Error> {
        unimplemented!("Ureq adapter only has sync json conversion implemented");
    }
}

pub struct RequestWithBody {
    pub(crate) req: http::request::Builder,
    pub(crate) body: Option<Vec<u8>>,
}
