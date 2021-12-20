use crate::{api::ApiResource, Result};

impl_api_ty!(
    Network => id
);

impl<'podman> Network<'podman> {
    api_doc! {
    Network => ExistsLibpod
    /// Quick way to determine if a network exists by name or id.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    /// match podman.networks().get("some-network").exists().await {
    ///     Ok(exists) => if exists {
    ///         println!("network exists!");
    ///     } else {
    ///         println!("network doesn't exists!");
    ///     },
    ///     Err(e) => eprintln!("check failed: {}", e);
    /// }
    /// ```
    |
    pub async fn exists(&self) -> Result<bool> {
        self.podman
            .resource_exists(ApiResource::Networks, &self.id)
            .await
    }}
}
