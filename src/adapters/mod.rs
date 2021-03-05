use http::header::AUTHORIZATION;
use http::request::Request;
use http::response::Response;

use serde::de::DeserializeOwned;
use serde::Deserialize;

use crate::auth::Auth;

#[cfg(feature = "isahc")]
pub mod isahc;

type Error = Box<dyn std::error::Error + 'static>;

#[cfg(not(feature = "isahc"))]
impl<T, A, E> RequestAdapter<T, E> for Request<A>
where
    T: for<'de> Deserialize<'de>,
    E: for<'de> Deserialize<'de> + std::error::Error,
{
    fn retrieve(self) -> Result<T, Error> {
        unimplemented!();
    }
}

pub trait RequestAdapter<T, E>
where
    T: for<'de> Deserialize<'de>,
    E: for<'de> Deserialize<'de> + std::error::Error,
{
    fn retrieve(self) -> Result<T, Error>;
}

pub trait StatusExt<T>
where Self: Sized, 
        T: std::io::Read,
{
        fn resolve(status: http::StatusCode, res: &mut Response<T>) -> Result<Self, serde_json::Error>;
    //type Error;
    //fn resolve(self, res: &mut Response<T>) -> Result<Self::Error, serde_json::Error>;
}

pub trait Json<A>
where
    A: for<'de> Deserialize<'de>,
{
    fn to_json(&mut self) -> Result<A, serde_json::Error>;
}

pub trait RequestBuilderExt {
    fn authenticate(self, auth: &Auth) -> Self;
}

impl RequestBuilderExt for http::request::Builder {
    fn authenticate(self, auth: &Auth) -> Self {
        match auth {
            Auth::Basic { user, pass } => {
                let creds = format!("{}:{}", user, pass);
                self.header(
                    AUTHORIZATION,
                    format!("Basic {}", base64::encode(creds.as_bytes())),
                )
            }
            Auth::OAuth { token } => self.header(AUTHORIZATION, format!("token {}", token)),
            Auth::None => self,
        }
    }
}
