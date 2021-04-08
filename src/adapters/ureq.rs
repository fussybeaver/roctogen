use http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE, USER_AGENT};

use ureq::{Request, Response};

use log::debug;

use crate::auth::Auth;
use super::{FromJson, GitHubRequest, GitHubRequestBuilder, GitHubResponseExt};

use serde::{ser, Deserialize};
use serde_json::value::Value;

pub(crate) struct RequestWithBody {
    pub(crate) req: Request,
    pub(crate) body: Option<Value>
}

#[derive(thiserror::Error, Debug)]
pub enum AdapterError {
    #[error(transparent)]
    Http(#[from] http::Error),
    #[error(transparent)]
    Ureq(#[from] ureq::Error),
    #[error(transparent)]
    IOError(#[from] std::io::Error),
}

pub(crate) fn fetch(request: RequestWithBody) -> Result<Response, AdapterError> {
    match if let Some(body) = request.body {
        request.req.send_json(body) 
    } else {
        request.req.call()
    } {
        Ok(res) => Ok(res),
        Err(ureq::Error::Status(_, res)) => Ok(res),
        Err(ureq::Error::Transport(transport)) => {
            let err: ureq::Error = transport.into();
            Err(err.into())
        }
    }
}

pub(crate) async fn fetch_async(_request: RequestWithBody) -> Result<Response, AdapterError> {
    unimplemented!("Ureq adapter only has sync fetch implemented")
}

impl GitHubResponseExt for Response {
    fn is_success(&self) -> bool {
        300 > self.status() && self.status() >= 200
    }

    fn status_code(&self) -> u16 {
        self.status()
    }
}

pub(crate) fn to_json<E: for<'de> Deserialize<'de> + std::fmt::Debug>(res: Response) -> Result<E, AdapterError> {
    let json = res.into_json()?;

    debug!("Response: {:?}", &json);

    Ok(json)
}

pub(crate) async fn to_json_async<E: for<'de> Deserialize<'de> + Unpin + std::fmt::Debug>(_res: Response) -> Result<E, AdapterError> {
    unimplemented!("Ureq adapter only has sync json conversion implemented");
}

impl<E> FromJson<E, Value> for E
where
    E: ser::Serialize + std::fmt::Debug,
{
    fn from_json(model: E) -> Result<Value, serde_json::Error> {

        debug!("Error: {:?}", model);

        Ok(serde_json::to_value(&model)?.into())
    }
}

impl GitHubRequestBuilder<Value> for RequestWithBody
{
    fn build(req: GitHubRequest<Value>, auth: &Auth) -> Result<Self, AdapterError> {
        let mut builder = ureq::request(req.method, &req.uri);

        builder = builder
            .set(ACCEPT.as_str(), "application/vnd.github.v3+json")
            .set(USER_AGENT.as_str(), "roctogen")
            .set(CONTENT_TYPE.as_str(), "application/json");
        
        for header in req.headers.iter() {
            builder = builder.set(header.0, header.1);
        }

        builder = match auth {
            Auth::Basic { user, pass } => {
                let creds = format!("{}:{}", user, pass);
                builder.set(
                    AUTHORIZATION.as_str(),
                    &format!("Basic {}", base64::encode(creds.as_bytes())),
                )
            }
            Auth::Token(token) => builder.set(AUTHORIZATION.as_str(), &format!("token {}", token)),
            Auth::Bearer(bearer) => builder.set(AUTHORIZATION.as_str(), &format!("Bearer {}", bearer)),
            Auth::None => builder,
        };

        Ok(RequestWithBody { req: builder, body: req.body })
    }
}
