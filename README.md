[![license](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![GitHub workflow](https://github.com/fussybeaver/roctokit/actions/workflows/default.yml/badge.svg)](https://github.com/fussybeaver/roctokit/actions/workflows/default.yml)

# Roctokit: A Rust Client Library for the GitHub v3 API

**Roctokit** is a Rust library providing high-level client interfaces for interacting with the [GitHub REST API](https://docs.github.com/en/rest/repos). Built on models and endpoints generated from [GitHub's OpenAPI specification](https://github.com/github/rest-api-description), it offers an ergonomic and efficient way to work with GitHub's v3 API.

Roctokit includes stock client implementations supporting both synchronous and asynchronous HTTP clients, as well as WebAssembly compatibility. Additionally, it allows for custom client implementations, enabling advanced use cases such as [handling GitHub’s rate limits](./examples/search) or building streams from paginated results by extending its `adapter` subsystem.

## Features

- **Multi-client support**: Use the built-in HTTP clients or implement your own.
- **WebAssembly compatibility**: Works seamlessly in browser environments.
- **Extensible adapters**: Customise request handling for rate limiting, pagination, or alternative transport layers.

## Supported Client Implementations

Roctokit provides out-of-the-box support for the following HTTP clients:

- **`reqwest`**: Enables asynchronous requests via the [Reqwest client](https://github.com/seanmonstar/reqwest).
- **`ureq`**: Provides synchronous requests using the [Ureq client](https://github.com/algesten/ureq).
- **`wasm`**: Uses the browser's [Fetch API](https://developer.mozilla.org/en-US/docs/Web/API/Fetch_API/Using_Fetch) for WebAssembly environments.

## Repository Structure

This repository is divided into two main components:

- **[roctogen](./roctogen/README.md)**: A Rust library that provides low-level API bindings and models generated from the [GitHub OpenAPI specification](https://github.com/github/rest-api-description).
- **[roctokit](./roctokit/README.md)**: A higher-level client built on top of `roctogen`, offering more user-friendly abstractions for interacting with the GitHub API.

## Getting Started

To start using Roctokit, begin with a basic example provided in the [roctogen README](./roctogen/README.md#Usage). Once familiar with the basics, explore the [endpoint documentation](https://docs.rs/roctogen/latest/roctogen/endpoints/index.html) to determine which GitHub API endpoints are available. For more advanced use cases, consider implementing a custom adapter, which covered in the [roctokit README](./core/README.md).

## License

This project is licensed under the Apache 2.0 License. 


