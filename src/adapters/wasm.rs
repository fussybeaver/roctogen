use crate::auth::Auth;
use super::{FromJson, GitHubRequest, GitHubRequestBuilder, GitHubResponseExt, ToJson};

use chrono::{DateTime, Duration, Utc};
use log::debug;
use js_sys::{Object, Promise, Reflect};
use serde::{Deserialize, ser};
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
}

#[derive(thiserror::Error, Debug)]
#[error("{msg}")]
pub struct JsValueError {
    msg: String,
    origin: JsValue
}

#[derive(thiserror::Error, Debug)]
#[error("{msg}")]
pub struct ObjectError {
    msg: String,
    origin: Object
}

impl From<JsValue> for AdapterError {
    fn from(js_value: JsValue) -> Self {
        AdapterError::Value(JsValueError {
            msg: format!("{:#?}", js_value),
            origin: js_value
        })
    }
}

impl From<Object> for AdapterError {
    fn from(js_obj: Object) -> Self {
        AdapterError::Object(ObjectError {
            msg: format!("{:#?}", js_obj),
            origin: js_obj
        })
    }
}

pub(crate) async fn fetch_async(request: Request) -> Result<GitHubResponse, AdapterError> {
    let scope = scope()?;

    let resp_value = JsFuture::from(scope.fetch(&request)).await?;
    
    debug!("Response: {:?}", &resp_value);

    let resp: Response = resp_value.dyn_into()?;
    let json: JsValue = JsFuture::from(resp.json()?).await?;

    debug!("Body: {:?}", &json); 

    Ok(GitHubResponse {
        json,
        resp
    })
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
}

impl<E> ToJson<E> for GitHubResponse
where
    E: for<'de> Deserialize<'de>,
{
    fn to_json(self) -> Result<E, serde_json::Error> {
        self.json.into_serde()
    }
}

pub(crate) type FromJsonType = JsValue;

impl<E> FromJson<E> for E
where
    E: ser::Serialize,
{
    fn from_json(model: E) -> Result<FromJsonType, serde_json::Error> {
        JsValue::from_serde(&model)
    }
}

#[derive(Debug, Serialize)]
struct JWTClaim<'claim> {
    iat: DateTime<Utc>,
    exp: DateTime<Utc>,
    iss: &'claim str
}

impl GitHubRequestBuilder for Request
{
    fn build(req: GitHubRequest, auth: &Auth) -> Result<Self, AdapterError> {

        let mut opts = RequestInit::new();
        opts.method(&req.method);
        if let Some(body) = req.body {
            opts.body(Some(&body));
        }

        let request = Request::new_with_str_and_init(&req.uri, &opts)?;
        let headers = request.headers();
        headers.set("Accept", "application/vnd.github.v3+json")?;
        headers.set("Content-Type", "application/json")?;
        headers.set("User-Agent", "roctogen")?;

        for header in req.headers.iter() {
            headers.set(header.0, header.1)?;
        }

        match auth {
            Auth::Basic { user, pass } => {
                let creds = format!("{}:{}", user, pass);
                headers.set(
                    "Authorization",
                    &format!("Basic {}", base64::encode(creds.as_bytes())),
                )?;
            }
            Auth::Token(token) => headers.set("Authorization", &format!("token {}", token))?,
            Auth::Bearer(bearer) => headers.set("Authorization", &format!("Bearer {}", bearer))?,
            Auth::None => (),
        }


        debug!("Built request object: {:?}", &request);


        Ok(request)
    }
}
