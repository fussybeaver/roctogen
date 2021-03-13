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
pub(crate) fn fetch<T, Y>(request: http::Request<T>) -> Result<http::Response<Y>, AdapterError> {
    unimplemented!();
}

#[cfg(all(not(feature = "isahc"), not(target_arch = "wasm32")))]
pub(crate) async fn fetch_async<T, Y>(request: http::Request<T>) -> Result<http::Response<Y>, AdapterError> {
    unimplemented!();
}

#[cfg(all(not(feature = "isahc"), not(target_arch = "wasm32")))]
pub(crate) type FromJsonType = Vec<u8>;

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

