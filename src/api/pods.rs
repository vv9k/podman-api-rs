use crate::{api::ApiResource, conn::Payload, models, opts, util::url, Result};

use futures_util::stream::Stream;

impl_api_ty!(
    Pod => id
);

impl Pod {
    api_doc! {
    Pod => StartLibpod
    /// Start this pod.
    ///
    /// Parameters:
    ///  * detach_keys - Override the key sequence for detaching a pod. Format is a single
    ///                  character [a-Z] or ctrl- where is one of: a-z, @, ^, [, , or _.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// async {
    ///     use podman_api::Podman;
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    ///     if let Err(e) = podman.pods().get("79c93f220e3e").start().await {
    ///         eprintln!("{}", e);
    ///     }
    /// };
    /// ```
    |
    pub async fn start(&self) -> Result<models::PodStartReport> {
        self.podman.post_json(&
            format!("/libpod/pods/{}/start", &self.id),
             Payload::empty()).await
    }}

    async fn _stop(&self, timeout: Option<usize>) -> Result<models::PodStopReport> {
        let ep = url::construct_ep(
            &format!("/libpod/pods/{}/stop", &self.id),
            timeout.map(|t| url::encoded_pair("t", t.to_string())),
        );
        self.podman.post_json(&ep, Payload::empty()).await
    }

    api_doc! {
    Pod => StopLibpod
    /// Stop this pod.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// async {
    ///     use podman_api::Podman;
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    ///     if let Err(e) = podman.pods().get("79c93f220e3e").stop().await {
    ///         eprintln!("{}", e);
    ///     }
    /// };
    /// ```
    |
    pub async fn stop(&self) -> Result<models::PodStopReport> {
        self._stop(None).await
    }}

    api_doc! {
    Pod => StopLibpod
    /// Stop this pod with a timeout.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// async {
    ///     use podman_api::Podman;
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    ///     if let Err(e) = podman.pods().get("79c93f220e3e").stop_with_timeout(10).await {
    ///         eprintln!("{}", e);
    ///     }
    /// };
    /// ```
    |
    pub async fn stop_with_timeout(&self, t: usize) -> Result<models::PodStopReport> {
        self._stop(Some(t)).await
    }}

    api_doc! {
    Pod => InspectLibpod
    /// Return low-level information about this pod.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// async {
    ///     use podman_api::Podman;
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    ///     match podman.pods().get("79c93f220e3e").inspect().await {
    ///         Ok(info) => println!("{:?}", info),
    ///         Err(e) => eprintln!("{}", e),
    ///     }
    /// };
    /// ```
    |
    pub async fn inspect(&self) -> Result<models::LibpodPodInspectResponse> {
        self.podman
            .get_json(&format!("/libpod/pods/{}/json", &self.id))
            .await
    }}

    api_doc! {
    Pod => KillLibpod
    /// Send a signal to this pod.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// async {
    ///     use podman_api::Podman;
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    ///     if let Err(e) = podman.pods().get("79c93f220e3e").send_signal("SIGINT").await {
    ///         eprintln!("{}", e);
    ///     }
    /// };
    /// ```
    |
    pub async fn send_signal(&self, signal: impl Into<String>) -> Result<models::PodKillReport> {
        let ep = url::construct_ep(
            &format!("/libpod/pods/{}/kill", &self.id),
            Some(url::encoded_pair("signal", signal.into())),
        );
        self.podman.post_json(&ep, Payload::empty()).await
    }}

    api_doc! {
    Pod => KillLibpod
    /// Kill this pod.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// async {
    ///     use podman_api::Podman;
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    ///     if let Err(e) = podman.pods().get("79c93f220e3e").kill().await {
    ///         eprintln!("{}", e);
    ///     }
    /// };
    /// ```
    |
    pub async fn kill(&self) -> Result<models::PodKillReport> {
        self.send_signal("SIGKILL").await
    }}

    api_doc! {
    Pod => PauseLibpod
    /// Pause this pod.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// async {
    ///     use podman_api::Podman;
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    ///     if let Err(e) = podman.pods().get("79c93f220e3e").pause().await {
    ///         eprintln!("{}", e);
    ///     }
    /// };
    /// ```
    |
    pub async fn pause(&self) -> Result<models::PodPauseReport> {
        self.podman
            .post_json(
                &format!("/libpod/pods/{}/pause", &self.id),
                Payload::empty(),
            )
            .await
    }}

    api_doc! {
    Pod => UnpauseLibpod
    /// Unpause this pod
    ///
    /// Examples:
    ///
    /// ```no_run
    /// async {
    ///     use podman_api::Podman;
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    ///     if let Err(e) = podman.pods().get("79c93f220e3e").unpause().await {
    ///         eprintln!("{}", e);
    ///     }
    /// };
    /// ```
    |
    pub async fn unpause(&self) -> Result<models::PodUnpauseReport> {
        self.podman
            .post_json(
                &format!("/libpod/pods/{}/unpause", &self.id),
                Payload::empty(),
            )
            .await
    }}

    api_doc! {
    Pod => RestartLibpod
    /// Restart this pod.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// async {
    ///     use podman_api::Podman;
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    ///     if let Err(e) = podman.pods().get("79c93f220e3e").restart().await {
    ///         eprintln!("{}", e);
    ///     }
    /// };
    /// ```
    |
    pub async fn restart(&self) -> Result<models::PodRestartReport> {
        let ep = format!("/libpod/pods/{}/restart", &self.id);
        self.podman.post_json(&ep, Payload::empty()).await
    }}

    async fn _delete(&self, force: bool) -> Result<models::PodRmReport> {
        let ep = url::construct_ep(
            format!("/libpod/pods/{}", &self.id),
            Some(url::encoded_pair("force", force.to_string())),
        );
        self.podman.delete_json(&ep).await
    }

    api_doc! {
    Pod => DeleteLibpod
    /// Delete this pod.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// async {
    ///     use podman_api::Podman;
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    ///     if let Err(e) = podman.pods().get("79c93f220e3e").delete().await {
    ///         eprintln!("{}", e);
    ///     }
    /// };
    /// ```
    |
    pub async fn delete(&self) -> Result<models::PodRmReport> {
        self._delete(false).await
    }}

    api_doc! {
    Pod => DeleteLibpod
    /// Force remove this pod.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// async {
    ///     use podman_api::Podman;
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    ///     if let Err(e) = podman.pods().get("79c93f220e3e").remove().await {
    ///         eprintln!("{}", e);
    ///     }
    /// };
    /// ```
    |
    pub async fn remove(&self) -> Result<models::PodRmReport> {
        self._delete(true).await
    }}

    api_doc! {
    Pod => ExistsLibpod
    /// Quick way to determine if a pod exists by name or ID.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// async {
    ///     use podman_api::Podman;
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    ///     match podman.pods().get("79c93f220e3e").exists().await {
    ///         Ok(exists) => if exists {
    ///             println!("pod exists!");
    ///         } else {
    ///             println!("pod doesn't exists!");
    ///         },
    ///         Err(e) => eprintln!("check failed: {}", e),
    ///     }
    /// };
    /// ```
    |
    pub async fn exists(&self) -> Result<bool> {
        self.podman.resource_exists(ApiResource::Pods, &self.id).await
    }}

    api_doc! {
    Pod => TopLibpod
    /// List processes inside this pod.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// async {
    ///     use podman_api::Podman;
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    ///     match podman.pods().get("79c93f220e3e").top(&Default::default()).await {
    ///         Ok(info) => println!("{:?}", info),
    ///         Err(e) => eprintln!("{}", e),
    ///     }
    /// };
    /// ```
    |
    pub async fn top(&self, opts: &opts::PodTopOpts) -> Result<models::LibpodPodTopResponse> {
        let ep = url::construct_ep(format!("/libpod/pods/{}/top", &self.id), opts.serialize());
        self.podman.get_json(&ep).await
    }}

    api_doc! {
    Pod => TopLibpod
    /// List processes inside this pod.
    ///
    /// Only supported as of version > 4.0
    ///
    /// Examples:
    ///
    /// ```no_run
    /// async {
    ///     use podman_api::Podman;
    ///     use futures_util::StreamExt;
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    ///     let mut stream = podman.pods().get("79c93f220e3e").top_stream(&Default::default());
    ///     while let Some(chunk) = stream.next().await {
    ///         match chunk{
    ///             Ok(chunk) => println!("{:?}", chunk),
    ///             Err(e) => eprintln!("{}", e),
    ///         }
    ///     }
    /// };
    /// ```
    |
    pub fn top_stream(
        &self,
        opts: &opts::PodTopOpts,
    ) -> impl Stream<Item = Result<models::LibpodPodTopResponse>> + Unpin + '_ {
        let ep = url::construct_ep(
            format!("/libpod/pods/{}/top", &self.id),
            opts.stream().serialize(),
        );
        Box::pin(self.podman.stream_get_json(ep))
    }}

    api_doc! {
    Generate => SystemdLibpod
    /// Generate Systemd Units based on this pod.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// async {
    ///     use podman_api::Podman;
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    ///     match podman
    ///         .pods()
    ///         .get("ea03584c0fd6")
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
}

impl Pods {
    api_doc! {
    Pod => ListLibpod
    /// Returns a list of pods.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// async {
    ///     use podman_api::Podman;
    ///     use podman_api::opts::{PodListOpts, PodListFilter};
    ///     use podman_api::models::PodStatus;
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    ///     for pod in podman
    ///         .pods()
    ///         .list(
    ///             &PodListOpts::builder()
    ///                 .filter([PodListFilter::Status(PodStatus::Degraded)])
    ///                 .build(),
    ///         )
    ///         .await
    ///         .unwrap()
    ///     {
    ///         println!("{:?}", pod);
    ///     }
    /// };
    /// ```
    |
    pub async fn list(&self, opts: &opts::PodListOpts) -> Result<Vec<models::ListPodsReport>> {
        let ep = url::construct_ep("/libpod/pods/json", opts.serialize());
        self.podman.get_json(&ep).await
    }}

    api_doc! {
    Pod => ListLibpod
    /// Returns a list of pods.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// async {
    ///     use podman_api::Podman;
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    ///     match podman.pods().prune().await {
    ///         Ok(info) => println!("{:?}", info),
    ///         Err(e) => eprintln!("{}", e),
    ///     }
    /// };
    /// ```
    |
    pub async fn prune(&self) -> Result<Vec<models::PodPruneReport>> {
        self.podman.get_json("/libpod/pods/prune").await
    }}

    api_doc! {
    Pod => StatsAllLibpod
    /// Display a live stream of resource usage statistics for the containers in one or more pods.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// async {
    ///     use podman_api::Podman;
    ///     use podman_api::opts::PodStatsOpts;
    ///     use futures_util::StreamExt;
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    ///     let mut stream = podman.pods().stats(&PodStatsOpts::builder().all(true).build());
    ///     while let Some(chunk) = stream.next().await {
    ///         match chunk{
    ///             Ok(chunk) => println!("{:?}", chunk),
    ///             Err(e) => eprintln!("{}", e),
    ///         }
    ///     }
    /// };
    /// ```
    |
    pub fn stats(
        &self,
        opts: &opts::PodStatsOpts,
    ) -> impl Stream<Item = Result<models::LibpodPodTopResponse>> + Unpin + '_ {
        let ep = url::construct_ep(
            "/libpod/pods/stats",
            opts.serialize(),
        );
        Box::pin(self.podman.stream_get_json(ep))
    }}

    api_doc! {
    Pod => CreateLibpod
    /// Create a container with specified options.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// async {
    ///     use podman_api::Podman;
    ///     use podman_api::opts::PodCreateOpts;
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    ///     match podman
    ///         .pods()
    ///         .create(&PodCreateOpts::builder().name("my-pod").build())
    ///         .await
    ///     {
    ///         Ok(pod) => { /* do something with the pod */ }
    ///         Err(e) => eprintln!("{}", e),
    ///     }
    /// };
    /// ```
    |
    pub async fn create(&self, opts: &opts::PodCreateOpts) -> Result<Pod> {
        self.podman
            .post_json(
                &"/libpod/containers/create",
                Payload::Json(opts.serialize()?),
            )
            .await
            .map(|resp: models::IdResponse| Pod::new(self.podman.clone(), resp.id))
    }}
}
