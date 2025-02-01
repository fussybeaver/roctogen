use http::{Request, Response};
use serde::{ser, Deserialize};

use crate::auth::Auth;

#[cfg(target_arch = "wasm32")]
pub mod wasm;

#[cfg(feature = "reqwest")]
pub mod reqwest;

#[cfg(feature = "ureq")]
pub mod ureq;

#[derive(thiserror::Error, Debug)]
pub enum AdapterError {
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
    #[error(transparent)]
    SerdeUrl(#[from] serde_urlencoded::ser::Error),
    #[error("client error: {description}")]
    Client {
        description: String,
        #[source]
        source: Option<Box<dyn std::error::Error>>,
    },
    #[error("endpoint error: {description}")]
    Endpoint {
        description: String,
        status_code: u16,
        #[source]
        source: Option<Box<dyn std::error::Error>>,
    },
}


#[cfg(all(
    not(feature = "reqwest"),
    not(feature = "ureq"),
    not(target_arch = "wasm32")
))]
impl GitHubResponseExt for http::Response<Vec<u8>> {
    fn is_success(&self) -> bool {
        unimplemented!("Use a client adapter feature, or target wasm");
    }

    fn status_code(&self) -> u16 {
        unimplemented!("Use a client adapter feature, or target wasm");
    }
    #[allow(refining_impl_trait)]
    fn to_json<E: for<'de> Deserialize<'de> + std::fmt::Debug>(self) -> Result<E, serde_json::Error> {
        unimplemented!("Use a client adapter feature, or target wasm");
    }

    async fn to_json_async<E: for<'de> Deserialize<'de> + Unpin + std::fmt::Debug>(
        self,
    ) -> Result<E, serde_json::Error> {
        unimplemented!("Use a client adapter feature, or target wasm");
    }
}

#[allow(dead_code)]
pub struct GitHubRequest<'a, T> {
    pub uri: String,
    pub method: &'a str,
    pub body: Option<T>,
    pub headers: Vec<(&'a str, &'a str)>,
}

pub trait GitHubResponseExt {
    fn is_success(&self) -> bool;
    fn status_code(&self) -> u16;
    fn to_json<E: for<'de> Deserialize<'de> + std::fmt::Debug>(
        self,
    ) -> Result<E, serde_json::Error>;
    #[allow(async_fn_in_trait)]
    async fn to_json_async<E: for<'de> Deserialize<'de> + Unpin + std::fmt::Debug>(
        self,
    ) -> Result<E, serde_json::Error>;
}

pub trait Client
where
    AdapterError: From<Self::Err>,
{
    type Req;
    type Body;
    type Err;

    fn new(auth: &Auth) -> Result<Self, Self::Err>
    where
        Self: Sized;
    fn build(&self, req: GitHubRequest<Self::Body>) -> Result<Self::Req, Self::Err>;
    fn fetch(&self, req: Self::Req) -> Result<impl GitHubResponseExt, Self::Err>;
    #[allow(async_fn_in_trait)]
    async fn fetch_async(&self, req: Self::Req) -> Result<impl GitHubResponseExt, Self::Err>;
    fn from_json<A: ser::Serialize>(model: A) -> Result<Self::Body, Self::Err>;
}

#[cfg(target_arch = "wasm32")]
pub fn client(auth: &Auth) -> Result<wasm::Client, wasm::AdapterError> {
    wasm::Client::new(auth)
}
#[cfg(feature = "ureq")]
pub fn client(auth: &Auth) -> Result<ureq::Client, ureq::AdapterError> {
    ureq::Client::new(auth)
}
#[cfg(feature = "reqwest")]
pub fn client(auth: &Auth) -> Result<reqwest::Client, reqwest::AdapterError> {
    reqwest::Client::new(auth)
}
