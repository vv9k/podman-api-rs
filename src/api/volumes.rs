use crate::{api::ApiResource, Result};

impl_api_ty!(
    Volume => name
);

impl<'podman> Volume<'podman> {
    api_doc! {
    Volume => ExistsLibpod
    /// Quick way to determine if a volume exists by name.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    /// match podman.volumes().get("some_vol").exists().await {
    ///     Ok(exists) => if exists {
    ///         println!("volume exists!");
    ///     } else {
    ///         println!("volume doesn't exists!");
    ///     },
    ///     Err(e) => eprintln!("check failed: {}", e);
    /// }
    /// ```
    |
    pub async fn exists(&self) -> Result<bool> {
        self.podman
            .resource_exists(ApiResource::Volumes, &self.name)
            .await
    }}
}
