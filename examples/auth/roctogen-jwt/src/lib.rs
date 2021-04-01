extern crate cfg_if;
extern crate wasm_bindgen;

mod utils;

use cfg_if::cfg_if;
use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::ServiceWorkerGlobalScope;

use roctogen::api::apps;
use roctogen::auth::Auth;

use log::{debug, info, Level, error};
use serde::{Deserialize, Serialize};

cfg_if! {
    if #[cfg(feature = "debug")] {
        fn init_log() {
            console_log::init_with_level(Level::Debug).expect("error initializing logger");
        }
    } else {
        fn init_log() {
            console_log::init_with_level(Level::Info).expect("error initializing logger");
        }
    }
}

#[derive(Debug, Serialize)]
struct JwtClaims {
    iat: i64,
    exp: i64,
    // GitHub App's identifier number
    iss: u64,
}

#[wasm_bindgen]
extern "C" {
    type Error;

    #[wasm_bindgen(constructor)]
    fn new(message: &str) -> Error;
}

// Converts type to a JS `Error`.
fn to_js_error(e: impl std::fmt::Display) -> Error {
    Error::new(&e.to_string())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CompleteHeader {
    alg: &'static str,
    typ: &'static str,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Algorithm<'a> {
    name: &'a str,
    hash: AlgoHash<'a>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct AlgoHash<'a> {
    name: &'a str
}


/// Create a JWT token, using a claim based on an existing GitHub App and the associated RSA pkcs8
/// private key. This method uses the crypto subtle API exposed by the underlying webassembly
/// engine, and currently only runs in a worker scope.
async fn create_token(claims: JwtClaims, private_key_pem: &str) -> Result<String, JsValue>
{

    let global = js_sys::global();
    let worker = global.dyn_into::<ServiceWorkerGlobalScope>()?;

    let private_key = pem::parse(private_key_pem).map_err(to_js_error)?.contents;
    let crypto_subtle = worker.crypto()?.subtle();

    let key_usages = js_sys::Array::new_with_length(1);
    key_usages.set(0, "sign".into());
    let priv_key_arr = unsafe{ Uint8Array::new(&Uint8Array::view(&private_key)) };

    let algo = Algorithm {
        name: "RSASSA-PKCS1-v1_5",
        hash: AlgoHash {
            name: "SHA-256"
        }
    };

    let js_algo: js_sys::Object = JsValue::from_serde(&algo).map_err(to_js_error)?.into();

    debug!("{:?}", &js_algo);

    let promise = crypto_subtle.import_key_with_object("pkcs8", &priv_key_arr, &js_algo, false, &key_usages)?;
    let private_key = JsFuture::from(promise).await?;
    let crypto_key: web_sys::CryptoKey = private_key.dyn_into()?;

    let complete_header = CompleteHeader {
        alg: "RS256",
        typ: "JWT",
    };

    let header = serde_json::to_string(&complete_header).map_err(to_js_error)?;
    let mut buffer = base64::encode_config(&header, base64::URL_SAFE_NO_PAD);

    buffer.push('.');
    let claims = serde_json::to_string(&claims).map_err(to_js_error)?;
    base64::encode_config_buf(&claims, base64::URL_SAFE_NO_PAD, &mut buffer);

    let promise = crypto_subtle.sign_with_object_and_u8_array(&js_algo, &crypto_key, unsafe { buffer.as_bytes_mut() })?;
    let signature_arr_buf = JsFuture::from(promise).await?;

    let sig_uint_arr = js_sys::Uint8Array::new(&signature_arr_buf).to_vec();

    buffer.push('.');
    base64::encode_config_buf(&sig_uint_arr, base64::URL_SAFE_NO_PAD, &mut buffer); 

    Ok(buffer)
}

#[wasm_bindgen]
pub async fn run(jwt_js: JsValue, app_id_js: JsValue) -> Result<JsValue, JsValue> {
    init_log();

    info!("starting....");

    let jwt: String = jwt_js.dyn_into::<js_sys::JsString>()?.into();
    let app_id: f64 = js_sys::Number::from(app_id_js).value_of();

    let claims = JwtClaims {
        iat: (chrono::Utc::now() - chrono::Duration::seconds(60)).timestamp(),
        exp: (chrono::Utc::now() + chrono::Duration::minutes(10)).timestamp(),
        iss: app_id.round() as u64
    };

    debug!("claims: {:?}", &claims);

    let token = create_token(claims, &jwt).await?;

    let auth = Auth::Bearer(token);

    let req = apps::new(&auth).get_authenticated_async().await;
    match req {
        Ok(ref plans) => {
            info!("ok: {:?}", plans);
            Ok(JsValue::from_serde(&plans).map_err(to_js_error)?)
        }
        Err(ref e) => {
            error!("err: {}", e);
            Ok(JsValue::NULL)
        },
    }
}
