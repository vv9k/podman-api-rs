#[macro_use]
mod builder;
mod id;
mod podman;
mod util;

pub mod api;
pub mod conn;
pub mod models;
pub mod opts;

pub use api::ApiVersion;
pub use id::Id;
pub use podman::Podman;

/// Latest libpod API version supported by this crate
pub const LATEST_API_VERSION: ApiVersion = ApiVersion::new(3, 4, 4);
macro_rules! _version {
    () => {
        "v3.4.4"
    };
}
pub(crate) use _version as version;

/// Common result type used throughout this crate
pub type Result<T> = std::result::Result<T, Error>;

use futures_util::io::Error as IoError;
use hyper::{self, StatusCode};
#[cfg(feature = "tls")]
use openssl::error::ErrorStack;
use serde_json::Error as SerdeError;
use std::string::FromUtf8Error;
use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
/// Common error type for all functions of this library
pub enum Error {
    #[error(transparent)]
    SerdeJsonError(#[from] SerdeError),
    #[error(transparent)]
    Hyper(#[from] hyper::Error),
    #[error(transparent)]
    Http(#[from] hyper::http::Error),
    #[error(transparent)]
    #[allow(clippy::upper_case_acronyms)]
    IO(#[from] IoError),
    #[error(transparent)]
    Encoding(#[from] FromUtf8Error),
    #[error("The response is invalid - {0}")]
    InvalidResponse(String),
    #[error("error {code} - {message}")]
    Fault { code: StatusCode, message: String },
    #[error("The HTTP connection was not upgraded by the podman host")]
    ConnectionNotUpgraded,
    #[cfg(feature = "tls")]
    #[error(transparent)]
    ErrorStack(#[from] ErrorStack),
    #[error("Provided scheme `{0}` is not supported")]
    UnsupportedScheme(String),
    #[error("Provided URI is missing authority part after scheme")]
    MissingAuthority,
    #[error("Failed to parse url - {0}")]
    InvalidUrl(url::ParseError),
    #[error("Failed to parse uri - {0}")]
    InvalidUri(http::uri::InvalidUri),
    #[error("Invalid port - {0}")]
    InvalidPort(String),
    #[error("Invalid protocol - {0}")]
    InvalidProtocol(String),
    #[error("Invalid version - {0}")]
    MalformedVersion(String),
    #[error("Failed to serialize opts - {0}")]
    OptsSerialization(String),
}
