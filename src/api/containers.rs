use crate::{
    api::{ApiResource, Exec},
    conn::{tty, Headers, Payload},
    models, opts, Result,
};

use containers_api::url;
use futures_util::stream::{Stream, TryStreamExt};
use std::path::Path;

impl_api_ty!(
    Container => id
);

impl Container {
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
    /// async {
    ///     use podman_api::Podman;
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    ///     if let Err(e) = podman.containers().get("79c93f220e3e").start(None).await {
    ///         eprintln!("{}", e);
    ///     }
    /// };
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
    /// async {
    ///     use podman_api::Podman;
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    ///     if let Err(e) = podman.containers().get("79c93f220e3e").stop(&Default::default()).await {
    ///         eprintln!("{}", e);
    ///     }
    /// };
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
    /// async {
    ///     use podman_api::Podman;
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    ///     match podman.containers().get("79c93f220e3e").inspect().await {
    ///         Ok(info) => println!("{:?}", info),
    ///         Err(e) => eprintln!("{}", e),
    ///     }
    /// };
    /// ```
    |
    pub async fn inspect(&self) -> Result<models::ContainerInspectResponseLibpod> {
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
    /// async {
    ///     use podman_api::Podman;
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    ///     if let Err(e) = podman.containers().get("79c93f220e3e").send_signal("INT").await {
    ///         eprintln!("{}", e);
    ///     }
    /// };
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
    /// async {
    ///     use podman_api::Podman;
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    ///     if let Err(e) = podman.containers().get("79c93f220e3e").kill().await {
    ///         eprintln!("{}", e);
    ///     }
    /// };
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
    /// async {
    ///     use podman_api::Podman;
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    ///     if let Err(e) = podman.containers().get("79c93f220e3e").pause().await {
    ///         eprintln!("{}", e);
    ///     }
    /// };
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
    /// async {
    ///     use podman_api::Podman;
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    ///     if let Err(e) = podman.containers().get("79c93f220e3e").unpause().await {
    ///         eprintln!("{}", e);
    ///     }
    /// };
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
    /// async {
    ///     use podman_api::Podman;
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    ///     if let Err(e) = podman.containers().get("79c93f220e3e").restart_with_timeout(20).await {
    ///         eprintln!("{}", e);
    ///     }
    /// };
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
    /// async {
    ///     use podman_api::Podman;
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    ///     if let Err(e) = podman.containers().get("79c93f220e3e").restart().await {
    ///         eprintln!("{}", e);
    ///     }
    /// };
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
    /// async {
    ///     use podman_api::Podman;
    ///     use podman_api::opts::ContainerDeleteOpts;
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    ///     if let Err(e) = podman
    ///         .containers()
    ///         .get("79c93f220e3e")
    ///         .delete(&ContainerDeleteOpts::builder().volumes(true).build())
    ///         .await
    ///     {
    ///         eprintln!("{}", e);
    ///     }
    /// };
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
    /// async {
    ///     use podman_api::Podman;
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    ///     if let Err(e) = podman.containers().get("79c93f220e3e").remove().await {
    ///         eprintln!("{}", e);
    ///     }
    /// };
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
    /// async {
    ///     use podman_api::Podman;
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    ///     match podman.containers().get("79c93f220e3e").mount().await {
    ///         Ok(id) => println!("mounted container {}", id),
    ///         Err(e) => eprintln!("{}", e),
    ///     }
    /// };
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
    /// async {
    ///     use podman_api::Podman;
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    ///     if let Err(e) = podman.containers().get("79c93f220e3e").unmount().await {
    ///         eprintln!("{}", e);
    ///     }
    /// };
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
    /// async {
    ///     use podman_api::Podman;
    ///     use podman_api::opts::ContainerCheckpointOpts;
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    ///     let container = podman.containers().get("79c93f220e3e");
    ///     let mut container_stream = container.checkpoint(
    ///         &ContainerCheckpointOpts::builder()
    ///             .leave_running(true)
    ///             .print_stats(true)
    ///             .build(),
    ///     );
    ///
    ///     while let Some(chunk) = container_stream.next().await {
    ///         println!("{:?}", chunk);
    ///     }
    /// };
    /// ```
    |
    pub fn checkpoint(
        &self,
        opts: &opts::ContainerCheckpointOpts,
    ) -> impl Stream<Item = Result<Vec<u8>>> + Unpin + '_ {
        let ep = url::construct_ep(
            format!("/libpod/containers/{}/checkpoint", &self.id),
            opts.serialize(),
        );
        Box::pin(self.podman
            .stream_post(ep, Payload::empty(), Headers::none())
            .map_ok(|c| c.to_vec()))
    }}

    api_doc! {
    Container => ImageCommitLibpod
    /// Create a new image from this container.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// async {
    ///     use podman_api::Podman;
    ///     use podman_api::opts::ContainerCommitOpts;
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    ///     if let Err(e) = podman
    ///         .containers()
    ///         .get("79c93f220e3e")
    ///         .commit(
    ///             &ContainerCommitOpts::builder()
    ///                 .pause(true)
    ///                 .repo("image-name")
    ///                 .tag("1.0.0")
    ///                 .build(),
    ///         )
    ///         .await
    ///     {
    ///         eprintln!("{}", e);
    ///     }
    /// };
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
    /// async {
    ///     use podman_api::Podman;
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    ///     let exec = podman
    ///     .containers()
    ///     .get("79c93f220e3e")
    ///     .create_exec(
    ///         &podman_api::opts::ExecCreateOpts::builder()
    ///             .command(["cat", "/some/path/in/container"])
    ///             .attach_stdout(true)
    ///             .attach_stderr(true)
    ///             .build(),
    ///     )
    ///     .await
    ///     .unwrap();
    /// };
    /// ```
    |
    pub async fn create_exec(&self, opts: &opts::ExecCreateOpts) -> Result<Exec> {
        let ep = format!("/libpod/containers/{}/exec", self.id);

        self.podman
            .post_json(&ep, Payload::Json(opts.serialize()?))
            .await
            .map(|resp: models::IdResponse| Exec::new(self.podman.clone(), resp.id))
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
    /// async {
    ///     use podman_api::Podman;
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    ///     if let Err(e) = podman.containers().get("79c93f220e3e").rename("my-container").await {
    ///         eprintln!("{}", e);
    ///     }
    /// };
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
    /// async {
    ///     use podman_api::Podman;
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    ///     if let Err(e) = podman.containers().get("79c93f220e3e").init().await {
    ///         eprintln!("{}", e);
    ///     }
    /// };
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
    /// async {
    ///     use podman_api::Podman;
    ///     use podman_api::opts::ContainerWaitOpts;
    ///     use podman_api::models::ContainerStatus;
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    ///     if let Err(e) = podman
    ///         .containers()
    ///         .get("79c93f220e3e")
    ///         .wait(
    ///             &ContainerWaitOpts::builder()
    ///                 .conditions([ContainerStatus::Configured])
    ///                 .interval("300ms")
    ///                 .build(),
    ///         )
    ///         .await
    ///     {
    ///         eprintln!("{}", e);
    ///     }
    /// };
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
    /// async {
    ///     use podman_api::Podman;
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    ///     match podman.containers().get("79c93f220e3e").exists().await {
    ///         Ok(exists) => if exists {
    ///             println!("container exists!");
    ///         } else {
    ///             println!("container doesn't exists!");
    ///         },
    ///         Err(e) => eprintln!("check failed: {}", e),
    ///     }
    /// };
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
    /// async {
    ///     use podman_api::Podman;
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///     let container = podman.containers().get("79c93f220e3e");
    ///     let tty_multiplexer = container
    ///             .attach(&Default::default())
    ///             .await
    ///             .unwrap();
    ///     let (mut reader, _writer) = tty_multiplexer.split();
    ///
    ///     while let Some(tty_result) = reader.next().await {
    ///         match tty_result {
    ///             Ok(chunk) => println!("{:?}", chunk),
    ///             Err(e) => eprintln!("Error: {}", e),
    ///         }
    ///     }
    /// };
    /// ```
    |
    pub async fn attach(
        &self,
        opts: &opts::ContainerAttachOpts,
    ) -> Result<tty::Multiplexer<'_>> {
        let ep = url::construct_ep(
            format!("/libpod/containers/{}/attach", &self.id),
            opts.stream().serialize(),
        );
        self.podman
            .stream_post_upgrade(ep, Payload::empty())
            .await
            .map(tty::Multiplexer::new)
    }}

    api_doc! {
    Container => ChangesLibpod
    /// Returns which files in this container's filesystem have been added, deleted, or modified.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// async {
    ///     use podman_api::Podman;
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    ///     match podman
    ///         .containers()
    ///         .get("79c93f220e3e")
    ///         .changes(&Default::default())
    ///         .await
    ///     {
    ///         Ok(changes) => println!("{:?}", changes),
    ///         Err(e) => eprintln!("{}", e),
    ///     }
    /// };
    /// ```
    |
    pub async fn changes(
        &self,
        opts: &opts::ChangesOpts,
    ) -> Result<Vec<models::ContainerChangeResponseItem>> {
        let ep = url::construct_ep(
            &format!("/libpod/containers/{}/changes", &self.id),
            opts.serialize(),
        );
        self.podman.get_json(&ep).await
    }}

    api_doc! {
    Container => LogsLibpod
    /// Get logs from this container.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// use futures_util::StreamExt;
    /// async {
    ///     use podman_api::Podman;
    ///     use podman_api::opts::ContainerLogsOpts;
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    ///     let container = podman.containers().get("3f278d2d0d79");
    ///     let mut logs = container.logs(
    ///         &ContainerLogsOpts::builder()
    ///             .stdout(true)
    ///             .stderr(true)
    ///             .follow(true)
    ///             .build(),
    ///     );
    ///
    ///     while let Some(chunk) = logs.next().await {
    ///         match chunk {
    ///             Ok(chunk) => println!("{}", String::from_utf8_lossy(&chunk)),
    ///             Err(e) => eprintln!("{}", e),
    ///         }
    ///     }
    /// };
    /// ```
    |
    pub fn logs(
        &self,
        opts: &opts::ContainerLogsOpts,
    ) -> impl Stream<Item = Result<Vec<u8>>> + '_  {
        let ep = url::construct_ep(
            format!("/libpod/containers/{}/logs", &self.id),
            opts.serialize(),
        );
        Box::pin(self.podman.stream_get(ep).map_ok(|c| c.to_vec()))
    }}

    api_doc! {
    Container => StatsAllLibpod
    /// Return a single resource usage statistics of this container.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// async {
    ///     use podman_api::Podman;
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    ///     match podman.containers().get("fc93f220e3e").stats().await {
    ///         Ok(stats) => println!("{:?}", stats),
    ///         Err(e) => eprintln!("{}", e),
    ///     }
    /// };
    /// ```
    |
    pub async fn stats(&self) -> Result<models::ContainerStats200Response> {
        self.podman
            .containers()
            .stats(
                &opts::ContainerStatsOpts::builder()
                    .containers([self.id.to_string()])
                    .build(),
            )
            .await
    }}

    api_doc! {
    Container => StatsAllLibpod
    /// Return a stream of resource usage statistics of this container.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// use futures_util::StreamExt;
    /// async {
    ///     use podman_api::Podman;
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    ///     let container = podman.containers().get("fc93f220e3e");
    ///     let mut stats = container.stats_stream(None);
    ///
    ///     while let Some(chunk) = stats.next().await {
    ///         match chunk {
    ///             Ok(chunk) => println!("{:?}", chunk),
    ///             Err(e) => eprintln!("{}", e),
    ///         }
    ///     }
    /// };
    /// ```
    |
    pub fn stats_stream(
        &self,
        interval: Option<usize>,
    ) -> impl Stream<Item = Result<models::ContainerStats200Response>>  + '_ {
        let opts = opts::ContainerStatsOpts::builder()
                .containers([self.id.to_string()])
                .interval(interval.unwrap_or(5))
                .build();
        let ep = url::construct_ep("/libpod/containers/stats", opts.stream().serialize());

        Box::pin(self.podman.stream_get_json(ep))
    }}

    api_doc! {
    Container => TopLibpod
    /// List processes running inside this container.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// async {
    ///     use podman_api::Podman;
    /// };
    /// async {
    ///     use podman_api::Podman;
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    ///     match podman.containers().get("fc93f220e3e").top(&Default::default()).await {
    ///         Ok(stats) => println!("{:?}", stats),
    ///         Err(e) => eprintln!("{}", e),
    ///     }
    /// };
    /// ```
    |
    pub async fn top(&self, opts: &opts::ContainerTopOpts) -> Result<models::ContainerTopOkBody> {
        let ep = url::construct_ep(
            format!("/libpod/containers/{}/top", &self.id),
            opts.oneshot().serialize(),
        );

        self.podman.get_json(&ep).await
    }}

    api_doc! {
    Container => TopLibpod
    /// List processes running inside this container as a stream. (As of libpod version 4.0)
    ///
    /// Examples:
    ///
    /// ```no_run
    /// use futures_util::StreamExt;
    /// async {
    ///     use podman_api::Podman;
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    ///     let container = podman.containers().get("fc93f220e3e");
    ///     let mut top = container
    ///         .top_stream(&Default::default());
    ///
    ///     while let Some(chunk) = top.next().await {
    ///         match chunk {
    ///             Ok(chunk) => println!("{:?}", chunk),
    ///             Err(e) => eprintln!("{}", e),
    ///         }
    ///     }
    /// };
    /// ```
    |
    pub fn top_stream(
        &self,
        opts: &opts::ContainerTopOpts,
    ) -> impl Stream<Item = Result<models::ContainerTopOkBody>> + '_ {
        let ep = url::construct_ep(
            format!("/libpod/containers/{}/top", &self.id),
            opts.stream().serialize(),
        );

        Box::pin(self.podman.stream_get_json(ep))
    }}

    api_doc! {
    Generate => SystemdLibpod
    /// Generate Systemd Units based on this container.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// async {
    ///     use podman_api::Podman;
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    ///     match podman
    ///         .containers()
    ///         .get("fc93f220e3e")
    ///         .generate_systemd_units(&Default::default())
    ///         .await
    ///     {
    ///         Ok(info) => println!("{:?}", info),
    ///         Err(e) => eprintln!("{}", e),
    ///     }
    /// };
    /// ```
    |
    pub async fn generate_systemd_units(
        &self,
        opts: &opts::SystemdUnitsOpts,
    ) -> Result<serde_json::Value> {
        self.podman.generate_systemd_units(opts, &self.id).await
    }}

    api_doc! {
    Generate => KubeLibpod
    /// Generate Kubernetes YAML based on this container
    ///
    /// Parameters:
    /// * service - Generate YAML for a Kubernetes service object.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// async {
    ///     use podman_api::Podman;
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    ///     match podman
    ///         .containers()
    ///         .get("fc93f220e3e")
    ///         .generate_kube_yaml(false)
    ///         .await
    ///     {
    ///         Ok(yaml) => println!("{:?}", yaml),
    ///         Err(e) => eprintln!("{}", e),
    ///     }
    /// };
    /// ```
    |
    pub async fn generate_kube_yaml(&self, service: bool) -> Result<String> {
        self.podman.generate_kube_yaml(service, &self.id).await
    }}

    api_doc! {
    Network => ConnectLibpod
    /// Connect this container to a network
    ///
    /// Examples:
    ///
    /// ```no_run
    /// async {
    ///     use podman_api::Podman;
    ///     use podman_api::opts::NetworkConnectOpts;
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    ///     if let Err(e) = podman
    ///             .containers()
    ///             .get("fc93f220e3e")
    ///             .connect("my-network", &Default::default())
    ///             .await
    ///     {
    ///         eprintln!("{}", e);
    ///     }
    /// };
    /// ```
    |
    pub async fn connect(
        &self,
        network: impl Into<crate::Id>,
        opts: &opts::NetworkConnectOpts,
    ) -> Result<()> {
        let network = self.podman.networks().get(network.into());
        let opts = opts.for_container(&self.id);
        network.connect_container(&opts).await
    }}

    api_doc! {
    Network => DisconnectLibpod
    /// Disconnect this container from a network.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// async {
    ///     use podman_api::Podman;
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    ///     if let Err(e) = podman.containers().get("fc93f220e3e").disconnect("my-network", true).await {
    ///         eprintln!("{}", e);
    ///     }
    /// };
    /// ```
    |
    pub async fn disconnect(&self, network: impl Into<crate::Id>, force: bool) -> Result<()> {
        let network = self.podman.networks().get(network.into());
        network
            .disconnect_container(
                &opts::NetworkDisconnectOpts::builder()
                    .container(self.id.as_ref())
                    .force(force)
                    .build(),
            )
            .await
    }}

    api_doc! {
    Container => HealthcheckLibpod
    /// Execute the defined healtcheck and return information about the result.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// async {
    ///     use podman_api::Podman;
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    ///     match podman.containers().get("fc93f220e3e").healtcheck().await {
    ///         Ok(healtcheck) => println!("{:?}", healtcheck),
    ///         Err(e) => eprintln!("{}", e),
    ///     }
    /// };
    /// ```
    |
    pub async fn healtcheck(&self) -> Result<models::HealthCheckResults> {
        self.podman
            .get_json(&format!("/libpod/containers/{}/healtcheck", &self.id))
            .await
    }}

    api_doc! {
    Container => Archive
    /// Copy a file/folder from the container.  The resulting stream is a tarball of the extracted
    /// files.
    ///
    /// If `path` is not an absolute path, it is relative to the container’s root directory. The
    /// resource specified by `path` must exist. To assert that the resource is expected to be a
    /// directory, `path` should end in `/` or `/`. (assuming a path separator of `/`). If `path`
    /// ends in `/.`  then this indicates that only the contents of the path directory should be
    /// copied.  A symlink is always resolved to its target.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// async {
    ///     use podman_api::Podman;
    ///     use futures_util::TryStreamExt;
    ///     use tar::Archive;
    ///
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    ///     let bytes = podman
    ///          .containers()
    ///          .get("fc93f220e3e")
    ///          .copy_from("/tmp/dir")
    ///          .try_concat()
    ///          .await.unwrap();
    ///
    ///      let mut archive = Archive::new(&bytes[..]);
    ///      let local_path = "/tmp";
    ///      archive.unpack(&local_path).unwrap();
    /// };
    /// ```
    |
    pub fn copy_from(&self, path: impl AsRef<Path>) -> impl Stream<Item = Result<Vec<u8>>> + '_ {
        self.podman
            .stream_get(format!(
                "/containers/{}/archive?{}",
                self.id,
                url::encoded_pair("path", path.as_ref().to_string_lossy())
            ))
            .map_ok(|c| c.to_vec())
    }}

    api_doc! {
    PutContainer => Archive
    /// Copy a tarball (see `body`) to the container.
    ///
    /// The tarball will be copied to the container and extracted at the given location (see `path`).
    ///
    /// Examples:
    ///
    /// ```no_run
    /// async {
    ///     use podman_api::Podman;
    ///     use futures_util::TryStreamExt;
    ///     use tar::Archive;
    ///
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    ///     let bytes = vec![];
    ///     let src_path = std::path::PathBuf::from("/tmp/dir");
    ///
    ///     let mut ar = tar::Builder::new(Vec::new());
    ///     let mut header = tar::Header::new_gnu();
    ///     header.set_size(bytes.len() as u64);
    ///     header.set_mode(0o0644);
    ///     ar.append_data(
    ///         &mut header,
    ///         src_path
    ///             .iter()
    ///             .skip(1)
    ///             .collect::<std::path::PathBuf>(),
    ///         std::io::Cursor::new(bytes),
    ///     ).unwrap();
    ///     let data = ar.into_inner().unwrap();
    ///
    ///     if let Err(e) = podman
    ///         .containers()
    ///         .get("fc93f220e3e")
    ///         .copy_to("/", data.into())
    ///         .await
    ///     {
    ///         eprintln!("Error: {}", e);
    ///     }
    /// };
    /// ```
    |
    pub async fn copy_to(&self, path: impl AsRef<Path>, body: crate::conn::hyper::Body) -> Result<()> {
        self.podman
            .put(
                &format!(
                    "/containers/{}/archive?{}",
                    self.id,
                    url::encoded_pair("path", path.as_ref().to_string_lossy())
                ),
                Payload::XTar(body),
            )
            .await
            .map(|_| ())
    }}

    api_doc! {
    PutContainer => Archive
    /// Copy a byte slice as file into (see `bytes`) the container.
    ///
    /// The file will be copied at the given location (see `path`) and will be owned by root
    /// with access mask 644.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// async {
    ///     use podman_api::Podman;
    ///     use futures_util::TryStreamExt;
    ///     use tar::Archive;
    ///
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    ///     use std::{fs::File, io::Read};
    ///
    ///     let mut file = File::open("/some/important/file").unwrap();
    ///     let mut bytes = Vec::new();
    ///     file.read_to_end(&mut bytes)
    ///         .expect("Cannot read file on the localhost.");
    ///
    ///     if let Err(e) = podman
    ///         .containers()
    ///         .get("fc93f220e3e")
    ///         .copy_file_into("/tmp/", &bytes)
    ///         .await
    ///     {
    ///         eprintln!("Error: {}", e);
    ///     }
    /// };
    /// ```
    |
    pub async fn copy_file_into<P: AsRef<Path>>(&self, path: P, bytes: &[u8]) -> Result<()> {
        let path = path.as_ref();

        let mut ar = tar::Builder::new(Vec::new());
        let mut header = tar::Header::new_gnu();
        header.set_size(bytes.len() as u64);
        header.set_mode(0o0644);
        ar.append_data(
            &mut header,
            path.to_path_buf()
                .iter()
                .skip(1)
                .collect::<std::path::PathBuf>(),
            bytes,
        )?;
        let data = ar.into_inner()?;

        self.copy_to(Path::new("/"), data.into()).await.map(|_| ())
    }}
}

impl Containers {
    api_doc! {
    Container => CreateLibpod
    /// Create a container with specified options.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// async {
    ///     use podman_api::Podman;
    ///     use podman_api::opts::ContainerCreateOpts;
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    ///     if let Err(e) = podman
    ///         .containers()
    ///         .create(
    ///             &ContainerCreateOpts::builder()
    ///                 .image("debian:11")
    ///                 .command(
    ///                     ["/usr/bin/httpd"]
    ///                 )
    ///                 .env([
    ///                     ("app", "web"),
    ///                 ])
    ///                 .build(),
    ///         )
    ///         .await
    ///     {
    ///         eprintln!("{}", e);
    ///     }
    /// };
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
    /// async {
    ///     use podman_api::Podman;
    ///     use podman_api::opts::{ContainerListOpts, ContainerListFilter};
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    ///     for container in podman
    ///         .containers()
    ///         .list(
    ///             &ContainerListOpts::builder()
    ///                 .all(true)
    ///                 .filter([ContainerListFilter::LabelKeyVal("app".into(), "web".into())])
    ///                 .build(),
    ///         )
    ///         .await
    ///         .unwrap()
    ///     {
    ///         println!("{:?}", container);
    ///     }
    /// };
    /// ```
    |
    pub async fn list(&self, opts: &opts::ContainerListOpts) -> Result<Vec<models::ListContainer>> {
        let ep = url::construct_ep("/libpod/containers/json", opts.serialize());
        self.podman.get_json(&ep).await
    }}

    api_doc! {
    Container => StatsAllLibpod
    /// Return a single resource usage statistics of one or more container. If not container is
    /// specified in the options, the statistics of all are returned.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// async {
    ///     use podman_api::Podman;
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    ///     match podman.containers().stats(&Default::default()).await {
    ///         Ok(stats) => println!("{:?}", stats),
    ///         Err(e) => eprintln!("{}", e),
    ///     }
    /// };
    /// ```
    |
    pub async fn stats(
        &self,
        opts: &opts::ContainerStatsOpts,
    ) -> Result<models::ContainerStats200Response> {
        let ep = url::construct_ep("/libpod/containers/stats", opts.oneshot().serialize());

        self.podman.get_json(&ep).await
    }}

    api_doc! {
    Container => StatsAllLibpod
    /// Return a stream of resource usage statistics of one or more container. If not container is
    /// specified in the options, the statistics of all are returned.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// use futures_util::StreamExt;
    /// async {
    ///     use podman_api::Podman;
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    ///     let containers = podman.containers();
    ///     let mut stats = containers
    ///         .stats_stream(&Default::default());
    ///
    ///     while let Some(chunk) = stats.next().await {
    ///         match chunk {
    ///             Ok(chunk) => println!("{:?}", chunk),
    ///             Err(e) => eprintln!("{}", e),
    ///         }
    ///     }
    /// };
    /// ```
    |
    pub fn stats_stream(
        &self,
        opts: &opts::ContainerStatsOpts,
    ) -> impl Stream<Item = Result<models::ContainerStats200Response>> + '_ {
        let ep = url::construct_ep("/libpod/containers/stats", opts.stream().serialize());

        Box::pin(self.podman.stream_get_json(ep))
    }}

    api_doc! {
    Container => ShowMountedLibpod
    /// List all mounted containers mount points.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// async {
    ///     use podman_api::Podman;
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    ///     match podman.containers().list_mounted().await {
    ///         Ok(mounts) => println!("{:?}", mounts),
    ///         Err(e) => eprintln!("{}", e),
    ///     }
    /// };
    /// ```
    |
    pub async fn list_mounted(&self) -> Result<serde_json::Value> {
        self.podman.get_json("/libpod/containers/showmounted").await
    }}

    api_doc! {
    Container => PruneLibpod
    /// Remove containers not in use.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// async {
    ///     use podman_api::Podman;
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    ///     match podman.containers().prune(&Default::default()).await {
    ///         Ok(report) => println!("{:?}", report),
    ///         Err(e) => eprintln!("{}", e),
    ///     }
    /// };
    /// ```
    |
    pub async fn prune(
        &self,
        opts: &opts::ContainerPruneOpts,
    ) -> Result<Vec<models::ContainersPruneReport>> {
        let ep = url::construct_ep("/libpod/containers/prune", opts.serialize());
        self.podman.post_json(&ep, Payload::empty()).await
    }}
}
