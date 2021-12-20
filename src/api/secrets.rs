use crate::{models, util::url, Result};

impl_api_ty!(
    Secret => id
);

impl<'podman> Secret<'podman> {
    api_doc! {
    Secret => InspectLibpod
    /// Inspect this secret returning detailed information about it.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    /// match podman.secrets().get("79c93f220e3e").inspect().await {
    ///     Ok(info) => println!("{:?}", info),
    ///     Err(e) => eprintln!("{}", e);
    /// }
    /// ```
    |
    pub async fn inspect(&self) -> Result<models::SecretInfoReport> {
        self.podman
            .get_json(&format!("/libpod/secrets/{}/json", &self.id))
            .await
    }}
}

impl<'podman> Secrets<'podman> {
    api_doc! {
    Secret => ListLibpod
    /// List available secrets.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    /// match podman.secrets().list().await {
    ///     Ok(info) => println!("{:?}", info),
    ///     Err(e) => eprintln!("{}", e);
    /// }
    /// ```
    |
    pub async fn list(&self) -> Result<Vec<models::SecretInfoReport>> {
        self.podman
            .get_json("/libpod/secrets/json")
            .await
    }}
}
