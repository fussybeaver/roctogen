//! Endpoints module and `PerPage` struct/impl
/* 
 * GitHub v3 REST API
 *
 * GitHub's v3 REST API.
 *
 * OpenAPI spec version: 1.1.4
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

pub const GITHUB_BASE_API_URL: &str = if cfg!(feature = "mock") {
    "http://localhost:8080"
} else {
    "https://api.github.com"
};

pub mod actions;
pub mod activity;
pub mod apps;
pub mod billing;
pub mod checks;
pub mod classroom;
pub mod code_scanning;
pub mod code_security;
pub mod codes_of_conduct;
pub mod codespaces;
pub mod copilot;
pub mod dependabot;
pub mod dependency_graph;
pub mod emojis;
pub mod gists;
pub mod gitignore;
pub mod git;
pub mod interactions;
pub mod issues;
pub mod licenses;
pub mod markdown;
pub mod meta;
pub mod migrations;
pub mod oidc;
pub mod orgs;
pub mod packages;
pub mod projects;
pub mod pulls;
pub mod rate_limit;
pub mod reactions;
pub mod repos;
pub mod search;
pub mod secret_scanning;
pub mod security_advisories;
pub mod teams;
pub mod users;

pub struct PerPage {
    per_page: u16,
    page: u16,
}

impl PerPage {
    pub fn new(per_page: u16) -> Self {
        PerPage { per_page, page: 0 }
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
