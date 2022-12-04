//! Rust interface to Podman.
#![cfg_attr(docsrs, feature(doc_cfg))]

#[macro_use]
mod builder;
mod podman;

pub mod api;
pub mod models;
pub mod opts;

/// Connection related items.
pub mod conn {
    pub(crate) use containers_api::conn::*;
    pub use containers_api::conn::{Error, Multiplexer, TtyChunk};
}

pub use containers_api::id::Id;
pub use containers_api::version::{ApiVersion, Error as VersionError};
pub use podman::Podman;

/// Latest libpod API version supported by this crate
pub const LATEST_API_VERSION: ApiVersion = ApiVersion::new(4, Some(2), Some(0));
macro_rules! _version {
    () => {
        "v4.2"
    };
}
pub(crate) use _version as version;

/// Common result type used throughout this crate
pub type Result<T> = std::result::Result<T, Error>;

// common crate imports
pub(crate) use futures_util::future::TryFutureExt;
pub(crate) use futures_util::stream::{Stream, TryStreamExt};
pub(crate) use serde_json::Value;

use containers_api::conn::hyper::StatusCode;
use futures_util::io::Error as IoError;
use serde_json::Error as SerdeError;
use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
/// Common error type for all functions of this library
pub enum Error {
    #[error(transparent)]
    SerdeJsonError(#[from] SerdeError),
    #[error(transparent)]
    #[allow(clippy::upper_case_acronyms)]
    IO(#[from] IoError),
    #[error("The response is invalid - {0}")]
    InvalidResponse(String),
    #[error("error {code} - {message}")]
    Fault { code: StatusCode, message: String },
    #[error("Provided scheme `{0}` is not supported")]
    UnsupportedScheme(String),
    #[error("Provided URI is missing authority part after scheme")]
    MissingAuthority,
    #[error("Failed to parse url - {0}")]
    InvalidUrl(url::ParseError),
    #[error("Invalid port - {0}")]
    InvalidPort(String),
    #[error("Invalid protocol - {0}")]
    InvalidProtocol(String),
    #[error("Failed to serialize opts - {0}")]
    OptsSerialization(String),
    #[error(transparent)]
    Error(#[from] containers_api::conn::Error),
    #[error("{0}")]
    StringError(String),
    #[error(transparent)]
    ServerError(#[from] models::JsonError),
    #[error("Cannot start an unchecked exec instance")]
    UncheckedExec,
}

impl Clone for Error {
    fn clone(&self) -> Self {
        match self {
            Error::SerdeJsonError(err) => Error::StringError(err.to_string()),
            Error::IO(err) => Error::StringError(err.to_string()),
            Error::Error(err) => Error::StringError(err.to_string()),
            e => e.clone(),
        }
    }
}
