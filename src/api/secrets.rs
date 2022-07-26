use crate::{models, opts, Result};

use containers_api::url;

impl_api_ty!(
    Secret => id
);

impl Secret {
    api_doc! {
    Secret => InspectLibpod
    /// Inspect this secret returning detailed information about it.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// async {
    ///     use podman_api::Podman;
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    ///     match podman.secrets().get("79c93f220e3e").inspect().await {
    ///         Ok(info) => println!("{:?}", info),
    ///         Err(e) => eprintln!("{}", e),
    ///     }
    /// };
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
    /// async {
    ///     use podman_api::Podman;
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    ///     if let Err(e) = podman.secrets().get("79c93f220e3e").delete().await {
    ///         eprintln!("{}", e);
    ///     }
    /// };
    /// ```
    |
    pub async fn delete(&self) -> Result<()> {
        self.podman
            .delete(&format!("/libpod/secrets/{}/", &self.id))
            .await
            .map(|_| ())
    }}
}

impl Secrets {
    api_doc! {
    Secret => ListLibpod
    /// List available secrets.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// async {
    ///     use podman_api::Podman;
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    ///     match podman.secrets().list().await {
    ///         Ok(info) => println!("{:?}", info),
    ///         Err(e) => eprintln!("{}", e),
    ///     }
    /// };
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
    /// async {
    ///     use podman_api::Podman;
    ///     use podman_api::opts::SecretCreateOpts;
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    ///     match podman.secrets().create(
    ///         &SecretCreateOpts::builder("my-secret").build(),
    ///         "secret-value"
    ///     ).await {
    ///         Ok(info) => println!("{:?}", info),
    ///         Err(e) => eprintln!("{}", e),
    ///     }
    /// };
    /// ```
    |
    pub async fn create(
        &self,
        opts: &opts::SecretCreateOpts,
        secret: impl Into<String>,
    ) -> Result<Secret> {
        let ep = url::construct_ep("/libpod/secrets/create", opts.serialize());
        self.podman
            .post_json(
                &ep,
                crate::conn::Payload::Json(serde_json::to_string(&secret.into())?),
            )
            .await
            .map(|resp: models::SecretCreateResponse| {
                Secret::new(self.podman.clone(), resp.id.unwrap_or_default())
            })
    }}
}
