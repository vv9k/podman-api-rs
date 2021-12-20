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

/// Allows easier construction of filter functions for multiple api endpoints
pub(crate) trait Filter {
    // TODO: Add a stronger return type. Not all filters are `key=val`, soma are only `key`
    fn query_key_val(&self) -> (&'static str, String);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum ApiResource {
    Containers,
    Exec,
    Images,
    Manifests,
    Networks,
    Pods,
    Secrets,
    Volumes,
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
        }
    }
}
