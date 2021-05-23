//! [![license](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
//! [![docs](https://docs.rs/roctogen/badge.svg)](https://docs.rs/roctogen/)
//! [![GitHub workflow](https://github.com/github/docs/actions/workflows/default.yml/badge.svg)](https://github.com/fussybeaver/roctogen/actions/workflows/default.yml)
//!
//! # Roctogen: a rust client library for the GitHub v3 API  
//!
//! This client API is generated from the [upstream OpenAPI
//! specification](https://github.com/github/rest-api-description/). The library currently supports
//! webassembly and both tokio and non-tokio based asynchronous requests and minimal dependency blocking 
//! synchronous requests with a choice of different clients, enabled through cargo features:
//!
//!   - `isahc` feature (*sync* and non-tokio based *async*): [Isahc HTTP client](https://github.com/sagebind/isahc)
//!   - `reqwest` feature (*async*) [Reqwest client](https://github.com/seanmonstar/reqwest)
//!   - `ureq` feature (*sync*) [Ureq client](https://github.com/algesten/ureq) 
//!
//! # Install
//!
//! Add the following to your `Cargo.toml` file
//!
//! ```nocompile
//! [dependencies]
//! roctogen = "0.6"
//! ```
//!
//! # API
//! ## Documentation
//!
//! [API docs](https://docs.rs/roctogen/latest).
//! 
//! [Endpoints](https://docs.rs/roctogen/latest/roctogen/endpoints/index.html).
//!
//! Supported endpoints:
//! ---
//! 
//!   - [Meta](https://docs.rs/roctogen/latest/roctogen/endpoints/meta/struct.Meta.html)
//!   - [Issues](https://docs.rs/roctogen/latest/roctogen/endpoints/issues/struct.Issues.html)
//!   - [Licenses](https://docs.rs/roctogen/latest/roctogen/endpoints/licenses/struct.Licenses.html)
//!   - [Reactions](https://docs.rs/roctogen/latest/roctogen/endpoints/reactions/struct.Reactions.html)
//!   - [Activity](https://docs.rs/roctogen/latest/roctogen/endpoints/activity/struct.Activity.html)
//!   - [Projects](https://docs.rs/roctogen/latest/roctogen/endpoints/projects/struct.Projects.html)
//!   - [Orgs](https://docs.rs/roctogen/latest/roctogen/endpoints/orgs/struct.Orgs.html)
//!   - [Users](https://docs.rs/roctogen/latest/roctogen/endpoints/users/struct.Users.html)
//!   - [Apps](https://docs.rs/roctogen/latest/roctogen/endpoints/apps/struct.Apps.html)
//!   - [RateLimit](https://docs.rs/roctogen/latest/roctogen/endpoints/rate_limit/struct.RateLimit.html)
//!   - [Repos](https://docs.rs/roctogen/latest/roctogen/endpoints/repos/struct.Repos.html)
//!   - [SecretScanning](https://docs.rs/roctogen/latest/roctogen/endpoints/secret_scanning/struct.SecretScanning.html)
//!   - [Packages](https://docs.rs/roctogen/latest/roctogen/endpoints/packages/struct.Packages.html)
//!   - [Search](https://docs.rs/roctogen/latest/roctogen/endpoints/search/struct.Search.html)
//!   - [Teams](https://docs.rs/roctogen/latest/roctogen/endpoints/teams/struct.Teams.html)
//!   - [Markdown](https://docs.rs/roctogen/latest/roctogen/endpoints/markdown/struct.Markdown.html)
//!   - [OauthAuthorizations](https://docs.rs/roctogen/latest/roctogen/endpoints/oauth_authorizations/struct.OauthAuthorizations.html)
//!   - [Actions](https://docs.rs/roctogen/latest/roctogen/endpoints/actions/struct.Actions.html)
//!   - [Migrations](https://docs.rs/roctogen/latest/roctogen/endpoints/migrations/struct.Migrations.html)
//!   - [Gists](https://docs.rs/roctogen/latest/roctogen/endpoints/gists/struct.Gists.html)
//!   - [CodesOfConduct](https://docs.rs/roctogen/latest/roctogen/endpoints/codes_of_conduct/struct.CodesOfConduct.html)
//!   - [Pulls](https://docs.rs/roctogen/latest/roctogen/endpoints/pulls/struct.Pulls.html)
//!   - [Gitignore](https://docs.rs/roctogen/latest/roctogen/endpoints/gitignore/struct.Gitignore.html)
//!   - [EnterpriseAdmin](https://docs.rs/roctogen/latest/roctogen/endpoints/enterprise_admin/struct.EnterpriseAdmin.html)
//!   - [Git](https://docs.rs/roctogen/latest/roctogen/endpoints/git/struct.Git.html)
//!   - [Scim](https://docs.rs/roctogen/latest/roctogen/endpoints/scim/struct.Scim.html)
//!   - [CodeScanning](https://docs.rs/roctogen/latest/roctogen/endpoints/code_scanning/struct.CodeScanning.html)
//!   - [Checks](https://docs.rs/roctogen/latest/roctogen/endpoints/checks/struct.Checks.html)
//!   - [Billing](https://docs.rs/roctogen/latest/roctogen/endpoints/billing/struct.Billing.html)
//!   - [Interactions](https://docs.rs/roctogen/latest/roctogen/endpoints/interactions/struct.Interactions.html)
//!   - [Emojis](https://docs.rs/roctogen/latest/roctogen/endpoints/emojis/struct.Emojis.html)
//!
//! # Usage
//!
//! A quick example of this library:
//!
//! ```no_run
//! use roctogen::api::{self, repos};
//! use roctogen::auth::Auth;
//!
//! let auth = Auth::None;
//! let per_page = api::PerPage::new(10);
//! 
//! let mut params: repos::ReposListCommitsParams = per_page.as_ref().into();
//! params = params.author("fussybeaver").page(2);
//!
//! repos::new(&auth).list_commits("fussybeaver", "bollard", Some(params));
//! ```
//!
//! ## Async
//!
//! All the `async` methods are suffixed with `_async`, and are available on the wasm target or `isahc` and `reqwest` adapters.
//!
//! ## Webassembly
//!
//! To compile for webassembly, you can use [`wasm-pack`](https://github.com/rustwasm/wasm-pack) or compile with the
//! `wasm32-unknown-unknown` target:
//!
//! ```nocompile
//! $ wasm-pack build
//! ```
//!
//! ```nocompile
//! $ cargo build --target wasm32-unknown-unknown
//! ```
//!
//! If you are building a [cloudflare worker](https://workers.cloudflare.com/), you would use the
//! `wrangler` wrapper:
//!
//! ```nocompile
//! $ wrangler preview --watch
//! ```
//!
//! ## Client adapters
//!
//! Building on non-`wasm` targets generally requires adopting a feature for the desired
//! client adapter. 
//!
//! ### Isahc
//!
//! Compiling for the [`isahc`](https://github.com/sagebind/isahc) client required the `isahc` feature:
//!
//! ```nocompile
//! $ cargo build --features isahc
//! ```
//!
//! ### Reqwest
//!
//! Compiling for the [`reqwest`](https://github.com/seanmonstar/reqwest) client required the `reqwest` feature:
//!
//! ```nocompile
//! $ cargo build --features reqwest
//! ```
//!
//! ### Ureq
//!
//! Compiling for the [`ureq`](https://github.com/algesten/ureq) client required the `ureq` feature:
//!
//! ```nocompile
//! $ cargo build --features ureq
//! ```
//!
//! # GitHub preview features
//!
//! GitHub supports a phased rollout of non-stable endpoints behind header flags. These are
//! supported in this library through cargo feature flags. 
//!
//! ```nocompile
//! $ cargo build --features squirrel-girl
//! ```
//!
//! # Generate the API
//!
//! The majority of code is generated through the [Swagger OpenAPI
//! generator](https://github.com/swagger-api/swagger-codegen) (version 3).  Building requires the
//! [`mvn`](https://maven.apache.org/install.html) Java build tool, pegged at Java version 8 (so
//! you'll need an appropriate JDK).
//!
//! ```nocompile
//! $ mvn -D org.slf4j.simpleLogger.defaultLogLevel=info clean compiler:compile generate-resources
//! ```
//!
//! # Tests 
//!
//! Beware, tests that are not run with the `mock` feature are currently still doing real HTTP requests to the GitHub API.
//!
//! Run the wasm tests:
//!
//! ```nocompile
//! $ wasm-pack test --firefox --headless 
//! ```
//!
//! Run the sync tests:
//!
//! ```nocompile
//! $ cargo test --features isahc,mercy,squirrel-girl,inertia,starfox --target x86_64-unknown-linux-gnu -- --nocapture
//! ```
//!
//! In order to avoid GitHub's API rate limiting, you can run the non-wasm tests using wiremock.
//! You'll need to start wiremock in the background:
//!
//! ```nocompile
//! $ docker run -d --name wiremock -p 8080:8080 -v $PWD/tests/stubs:/home/wiremock
//! rodolpheche/wiremock
//! ```
//!
//! ### Regenerate the wiremock stubs
//!
//! You should regenerate the stubs if the remote API has changed:
//!
//! ```nocompile
//! $ docker run -d --name wiremock -p 8080:8080 -v $PWD/tests/stubs:/home/wiremock -u (id -u):(id -g) rodolpheche/wiremock --verbose --proxy-all="https://api.github.com" --record-mappings
//! ```
//!
#![allow(
    missing_docs,
    unused_imports,
)]

#[macro_use]
extern crate serde_derive;

use std::collections::HashMap;

pub mod adapters;
pub mod endpoints;
pub mod models;

pub mod auth {
    pub enum Auth {
        Basic {
            user: String,
            pass: String,
        },
        Token(String),
        Bearer(String),
        None,
    }
}

pub use endpoints as api;
