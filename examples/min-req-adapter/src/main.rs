use base64::{prelude::BASE64_STANDARD, Engine};
use roctogen::adapters::GitHubRequest;
use roctogen::adapters::GitHubResponseExt;
use roctogen::auth::Auth;
use serde::{ser, Deserialize};
use std::error::Error;

#[derive(thiserror::Error, Debug)]
pub enum AdapterError {
    #[error(transparent)]
    MinReq(#[from] minreq::Error),
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
    #[error(transparent)]
    IOError(#[from] std::io::Error),
    #[error("Ureq adapter only has sync fetch implemented")]
    UnimplementedAsync,
}

impl From<AdapterError> for roctogen::adapters::AdapterError {
    fn from(err: AdapterError) -> Self {
        Self::Client {
            description: err.to_string(),
            source: Some(Box::new(err)),
        }
    }
}

struct Response {
    pub inner: minreq::Response,
}

impl roctogen::adapters::Client for Client {
    type Req = minreq::Request;
    type Err = AdapterError where roctogen::adapters::AdapterError: From<Self::Err>;
    type Body = Vec<u8>;

    fn new(auth: &Auth) -> Result<Self, Self::Err> {
        Ok(Self {
            auth: auth.to_owned(),
        })
    }

    fn fetch(&self, req: Self::Req) -> Result<impl GitHubResponseExt, Self::Err> {
        Ok(Response { inner: req.send()? })
    }

    async fn fetch_async(&self, _request: Self::Req) -> Result<impl GitHubResponseExt, Self::Err> {
        Err::<Response, _>(AdapterError::UnimplementedAsync)
    }

    fn build(&self, req: GitHubRequest<Self::Body>) -> Result<Self::Req, Self::Err> {
        let mut min_req =
            minreq::Request::new(minreq::Method::Custom(String::from(req.method)), req.uri);
        min_req = min_req.with_header("accept", "application/vnd.github.v3+json");
        min_req = min_req.with_header("user-agent", "roctogen");
        min_req = min_req.with_header("content-type", "application/json");

        for header in req.headers.iter() {
            min_req = min_req.with_header(header.0, header.1);
        }

        min_req = match &self.auth {
            Auth::Basic { user, pass } => {
                let creds = format!("{}:{}", user, pass);
                min_req.with_header(
                    "authorization",
                    format!("Basic {}", BASE64_STANDARD.encode(creds.as_bytes())),
                )
            }
            Auth::Token(token) => min_req.with_header("authorization", format!("token {}", token)),
            Auth::Bearer(bearer) => {
                min_req.with_header("authorization", format!("Bearer {}", bearer))
            }
            Auth::None => min_req,
        };

        if let Some(body) = req.body {
            min_req = min_req.with_body(body);
        }

        Ok(min_req)
    }

    fn from_json<E: ser::Serialize>(model: E) -> Result<Self::Body, Self::Err> {
        Ok(serde_json::to_vec(&model)?)
    }
}

pub struct Client {
    auth: Auth,
}

impl GitHubResponseExt for Response {
    fn is_success(&self) -> bool {
        300 > self.inner.status_code && self.inner.status_code >= 200
    }

    fn status_code(&self) -> u16 {
        self.inner.status_code.try_into().unwrap()
    }

    fn to_json<E: for<'de> Deserialize<'de> + std::fmt::Debug>(
        self,
    ) -> Result<E, serde_json::Error> {
        Ok(serde_json::from_slice(self.inner.as_bytes())?)
    }

    async fn to_json_async<E: for<'de> Deserialize<'de> + Unpin + std::fmt::Debug>(
        self,
    ) -> Result<E, serde_json::Error> {
        unimplemented!("curl adapter only has sync json conversion implemented");
    }
}

fn main() {}
