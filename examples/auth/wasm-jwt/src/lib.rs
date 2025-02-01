#![cfg(target_arch = "wasm32")]

mod utils;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use roctogen::api::apps;
use roctogen::models;
use roctokit::adapters::{wasm, Client};
use roctokit::auth::Auth;

use log::{error, info, Level};

use roctogen_common::jwt::{create_claim, create_token};

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

#[wasm_bindgen]
extern "C" {
    type Error;

    #[wasm_bindgen(constructor)]
    fn new(message: &str) -> Error;
}

#[wasm_bindgen]
pub async fn run(jwt_js: JsValue, app_id_js: JsValue) -> Result<JsValue, JsValue> {
    init_log();

    info!("starting....");

    let jwt: String = jwt_js.dyn_into::<js_sys::JsString>()?.into();
    let app_id: f64 = js_sys::Number::from(app_id_js).value_of();

    let claims = create_claim(app_id.round() as u64);

    let token = create_token(claims, &jwt).await?;

    let auth = Auth::Bearer(token);

    let client = wasm::Client::new(&auth).expect("Cannot build client");
    let apps = apps::new(&client);

    let req = apps
        .list_installations_async(None::<apps::AppsListInstallationsParams>)
        .await;
    let installation_id = match req {
        Ok(ref installations) => {
            info!("installations: {:?}", installations);

            // Just use the first one...
            if let Some(models::Installation { id: Some(id), .. }) = installations.get(0) {
                id
            } else {
                return Err(Error::new("Installation id not found").into());
            }
        }
        Err(ref e) => {
            error!("error fetching installations: {}", e);
            return Err(Error::new(&e.to_string()).into());
        }
    };

    let post_apps = models::PostAppsCreateInstallationAccessToken::default();
    let req = apps
        .create_installation_access_token_async(*installation_id as i32, post_apps)
        .await;
    match req {
        Ok(ref installation_token) => {
            info!("ok: {:?}", installation_token);

            Ok(installation_token.token.as_ref().unwrap().into())
        }
        Err(ref e) => {
            error!("err: {}", e);
            return Err(Error::new(&e.to_string()).into());
        }
    }
}
