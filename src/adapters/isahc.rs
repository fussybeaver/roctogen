use http::request::Request;
use http::response::Response;

use isahc::prelude::*;
use isahc::Body;
use isahc::RequestExt;

use super::{Json, RequestAdapter, StatusExt};
use serde::Deserialize;

type Error = Box<dyn std::error::Error + 'static>;

impl<T, A, E> RequestAdapter<T, E> for Request<A>
where
    A: Into<Body>,
    T: for<'de> Deserialize<'de>,
    E: for<'de> Deserialize<'de> + std::error::Error + 'static + StatusExt<Body>,
{
    fn retrieve(self) -> Result<T, Error> {
        let mut res = self.send()?;
        if res.status().is_success() {
            let json: T = res.json()?;
            Ok(json)
        } else {
            let status = res.status();
            //let json = status.resolve(&mut res)?;
            let json: E = StatusExt::resolve(status, &mut res)?;

            //let json: E = res.json()?;

            Err(Box::new(json))
        }
    }
}

impl<T, E> Json<E> for Response<T>
where
    T: std::io::Read,
    E: for<'de> Deserialize<'de>,
{
    fn to_json(&mut self) -> Result<E, serde_json::Error> {
        self.json()
    }
}
