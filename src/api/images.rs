use crate::{conn::Payload, models, opts, util::url, Result};

impl_api_ty!(
    Image => id
);

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
    ///     .containers()
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
}
