use serde::de::DeserializeOwned;
use serde::Deserialize;

use crate::auth::Auth;

#[cfg(feature = "isahc")]
pub mod isahc;

#[cfg(feature = "isahc")]
pub use self::isahc::AdapterError;

#[cfg(feature = "isahc")]
pub(crate) use {
    self::isahc::fetch,
    self::isahc::fetch_async
};

#[cfg(target_arch = "wasm32")]
pub mod wasm;

#[cfg(target_arch = "wasm32")]
pub(crate) use wasm::fetch_async;

#[cfg(target_arch = "wasm32")]
pub use wasm::AdapterError;

pub(crate) trait Json<A>
where
    A: for<'de> Deserialize<'de>,
{
    fn to_json(self) -> Result<A, serde_json::Error>;
}

pub(crate) struct GitHubRequest {
    pub uri: String,
    pub method: &'static str,
    pub body: (),
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

