use crate::{
    api::ApiResource,
    conn::{Headers, Payload},
    models, opts,
    util::url,
    Result,
};

use futures_util::stream::{Stream, TryStreamExt};

impl_api_ty!(
    Image => id
);

impl<'podman> Image<'podman> {
    api_doc! {
    Image => InspectLibpod
    /// Obtain low-level information about this image.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    /// match podman.images().get("debian").inspect().await {
    ///     Ok(info) => println!("{:?}", info),
    ///     Err(e) => eprintln!("{}", e),
    /// }
    /// ```
    |
    pub async fn inspect(&self) -> Result<models::LibpodImageInspectResponse> {
        self.podman
            .get_json(&format!("/libpod/images/{}/json", &self.id))
            .await
    }}

    api_doc! {
    Image => HistoryLibpod
    /// Return parent layers of an image.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    /// match podman.images().get("debian").history().await {
    ///     Ok(info) => println!("{:?}", info),
    ///     Err(e) => eprintln!("{}", e),
    /// }
    /// ```
    |
    pub async fn history(&self) -> Result<models::HistoryResponseItem> {
        self.podman
            .get_json(&format!("/libpod/images/{}/history", &self.id))
            .await
    }}

    api_doc! {
    Image => ExistsLibpod
    /// Quick way to determine if a image exists by name or ID.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    /// match podman.images().get("debian").exists().await {
    ///     Ok(exists) => if exists {
    ///         println!("image exists!");
    ///     } else {
    ///         println!("image doesn't exists!");
    ///     },
    ///     Err(e) => eprintln!("check failed: {}", e),
    /// }
    /// ```
    |
    pub async fn exists(&self) -> Result<bool> {
        self.podman
            .resource_exists(ApiResource::Images, &self.id)
            .await
    }}

    api_doc! {
    Image => DeleteLibpod
    /// Delete this image from local storage. To forcefully remove an image use
    /// [`Image::remove`](Image::remove).
    ///
    /// Examples:
    ///
    /// ```no_run
    /// let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    /// if let Err(e) = podman.images().get("debian").delete().await
    ///     eprintln!("{}", e);
    /// }
    /// ```
    |
    pub async fn delete(&self) -> Result<()> {
        self.podman.delete(&format!("/libpod/images/{}", &self.id)).await.map(|_| ())
    }}

    api_doc! {
    Image => DeleteLibpod
    /// Remove this image forcefully from local storage. To remove the image normally use
    /// [`Image::delete`](Image::delete).
    ///
    /// Examples:
    ///
    /// ```no_run
    /// let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    /// if let Err(e) = podman.images().get("debian").remove().await
    ///     eprintln!("{}", e);
    /// }
    /// ```
    |
    pub async fn remove(&self) -> Result<()> {
        let ep = url::construct_ep(
            format!("/libpod/images/{}", &self.id),
            Some(url::encoded_pair("force", true)),
        );
        self.podman.delete(&ep).await.map(|_| ())
    }}

    api_doc! {
    Image => TagLibpod
    /// Tag an image so that it becomes part of a repository.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    /// if let Err(e) = podman
    ///     .images()
    ///     .get("debian")
    ///     .tag(
    ///         &ImageTagOpts::builder()
    ///             .repo("my.custom.repo/debian")
    ///             .tag("1.0.0")
    ///             .build(),
    ///     )
    ///     .await
    ///     .unwrap()
    /// {
    ///     println!("{:?}", image);
    /// }
    /// ```
    |
    pub async fn tag(&self, opts: &opts::ImageTagOpts) -> Result<()> {
        let ep = url::construct_ep(
            format!("/libpod/images/{}/tag", &self.id),
            opts.serialize()
        );
        self.podman.post(&ep, Payload::empty()).await.map(|_| ())
    }}

    api_doc! {
    Image => UntagLibpod
    /// Untag an image. If repo and tag are not specified, all tags are removed
    /// from the image.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    /// if let Err(e) = podman
    ///     .images()
    ///     .get("debian")
    ///     .untag(
    ///         &ImageTagOpts::builder()
    ///             .repo("my.custom.repo/debian")
    ///             .tag("1.0.0")
    ///             .build(),
    ///     )
    ///     .await
    ///     .unwrap()
    /// {
    ///     println!("{:?}", image);
    /// }
    /// ```
    |
    pub async fn untag(&self, opts: &opts::ImageTagOpts) -> Result<()> {
        let ep = url::construct_ep(
            format!("/libpod/images/{}/tag", &self.id),
            opts.serialize()
        );
        self.podman.post(&ep, Payload::empty()).await.map(|_| ())
    }}

    api_doc! {
    Image => GetLibpod
    /// Export this image.
    |
    pub fn export(
        &self,
        opts: &opts::ImageExportOpts,
    ) -> impl Stream<Item = Result<Vec<u8>>> + Unpin + 'podman {
        let ep = url::construct_ep(format!("/libpod/images/{}/get", &self.id), opts.serialize());
        Box::pin(self.podman.stream_get(ep).map_ok(|c| c.to_vec()))
    }}

    api_doc! {
    Image => ChangesLibpod
    /// Returns which files in this image's filesystem have been added, deleted, or modified.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    /// match podman
    ///     .images()
    ///     .get("79c93f220e3e")
    ///     .changes(&Default::default())
    ///     .await
    /// {
    ///     Ok(changes) => println!("{:?}", changes),
    ///     Err(e) => eprintln!("{}", e),
    /// }
    /// ```
    |
    pub async fn changes(
        &self,
        opts: &opts::ChangesOpts,
    ) -> Result<Vec<models::ContainerChangeResponseItem>> {
        let ep = url::construct_ep(
            &format!("/libpod/images/{}/changes", &self.id),
            opts.serialize(),
        );
        self.podman.get_json(&ep).await
    }}

    api_doc! {
    Image => TreeLibpod
    /// Retrieve the image tree for this image.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    /// match podman
    ///     .images()
    ///     .get("79c93f220e3e")
    ///     .tree(&Default::default())
    ///     .await
    /// {
    ///     Ok(tree) => println!("{:?}", tree),
    ///     Err(e) => eprintln!("{}", e),
    /// }
    /// ```
    |
    pub async fn tree(
        &self,
        opts: &opts::ImageTreeOpts,
    ) -> Result<Vec<models::LibpodImageTreeResponse>> {
        let ep = url::construct_ep(
            format!("/libpod/images/{}/tree", &self.id),
            opts.serialize(),
        );
        self.podman.get_json(&ep).await
    }}

    api_doc! {
    Image => PushLibpod
    /// Push this image to a container registry.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    /// match podman.images().get("alpine").push(
    ///     &ImagePushOpts::builder()
    ///         .destinations("my-destination")
    ///         .tls_verify(true)
    ///         .auth(Some(
    ///             RegistryAuth::builder()
    ///                 .username("test")
    ///                 .password("test")
    ///                 .server_address("https://my-registry")
    ///                 .build(),
    ///         ))
    ///         .build(),
    /// ) {
    ///     Ok(s) => println!("{}", s),
    ///     Err(e) => eprintln!("{}", e),
    /// };
    /// ```
    |
    pub async fn push(&self, opts: &opts::ImagePushOpts) -> Result<String> {
        let headers = opts
            .auth_header()
            .map(|a| Headers::single(crate::conn::AUTH_HEADER, a));

        let ep = url::construct_ep(
            format!("/libpod/images/{}/push", &self.id),
            opts.serialize(),
        );

        self.podman
            .post_headers(&ep, Payload::empty(), headers)
            .await
    }}
}

impl<'podman> Images<'podman> {
    api_doc! {
    Image => BuildLibpod
    /// Build an image from the given Dockerfile(s)
    ///
    /// Examples:
    ///
    /// ```no_run
    /// let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    /// if let Err(e) = podman
    ///     .images()
    ///     .create(
    ///         &ImageBuildOpts::builder()
    ///             .remote("http://some.url.to/Dockerfile")
    ///             .tag("myimage:1.0.0")
    ///             .build(),
    ///     )
    ///     .await
    /// {
    ///     eprintln!("{}", e);
    /// }
    /// ```
    |
    pub async fn build(
        &self,
        opts: &opts::ImageBuildOpts,
    ) -> Result<models::LibpodImageBuildResponse> {
        let ep = url::construct_ep("/libpod/build", opts.serialize());
        self.podman
            .post_json(
                &ep,
                Payload::empty(),
            )
            .await
    }}

    api_doc! {
    Image => ListLibpod
    /// Returns a list of images.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    /// for image in podman
    ///     .images()
    ///     .list(
    ///         &ImageListOpts::builder()
    ///             .all(true)
    ///             .filter([ImageListFilter::Dangling(true)])
    ///             .build(),
    ///     )
    ///     .await
    ///     .unwrap()
    /// {
    ///     println!("{:?}", image);
    /// }
    /// ```
    |
    pub async fn list(&self, opts: &opts::ImageListOpts) -> Result<Vec<models::LibpodImageSummary>> {
        let ep = url::construct_ep("/libpod/images/json", opts.serialize());
        self.podman.get_json(&ep).await
    }}

    api_doc! {
    Image => PullLibpod
    /// Pull one or more images from a container registry.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    /// if let Err(e) = podman
    ///     .images()
    ///     .pull(
    ///         &PullOpts::builder()
    ///             .reference("rockylinux/rockylinux:8")
    ///             .build(),
    ///     )
    ///     .await
    /// {
    ///     eprintln!("{}", e);
    /// }
    |
    pub async fn pull(
        &self,
        opts: &opts::PullOpts,
    ) -> Result<models::LibpodImagesPullReport> {
        let headers = opts
            .auth_header()
            .map(|a| Headers::single(crate::conn::AUTH_HEADER, a));

        self.podman.post_json_headers(
            url::construct_ep("/libpod/images/pull", opts.serialize()),
            Payload::empty(),
            headers,
        ).await
    }}

    api_doc! {
    Image => LoadLibpod
    /// Load an image (oci-archive or docker-archive) stream.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    /// let image = std::fs::read("image_archive")?;
    ///
    /// match podman.images().load(&image).await {
    ///     Ok(info) => println!("{:?}", info),
    ///     Err(e) => eprintln!("{}", e),
    /// }
    /// ```
    |
    pub async fn load(&self, image: impl AsRef<[u8]>) -> Result<models::ImageLoadReport> {
        let archive = image.as_ref().to_vec();
        self.podman
            .post_json("/libpod/images/load", Payload::XTar(archive))
            .await
    }}

    api_doc! {
    Image => ImportLibpod
    /// Import a previously exported tarball as an image.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    /// if let Err(e) = podman
    ///     .images()
    ///     .pull(
    ///         &PullOpts::builder()
    ///             .reference("rockylinux/rockylinux:8")
    ///             .build(),
    ///     )
    ///     .await
    /// {
    ///     eprintln!("{}", e);
    /// }
    |
    pub async fn import(
        &self,
        opts: &opts::ImageImportOpts,
        image: impl AsRef<[u8]>,
    ) -> Result<models::LibpodImagesPullReport> {
        let archive = image.as_ref().to_vec();
        self.podman
            .post_json(
                url::construct_ep("/libpod/images/import", opts.serialize()),
                Payload::XTar(archive),
            )
            .await
    }}

    api_doc! {
    Image => DeleteAllLibpod
    /// Remove multiple images. To remove a single image use
    /// [`Image::delete`](Image::delete) or [`Image::remove`](Image::remove).
    ///
    /// Examples:
    ///
    /// ```no_run
    /// let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    /// match podman
    ///     .images()
    ///     .remove_multiple(&ImagesRemoveOpts::builder().all(true).force(true).build())
    ///     .await
    /// {
    ///     Ok(info) => println!("{:?}", info),
    ///     Err(e) => eprintln!("{}", e),
    /// }
    /// ```
    |
    pub async fn remove(
        &self,
        opts: &opts::ImagesRemoveOpts,
    ) -> Result<models::LibpodImagesRemoveReport> {
        let ep = url::construct_ep("/libpod/images/remove", opts.serialize());
        self.podman.delete_json(&ep).await
    }}

    api_doc! {
    Image => PruneLibpod
    /// Remove images that are not being used by a container.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    /// match podman
    ///     .images()
    ///     .prune(
    ///         &ImagePruneOpts::builder()
    ///             .all(true)
    ///             .build()
    ///     ).await {
    ///         Ok(report) => println!("{:?}", report),
    ///         Err(e) => eprintln!("{}", e),
    /// }
    /// ```
    |
    pub async fn prune(
        &self,
        opts: &opts::ImagePruneOpts,
    ) -> Result<Vec<models::ContainersPruneReport>> {
        let ep = url::construct_ep("/libpod/images/prune", opts.serialize());
        self.podman.post_json(&ep, Payload::empty()).await
    }}
}
