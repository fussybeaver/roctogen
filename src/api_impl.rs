use serde::Deserialize;

use crate::adapters::{AdapterError, GitHubRequest, GitHubRequestBuilder, GitHubResponseExt, Json};
use crate::auth::Auth;
use crate::models::*;

pub const GITHUB_BASE_API_URL: &str = "https://api.github.com";

pub struct PerPage {
    per_page: u16,
    page: u16,
}

impl PerPage {
    pub fn new(per_page: u16) -> Self {
        Self { per_page, page: 0 }
    }

    pub fn page(&mut self, page: u16) -> &mut Self {
        self.page = page;
        self
    }
}

impl std::convert::AsRef<PerPage> for PerPage {
    fn as_ref(&self) -> &PerPage {
        self
    }
}

pub mod repos {
    use super::*;

    pub struct Repos<'api> {
        auth: &'api Auth,
    }

    pub fn new(auth: &Auth) -> Repos {
        Repos { auth }
    }

    #[derive(Debug, thiserror::Error)]
    pub enum ReposListCommitsError {
        #[error(transparent)]
        AdapterError(#[from] AdapterError),
        #[error(transparent)]
        SerdeJson(#[from] serde_json::Error),
        #[error(transparent)]
        SerdeUrl(#[from] serde_urlencoded::ser::Error),

        // -- endpoint errors
        #[error("Internal Error")]
        Status500(BasicError),
        #[error("Bad Request")]
        Status400(BasicError),
        #[error("Resource Not Found")]
        Status404(BasicError),
        #[error("Conflict")]
        Status409(BasicError),
        #[error("Status code: {}", code)]
        Generic { code: u16 },
    }

    #[derive(Default, Serialize)]
    pub struct ReposListCommitsParams {
        sha: Option<String>,
        path: Option<String>,
        author: Option<String>,
        since: Option<String>,
        until: Option<String>,
        per_page: Option<u16>,
        page: Option<u16>,
    }

    impl ReposListCommitsParams {
        pub fn new() -> Self {
            Self::default()
        }

        pub fn path(&mut self, path: impl Into<String>) -> &mut Self {
            self.path = Some(path.into());
            self
        }

        pub fn author(&mut self, author: impl Into<String>) -> &mut Self {
            self.author = Some(author.into());
            self
        }

        pub fn since(&mut self, since: impl Into<String>) -> &mut Self {
            self.since = Some(since.into());
            self
        }

        pub fn until(&mut self, until: impl Into<String>) -> &mut Self {
            self.until = Some(until.into());
            self
        }

        pub fn per_page(&mut self, per_page: u16) -> &mut Self {
            self.per_page = Some(per_page);
            self
        }

        pub fn page(&mut self, page: u16) -> &mut Self {
            self.page = Some(page);
            self
        }
    }

    impl<'enc> From<&'enc PerPage> for ReposListCommitsParams {
        fn from(per_page: &'enc PerPage) -> Self {
            let mut this = Self::new();
            this.per_page(per_page.per_page);
            this.page(per_page.page);
            this
        }
    }

    impl<'api> Repos<'api> {
        pub async fn list_commits_async(
            &self,
            owner: &str,
            repo: &str,
            query_params: Option<impl Into<ReposListCommitsParams>>,
        ) -> Result<Vec<Commit>, ReposListCommitsError> {
            let mut request_uri = format!(
                "{}/repos/{}/{}/commits",
                super::GITHUB_BASE_API_URL,
                owner,
                repo
            );

            if let Some(params) = query_params {
                request_uri.push_str("?");
                request_uri.push_str(&serde_urlencoded::to_string(params.into())?);
            }

            let req = GitHubRequest {
                uri: request_uri,
                body: (),
                method: "GET",
            };

            let request = GitHubRequestBuilder::build(req, self.auth)?;

            // --

            let github_response = crate::adapters::fetch_async(request).await?;

            // --

            if github_response.is_success() {
                Ok(github_response.to_json()?)
            } else {
                match github_response.status_code() {
                    500 => Err(ReposListCommitsError::Status500(github_response.to_json()?)),
                    400 => Err(ReposListCommitsError::Status400(github_response.to_json()?)),
                    404 => Err(ReposListCommitsError::Status404(github_response.to_json()?)),
                    409 => Err(ReposListCommitsError::Status409(github_response.to_json()?)),
                    code => Err(ReposListCommitsError::Generic { code }),
                }
            }
        }

        #[cfg(not(target_arch = "wasm32"))]
        pub fn list_commits(
            &self,
            owner: &str,
            repo: &str,
            query_params: Option<impl Into<ReposListCommitsParams>>,
        ) -> Result<Vec<Commit>, ReposListCommitsError> {
            let mut request_uri = format!(
                "{}/repos/{}/{}/commits",
                super::GITHUB_BASE_API_URL,
                owner,
                repo
            );

            if let Some(params) = query_params {
                request_uri.push_str("?");
                request_uri.push_str(&serde_urlencoded::to_string(params.into())?);
            }

            let req = GitHubRequest {
                uri: request_uri,
                body: (),
                method: "GET",
            };

            let request = GitHubRequestBuilder::build(req, self.auth)?;

            // --

            let github_response = crate::adapters::fetch(request)?;

            // --

            if github_response.is_success() {
                Ok(github_response.to_json()?)
            } else {
                match github_response.status_code() {
                    500 => Err(ReposListCommitsError::Status500(github_response.to_json()?)),
                    400 => Err(ReposListCommitsError::Status400(github_response.to_json()?)),
                    404 => Err(ReposListCommitsError::Status404(github_response.to_json()?)),
                    409 => Err(ReposListCommitsError::Status409(github_response.to_json()?)),
                    code => Err(ReposListCommitsError::Generic { code }),
                }
            }
        }
    }
}
