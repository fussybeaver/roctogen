use super::{GitHubRequest, GitHubResponseExt};
use crate::auth::Auth;
use base64::{prelude::BASE64_STANDARD, Engine};

use chrono::{DateTime, Duration, Utc};
use js_sys::{Object, Promise, Reflect};
use log::debug;
use serde::{ser, Deserialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, Response, ServiceWorkerGlobalScope, Window};

trait WasmScope {
    fn fetch(&self, request: &Request) -> Promise;
}

impl WasmScope for Window {
    fn fetch(&self, request: &Request) -> Promise {
        self.fetch_with_request(request)
    }
}

impl WasmScope for ServiceWorkerGlobalScope {
    fn fetch(&self, request: &Request) -> Promise {
        self.fetch_with_request(request)
    }
}

fn scope() -> Result<Box<dyn WasmScope>, AdapterError> {
    let global = js_sys::global();
    if Reflect::has(&global, &JsValue::from_str("document"))? {
        debug!("Found document, using Window for scope");
        Ok(Box::new(global.dyn_into::<Window>()?))
    } else {
        debug!("Using ServiceWorkerGlobalScope for scope");
        Ok(Box::new(global.dyn_into::<ServiceWorkerGlobalScope>()?))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum AdapterError {
    #[error(transparent)]
    Value(JsValueError),
    #[error(transparent)]
    Object(ObjectError),
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
    #[error(transparent)]
    IOError(#[from] std::io::Error),
    #[error("Wasm adapter only has async fetch implemented")]
    UnimplementedSync,
}

impl From<AdapterError> for crate::adapters::AdapterError {
    fn from(err: AdapterError) -> Self {
        Self::Client {
            description: err.to_string(),
            source: Some(Box::new(err)),
        }
    }
}

#[derive(thiserror::Error, Debug)]
#[error("{msg}")]
pub struct JsValueError {
    msg: String,
    origin: JsValue,
}

#[derive(thiserror::Error, Debug)]
#[error("{msg}")]
pub struct ObjectError {
    msg: String,
    origin: Object,
}

impl super::Client for Client {
    type Req = web_sys::Request;
    type Body = JsValue;
    type Err = AdapterError where crate::adapters::AdapterError: From<Self::Err>;

    fn new(auth: &Auth) -> Result<Self, Self::Err> {
        Ok(Self {
            auth: auth.to_owned(),
            scope: scope()?,
        })
    }

    fn fetch(&self, _req: Self::Req) -> Result<impl GitHubResponseExt, Self::Err> {
        Err::<GitHubResponse, _>(AdapterError::UnimplementedSync)
    }

    async fn fetch_async(&self, request: Self::Req) -> Result<impl GitHubResponseExt, Self::Err> {
        let resp_value = JsFuture::from(self.scope.fetch(&request)).await?;

        debug!("Response: {:?}", &resp_value);

        let resp: Response = resp_value.dyn_into()?;
        let json: JsValue = JsFuture::from(resp.json()?).await?;

        debug!("Body: {:?}", &json);

        Ok(GitHubResponse { json, resp })
    }

fn build(&self, req: GitHubRequest<Self::Body>) -> Result<Self::Req, Self::Err> {
        let opts = RequestInit::new();
        opts.set_method(&req.method);
        if let Some(body) = req.body {
            debug!("Adding request body: {:?}", &body);

            opts.set_body(&js_sys::JSON::stringify(&body)?.into());
        }

        let request = Request::new_with_str_and_init(&req.uri, &opts)?;
        let headers = request.headers();
        headers.set("Accept", "application/vnd.github.v3+json")?;
        headers.set("Content-Type", "application/json")?;
        headers.set("User-Agent", "roctogen")?;

        for header in req.headers.iter() {
            headers.set(header.0, header.1)?;
        }

        match &self.auth {
            Auth::Basic { user, pass } => {
                let creds = format!("{}:{}", user, pass);
                headers.set(
                    "Authorization",
                    &format!("Basic {}", BASE64_STANDARD.encode(creds.as_bytes())),
                )?;
            }
            Auth::Token(token) => headers.set("Authorization", &format!("token {}", token))?,
            Auth::Bearer(bearer) => headers.set("Authorization", &format!("Bearer {}", bearer))?,
            Auth::None => (),
        }

        debug!("Built request object: {:?}", &request);

        Ok(request)
    }

    fn from_json<E: ser::Serialize>(model: E) -> Result<Self::Body, Self::Err> {
        Ok(JsValue::from_serde(&model)?)
        }
}

pub struct Client {
    auth: Auth,
    scope: Box<dyn WasmScope>,
}

impl From<JsValue> for AdapterError {
    fn from(js_value: JsValue) -> Self {
        AdapterError::Value(JsValueError {
            msg: format!("{:#?}", js_value),
            origin: js_value,
        })
    }
}

impl From<Object> for AdapterError {
    fn from(js_obj: Object) -> Self {
        AdapterError::Object(ObjectError {
            msg: format!("{:#?}", js_obj),
            origin: js_obj,
        })
    }
}

pub(crate) struct GitHubResponse {
    pub json: JsValue,
    pub resp: Response,
}

impl GitHubResponseExt for GitHubResponse {
    fn is_success(&self) -> bool {
        self.resp.ok()
    }

    fn status_code(&self) -> u16 {
        self.resp.status()
    }
    fn to_json<E: for<'de> Deserialize<'de> + std::fmt::Debug>(self) -> Result<E, serde_json::Error> {
        unimplemented!("Reqwest adapter only has async json conversion implemented");
    }

    async fn to_json_async<E: for<'de> Deserialize<'de> + Unpin + std::fmt::Debug>(
        self,
    ) -> Result<E, serde_json::Error> {
        Ok(self.json.into_serde()?)
    }
}
