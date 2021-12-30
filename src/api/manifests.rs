use crate::{api::ApiResource, conn::Payload, models, opts, util::url, Result};

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

    api_doc! {
    Manifest => InspectLibpod
    /// Display details about this manifest list.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    /// match podman.manifests().get("my-manifest").inspect().await {
    ///     Ok(info) => println!("{:?}", info),
    ///     Err(e) => eprintln!("{}", e),
    /// }
    /// ```
    |
    pub async fn inspect(&self) -> Result<models::Schema2List> {
        self.podman
            .get_json(&format!("/libpod/manifests/{}/json", &self.id))
            .await
    }}

    api_doc! {
    Manifest => AddLibpod
    /// Add an image to this manifest list.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    /// let manifest = podman.manifests().get("my-manifest");
    /// match manifest
    ///     .add_image(&ManifestImageAddOpts::builder().images(["centos"]).build())
    ///     .await
    /// {
    ///     Ok(id) => println!("{:?}", id),
    ///     Err(e) => eprintln!("{}", e),
    /// }
    /// ```
    |
    pub async fn add_image(&self, opts: &opts::ManifestImageAddOpts) -> Result<models::IdResponse> {
        self.podman
            .post_json(
                &format!("/libpod/manifests/{}/add", &self.id),
                Payload::Json(opts.serialize()?),
            )
            .await
    }}
}

impl<'podman> Manifests<'podman> {
    api_doc! {
    Manifest => CreateLibpod
    /// Create a manifest list.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    /// match podman
    ///     .manifests()
    ///     .create(
    ///         &ManifestCreateOpts::builder("my-manifest")
    ///             .image("alpine")
    ///             .build(),
    ///     )
    ///     .await
    /// {
    ///     Ok(manifest) => { /* do something with the manifest */ }
    ///     Err(e) => eprintln!("{}", e),
    /// }
    /// ```
    |
    pub async fn create(&self, opts: &opts::ManifestCreateOpts) -> Result<Manifest<'_>> {
        let ep = url::construct_ep("/libpod/manifests/create", opts.serialize());
        self.podman
            .post_json(&ep, Payload::empty())
            .await
            .map(|resp: models::IdResponse| self.podman.manifests().get(resp.id))
    }}
}
