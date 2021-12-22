use crate::{
    api::{ApiResource, Exec},
    conn::{tty, Headers, Payload},
    models, opts,
    util::url,
    Result,
};

use futures_util::stream::{Stream, TryStreamExt};

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
        let ep = url::construct_ep(
            &format!("/libpod/containers/{}/start", &self.id),
            detach_keys.map(|d| url::encoded_pair("detachKeys", d)),
        );
        self.podman.post(&ep, Payload::empty()).await.map(|_| ())
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
        let ep = url::construct_ep(
            &format!("/libpod/containers/{}/stop", &self.id),
            opts.serialize(),
        );
        self.podman.post(&ep, Payload::empty()).await.map(|_| ())
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
        let ep = url::construct_ep(
            &format!("/libpod/containers/{}/json", &self.id),
            Some(url::encoded_pair("size", "true")),
        );
        self.podman.get_json(&ep).await
    }}

    api_doc! {
    Container => KillLibpod
    /// Send a signal to this container.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    /// if let Err(e) = podman.containers().get("79c93f220e3e").send_signal("INT").await {
    ///     eprintln!("{}", e);
    /// }
    /// ```
    |
    pub async fn send_signal(&self, signal: impl Into<String>) -> Result<()> {
        let ep = url::construct_ep(
            &format!("/libpod/containers/{}/kill", &self.id),
            Some(url::encoded_pair("signal", signal.into())),
        );
        self.podman.post(&ep, Payload::empty()).await.map(|_| ())
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
        self.send_signal("TERM").await
    }}

    api_doc! {
    Container => PauseLibpod
    /// Use the cgroups freezer to suspend all processes in this container.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    /// if let Err(e) = podman.containers().get("79c93f220e3e").pause().await {
    ///     eprintln!("{}", e);
    /// }
    /// ```
    |
    pub async fn pause(&self) -> Result<()> {
        self.podman
            .post(
                &format!("/libpod/containers/{}/pause", &self.id),
                Payload::empty(),
            )
            .await
            .map(|_| ())
    }}

    api_doc! {
    Container => UnpauseLibpod
    /// Unpause this container
    ///
    /// Examples:
    ///
    /// ```no_run
    /// let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    /// if let Err(e) = podman.containers().get("79c93f220e3e").unpause().await {
    ///     eprintln!("{}", e);
    /// }
    /// ```
    |
    pub async fn unpause(&self) -> Result<()> {
        self.podman
            .post(
                &format!("/libpod/containers/{}/unpause", &self.id),
                Payload::empty(),
            )
            .await
            .map(|_| ())
    }}

    api_doc! {
    Container => RestartLibpod
    /// Restart this container with a timeout.
    ///
    /// Parameters:
    ///  * t - number of seconds to wait before killing the container
    ///
    /// Examples:
    ///
    /// ```no_run
    /// let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    /// if let Err(e) = podman.containers().get("79c93f220e3e").restart_with_timeout(20).await {
    ///     eprintln!("{}", e);
    /// }
    /// ```
    |
    pub async fn restart_with_timeout(&self, t: usize) -> Result<()> {
        let ep = url::construct_ep(
            &format!("/libpod/containers/{}/restart", &self.id),
            Some(url::encoded_pair("t", t.to_string())),
        );
        self.podman.post(&ep, Payload::empty()).await.map(|_| ())
    }}

    api_doc! {
    Container => RestartLibpod
    /// Restart this container.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    /// if let Err(e) = podman.containers().get("79c93f220e3e").restart().await {
    ///     eprintln!("{}", e);
    /// }
    /// ```
    |
    pub async fn restart(&self) -> Result<()> {
        let ep = format!("/libpod/containers/{}/restart", &self.id);
        self.podman.post(&ep, Payload::empty()).await.map(|_| ())
    }}

    api_doc! {
    Container => DeleteLibpod
    /// Delete this container.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    /// if let Err(e) = podman
    ///     .containers()
    ///     .get("79c93f220e3e")
    ///     .delete(&ContainerDeleteOpts::builder().volumes(true).build())
    ///     .await
    /// {
    ///     eprintln!("{}", e);
    /// }
    /// ```
    |
    pub async fn delete(&self, opts: &opts::ContainerDeleteOpts) -> Result<()> {
        let ep = url::construct_ep(format!("/libpod/containers/{}", &self.id), opts.serialize());
        self.podman.delete(&ep).await.map(|_| ())
    }}

    api_doc! {
    Container => DeleteLibpod
    /// Force remove this container.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    /// if let Err(e) = podman.containers().get("79c93f220e3e").remove().await {
    ///     eprintln!("{}", e);
    /// }
    /// ```
    |
    pub async fn remove(&self) -> Result<()> {
        self.delete(&opts::ContainerDeleteOpts::builder().force(true).build())
            .await
    }}

    api_doc! {
    Container => MountLibpod
    /// Mount this container to the filesystem.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    /// match podman.containers().get("79c93f220e3e").mount().await {
    ///     Ok(id) => println!("mounted container {}", id),
    ///     Err(e) => eprintln!("{}", e);
    /// }
    /// ```
    |
    pub async fn mount(&self) -> Result<crate::Id> {
        self.podman
            .post_json(
                &format!("/libpod/containers/{}/mount", &self.id),
                Payload::empty(),
            )
            .await
            .map(|id: String| id.into())
    }}

    api_doc! {
    Container => UnmountLibpod
    /// Unmount this container from the filesystem.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    /// if let Err(e) = podman.containers().get("79c93f220e3e").unmount().await {
    ///     eprintln!("{}", e);
    /// }
    /// ```
    |
    pub async fn unmount(&self) -> Result<()> {
        self.podman
            .post_json(
                &format!("/libpod/containers/{}/unmount", &self.id),
                Payload::empty(),
            )
            .await
    }}

    api_doc! {
    Container => CheckpointLibpod
    /// Checkpoint this container.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// use futures_util::StreamExt;
    /// let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    /// let mut container_stream = podman.containers().get("79c93f220e3e").checkpoint(
    ///     &ContainerCheckpointOpts::builder()
    ///         .leave_running(true)
    ///         .print_stats(true)
    ///         .build(),
    /// );
    ///
    /// while let Some(chunk) = container_stream.next().await {
    ///     println!("{:?}", chunk);
    /// }
    /// ```
    |
    pub async fn checkpoint(
        &self,
        opts: &opts::ContainerCheckpointOpts,
    ) -> impl Stream<Item = Result<Vec<u8>>> + 'podman {
        let ep = url::construct_ep(
            format!("/libpod/containers/{}/checkpoint", &self.id),
            opts.serialize(),
        );
        self.podman
            .stream_post(ep, Payload::empty(), Headers::none())
            .map_ok(|c| c.to_vec())
    }}

    api_doc! {
    Container => ImageCommitLibpod
    /// Create a new image from this container.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    /// if let Err(e) = podman
    ///     .containers()
    ///     .get("79c93f220e3e")
    ///     .commit(
    ///         &ContainerCommitOpts::builder()
    ///             .pause(true)
    ///             .repo("image-name")
    ///             .tag("1.0.0")
    ///             .build(),
    ///     )
    ///     .await
    /// {
    ///     eprintln!("{}", e);
    /// }
    /// ```
    |
    pub async fn commit(&self, opts: &opts::ContainerCommitOpts) -> Result<()> {
        let opts = opts.for_container(self.id.clone());
        let ep = url::construct_ep("/libpod/commit", opts.serialize());
        self.podman.post(&ep, Payload::empty()).await.map(|_| ())
    }}

    api_doc! {
    Container => ExecLibpod
    /// Create an exec session to run a command inside this container. Exec sessions will be
    /// automatically removed 5 minutes after they exit.
    ///
    /// This endpoint only creates the exec. To start it use [Exec::start](Exec::start).
    ///
    /// Examples:
    ///
    /// ```no_run
    /// let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    /// let exec = podman
    /// .containers()
    /// .get("79c93f220e3e")
    /// .create_exec(
    ///     &podman_api::opts::ExecCreateOpts::builder()
    ///         .command(["cat", "/some/path/in/container"])
    ///         .attach_stdout(true)
    ///         .attach_stderr(true)
    ///         .build(),
    /// )
    /// .await
    /// .unwrap();
    /// ```
    |
    pub async fn create_exec(&self, opts: &opts::ExecCreateOpts) -> Result<Exec<'_>> {
        let ep = format!("/libpod/containers/{}/exec", self.id);

        self.podman
            .post_json(&ep, Payload::Json(opts.serialize()?))
            .await
            .map(|resp: models::IdResponse| Exec::new(self.podman, resp.id))
    }}

    api_doc! {
    Container => RenameLibpod
    /// Change the name of this container.
    ///
    /// Parameters:
    ///  * new_name - new name to give for this container
    ///
    /// Examples:
    ///
    /// ```no_run
    /// let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    /// if let Err(e) = podman.containers().get("79c93f220e3e").rename("my-container").await {
    ///     eprintln!("{}", e);
    /// }
    /// ```
    |
    pub async fn rename(&self, new_name: impl AsRef<str>) -> Result<()> {
        let ep = url::construct_ep(
            &format!("/libpod/containers/{}/rename", &self.id),
            Some(url::encoded_pair("name", new_name.as_ref())),
        );
        self.podman.post(&ep, Payload::empty()).await.map(|_| ())
    }}

    api_doc! {
    Container => InitLibpod
    /// Performs all tasks necessary for initializing the container but does not start the container.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    /// if let Err(e) = podman.containers().get("79c93f220e3e").init().await {
    ///     eprintln!("{}", e);
    /// }
    /// ```
    |
    pub async fn init(&self) -> Result<()> {
        self.podman
            .post(
                &format!("/libpod/containers/{}/init", &self.id),
                Payload::empty(),
            )
            .await
            .map(|_| ())
    }}

    api_doc! {
    Container => WaitLibpod
    /// Wait for this container to meet a given condition.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    /// if let Err(e) = podman
    ///     .containers()
    ///     .get("79c93f220e3e")
    ///     .wait(
    ///         &ContainerWaitOpts::builder()
    ///             .conditions([ContainerStatus::Configured])
    ///             .interval("300ms")
    ///             .build(),
    ///     )
    ///     .await
    /// {
    ///     eprintln!("{}", e);
    /// }
    /// ```
    |
    pub async fn wait(&self, opts: &opts::ContainerWaitOpts) -> Result<()> {
        let ep = url::construct_ep(&format!("/libpod/containers/{}/wait", &self.id), opts.serialize());
        self.podman.post(&ep, Payload::empty()).await.map(|_| ())
    }}

    api_doc! {
    Container => ExistsLibpod
    /// Quick way to determine if a container exists by name or ID
    ///
    /// Examples:
    ///
    /// ```no_run
    /// let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    /// match podman.containers().get("79c93f220e3e").exists().await {
    ///     Ok(exists) => if exists {
    ///         println!("container exists!");
    ///     } else {
    ///         println!("container doesn't exists!");
    ///     },
    ///     Err(e) => eprintln!("check failed: {}", e);
    /// }
    /// ```
    |
    pub async fn exists(&self) -> Result<bool> {
        self.podman
            .resource_exists(ApiResource::Containers, &self.id)
            .await
    }}

    api_doc! {
    Container => AttachLibpod
    /// Attach to this container.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// use futures_util::StreamExt;
    /// let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    /// let tty_multiplexer = podman.containers().get("79c93f220e3e").attach().await?;
    /// let (mut reader, _writer) = tty_multiplexer.split();
    ///
    /// while let Some(tty_result) = reader.next().await {
    ///     match tty_result {
    ///         Ok(chunk) => println!("{:?}", chunk),
    ///         Err(e) => eprintln!("Error: {}", e),
    ///     }
    /// }
    /// ```
    |
    pub async fn attach(
        &self,
        opts: &opts::ContainerAttachOpts,
    ) -> Result<tty::Multiplexer<'podman>> {
        let ep = url::construct_ep(
            format!("/libpod/containers/{}/attach", &self.id),
            opts.stream().serialize(),
        );
        self.podman
            .stream_post_upgrade(ep, Payload::empty())
            .await
            .map(tty::Multiplexer::new)
    }}
}

impl<'podman> Containers<'podman> {
    api_doc! {
    Container => CreateLibpod
    /// Create a container with specified options.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    /// if let Err(e) = podman
    ///     .containers()
    ///     .create(
    ///         &ContainerCreateOpts::builder()
    ///             .image("debian:11")
    ///             .command(
    ///                 ["/usr/bin/httpd"]
    ///             )
    ///             .env([
    ///                 ("app", "web"),
    ///             ])
    ///             .build(),
    ///     )
    ///     .await
    /// {
    ///     eprintln!("{}", e);
    /// }
    /// ```
    |
    pub async fn create(
        &self,
        opts: &opts::ContainerCreateOpts,
    ) -> Result<models::ContainerCreateCreatedBody> {
        self.podman
            .post_json(
                &"/libpod/containers/create",
                Payload::Json(opts.serialize()?),
            )
            .await
    }}

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
    ///             .filter([ContainerListFilter::LabelKeyVal("app".into(), "web".into())])
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
