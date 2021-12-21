use crate::{models, opts, util::url, Result};

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

    api_doc! {
    Secret => DeleteLibpod
    /// Remove this secret
    ///
    /// Examples:
    ///
    /// ```no_run
    /// let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    /// if let Err(e) = podman.secrets().delete().await {
    ///     eprintln!("{}", e);
    /// }
    /// ```
    |
    pub async fn delete(&self) -> Result<()> {
        self.podman
            .delete(&format!("/libpod/secrets/{}/", &self.id))
            .await
            .map(|_| ())
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

    api_doc! {
    Secret => CreateLibpod
    /// Create a new secret.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    /// match podman.secrets().create().await {
    ///     Ok(info) => println!("{:?}", info),
    ///     Err(e) => eprintln!("{}", e);
    /// }
    /// ```
    |
    pub async fn create(
        &self,
        opts: &opts::SecretCreateOpts,
        secret: impl Into<String>,
    ) -> Result<Secret<'_>> {
        let ep = url::construct_ep("/libpod/secrets/create", opts.serialize());
        self.podman
            .post_json(
                &ep,
                crate::conn::Payload::Json(serde_json::to_string(&secret.into())?),
            )
            .await
            .map(|resp: models::LibpodSecretCreateResponse| {
                Secret::new(self.podman, resp.ID.unwrap_or_default())
            })
    }}
}
