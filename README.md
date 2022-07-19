# podman-api

[![GitHub Actions](https://github.com/vv9k/podman-api-rs/workflows/Main/badge.svg)](https://github.com/vv9k/podman-api-rs/actions) [![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE) [![Released API docs](https://docs.rs/podman-api/badge.svg)](http://docs.rs/podman-api)


> Rust interface to Podman

Current version of this crate targets libpod API version v4.2

# Usage

Add the following to your `Cargo.toml` file

```toml
[dependencies]
podman-api = "0.4"
```

## SSL Connection

To enable HTTPS connection to podman add a `tls` flag to `Cargo.toml`.

## Default features

By default, only `chrono` feature is enabled. To disable it use:

```toml
podman-api = { version = "0.4", default-features = false }
```

## Examples

Examples for most API endpoints can be found in the [library documentation](https://docs.rs/podman-api/).


# License
[MIT](https://github.com/vv9k/podman-api-rs/blob/master/LICENSE)
