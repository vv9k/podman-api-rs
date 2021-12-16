use crate::{conn::Payload, models, opts, util::url, Result};

impl_api_ty!(
    Container => id
);

impl<'podman> Container<'podman> {
    api_doc! {
    Container => StartLibpod
    /// Start this container.
    ///
    /// Parameters:
    ///  * detach_keys - Override the key sequence for detaching a container. Format is a single
    ///                  character [a-Z] or ctrl- where is one of: a-z, @, ^, [, , or _.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    /// if let Err(e) = podman.containers().get("79c93f220e3e").start(None).await {
    ///     eprintln!("{}", e);
    /// }
    /// ```
    |
    pub async fn start(&self, detach_keys: Option<String>) -> Result<()> {
        let ep = url::construct_ep(&format!("/libpod/containers/{}/start", &self.id), detach_keys.map(|d| url::encoded_pair("detachKeys", d)));
        self.podman.post(&ep, Payload::None::<&str>).await.map(|_| ())
    }}

    api_doc! {
    Container => StopLibpod
    /// Stop this container.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    /// if let Err(e) = podman.containers().get("79c93f220e3e").stop(&Default::default()).await {
    ///     eprintln!("{}", e);
    /// }
    /// ```
    |
    pub async fn stop(&self, opts: &opts::ContainerStopOpts) -> Result<()> {
        let ep = url::construct_ep(&format!("/libpod/containers/{}/stop", &self.id), opts.serialize());
        self.podman.post(&ep, Payload::None::<&str>).await.map(|_| ())
    }}

    api_doc! {
    Container => InspectLibpod
    /// Return low-level information about this container.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    /// match podman.containers().get("79c93f220e3e").inspect().await {
    ///     Ok(info) => println!("{:?}", info),
    ///     Err(e) => eprintln!("{}", e);
    /// }
    /// ```
    |
    pub async fn inspect(&self) -> Result<models::LibpodContainerInspectResponse> {
        let ep = url::construct_ep(&format!("/libpod/containers/{}/json", &self.id), Some(url::encoded_pair("size", "true")));
        self.podman.get_json(&ep).await
    }}

    api_doc! {
    Container => KillLibpod
    /// Send a signal to this container, defaults to killing the container
    ///
    /// Examples:
    ///
    /// ```no_run
    /// let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    /// if let Err(e) = podman.containers().get("79c93f220e3e").kill_signal("INT").await {
    ///     eprintln!("{}", e);
    /// }
    /// ```
    |
    pub async fn kill_signal(&self, signal: impl Into<String>) -> Result<()> {
        let ep = url::construct_ep(&format!("/libpod/containers/{}/kill", &self.id), Some(url::encoded_pair("signal", signal.into())));
        self.podman.post(&ep, Payload::None::<&str>).await.map(|_| ())
    }}

    api_doc! {
    Container => KillLibpod
    /// Kill this container.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    /// if let Err(e) = podman.containers().get("79c93f220e3e").kill().await {
    ///     eprintln!("{}", e);
    /// }
    /// ```
    |
    pub async fn kill(&self) -> Result<()> {
        self.kill_signal("TERM").await
    }}
}

impl<'podman> Containers<'podman> {
    api_doc! {
    Container => ListLibpod
    /// Returns a list of containers.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    /// for container in podman
    ///     .containers()
    ///     .list(
    ///         &ContainerListOpts::builder()
    ///             .all(true)
    ///             .filter([ContainerListFilter::Label {
    ///                 key: "app".into(),
    ///                 value: "web".into(),
    ///             }])
    ///             .build(),
    ///     )
    ///     .await
    ///     .unwrap()
    /// {
    ///     println!("{:?}", container);
    /// }
    /// ```
    |
    pub async fn list(&self, opts: &opts::ContainerListOpts) -> Result<Vec<models::ListContainer>> {
        let ep = url::construct_ep("/libpod/containers/json", opts.serialize());
        self.podman.get_json(&ep).await
    }}
}
