[![license](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![docs](https://docs.rs/roctogen/badge.svg)](https://docs.rs/roctogen/)
[![GitHub workflow](https://github.com/fussybeaver/roctogen/actions/workflows/default.yml/badge.svg)](https://github.com/fussybeaver/roctogen/actions/workflows/default.yml)

## Roctokit: Rust Client Library for GitHub v3 API

**Roctokit** is a Rust library containing client interfaces for models and endpoints generated from the [GitHub REST API OpenAPI
specification](https://github.com/github/rest-api-description/), providing comprehensive support for the GitHub v3 API. 

**Roctokit** provides stock interfaces supporting both synchronous and asynchronous HTTP clients, including WebAssembly compatibility. More flexible implementations such as [handling GitHub's rate limiting or building streams out of paginated results](./examples/search) is also available by extending the `adapter` subsystem.

### Stock client interfaces

  - `reqwest`: Enables asynchronous requests using the [Reqwest client](https://github.com/seanmonstar/reqwest)
- `ureq`: Provides synchronous requests with the [Ureq client](https://github.com/algesten/ureq)
  - `wasm`: Use a wasm interface with the underlying [fetch API](TODO).

### Repository Structure

This repository is split into two main components:

- [roctogen](./roctogen/README.md): A Rust library generated from the GitHub REST API OpenAPI specification, providing models and low-level API bindings.

- [roctokit](./roctokit/README.md): Higher-level abstractions for interacting with the generated GitHub models.

### Getting Started

An simple example is explained in the [roctogen README](./roctogen/README.md#Usage). Following this, refer to the [endpoints documentation](https://docs.rs/roctogen/latest/roctogen/endpoints/index.html) to understand which GitHub API to use.

### License

This project is licensed under the Apache 2.0 License.
