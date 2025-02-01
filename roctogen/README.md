[![crates.io](https://img.shields.io/crates/v/roctogen.svg)](https://crates.io/crates/roctogen)
[![license](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![docs](https://docs.rs/roctogen/badge.svg)](https://docs.rs/roctogen/)

## Roctogen: Rust Client Library for GitHub v3 API

**Roctogen** is a Rust library generated from the [GitHub REST API OpenAPI
specification](https://github.com/github/rest-api-description/). providing
comprehensive support for the GitHub v3 API. It provides generated models
and endpoints that are intended to be used with the sister library
[**roctokit**](https://crates.io/crates/roctokit), that provides client interfaces.

### Documentation

- [API docs](https://docs.rs/roctogen/latest).
- [Endpoints](https://docs.rs/roctogen/latest/roctogen/endpoints/index.html).

#### Supported endpoints:

Roctogen supports a wide range of GitHub API endpoints, including:

  - [Meta](https://docs.rs/roctogen/latest/roctogen/endpoints/meta/struct.Meta.html)
  - [Issues](https://docs.rs/roctogen/latest/roctogen/endpoints/issues/struct.Issues.html)
  - [Licenses](https://docs.rs/roctogen/latest/roctogen/endpoints/licenses/struct.Licenses.html)
  - [Reactions](https://docs.rs/roctogen/latest/roctogen/endpoints/reactions/struct.Reactions.html)
  - [Activity](https://docs.rs/roctogen/latest/roctogen/endpoints/activity/struct.Activity.html)
  - [Projects](https://docs.rs/roctogen/latest/roctogen/endpoints/projects/struct.Projects.html)
  - [Orgs](https://docs.rs/roctogen/latest/roctogen/endpoints/orgs/struct.Orgs.html)
  - [Users](https://docs.rs/roctogen/latest/roctogen/endpoints/users/struct.Users.html)
  - [Apps](https://docs.rs/roctogen/latest/roctogen/endpoints/apps/struct.Apps.html)
  - [RateLimit](https://docs.rs/roctogen/latest/roctogen/endpoints/rate_limit/struct.RateLimit.html)
  - [Repos](https://docs.rs/roctogen/latest/roctogen/endpoints/repos/struct.Repos.html)
  - [SecretScanning](https://docs.rs/roctogen/latest/roctogen/endpoints/secret_scanning/struct.SecretScanning.html)
  - [SecurityAdvisories](https://docs.rs/roctogen/latest/roctogen/endpoints/security_advisories/struct.SecurityAdvisories.html)
  - [Packages](https://docs.rs/roctogen/latest/roctogen/endpoints/packages/struct.Packages.html)
  - [Search](https://docs.rs/roctogen/latest/roctogen/endpoints/search/struct.Search.html)
  - [Classroom](https://docs.rs/roctogen/latest/roctogen/endpoints/classroom/struct.Classroom.html)
  - [Teams](https://docs.rs/roctogen/latest/roctogen/endpoints/teams/struct.Teams.html)
  - [PrivateRegistries](https://docs.rs/roctogen/latest/roctogen/endpoints/private_registries/struct.PrivateRegistries.html)
  - [Oidc](https://docs.rs/roctogen/latest/roctogen/endpoints/oidc/struct.Oidc.html)
  - [Markdown](https://docs.rs/roctogen/latest/roctogen/endpoints/markdown/struct.Markdown.html)
  - [Actions](https://docs.rs/roctogen/latest/roctogen/endpoints/actions/struct.Actions.html)
  - [Migrations](https://docs.rs/roctogen/latest/roctogen/endpoints/migrations/struct.Migrations.html)
  - [CodeSecurity](https://docs.rs/roctogen/latest/roctogen/endpoints/code_security/struct.CodeSecurity.html)
  - [Gists](https://docs.rs/roctogen/latest/roctogen/endpoints/gists/struct.Gists.html)
  - [HostedCompute](https://docs.rs/roctogen/latest/roctogen/endpoints/hosted_compute/struct.HostedCompute.html)
  - [DependencyGraph](https://docs.rs/roctogen/latest/roctogen/endpoints/dependency_graph/struct.DependencyGraph.html)
  - [Copilot](https://docs.rs/roctogen/latest/roctogen/endpoints/copilot/struct.Copilot.html)
  - [Dependabot](https://docs.rs/roctogen/latest/roctogen/endpoints/dependabot/struct.Dependabot.html)
  - [CodesOfConduct](https://docs.rs/roctogen/latest/roctogen/endpoints/codes_of_conduct/struct.CodesOfConduct.html)
  - [Pulls](https://docs.rs/roctogen/latest/roctogen/endpoints/pulls/struct.Pulls.html)
  - [Gitignore](https://docs.rs/roctogen/latest/roctogen/endpoints/gitignore/struct.Gitignore.html)
  - [Git](https://docs.rs/roctogen/latest/roctogen/endpoints/git/struct.Git.html)
  - [CodeScanning](https://docs.rs/roctogen/latest/roctogen/endpoints/code_scanning/struct.CodeScanning.html)
  - [Checks](https://docs.rs/roctogen/latest/roctogen/endpoints/checks/struct.Checks.html)
  - [Billing](https://docs.rs/roctogen/latest/roctogen/endpoints/billing/struct.Billing.html)
  - [Interactions](https://docs.rs/roctogen/latest/roctogen/endpoints/interactions/struct.Interactions.html)
  - [Codespaces](https://docs.rs/roctogen/latest/roctogen/endpoints/codespaces/struct.Codespaces.html)
  - [Emojis](https://docs.rs/roctogen/latest/roctogen/endpoints/emojis/struct.Emojis.html)

For a full list of supported endpoints, refer to the [API documentation](https://docs.rs/roctogen/latest/roctogen/endpoints/index.html).

### Usage

Here's a basic example demonstrating how to use Roctogen:

```rust
use roctogen::api::{self, repos};
use roctokit::adapters::client;
use roctokit::auth::Auth;

let auth = Auth::None;
let client = client(&auth).expect("Cannot create new client");
let per_page = api::PerPage::new(10);

let mut params: repos::ReposListCommitsParams = per_page.as_ref().into();
params = params.author("fussybeaver").page(2);

repos::new(&client).list_commits("fussybeaver", "bollard", Some(params));
```

### Generating the API

Roctogen's code is largely generated from the GitHub REST API specification
using the [Swagger OpenAPI
generator](https://github.com/swagger-api/swagger-codegen) (version 3).
Building the API requires the Java-based `mvn` tool with JDK 8 installed:

```nocompile
$ mvn -D org.slf4j.simpleLogger.defaultLogLevel=info clean compiler:compile generate-resources
```


License: Apache-2.0
