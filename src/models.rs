//! Models generated from libpod swagger spec

pub use podman_api_stubs::models::*;

use crate::{Error, Result};

#[cfg(feature = "chrono")]
use {
    crate::util::datetime::*,
    chrono::{DateTime, Utc},
};

use hyper::header::HeaderMap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::string::ToString;

pub type Attributes = HashMap<String, String>;

#[derive(Serialize, Debug)]
/// Data returned from /ping endpoint by libpod
pub struct LibpodPingInfo {
    /// Max compatibility API Version the server supports
    pub api_version: String,
    /// Max Podman API Version the server supports. Available if service is backed by Podman, therefore may be used to determine if talking to Podman engine or another engine
    pub libpod_api_version: String,
    /// Default version of libpod image builder. Available if service is backed by Podman, therefore may be used to determine if talking to Podman engine or another engine
    pub libpod_buildah_version: String,
    /// Default version of docker image builder
    pub buildkit_version: Option<String>,
    /// Always no-cache
    pub cache_control: String,
    /// If the server is running with experimental mode enabled, always true
    pub docker_experimental: bool,
    /// Always no-cache
    pub pragma: String,
}

impl TryFrom<&HeaderMap> for LibpodPingInfo {
    type Error = Error;

    fn try_from(value: &HeaderMap) -> Result<Self> {
        macro_rules! extract_str {
            ($id:literal) => {{
                if let Some(val) = value.get($id) {
                    val.to_str().map(ToString::to_string).map_err(|e| {
                        Error::InvalidResponse(format!(
                            "failed to convert header to string - {}",
                            e
                        ))
                    })?
                } else {
                    return Err(Error::InvalidResponse(format!(
                        "expected `{}` field in headers",
                        $id
                    )));
                }
            }};
        }

        Ok(LibpodPingInfo {
            api_version: extract_str!("api-version"),
            libpod_api_version: extract_str!("libpod-api-version"),
            buildkit_version: value
                .get("buildkit-version")
                .and_then(|v| v.to_str().map(ToString::to_string).ok()),
            docker_experimental: extract_str!("docker-experimental").parse().map_err(|e| {
                Error::InvalidResponse(format!("expected header value to be bool - {}", e))
            })?,
            cache_control: extract_str!("cache-control"),
            pragma: extract_str!("pragma"),
            libpod_buildah_version: extract_str!("libpod-buildah-version"),
        })
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Event {
    #[serde(rename = "Type")]
    pub typ: String,
    #[serde(rename = "Action")]
    pub action: String,
    #[serde(rename = "Actor")]
    pub actor: Actor,
    pub status: Option<String>,
    pub id: Option<String>,
    pub from: Option<String>,
    #[cfg(feature = "chrono")]
    #[serde(deserialize_with = "datetime_from_unix_timestamp")]
    pub time: DateTime<Utc>,
    #[cfg(not(feature = "chrono"))]
    pub time: u64,
    #[cfg(feature = "chrono")]
    #[serde(deserialize_with = "datetime_from_nano_timestamp", rename = "timeNano")]
    pub time_nano: DateTime<Utc>,
    #[cfg(not(feature = "chrono"))]
    #[serde(rename = "timeNano")]
    pub time_nano: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Actor {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "Attributes")]
    pub attributes: Attributes,
}
