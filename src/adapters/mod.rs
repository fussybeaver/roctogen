use http::{Request, Response};
use serde::{ser, Deserialize};

use crate::auth::Auth;

#[cfg(target_arch = "wasm32")]
pub mod wasm;

#[cfg(target_arch = "wasm32")]
pub type Req = web_sys::Request;

#[cfg(target_arch = "wasm32")]
pub use wasm::AdapterError;

#[cfg(feature = "reqwest")]
pub mod reqwest;

#[cfg(feature = "reqwest")]
pub use self::reqwest::AdapterError;

#[cfg(feature = "reqwest")]
pub type Req = ::reqwest::Request;

#[cfg(feature = "ureq")]
pub mod ureq;

#[cfg(feature = "ureq")]
pub use self::ureq::AdapterError;

#[cfg(feature = "ureq")]
pub type Req = self::ureq::RequestWithBody;

#[cfg(all(
    not(feature = "reqwest"),
    not(feature = "ureq"),
    not(target_arch = "wasm32")
))]
#[derive(thiserror::Error, Debug)]
pub enum AdapterError {}

#[cfg(all(
    not(feature = "reqwest"),
    not(feature = "ureq"),
    not(target_arch = "wasm32")
))]
pub type Req = http::Request<Vec<u8>>;

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
    fn to_json<E: for<'de> Deserialize<'de> + std::fmt::Debug>(self) -> Result<E, AdapterError> {
        unimplemented!("Use a client adapter feature, or target wasm");
    }

    async fn to_json_async<E: for<'de> Deserialize<'de> + Unpin + std::fmt::Debug>(
        self,
    ) -> Result<E, AdapterError> {
        unimplemented!("Use a client adapter feature, or target wasm");
    }
}

#[cfg(all(
    not(feature = "reqwest"),
    not(feature = "ureq"),
    not(target_arch = "wasm32")
))]
impl<C: Client> GitHubRequestBuilder<Vec<u8>, C> for http::Request<Vec<u8>> {
    fn build(_req: GitHubRequest<Vec<u8>>, _client: &C) -> Result<Self, AdapterError> {
        unimplemented!("Use a client adapter feature, or target wasm");
    }
}

#[cfg(all(
    not(feature = "reqwest"),
    not(feature = "ureq"),
    not(target_arch = "wasm32")
))]
impl<E, T> FromJson<E, T> for E
where
    E: ser::Serialize + std::fmt::Debug,
{
    fn from_json(_model: E) -> Result<T, serde_json::Error> {
        unimplemented!("Use a client adapter feature, or target wasm");
    }
}

pub(crate) trait FromJson<A, T>
where
    A: ser::Serialize,
{
    fn from_json(model: A) -> Result<T, serde_json::Error>;
}

#[allow(dead_code)]
pub(crate) struct GitHubRequest<T> {
    pub uri: String,
    pub method: &'static str,
    pub body: Option<T>,
    pub headers: Vec<(&'static str, &'static str)>,
}

pub(crate) trait GitHubRequestBuilder<T, C: Client>
where
    Self: Sized,
{
    fn build(req: GitHubRequest<T>, client: &C) -> Result<Self, AdapterError>;
}

pub(crate) trait GitHubResponseExt {
    fn is_success(&self) -> bool;
    fn status_code(&self) -> u16;
    fn to_json<E: for<'de> Deserialize<'de> + std::fmt::Debug>(self) -> Result<E, AdapterError>;
    #[allow(refining_impl_trait)]
    async fn to_json_async<E: for<'de> Deserialize<'de> + Unpin + std::fmt::Debug>(
        self,
    ) -> Result<E, AdapterError>;
}

pub trait Client {
    type Req;

    fn new(auth: &Auth) -> Self
    where
        Self: Sized;
    fn get_auth(&self) -> &Auth;
    fn fetch(&self, req: Self::Req) -> Result<impl GitHubResponseExt, AdapterError>;
    #[allow(async_fn_in_trait)]
    async fn fetch_async(&self, req: Self::Req) -> Result<impl GitHubResponseExt, AdapterError>;
}

#[cfg(target_arch = "wasm32")]
pub fn client(auth: &Auth) -> impl Client<Req = Req> {
    wasm::Client::new(auth)
}
#[cfg(feature = "ureq")]
pub fn client(auth: &Auth) -> impl Client<Req = Req> {
    ureq::Client::new(auth)
}
#[cfg(feature = "reqwest")]
pub fn client(auth: &Auth) -> impl Client<Req = Req> {
    reqwest::Client::new(auth)
}
