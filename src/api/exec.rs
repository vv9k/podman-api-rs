use crate::{
    conn::{tty, Headers, Payload},
    opts, Result, Value,
};

use containers_api::url;

#[derive(Debug)]
/// [Api Reference](https://docs.podman.io/en/latest/_static/api.html?version=v4.2#tag/Exec)
pub struct Exec {
    podman: crate::Podman,
    id: crate::Id,
    is_tty: bool,
    is_unchecked: bool,
}

impl Exec {
    ///Exports an interface exposing operations against a Exec instance with TTY
    pub(crate) fn new_tty(podman: crate::Podman, id: impl Into<crate::Id>) -> Self {
        Exec {
            podman,
            id: id.into(),
            is_tty: true,
            is_unchecked: false,
        }
    }

    ///Exports an interface exposing operations against a Exec instance without TTY
    pub(crate) fn new_raw(podman: crate::Podman, id: impl Into<crate::Id>) -> Self {
        Exec {
            podman,
            id: id.into(),
            is_tty: false,
            is_unchecked: false,
        }
    }

    ///Exports an interface exposing operations against a Exec instance with unchecked TTY state
    pub(crate) fn new_unchecked(podman: crate::Podman, id: impl Into<crate::Id>) -> Self {
        Exec {
            podman,
            id: id.into(),
            is_tty: false,
            is_unchecked: true,
        }
    }

    ///A getter for Exec id
    pub fn id(&self) -> &crate::Id {
        &self.id
    }
}

#[derive(Debug)]
/// Handle for Podman Execs.
pub struct Execs {
    podman: crate::Podman,
}

impl Execs {
    ///Exports an interface for interacting with Podman Execs.
    pub fn new(podman: crate::Podman) -> Self {
        Execs { podman }
    }

    ///Returns a reference to a set of operations available to a specific Exec.
    pub fn get(&self, id: impl Into<crate::Id>) -> Exec {
        Exec::new_unchecked(self.podman.clone(), id)
    }
}

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
    pub async fn start<'exec>(
        &'exec self,
        opts: &'exec opts::ExecStartOpts,
    ) -> Result<tty::Multiplexer<'exec>> {
        if self.is_unchecked {
            return Err(crate::Error::UncheckedExec);
        }

        let ep = format!("/libpod/exec/{}/start", &self.id);

        let payload = Payload::Json(
            opts.serialize()
                .map_err(|e| crate::conn::Error::Any(Box::new(e)))?,
        );

        self.podman.post_upgrade_stream(ep, payload).await.map(|x| {
            if self.is_tty {
                tty::Multiplexer::new(x, tty::decode_raw)
            } else {
                tty::Multiplexer::new(x, tty::decode_chunk)
            }
        })
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
    pub async fn inspect(&self) -> Result<Value> {
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
        self.podman.post(&ep, Payload::None::<&str>, Headers::none()).await.map(|_| ())
    }}
}
