use crate::{
    api::ApiResource,
    conn::{Headers, Payload},
    models, opts, Result,
};

use containers_api::url;

impl_api_ty!(
    Manifest => name
);

impl Manifest {
    api_doc! {
    Manifest => ExistsLibpod
    |
    /// Quick way to determine if a manifest exists by name or id.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// async {
    ///     use podman_api::Podman;
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    ///     match podman.manifests().get("some-manifest").exists().await {
    ///         Ok(exists) => if exists {
    ///             println!("manifest exists!");
    ///         } else {
    ///             println!("manifest doesn't exists!");
    ///         },
    ///         Err(e) => eprintln!("check failed: {}", e),
    ///     }
    /// };
    /// ```
    pub async fn exists(&self) -> Result<bool> {
        self.podman
            .resource_exists(ApiResource::Manifests, &self.name)
            .await
    }}

    api_doc! {
    Manifest => InspectLibpod
    |
    /// Display details about this manifest list.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// async {
    ///     use podman_api::Podman;
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    ///     match podman.manifests().get("my-manifest").inspect().await {
    ///         Ok(info) => println!("{:?}", info),
    ///         Err(e) => eprintln!("{}", e),
    ///     }
    /// };
    /// ```
    pub async fn inspect(&self) -> Result<models::Schema2ListPublic> {
        self.podman
            .get_json(&format!("/libpod/manifests/{}/json", &self.name))
            .await
    }}

    api_doc! {
    Manifest => AddLibpod
    |
    /// Add an image to this manifest list.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// async {
    ///     use podman_api::Podman;
    ///     use podman_api::opts::ManifestImageAddOpts;
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    ///     let manifest = podman.manifests().get("my-manifest");
    ///     match manifest
    ///         .add_image(&ManifestImageAddOpts::builder().images(["centos"]).build())
    ///         .await
    ///     {
    ///         Ok(id) => println!("{:?}", id),
    ///         Err(e) => eprintln!("{}", e),
    ///     }
    /// };
    /// ```
    pub async fn add_image(&self, opts: &opts::ManifestImageAddOpts) -> Result<models::IdResponse> {
        self.podman
            .post_json(
                &format!("/libpod/manifests/{}/add", &self.name),
                Payload::Json(opts.serialize_vec()?),
                Headers::none(),
            )
            .await
    }}

    api_doc! {
    Manifest => DeleteLibpod
    |
    /// Remove an image digest from this manifest list.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// async {
    ///     use podman_api::Podman;
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    ///     match podman
    ///       .manifests()
    ///       .get("my-manifest")
    ///       .remove_image("sha256:a1801b843b1bfaf77c501e7a6d3f709401a1e0c83863037fa3aab063a7fdb9dc")
    ///       .await {
    ///         Ok(report) => println!("{:?}", report),
    ///         Err(e) => eprintln!("{}", e),
    ///     }
    /// };
    /// ```
    pub async fn remove_image(&self, digest: impl Into<String>) -> Result<models::ManifestRemoveReport> {
        let ep = url::construct_ep(
            format!("/libpod/manifests/{}", &self.name),
            Some(url::encoded_pair("digest", digest.into())),
        );

        self.podman.delete_json(&ep).await
    }}

    api_doc! {
    Manifest => PushLibpod
    |
    /// Push this manifest list to a registry.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// async {
    ///     use podman_api::Podman;
    ///     use podman_api::opts::ManifestPushOpts;
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    ///     let manifest = podman.manifests().get("my-manifest");
    ///     match manifest
    ///         .push(&ManifestPushOpts::builder("some-registry.addr").all(true).build())
    ///         .await
    ///     {
    ///         Ok(id) => println!("{:?}", id),
    ///         Err(e) => eprintln!("{}", e),
    ///     }
    /// };
    /// ```
    pub async fn push(&self, opts: &opts::ManifestPushOpts) -> Result<models::IdResponse> {
        let ep = url::construct_ep(
            format!("/libpod/manifests/{}/push", &self.name),
            opts.serialize(),
        );
        self.podman
            .post_json(&ep, Payload::empty(), Headers::none())
            .await
    }}

    api_doc! {
    Manifest => DeleteLibpod
    |
    /// Delete this manifest list.
    /// Examples:
    ///
    /// ```no_run
    /// async {
    ///     use podman_api::Podman;
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    ///     if let Err(e) = podman.manifests().get("my-manifest").delete().await {
    ///         eprintln!("{}", e);
    ///     }
    /// };
    /// ```
    pub async fn delete(&self) -> Result<()> {
        self.podman
            .delete(&format!("/libpod/manifests/{}", self.name))
            .await
            .map(|_| ())
    }}
}

impl Manifests {
    api_doc! {
    Manifest => CreateLibpod
    |
    /// Create a manifest list.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// async {
    ///     use podman_api::Podman;
    ///     use podman_api::opts::ManifestCreateOpts;
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    ///     match podman
    ///         .manifests()
    ///         .create(
    ///             &ManifestCreateOpts::builder("my-manifest")
    ///                 .images(["alpine"])
    ///                 .build(),
    ///         )
    ///         .await
    ///     {
    ///         Ok(manifest) => { /* do something with the manifest */ }
    ///         Err(e) => eprintln!("{}", e),
    ///     }
    /// };
    /// ```
    pub async fn create(&self, opts: &opts::ManifestCreateOpts) -> Result<Manifest> {
        let ep = url::construct_ep(format!("/libpod/manifests/{}", opts.name()), opts.serialize());
        self.podman
            .post_json(&ep, Payload::empty(), Headers::none())
            .await
            .map(|resp: models::IdResponse| self.podman.manifests().get(resp.id))
    }}
}
