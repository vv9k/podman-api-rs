//! Handles for each API endpoint like containers, images, volumes...

mod containers;
mod exec;
mod images;
mod manifests;
mod networks;
mod pods;
mod secrets;
mod volumes;

pub use containers::*;
pub use exec::*;
pub use images::*;
pub use manifests::*;
pub use networks::*;
pub use pods::*;
pub use secrets::*;
pub use volumes::*;

use crate::{Error, Result};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// Types of resources available in Podman. This is just a helper enum listing all possible
/// categories of endpoints.
pub enum ApiResource {
    Containers,
    Exec,
    Images,
    Manifests,
    Networks,
    Pods,
    Secrets,
    Volumes,
    System,
}

impl AsRef<str> for ApiResource {
    fn as_ref(&self) -> &str {
        use ApiResource::*;
        match self {
            Containers => "containers",
            Exec => "exec",
            Images => "images",
            Manifests => "manifests",
            Networks => "networks",
            Pods => "pods",
            Secrets => "secrets",
            Volumes => "volumes",
            System => "system",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
/// Structure representing API version used to determine compatibility between a client and a server.
pub struct ApiVersion {
    major: usize,
    minor: usize,
    patch: usize,
}

impl ApiVersion {
    pub const fn new(major: usize, minor: usize, patch: usize) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    pub fn make_endpoint(&self, ep: impl AsRef<str>) -> String {
        let ep = ep.as_ref();
        format!(
            "/v{}{}{}",
            self,
            if !ep.starts_with('/') { "/" } else { "" },
            ep
        )
    }
}

impl std::fmt::Display for ApiVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", self.major, self.minor)
    }
}

impl From<usize> for ApiVersion {
    fn from(v: usize) -> Self {
        ApiVersion {
            major: v,
            minor: 0,
            patch: 0,
        }
    }
}

impl From<(usize, usize)> for ApiVersion {
    fn from(v: (usize, usize)) -> Self {
        ApiVersion {
            major: v.0,
            minor: v.1,
            patch: 0,
        }
    }
}

impl From<(usize, usize, usize)> for ApiVersion {
    fn from(v: (usize, usize, usize)) -> Self {
        ApiVersion {
            major: v.0,
            minor: v.1,
            patch: v.2,
        }
    }
}

impl FromStr for ApiVersion {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let mut elems = s.split('.');
        macro_rules! parse_or_err {
            ($s:expr) => {
                if let Some(it) = elems.next() {
                    match it.parse::<usize>() {
                        Ok(it) => it,
                        Err(e) => return Err(Error::MalformedVersion(e.to_string())),
                    }
                } else {
                    return Err(Error::MalformedVersion($s.to_string()));
                }
            };
        }
        let major = parse_or_err!("expected major version");
        let minor = parse_or_err!("expected minor version");
        let patch = parse_or_err!("expected patch version");

        if elems.next().is_some() {
            return Err(Error::MalformedVersion(
                "unexpected extra tokens".to_string(),
            ));
        }

        Ok(Self {
            major,
            minor,
            patch,
        })
    }
}
