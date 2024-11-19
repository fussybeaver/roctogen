use std::time::Duration;

use base64::{prelude::BASE64_STANDARD, Engine};
use chrono::DateTime;
use chrono::Utc;
use futures::FutureExt;
use futures::StreamExt;
use futures::TryFutureExt;
use futures::TryStreamExt;
use futures_util::stream;
use http_body_util::BodyExt;
use http_body_util::Empty;
use http_body_util::Full;
use hyper::body::Buf;
use hyper::body::Bytes;
use hyper::body::Incoming;
use hyper::header::ToStrError;
use hyper::header::ACCEPT;
use hyper::header::AUTHORIZATION;
use hyper::header::CONTENT_TYPE;
use hyper::header::USER_AGENT;
use hyper_rustls::ConfigBuilderExt;
use hyper_util::client::legacy::connect::HttpConnector;
use log::debug;
use roctogen::adapters::GitHubRequest;
use roctogen::adapters::GitHubResponseExt;
use roctogen::api::search;
use roctogen::api::PerPage;
use roctogen::auth::Auth;
use serde::de::Error;
use serde::{ser, Deserialize};

// Jitter adds a number of seconds to the GitHub reset header
// to adjust timings in favour of request/response completion
const JITTER: u64 = 2;

#[derive(thiserror::Error, Debug)]
pub enum AdapterError {
    #[error(transparent)]
    Hyper(#[from] hyper::Error),
    #[error(transparent)]
    HyperUtil(#[from] hyper_util::client::legacy::Error),
    #[error(transparent)]
    Http(#[from] hyper::http::Error),
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
    #[error(transparent)]
    IOError(#[from] std::io::Error),
    #[error("Ureq adapter only has sync fetch implemented")]
    UnimplementedSync,
    #[error(transparent)]
    ToStrError(#[from] ToStrError),
    #[error(transparent)]
    ParseIntError(#[from] std::num::ParseIntError),
}

impl From<AdapterError> for roctogen::adapters::AdapterError {
    fn from(err: AdapterError) -> Self {
        Self::Client {
            description: err.to_string(),
            source: Some(Box::new(err)),
        }
    }
}

struct Response {
    pub inner: hyper::Response<Incoming>,
}

impl roctogen::adapters::Client for Client {
    type Req = hyper::Request<Full<Bytes>>;
    type Err = AdapterError where roctogen::adapters::AdapterError: From<Self::Err>;
    type Body = Empty<Bytes>;

    fn new(auth: &Auth) -> Result<Self, Self::Err> {
        let tls = rustls::ClientConfig::builder()
            .with_native_roots()?
            .with_no_client_auth();
        let connector = hyper_rustls::HttpsConnectorBuilder::new()
            .with_tls_config(tls)
            .https_or_http()
            .enable_http1()
            .build();
        let client_builder =
            hyper_util::client::legacy::Client::builder(hyper_util::rt::TokioExecutor::new());
        let pool: hyper_util::client::legacy::Client<_, _> = client_builder.build(connector);
        Ok(Self {
            auth: auth.to_owned(),
            pool,
        })
    }

    fn fetch(&self, _req: Self::Req) -> Result<impl GitHubResponseExt, Self::Err> {
        Err::<Response, _>(AdapterError::UnimplementedSync)
    }

    async fn fetch_async(&self, req: Self::Req) -> Result<impl GitHubResponseExt, Self::Err> {
        let res = self.pool.request(req).await?;
        if let (Some(reset), Some(remaining)) = (
            res.headers().get("x-ratelimit-reset"),
            res.headers().get("x-ratelimit-remaining"),
        ) {
            let reset = DateTime::from_timestamp(reset.to_str()?.parse()?, 0)
                .unwrap_or(chrono::DateTime::<Utc>::MAX_UTC);

            let _reset_rfc = reset.to_rfc3339();

            let remaining: u64 = remaining.to_str()?.parse()?;

            let time_to_reset = reset.signed_duration_since(Utc::now()).abs().num_seconds() as u64;
            let time_to_wait = Duration::from_secs((time_to_reset + JITTER) / remaining);

            println!(
                "GitHub will reset it's rate-limiting window for this token in {} seconds, you have {} remaining requests within that window. Sleeping {} seconds to prevent rate-limiting.",
                time_to_reset,
                remaining,
                time_to_wait.as_secs()
            );

            // Sleep so we don't trigger GitHub rate limiting
            tokio::time::sleep(time_to_wait).await;
        }

        Ok(Response { inner: res })
    }

    fn build(&self, req: GitHubRequest<Self::Body>) -> Result<Self::Req, Self::Err> {
        let mut builder = hyper::Request::builder();

        builder = builder
            .uri(req.uri)
            .method(req.method)
            .header(ACCEPT, "application/vnd.github.v3+json")
            .header(USER_AGENT, "roctogen")
            .header(CONTENT_TYPE, "application/json");

        for header in req.headers.iter() {
            builder = builder.header(header.0, header.1);
        }

        builder = match &self.auth {
            Auth::Basic { user, pass } => {
                let creds = format!("{}:{}", user, pass);
                builder.header(
                    AUTHORIZATION,
                    format!("Basic {}", BASE64_STANDARD.encode(creds.as_bytes())),
                )
            }
            Auth::Token(token) => builder.header(AUTHORIZATION, format!("token {}", token)),
            Auth::Bearer(bearer) => builder.header(AUTHORIZATION, format!("Bearer {}", bearer)),
            Auth::None => builder,
        };

        Ok(hyper::Request::from(builder.body(Full::new(Bytes::new()))?))
    }

    fn from_json<E: ser::Serialize>(_model: E) -> Result<Self::Body, Self::Err> {
        Ok(Empty::new())
    }
}

pub struct Client {
    auth: Auth,
    pool: hyper_util::client::legacy::Client<
        hyper_rustls::HttpsConnector<HttpConnector>,
        Full<Bytes>,
    >,
}

impl GitHubResponseExt for Response {
    fn is_success(&self) -> bool {
        self.inner.status().is_success()
    }

    fn status_code(&self) -> u16 {
        self.inner.status().as_u16()
    }

    fn to_json<E: for<'de> Deserialize<'de> + std::fmt::Debug>(
        self,
    ) -> Result<E, serde_json::Error> {
        unimplemented!("hyper adapter only has async json conversion implemented");
    }

    async fn to_json_async<E: for<'de> Deserialize<'de> + Unpin + std::fmt::Debug>(
        self,
    ) -> Result<E, serde_json::Error> {
        let body = self
            .inner
            .collect()
            .await
            .map_err(|e| serde_json::Error::custom(format!("{}", e)))?
            .aggregate();

        let json = serde_json::from_reader(body.reader())?;

        debug!("Body: {:?}", json);

        Ok(json)
    }
}

#[derive(Clone, Copy, Debug)]
struct IterableState {
    page: u16,
    count: i64,
    total: i64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client: Client = <Client as roctogen::adapters::Client>::new(&Auth::None).unwrap();
    let per_page = roctogen::api::PerPage::new(100);
    let search = roctogen::api::search::new(&client);
    let stream = unfold(&search, per_page.as_ref());

    let lst = stream.try_collect::<Vec<_>>().await?;

    println!("{:?}", lst);

    Ok(())
}

fn unfold<'a>(
    search: &'a search::Search<Client>,
    per_page: &'a PerPage,
) -> impl stream::Stream<
    Item = Result<roctogen::models::IssueSearchResultItem, roctogen::adapters::AdapterError>,
> + 'a {
    let state = IterableState {
        page: 1,
        count: 0,
        total: 1,
    };

    stream::try_unfold(state, move |acc| {
        if acc.count >= acc.total {
            futures::future::ok(None).left_future()
        } else {
            let mut params: search::SearchIssuesAndPullRequestsParams = per_page.into();
            params = params.q("org:fussybeaver");
            params = params.page(acc.page);

            println!("Requesting page {}", acc.page);
            let fut = search.issues_and_pull_requests_async(params);

            fut.map_ok(move |res| {
                let total = res.total_count.unwrap_or(1);
                let items = res.items.unwrap_or_else(Vec::new);
                let count = acc.count + items.len() as i64;
                let page = acc.page + 1;
                Some((stream::iter(items), IterableState { page, count, total }))
            })
            .right_future()
        }
    })
    .map_ok(|v| v.map(|v| Ok(v)))
    .try_flatten()
}
