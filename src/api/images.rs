use crate::{conn::Payload, models, opts, util::url, Result};

impl_api_ty!(
    Image => id
);

impl<'podman> Image<'podman> {
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
    ///     Err(e) => eprintln!("check failed: {}", e);
    /// }
    /// ```
    |
    pub async fn exists(&self) -> Result<bool> {
        let ep = format!("/libpod/images/{}/exists", &self.id);
        match self.podman.get(&ep).await {
            Ok(_) => Ok(true),
            Err(e) => match e {
                crate::Error::Fault {
                    code: http::StatusCode::NOT_FOUND,
                    message: _,
                } => Ok(false),
                e => Err(e),
            },
        }
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
}
