use http::request::Request;

use isahc::prelude::*;
use isahc::Body;
use isahc::RequestExt;

use super::RequestAdapter;
use serde::Deserialize;

type Error = Box<dyn std::error::Error + 'static>;

impl<T: for<'de> Deserialize<'de>, A: Into<Body>> RequestAdapter<T> for Request<A> {
    fn retrieve(self) -> Result<T, Error> {
        let mut res = self.send()?;
        Ok(res.json::<T>()?)
    }
}
