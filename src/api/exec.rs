use crate::{
    conn::{tty, Headers, Payload},
    opts, Result,
};

use containers_api::url;
use futures_util::future::TryFutureExt;
use futures_util::stream::{Stream, TryStreamExt};

impl_api_ty!(Exec => id);

impl Exec {
    api_doc! {
    Exec => StartLibpod
    |
    /// Starts a previously set up exec instance. If `detach` is true, this endpoint returns
    /// immediately after starting the command. Otherwise, it sets up an interactive session
    /// with the command.
    ///
    /// To create an exec instance use [`Container::create_exec`](crate::api::Container::create_exec).
    ///
    /// Examples:
    ///
    /// ```no_run
    /// async {
    ///     use podman_api::Podman;
    ///     use futures_util::StreamExt;
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///     let container = podman.containers().get("451b27c6b9d3");
    ///
    ///     let exec = container
    ///         .create_exec(
    ///             &podman_api::opts::ExecCreateOpts::builder()
    ///                 .command(["cat", "/some/path/in/container"])
    ///                 .build(),
    ///         )
    ///         .await
    ///         .unwrap();
    ///
    ///     let opts = Default::default();
    ///     let mut stream = exec.start(&opts);
    ///
    ///     while let Some(chunk) = stream.next().await {
    ///         println!("{:?}", chunk.unwrap());
    ///     }
    /// };
    /// ```
    pub fn start<'exec>(
        &'exec self,
        opts: &'exec opts::ExecStartOpts,
    ) -> impl Stream<Item = crate::conn::Result<tty::TtyChunk>> + 'exec {
        let ep = format!("/libpod/exec/{}/start", &self.id);
        Box::pin(
            async move {
                let payload = Payload::Json(
                    opts.serialize()
                        .map_err(|e| crate::conn::Error::Any(Box::new(e)))?,
                );
                let stream = Box::pin(
                    self.podman
                        .post_stream(ep, payload, Headers::none())
                        .map_err(|e| crate::conn::Error::Any(Box::new(e))),
                );

                Ok(tty::decode(stream))
            }
            .try_flatten_stream(),
        )
    }}

    api_doc! {
    Exec => InspectLibpod
    |
    /// Returns low-level information about an exec instance.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// async {
    ///     use podman_api::Podman;
    ///     use futures_util::StreamExt;
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///     let container = podman.containers().get("451b27c6b9d3");
    ///
    ///     let exec = container
    ///         .create_exec(
    ///             &podman_api::opts::ExecCreateOpts::builder()
    ///                 .command(["cat", "/some/path/in/container"])
    ///                 .build(),
    ///         )
    ///         .await
    ///         .unwrap();
    ///
    ///     match exec.inspect().await {
    ///         Ok(info) => println!("{:?}", info),
    ///         Err(e) => eprintln!("{}", e)
    ///     }
    /// };
    /// ```
    pub async fn inspect(&self) -> Result<serde_json::Value> {
        let ep = format!("/libpod/exec/{}/json", &self.id);
        self.podman.get_json(&ep).await
    }}

    api_doc! {
    Exec => ResizeLibpod
    |
    /// Resize the TTY session used by an exec instance. This endpoint only works if
    /// tty was specified as part of creating and starting the exec instance.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// use futures_util::StreamExt;
    /// async {
    ///     use podman_api::Podman;
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///     let container = podman.containers().get("451b27c6b9d3");
    ///
    ///     let exec = container
    ///         .create_exec(
    ///             &podman_api::opts::ExecCreateOpts::builder()
    ///                 .command(["cat", "/some/path/in/container"])
    ///                 .build(),
    ///         )
    ///         .await
    ///         .unwrap();
    ///
    ///     if let Err(e) = exec.resize(1280, 720).await {
    ///         eprintln!("{}", e);
    ///     }
    /// };
    /// ```
    pub async fn resize(&self, width: usize, heigth: usize) -> Result<()> {
        let ep = url::construct_ep(
            format!("/libpod/exec/{}/resize", &self.id),
            Some(url::encoded_pairs([
                ("h", heigth.to_string()),
                ("w", width.to_string()),
            ])),
        );
        self.podman.get_json(&ep).await
    }}
}
