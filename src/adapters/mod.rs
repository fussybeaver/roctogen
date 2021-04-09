use serde::{ser, Deserialize};

use crate::auth::Auth;

#[cfg(feature = "isahc")]
pub mod isahc;

#[cfg(feature = "isahc")]
pub use self::isahc::AdapterError;

#[cfg(feature = "isahc")]
pub(crate) use {
    self::isahc::fetch, self::isahc::fetch_async, self::isahc::to_json, self::isahc::to_json_async,
};

#[cfg(target_arch = "wasm32")]
pub mod wasm;

#[cfg(target_arch = "wasm32")]
pub(crate) use {wasm::fetch_async, wasm::to_json_async};

#[cfg(target_arch = "wasm32")]
pub use wasm::AdapterError;

#[cfg(feature = "reqwest")]
pub mod reqwest;

#[cfg(feature = "reqwest")]
pub use self::reqwest::AdapterError;

#[cfg(feature = "reqwest")]
pub(crate) use {
    self::reqwest::fetch, self::reqwest::fetch_async, self::reqwest::to_json,
    self::reqwest::to_json_async,
};

#[cfg(feature = "ureq")]
pub mod ureq;

#[cfg(feature = "ureq")]
pub use self::ureq::AdapterError;

#[cfg(feature = "ureq")]
pub(crate) use {
    self::ureq::fetch, self::ureq::fetch_async, self::ureq::to_json, self::ureq::to_json_async,
};

#[cfg(all(
    not(feature = "isahc"),
    not(feature = "reqwest"),
    not(feature = "ureq"),
    not(target_arch = "wasm32")
))]
#[derive(thiserror::Error, Debug)]
pub enum AdapterError {}

#[cfg(all(
    not(feature = "isahc"),
    not(feature = "reqwest"),
    not(feature = "ureq"),
    not(target_arch = "wasm32")
))]
pub(crate) fn fetch(
    _request: http::Request<Vec<u8>>,
) -> Result<http::Response<Vec<u8>>, AdapterError> {
    unimplemented!("Use a client adapter feature, or target wasm");
}

#[cfg(all(
    not(feature = "isahc"),
    not(feature = "reqwest"),
    not(feature = "ureq"),
    not(target_arch = "wasm32")
))]
pub(crate) async fn fetch_async(
    _request: http::Request<Vec<u8>>,
) -> Result<http::Response<Vec<u8>>, AdapterError> {
    unimplemented!("Use a client adapter feature, or target wasm");
}

#[cfg(all(
    not(feature = "isahc"),
    not(feature = "reqwest"),
    not(feature = "ureq"),
    not(target_arch = "wasm32")
))]
pub(crate) fn to_json<E: for<'de> Deserialize<'de>>(
    _res: http::Response<Vec<u8>>,
) -> Result<E, AdapterError> {
    unimplemented!("Use a client adapter feature, or target wasm");
}

#[cfg(all(
    not(feature = "isahc"),
    not(feature = "reqwest"),
    not(feature = "ureq"),
    not(target_arch = "wasm32")
))]
pub(crate) async fn to_json_async<E: for<'de> Deserialize<'de>>(_res: http::Response<Vec<u8>>) -> Result<E, AdapterError> {
    unimplemented!("Use a client adapter feature, or target wasm");
}

#[cfg(all(
    not(feature = "isahc"),
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
}

#[cfg(all(
    not(feature = "isahc"),
    not(feature = "reqwest"),
    not(feature = "ureq"),
    not(target_arch = "wasm32")
))]
impl GitHubRequestBuilder<Vec<u8>> for http::Request<Vec<u8>> {
    fn build(_req: GitHubRequest<Vec<u8>>, _auth: &Auth) -> Result<Self, AdapterError> {
        unimplemented!("Use a client adapter feature, or target wasm");
    }
}

#[cfg(all(
    not(feature = "isahc"),
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

pub(crate) struct GitHubRequest<T> {
    pub uri: String,
    pub method: &'static str,
    pub body: Option<T>,
    pub headers: Vec<(&'static str, &'static str)>,
}

pub(crate) trait GitHubRequestBuilder<T>
where
    Self: Sized,
{
    fn build(req: GitHubRequest<T>, auth: &Auth) -> Result<Self, AdapterError>;
}

pub(crate) trait GitHubResponseExt {
    fn is_success(&self) -> bool;
    fn status_code(&self) -> u16;
}
