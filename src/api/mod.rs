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
