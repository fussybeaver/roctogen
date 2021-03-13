use serde::{Deserialize, ser};

use crate::auth::Auth;

#[cfg(feature = "isahc")]
pub mod isahc;

#[cfg(feature = "isahc")]
pub use self::isahc::AdapterError;

#[cfg(feature = "isahc")]
pub(crate) use {
    self::isahc::fetch,
    self::isahc::fetch_async,
    self::isahc::FromJsonType,
};

#[cfg(target_arch = "wasm32")]
pub mod wasm;

#[cfg(target_arch = "wasm32")]
pub(crate) use {
    wasm::fetch_async,
    wasm::FromJsonType,
};

#[cfg(target_arch = "wasm32")]
pub use wasm::AdapterError;

#[cfg(all(not(feature = "isahc"), not(target_arch = "wasm32")))]
#[derive(thiserror::Error, Debug)]
pub enum AdapterError {}

#[cfg(all(not(feature = "isahc"), not(target_arch = "wasm32")))]
pub(crate) fn fetch(_request: http::Request<Vec<u8>>) -> Result<http::Response<Vec<u8>>, AdapterError> {
    unimplemented!("Use a client adapter feature, or target wasm");
}

#[cfg(all(not(feature = "isahc"), not(target_arch = "wasm32")))]
pub(crate) async fn fetch_async(_request: http::Request<Vec<u8>>) -> Result<http::Response<Vec<u8>>, AdapterError> {
    unimplemented!("Use a client adapter feature, or target wasm");
}

#[cfg(all(not(feature = "isahc"), not(target_arch = "wasm32")))]
pub(crate) type FromJsonType = Vec<u8>;

#[cfg(all(not(feature = "isahc"), not(target_arch = "wasm32")))]
impl GitHubResponseExt for http::Response<Vec<u8>> {
    fn is_success(&self) -> bool {
        unimplemented!("Use a client adapter feature, or target wasm");
    }

    fn status_code(&self) -> u16 {
        unimplemented!("Use a client adapter feature, or target wasm");
    }
}

#[cfg(all(not(feature = "isahc"), not(target_arch = "wasm32")))]
impl<E> ToJson<E> for http::Response<Vec<u8>>
where
    E: for<'de> Deserialize<'de>,
{
    fn to_json(self) -> Result<E, serde_json::Error> {
        unimplemented!("Use a client adapter feature, or target wasm");
    }
}

#[cfg(all(not(feature = "isahc"), not(target_arch = "wasm32")))]
impl GitHubRequestBuilder for http::Request<Vec<u8>> 
{
    fn build(_req: GitHubRequest, _auth: &Auth) -> Result<Self, AdapterError> {
        unimplemented!("Use a client adapter feature, or target wasm");
    }
}

#[cfg(all(not(feature = "isahc"), not(target_arch = "wasm32")))]
impl<E> FromJson<E> for E
where
    E: ser::Serialize + std::fmt::Debug,
{
    fn from_json(_model: E) -> Result<FromJsonType, serde_json::Error> {

        unimplemented!("Use a client adapter feature, or target wasm");
    }
}


pub(crate) trait ToJson<A>
where
    A: for<'de> Deserialize<'de>,
{
    fn to_json(self) -> Result<A, serde_json::Error>;
}

pub(crate) trait FromJson<A>
where
    A: ser::Serialize
{
    fn from_json(model: A) -> Result<FromJsonType, serde_json::Error>;
}

pub(crate) struct GitHubRequest {
    pub uri: String,
    pub method: &'static str,
    pub body: Option<FromJsonType>,
    pub headers: Vec<(&'static str, &'static str)>,
}

pub(crate) trait GitHubRequestBuilder
where
    Self: Sized,
{
    fn build(req: GitHubRequest, auth: &Auth) -> Result<Self, AdapterError>;
}

pub(crate) trait GitHubResponseExt {
    fn is_success(&self) -> bool;
    fn status_code(&self) -> u16;
}

