#![allow(
    missing_docs,
    trivial_casts,
    unused_variables,
    unused_mut,
    unused_imports,
    unused_extern_crates,
    non_camel_case_types
)]
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;

use std::io::Error;

#[allow(unused_imports)]
use std::collections::HashMap;

pub mod adapters;
//pub mod api_impl;
pub mod endpoints;
pub mod models;

pub mod auth {
    pub enum Auth {
        Basic { user: String, pass: String },
        OAuth { token: String },
        None,
    }
}

//pub use api_impl as api;
pub use endpoints as api;
