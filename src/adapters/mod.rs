use http::request::Request;

use serde::Deserialize;

#[cfg(feature = "isahc")]
pub mod isahc;

type Error = Box<dyn std::error::Error + 'static>;

#[cfg(not(feature = "isahc"))]
impl<T: for<'de> Deserialize<'de>, A> RequestAdapter<T> for Request<A> {
    fn retrieve(self) -> Result<T, Error> {
        unimplemented!();
    }
}

pub trait RequestAdapter<T> {
    fn retrieve(self) -> Result<T, Error>;
}
