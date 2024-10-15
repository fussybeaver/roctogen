use std::io::Read;

use base64::{prelude::BASE64_STANDARD, Engine};
use http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE, USER_AGENT},
    Request, Response,
};

use ureq::{Agent, AsSendBody};

use log::debug;

use super::{FromJson, GitHubRequest, GitHubRequestBuilder, GitHubResponseExt};
use crate::auth::Auth;

use serde::{de::DeserializeOwned, ser, Deserialize};
use serde_json::value::Value;

pub struct RequestWithBody {
    pub(crate) req: http::request::Builder,
    pub(crate) body: Option<Vec<u8>>,
}

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
}

impl super::Client for Client {
    type Req = RequestWithBody;

    fn new(auth: &Auth) -> Self {
        Self {
            auth: auth.to_owned(),
            agent: Agent::new_with_config(ureq::Config {
                http_status_as_error: false,
                ..Default::default()
            }),
        }
    }

    fn get_auth(&self) -> &Auth {
        &self.auth
    }

    fn fetch(&self, request: Self::Req) -> Result<impl GitHubResponseExt, AdapterError> {
        let res = if let Some(body) = request.body {
            self.agent.run(request.req.body(body)?)
        } else {
            self.agent.run(request.req.body(())?)
        };

        match res {
            Ok(res) => Ok(res),
            Err(e) => Err(ureq::Error::into(e)),
        }
    }

    async fn fetch_async(
        &self,
        request: Self::Req,
    ) -> Result<impl GitHubResponseExt, AdapterError> {
        unimplemented!("Ureq adapter only has sync fetch implemented");
        Err::<Response<ureq::Body>, _>(
            std::io::Error::new(std::io::ErrorKind::Other, "oh no!").into(),
        )
    }
}

pub struct Client {
    pub(crate) auth: Auth,
    pub(crate) agent: Agent,
}

impl GitHubResponseExt for Response<ureq::Body> {
    fn is_success(&self) -> bool {
        300 > self.status().as_u16() && self.status().as_u16() >= 200
    }

    fn status_code(&self) -> u16 {
        self.status().as_u16()
    }

    fn to_json<E: for<'de> Deserialize<'de> + std::fmt::Debug>(self) -> Result<E, AdapterError> {
        Ok(serde_json::from_reader(self.into_body().as_reader())?)
    }

    async fn to_json_async<E: for<'de> Deserialize<'de> + Unpin + std::fmt::Debug>(
        self,
    ) -> Result<E, AdapterError> {
        unimplemented!("Ureq adapter only has sync json conversion implemented");
    }
}

impl<E> FromJson<E, Value> for E
where
    E: ser::Serialize + std::fmt::Debug,
{
    fn from_json(model: E) -> Result<Value, serde_json::Error> {
        debug!("Error: {:?}", model);

        Ok(serde_json::to_value(&model)?.into())
    }
}

impl<E> FromJson<E, Vec<u8>> for E
where
    E: ser::Serialize + std::fmt::Debug,
{
    fn from_json(model: E) -> Result<Vec<u8>, serde_json::Error> {
        debug!("Error: {:?}", model);

        Ok(serde_json::to_vec(&model)?)
    }
}

impl<C: super::Client> GitHubRequestBuilder<Vec<u8>, C> for RequestWithBody {
    fn build(req: GitHubRequest<Vec<u8>>, client: &C) -> Result<Self, AdapterError> {
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

        builder = match client.get_auth() {
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
}
