use crate::{api::ApiResource, conn::Payload, models, opts, util::url, Result};

impl_api_ty!(
    Volume => name
);

impl Volume {
    api_doc! {
    Volume => ExistsLibpod
    /// Quick way to determine if this volume exists.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// async {
    ///     use podman_api::Podman;
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    ///     match podman.volumes().get("some_vol").exists().await {
    ///         Ok(exists) => if exists {
    ///             println!("volume exists!");
    ///         } else {
    ///             println!("volume doesn't exists!");
    ///         },
    ///         Err(e) => eprintln!("check failed: {}", e),
    ///     }
    /// };
    /// ```
    |
    pub async fn exists(&self) -> Result<bool> {
        self.podman
            .resource_exists(ApiResource::Volumes, &self.name)
            .await
    }}

    api_doc! {
    Volume => InspectLibpod
    /// Obtain low-level information about this volume.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// async {
    ///     use podman_api::Podman;
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    ///     match podman.volumes().get("my-vol").inspect().await {
    ///         Ok(info) => println!("{:?}", info),
    ///         Err(e) => eprintln!("{}", e),
    ///     }
    /// };
    /// ```
    |
    pub async fn inspect(&self) -> Result<models::LibpodVolumeInspectResponse> {
        self.podman
            .get_json(&format!("/libpod/volumes/{}/json", &self.name))
            .await
    }}

    api_doc! {
    Volume => DeleteLibpod
    /// Delete this volume. To forcefully remove an volume use
    /// [`Volume::remove`](Volume::remove).
    ///
    /// Examples:
    ///
    /// ```no_run
    /// async {
    ///     use podman_api::Podman;
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    ///     if let Err(e) = podman.volumes().get("my-vol").delete().await {
    ///         eprintln!("{}", e);
    ///     }
    /// };
    /// ```
    |
    pub async fn delete(&self) -> Result<()> {
        self.podman.delete(&format!("/libpod/volumes/{}", &self.name)).await.map(|_| ())
    }}

    api_doc! {
    Volume => DeleteLibpod
    /// Remove this volume forcefully. To remove the volume normally use
    /// [`Volume::delete`](Volume::delete).
    ///
    /// Examples:
    ///
    /// ```no_run
    /// async {
    ///     use podman_api::Podman;
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    ///     if let Err(e) = podman.volumes().get("my-vol").remove().await {
    ///         eprintln!("{}", e);
    ///     }
    /// };
    /// ```
    |
    pub async fn remove(&self) -> Result<()> {
        let ep = url::construct_ep(
            format!("/libpod/volumes/{}", &self.name),
            Some(url::encoded_pair("force", true)),
        );
        self.podman.delete(&ep).await.map(|_| ())
    }}
}

impl Volumes {
    api_doc! {
    Volume => CreateLibpod
    /// Create a volume with specified options.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// async {
    ///     use podman_api::Podman;
    ///     use podman_api::opts::VolumeCreateOpts;
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    ///     match podman
    ///         .volumes()
    ///         .create(
    ///             &VolumeCreateOpts::builder()
    ///                 .driver("my-driver")
    ///                 .name("my-vol")
    ///                 .build(),
    ///         )
    ///         .await
    ///     {
    ///         Ok(info) => println!("{:?}", info),
    ///         Err(e) => eprintln!("{}", e),
    ///     }
    /// };
    /// ```
    |
    pub async fn create(
        &self,
        opts: &opts::VolumeCreateOpts,
    ) -> Result<models::LibpodContainerInspectResponse> {
        self.podman
            .post_json(
                "/libpod/volumes/create",
                Payload::Json(opts.serialize()?),
            )
            .await
    }}

    api_doc! {
    Volume => ListLibpod
    /// Returns a list of volumes.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// async {
    ///     use podman_api::Podman;
    ///     use podman_api::opts::{VolumeListOpts, VolumeListFilter};
    ///
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    ///     for volume in podman
    ///         .volumes()
    ///         .list(
    ///             &VolumeListOpts::builder()
    ///                 .filter([VolumeListFilter::Driver("my-sd".into())])
    ///                 .build(),
    ///         )
    ///         .await
    ///         .unwrap()
    ///     {
    ///         println!("{:?}", volume);
    ///     }
    /// };
    /// ```
    |
    pub async fn list(&self, opts: &opts::VolumeListOpts) -> Result<Vec<models::Volume>> {
        let ep = url::construct_ep("/libpod/volumes/json", opts.serialize());
        self.podman.get_json(&ep).await
    }}

    api_doc! {
    Volume => PruneLibpod
    /// Delete unused volumes.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// async {
    ///     use podman_api::Podman;
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    ///     match podman.volumes().prune(&Default::default()).await {
    ///         Ok(info) => println!("{:?}", info),
    ///         Err(e) => eprintln!("{}", e),
    ///     }
    /// };
    /// ```
    |
    pub async fn prune(&self, opts: &opts::VolumePruneOpts) -> Result<Vec<models::PruneReport>> {
        let ep = url::construct_ep("/libpod/volumes/prune", opts.serialize());
        self.podman.post_json(&ep, Payload::empty()).await
    }}
}
