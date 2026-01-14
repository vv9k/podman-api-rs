use crate::{
    api::ApiResource,
    conn::{Headers, Payload},
    models, opts, Error, Result, Stream, TryStreamExt,
};

use containers_api::{tarball, url};

impl_api_ty!(
    Image => id
);

impl Image {
    api_doc! {
    Image => InspectLibpod
    |
    /// Obtain low-level information about this image.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// async {
    ///     use podman_api::Podman;
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    ///     match podman.images().get("debian").inspect().await {
    ///         Ok(info) => println!("{:?}", info),
    ///         Err(e) => eprintln!("{}", e),
    ///     }
    /// };
    /// ```
    pub async fn inspect(&self) -> Result<models::InspectImageResponseLibpod> {
        self.podman
            .get_json(&format!("/libpod/images/{}/json", &self.id))
            .await
    }}

    api_doc! {
    Image => HistoryLibpod
    |
    /// Return parent layers of an image.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// async {
    ///     use podman_api::Podman;
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    ///     match podman.images().get("debian").history().await {
    ///         Ok(info) => println!("{:?}", info),
    ///         Err(e) => eprintln!("{}", e),
    ///     }
    /// };
    /// ```
    pub async fn history(&self) -> Result<Vec<models::HistoryResponse>> {
        self.podman
            .get_json(&format!("/libpod/images/{}/history", &self.id))
            .await
    }}

    api_doc! {
    Image => ExistsLibpod
    |
    /// Quick way to determine if a image exists by name or ID.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// async {
    ///     use podman_api::Podman;
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    ///     match podman.images().get("debian").exists().await {
    ///         Ok(exists) => if exists {
    ///             println!("image exists!");
    ///         } else {
    ///             println!("image doesn't exists!");
    ///         },
    ///         Err(e) => eprintln!("check failed: {}", e),
    ///     }
    /// };
    /// ```
    pub async fn exists(&self) -> Result<bool> {
        self.podman
            .resource_exists(ApiResource::Images, &self.id)
            .await
    }}

    api_doc! {
    Image => DeleteLibpod
    |
    /// Delete this image from local storage. To forcefully remove an image use
    /// [`Image::remove`](Image::remove).
    ///
    /// Examples:
    ///
    /// ```no_run
    /// async {
    ///     use podman_api::Podman;
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    ///     if let Err(e) = podman.images().get("debian").delete().await {
    ///         eprintln!("{}", e);
    ///     }
    /// };
    /// ```
    pub async fn delete(&self) -> Result<()> {
        self.podman
            .delete(&format!("/libpod/images/{}", &self.id))
            .await
            .map(|_| ())
    }}

    api_doc! {
    Image => DeleteLibpod
    |
    /// Remove this image forcefully from local storage. To remove the image normally use
    /// [`Image::delete`](Image::delete).
    ///
    /// Examples:
    ///
    /// ```no_run
    /// async {
    ///     use podman_api::Podman;
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    ///     if let Err(e) = podman.images().get("debian").remove().await {
    ///         eprintln!("{}", e);
    ///     }
    /// };
    /// ```
    pub async fn remove(&self) -> Result<()> {
        let ep = url::construct_ep(
            format!("/libpod/images/{}", &self.id),
            Some(url::encoded_pair("force", true)),
        );
        self.podman.delete(&ep).await.map(|_| ())
    }}

    api_doc! {
    Image => TagLibpod
    |
    /// Tag an image so that it becomes part of a repository.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// async {
    ///     use podman_api::Podman;
    ///     use podman_api::opts::ImageTagOpts;
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    ///     if let Err(e) = podman
    ///         .images()
    ///         .get("debian")
    ///         .tag(
    ///             &ImageTagOpts::builder()
    ///                 .repo("my.custom.repo/debian")
    ///                 .tag("1.0.0")
    ///                 .build(),
    ///         )
    ///         .await
    ///     {
    ///         eprintln!("{}", e);
    ///     }
    /// };
    /// ```
    pub async fn tag(&self, opts: &opts::ImageTagOpts) -> Result<()> {
        let ep = url::construct_ep(format!("/libpod/images/{}/tag", &self.id), opts.serialize());
        self.podman
            .post(&ep, Payload::empty(), Headers::none())
            .await
            .map(|_| ())
    }}

    api_doc! {
    Image => UntagLibpod
    |
    /// Untag an image. If repo and tag are not specified, all tags are removed
    /// from the image.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// async {
    ///     use podman_api::Podman;
    ///     use podman_api::opts::ImageTagOpts;
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    ///     if let Err(e) = podman
    ///         .images()
    ///         .get("debian")
    ///         .untag(
    ///             &ImageTagOpts::builder()
    ///                 .repo("my.custom.repo/debian")
    ///                 .tag("1.0.0")
    ///                 .build(),
    ///         )
    ///         .await
    ///     {
    ///         eprintln!("{}", e);
    ///     }
    /// };
    /// ```
    pub async fn untag(&self, opts: &opts::ImageTagOpts) -> Result<()> {
        let ep = url::construct_ep(format!("/libpod/images/{}/untag", &self.id), opts.serialize());
        self.podman
            .post(&ep, Payload::empty(), Headers::none())
            .await
            .map(|_| ())
    }}

    api_doc! {
    Image => GetLibpod
    |
    /// Export this image.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// async {
    ///     use podman_api::Podman;
    ///     use futures_util::stream::TryStreamExt;
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///     let image = podman.images().get("myimage");
    ///
    ///     let export_stream = image.export(&Default::default());
    ///     let export_data = export_stream.try_concat().await.expect("image archive");
    ///     assert!(!export_data.is_empty());
    /// };
    /// ```
    pub fn export(
        &self,
        opts: &opts::ImageExportOpts,
    ) -> impl Stream<Item = Result<Vec<u8>>> + Unpin + '_ {
        let ep = url::construct_ep(format!("/libpod/images/{}/get", &self.id), opts.serialize());
        Box::pin(self.podman.get_stream(ep).map_ok(|c| c.to_vec()))
    }}

    api_doc! {
    Image => ChangesLibpod
    |
    /// Returns which files in this image's filesystem have been added, deleted, or modified.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// async {
    ///     use podman_api::Podman;
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    ///     match podman
    ///         .images()
    ///         .get("79c93f220e3e")
    ///         .changes(&Default::default())
    ///         .await
    ///     {
    ///         Ok(changes) => println!("{:?}", changes),
    ///         Err(e) => eprintln!("{}", e),
    ///     }
    /// };
    /// ```
    pub async fn changes(
        &self,
        opts: &opts::ChangesOpts,
    ) -> Result<Vec<models::FilesystemChange>> {
        let ep = url::construct_ep(
            format!("/libpod/images/{}/changes", &self.id),
            opts.serialize(),
        );
        self.podman.get_json(&ep).await
    }}

    api_doc! {
    Image => TreeLibpod
    |
    /// Retrieve the image tree for this image.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// async {
    ///     use podman_api::Podman;
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    ///     match podman
    ///         .images()
    ///         .get("79c93f220e3e")
    ///         .tree(&Default::default())
    ///         .await
    ///     {
    ///         Ok(tree) => println!("{:?}", tree),
    ///         Err(e) => eprintln!("{}", e),
    ///     }
    /// };
    /// ```
    pub async fn tree(&self, opts: &opts::ImageTreeOpts) -> Result<models::TreeResponse> {
        let ep = url::construct_ep(
            format!("/libpod/images/{}/tree", &self.id),
            opts.serialize(),
        );
        self.podman.get_json(&ep).await
    }}

    api_doc! {
    Image => PushLibpod
    |
    /// Push this image to a container registry.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// async {
    ///     use podman_api::opts::{ImagePushOpts, RegistryAuth};
    ///     use podman_api::Podman;
    ///     use futures_util::StreamExt;
    ///
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    ///     let image = podman.images().get("alpine");
    ///     let mut events = image.push(
    ///         &ImagePushOpts::builder()
    ///             .destination("my-destination")
    ///             .tls_verify(true)
    ///             .auth(
    ///                 RegistryAuth::builder()
    ///                     .username("test")
    ///                     .password("test")
    ///                     .server_address("https://my-registry")
    ///                     .build(),
    ///             )
    ///             .build(),
    ///     );
    ///
    ///     for event in events.next().await {
    ///         match event {
    ///             Ok(line) => println!("{line}"),
    ///             Err(e) => eprintln!("{e}"),
    ///         }
    ///     }
    /// };
    /// ```
    pub fn push(&self, opts: &opts::ImagePushOpts) -> impl Stream<Item = Result<String>> + Unpin + '_ {
        let headers = opts
            .auth_header()
            .map(|a| Headers::single(crate::conn::AUTH_HEADER, a));

        let ep = url::construct_ep(
            format!("/libpod/images/{}/push", &self.id),
            opts.serialize(),
        );

        let reader = Box::pin(
            self.podman
                .post_stream(
                    ep,
                    Payload::empty(),
                    headers,
                )
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e)),
        )
        .into_async_read();

        Box::pin(
            futures_codec::FramedRead::new(reader, futures_codec::LinesCodec)
                .map_err(Error::IO)
        )
    }}
}

impl Images {
    api_doc! {
    Image => BuildLibpod
    |
    /// Build an image from the given Dockerfile(s)
    ///
    /// Examples:
    ///
    /// ```no_run
    /// async {
    ///     use podman_api::Podman;
    ///     use futures_util::StreamExt;
    ///     use podman_api::opts::ImageBuildOpts;
    ///
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    ///     let opts = ImageBuildOpts::builder("http://some.url.to/Dockerfile")
    ///             .tag("myimage:1.0.0")
    ///             .build();
    ///
    ///     let images = podman.images();
    ///     match images.build(&opts) {
    ///         Ok(mut build_stream) => while let Some(chunk) = build_stream.next().await {
    ///             match chunk {
    ///                 Ok(chunk) => println!("{:?}", chunk),
    ///                 Err(e) => eprintln!("{}", e),
    ///             }
    ///         },
    ///         Err(e) => eprintln!("{}", e),
    ///     };
    /// };
    /// ```
    pub fn build(
        &self,
        opts: &opts::ImageBuildOpts,
    ) -> Result<impl Stream<Item = Result<models::ImageBuildLibpod200Response>> + Unpin + '_> {
        let mut bytes = Vec::default();
        let path = opts
            .get_param("path")
            .ok_or_else(|| Error::OptsSerialization("expected a path to build context".into()))?;
        tarball::dir(&mut bytes, path)?;

        let ep = url::construct_ep("/libpod/build", opts.serialize());
        let reader = Box::pin(
            self.podman
                .post_stream(ep, Payload::Tar(bytes), Headers::none())
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e)),
        )
        .into_async_read();

        Ok(Box::pin(
            futures_codec::FramedRead::new(reader, futures_codec::LinesCodec)
                .map_err(Error::IO)
                .and_then(|s: String| async move {
                    match serde_json::from_str(&s) {
                        Ok(s) => Ok(s),
                        Err(e) => match serde_json::from_str::<models::JsonError>(&s) {
                            Ok(e) => Err(Error::ServerError(e)),
                            Err(_) => Err(e.into())
                        }
                    }
                }),
        ))
    }}

    api_doc! {
    Image => ListLibpod
    |
    /// Returns a list of images.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// async {
    ///     use podman_api::Podman;
    ///     use podman_api::opts::{ImageListOpts, ImageListFilter};
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    ///     for image in podman
    ///         .images()
    ///         .list(
    ///             &ImageListOpts::builder()
    ///                 .all(true)
    ///                 .filter([ImageListFilter::Dangling(true)])
    ///                 .build(),
    ///         )
    ///         .await
    ///         .unwrap()
    ///     {
    ///         println!("{:?}", image);
    ///     }
    /// };
    /// ```
    pub async fn list(
        &self,
        opts: &opts::ImageListOpts,
    ) -> Result<Vec<models::LibpodImageSummary>> {
        let ep = url::construct_ep("/libpod/images/json", opts.serialize());
        self.podman.get_json(&ep).await
    }}

    api_doc! {
    Image => PullLibpod
    |
    /// Pull one or more images from a container registry.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// async {
    ///     use futures_util::{StreamExt, TryStreamExt};
    ///     use podman_api::{Error, Podman};
    ///     use podman_api::opts::PullOpts;
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    ///     let events = podman
    ///         .images()
    ///         .pull(
    ///             &PullOpts::builder()
    ///                 .reference("docker.io/library/alpine")
    ///                 .build(),
    ///             )
    ///             .map(|report| {
    ///                 report.and_then(|report| match report.error {
    ///                     Some(error) => Err(Error::InvalidResponse(error)),
    ///                     None => Ok(report),
    ///                 })
    ///             })
    ///             .try_collect::<Vec<_>>()
    ///             .await;
    ///
    ///     if let Err(e) = events {
    ///         eprintln!("{}", e);
    ///     }
    /// };
    /// ```
    pub fn pull(
        &self,
        opts: &opts::PullOpts,
    ) -> impl Stream<Item = Result<models::LibpodImagesPullReport>> + Unpin + '_ {
        let ep = url::construct_ep("/libpod/images/pull", opts.serialize());
        let reader = Box::pin(
            self.podman
                .post_stream(
                    ep,
                    Payload::empty(),
                    opts.auth_header()
                        .map(|a| Headers::single(crate::conn::AUTH_HEADER, a)),
                )
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e)),
        )
        .into_async_read();

        Box::pin(
            futures_codec::FramedRead::new(reader, futures_codec::LinesCodec)
                .map_err(Error::IO)
                .and_then(|s: String| async move {
                    serde_json::from_str(&s).map_err(Error::SerdeJsonError)
                }),
        )
    }}

    api_doc! {
    Image => LoadLibpod
    |
    /// Load an image (oci-archive or docker-archive) stream.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// async {
    ///     use podman_api::Podman;
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    ///     let image = std::fs::read("image_archive").unwrap();
    ///
    ///     match podman.images().load(&image).await {
    ///         Ok(info) => println!("{:?}", info),
    ///         Err(e) => eprintln!("{}", e),
    ///     }
    /// };
    /// ```
    pub async fn load(&self, image: impl AsRef<[u8]>) -> Result<models::ImageLoadReport> {
        let archive = image.as_ref().to_vec();
        self.podman
            .post_json(
                "/libpod/images/load",
                Payload::XTar(archive),
                Headers::none(),
            )
            .await
    }}

    api_doc! {
    Image => ImportLibpod
    |
    /// Import a previously exported tarball as an image.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// async {
    ///     use podman_api::Podman;
    ///     use podman_api::opts::ImageImportOpts;
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    ///     let image = vec![0, 1];
    ///
    ///     if let Err(e) = podman
    ///         .images()
    ///         .import(
    ///             &ImageImportOpts::builder()
    ///                 .reference("rockylinux/rockylinux:8")
    ///                 .build(),
    ///             image
    ///         )
    ///         .await
    ///     {
    ///         eprintln!("{}", e);
    ///     }
    /// };
    /// ```
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
                Headers::none(),
            )
            .await
    }}

    api_doc! {
    Image => DeleteAllLibpod
    |
    /// Remove multiple images. To remove a single image use
    /// [`Image::delete`](Image::delete) or [`Image::remove`](Image::remove).
    ///
    /// Examples:
    ///
    /// ```no_run
    /// async {
    ///     use podman_api::Podman;
    ///     use podman_api::opts::ImagesRemoveOpts;
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    ///     match podman
    ///         .images()
    ///         .remove(&ImagesRemoveOpts::builder().all(true).force(true).build())
    ///         .await
    ///     {
    ///         Ok(info) => println!("{:?}", info),
    ///         Err(e) => eprintln!("{}", e),
    ///     }
    /// };
    /// ```
    pub async fn remove(
        &self,
        opts: &opts::ImagesRemoveOpts,
    ) -> Result<models::LibpodImagesRemoveReport> {
        let ep = url::construct_ep("/libpod/images/remove", opts.serialize());
        self.podman.delete_json(&ep).await
    }}

    api_doc! {
    Image => PruneLibpod
    |
    /// Remove images that are not being used by a container.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// async {
    ///     use podman_api::Podman;
    ///     use podman_api::opts::ImagePruneOpts;
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    ///     match podman
    ///         .images()
    ///         .prune(
    ///             &ImagePruneOpts::builder()
    ///                 .all(true)
    ///                 .build()
    ///         ).await {
    ///             Ok(report) => println!("{:?}", report),
    ///             Err(e) => eprintln!("{}", e),
    ///     }
    /// };
    /// ```
    pub async fn prune(
        &self,
        opts: &opts::ImagePruneOpts,
    ) -> Result<Option<Vec<models::PruneReport>>> {
        let ep = url::construct_ep("/libpod/images/prune", opts.serialize());
        self.podman
            .post_json(&ep, Payload::empty(), Headers::none())
            .await
    }}

    api_doc! {
    Image => SearchLibpod
    |
    /// Search registries for images.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// async {
    ///     use podman_api::Podman;
    ///     use podman_api::opts::ImageSearchOpts;
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    ///     match podman
    ///         .images()
    ///         .search(
    ///             &ImageSearchOpts::builder()
    ///                 .list_tags(true)
    ///                 .build()
    ///         ).await {
    ///             Ok(images) => println!("{:?}", images),
    ///             Err(e) => eprintln!("{}", e),
    ///     }
    /// };
    /// ```
    pub async fn search(
        &self,
        opts: &opts::ImageSearchOpts,
    ) -> Result<Vec<models::RegistrySearchResponse>> {
        let ep = url::construct_ep("/libpod/images/search", opts.serialize());
        self.podman.get_json(&ep).await
    }}

    api_doc! {
    Image => ExportLibpod
    |
    /// Export multiple images into a single object.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// async {
    ///     use podman_api::Podman;
    ///     use podman_api::opts::ImagesExportOpts;
    ///     use futures_util::stream::TryStreamExt;
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///     let images = podman.images();
    ///
    ///     let full_id_a = "3290fj209...".to_string();
    ///     let full_id_b = "ioajfoi32...".to_string();
    ///
    ///     let export_opts = ImagesExportOpts::builder()
    ///         .references([full_id_a, full_id_b])
    ///         .build();
    ///
    ///     let export_stream = images.export(&export_opts);
    ///     let export_data = export_stream.try_concat().await.expect("images archive");
    ///     assert!(!export_data.is_empty());
    /// };
    /// ```
    pub fn export(&self, opts: &opts::ImagesExportOpts) -> impl Stream<Item = Result<Vec<u8>>> + Unpin + '_ {
        let ep = url::construct_ep("/libpod/images/export", opts.serialize());
        Box::pin(self.podman.get_stream(ep).map_ok(|c| c.to_vec()))
    }}
}
