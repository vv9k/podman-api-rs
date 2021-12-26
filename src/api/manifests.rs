use crate::{api::ApiResource, Result};

impl_api_ty!(
    Manifest => id
);

impl<'podman> Manifest<'podman> {
    api_doc! {
    Manifest => ExistsLibpod
    /// Quick way to determine if a manifest exists by name or id.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    /// match podman.manifests().get("some-manifest").exists().await {
    ///     Ok(exists) => if exists {
    ///         println!("manifest exists!");
    ///     } else {
    ///         println!("manifest doesn't exists!");
    ///     },
    ///     Err(e) => eprintln!("check failed: {}", e),
    /// }
    /// ```
    |
    pub async fn exists(&self) -> Result<bool> {
        self.podman
            .resource_exists(ApiResource::Manifests, &self.id)
            .await
    }}
}
