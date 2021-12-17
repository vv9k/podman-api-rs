//! Options used for configuring the behavior of certain API endpoints

use crate::{api::Filter, models};

pub type EventsConstraint = (String, Vec<String>);

impl_opts_builder!(
    url =>
    /// Used to filter events returned by [Podman::events](crate::Podman::events).
    Events
);

impl EventsOptsBuilder {
    impl_url_str_field!(
        /// Start streaming events from this time
        since: S => "since"
    );

    impl_url_str_field!(
        /// Stop streaming events later than this
        until: U => "until"
    );

    impl_url_bool_field!(
        /// when false, do not follow events
        stream => "stream"
    );

    /// A list of constraints for events
    pub fn filters<F>(mut self, filters: F) -> Self
    where
        F: IntoIterator<Item = EventsConstraint>,
    {
        let filters: std::collections::HashMap<_, _> = filters.into_iter().collect();
        self.params.insert(
            "filters",
            serde_json::to_string(&filters).unwrap_or_default(),
        );
        self
    }
}

//####################################################################################################
//
// Containers
//
//####################################################################################################

impl_opts_builder!(url =>
    /// Adjust the list of returned containers with this options.
    ContainerList
);

#[derive(Debug)]
/// Used to filter listed containers by one of the variants.
pub enum ContainerListFilter {
    // TODO: add stronger types for parameters
    //
    /// Image name or <image-name>[:<tag>], <image id>, or <image@digest>
    Ancestor(String),
    /// Container ID or name
    Before(String),
    /// <port>[/<proto>] or <startport-endport>/[<proto>]
    Expose(String),
    /// Containers with exit code of
    Exited(i32),
    Health(models::ContainerHealth),
    /// A container's ID
    Id(crate::Id),
    IsTask(bool),
    /// Container label
    Label {
        key: String,
        value: String,
    },
    /// A container's name
    Name(String),
    /// Network ID or name
    Network(String),
    /// Pod ID or name
    Pod(String),
    /// <port>[/<proto>] or <startport-endport>/[<proto>]
    Publish(String),
    /// Container ID or name
    Since(String),
    Status(models::ContainerStatus),
    /// Volume name or mount point destination
    Volume(String),
}

impl Filter for ContainerListFilter {
    fn query_key_val(&self) -> (&'static str, String) {
        use ContainerListFilter::*;
        match &self {
            Ancestor(ancestor) => ("ancestor", ancestor.clone()),
            Before(container) => ("before", container.clone()),
            Expose(port) => ("expose", port.clone()),
            Exited(code) => ("exited", code.to_string()),
            Health(health) => ("health", health.as_ref().to_string()),
            Id(id) => ("id", id.to_string()),
            IsTask(is_task) => ("is-task", is_task.to_string()),
            Label { key, value } => ("label", format!("{}={}", key, value)),
            Name(name) => ("name", name.clone()),
            Network(net) => ("network", net.clone()),
            Pod(pod) => ("pod", pod.clone()),
            Publish(port) => ("publish", port.clone()),
            Since(container) => ("since", container.clone()),
            Status(status) => ("status", status.as_ref().to_string()),
            Volume(vol) => ("volume", vol.clone()),
        }
    }
}

impl ContainerListOptsBuilder {
    impl_url_bool_field!(
        /// Return all containers. By default, only running containers are shown
        all => "all"
    );

    impl_url_field!(
        /// Return this number of most recently created containers, including non-running ones.
        limit: usize => "limit"
    );

    impl_url_bool_field!(
        /// Return the size of container as fields `size_rw` and `size_root_fs`.
        size => "size"
    );

    impl_url_bool_field!(
        /// Sync container state with OCI runtime
        sync => "sync"
    );

    impl_filter_func!(ContainerListFilter);
}

impl_opts_builder!(url =>
    /// Adjust the way a container is stopped.
    ContainerStop
);

impl ContainerStopOptsBuilder {
    impl_url_bool_field!(
        /// Stop all containers
        all => "all"
    );

    impl_url_bool_field!(
        /// Do not return error if container is already stopped
        ignore => "Ignore"
    );

    impl_url_field!(
        /// number of seconds to wait before killing container
        timeout: usize => "Timeout"
    );
}

impl_opts_builder!(url =>
    /// Adjust the way a container is deleted.
    ContainerDelete
);

impl ContainerDeleteOptsBuilder {
    impl_url_bool_field!(
        /// Force delete the container.
        force => "force"
    );

    impl_url_bool_field!(
        /// Delete associated volumes.
        volumes => "v"
    );
}

impl_opts_builder!(url =>
    /// Adjust the way a container is checkpointed.
    ContainerCheckpoint
);

impl ContainerCheckpointOptsBuilder {
    impl_url_bool_field!(
        /// Export the checkpoint image to a tar.gz
        export => "export"
    );

    impl_url_bool_field!(
        /// Do not include root file-system changes when exporting
        ignore_root_fs => "ignoreRootFS"
    );

    impl_url_bool_field!(
        /// Keep all temporary checkpoint files
        keep => "keep"
    );

    impl_url_bool_field!(
        /// Leave the container running after writing checkpoint to disk
        leave_running => "leaveRunning"
    );

    impl_url_bool_field!(
        /// Add checkpoint statistics to the returned CheckpointReport
        print_stats => "printStats"
    );

    impl_url_bool_field!(
        /// Checkpoint a container with established TCP connections
        tcp_established => "tcpEstablished"
    );
}

impl_opts_builder!(url =>
    /// Adjust the way a new image is created from a container.
    ContainerCommit
);

impl ContainerCommitOpts {
    pub(crate) fn for_container(&self, container: crate::Id) -> Self {
        let mut new = self.clone();
        new.params.insert("container", container.to_string());
        new
    }
}

impl ContainerCommitOptsBuilder {
    impl_url_str_field!(
        /// Author of the image
        author: A => "author"
    );

    impl_url_vec_field!(
        /// Instructions to apply while committing in Dockerfile format (i.e. "CMD=/bin/foo")
        changes: C => "changes"
    );

    impl_url_str_field!(
        /// Commit message
        comment: C => "comment"
    );

    impl_url_str_field!(
        /// Format of the image manifest and metadata (default "oci")
        format: F => "format"
    );

    impl_url_bool_field!(
        /// Pause the container before committing it
        pause => "pause"
    );

    impl_url_str_field!(
        /// The repository name for the created image
        repo: R => "repo"
    );

    impl_url_str_field!(
        /// Tag name for the created image
        tag: T => "tag"
    );
}

//####################################################################################################
//
// Execs
//
//####################################################################################################

impl_opts_builder!(json =>
    /// Modify how an exec session is run inside a container.
    ExecCreate
);

#[derive(Debug, Clone)]
/// One of the variants accepted by [`ExecCreateOptsBuilder::user`](ExecCreateOptsBuilder::user).
pub enum UserOpt {
    User(String),
    UserGroup(String, String),
    Uid(isize),
    UidGid(isize, isize),
}

impl std::fmt::Display for UserOpt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use UserOpt::*;
        match self {
            User(user) => write!(f, "{}", user),
            Uid(uid) => write!(f, "{}", uid),
            UserGroup(user, group) => write!(f, "{}:{}", user, group),
            UidGid(uid, gid) => write!(f, "{}:{}", uid, gid),
        }
    }
}

impl ExecCreateOptsBuilder {
    impl_field!(
        /// Attach to stderr of the exec command
        attach_stderr: bool => "AttachStderr"
    );

    impl_field!(
        /// Attach to stdin of the exec command
        attach_stdin: bool => "AttachStdin"
    );

    impl_field!(
        /// Attach to stdout of the exec command
        attach_stdout: bool => "AttachStdout"
    );

    impl_vec_field!(
        /// Command to run, as a string or array of strings.
        command: C => "Cmd"
    );

    impl_str_field!(
        /// Override the key sequence for detaching a container. Format is a single character [a-Z] or ctrl- where is one of: a-z, @, ^, [, , or _.
        detach_keys: K => "DetachKeys"
    );

    /// A list of environment variables to use for the command execution.
    pub fn env<K, V>(mut self, vars: impl IntoIterator<Item = (K, V)>) -> Self
    where
        K: AsRef<str>,
        V: AsRef<str>,
    {
        self.params.insert(
            "Env",
            vars.into_iter()
                .map(|(k, v)| format!("{}={}", k.as_ref(), v.as_ref()))
                .collect(),
        );
        self
    }

    impl_field!(
        /// Runs the exec process with extended privileges
        privileged: bool => "Privileged"
    );

    impl_field!(
        /// Allocate a pseudo-TTY
        tty: bool => "Tty"
    );

    impl_str_enum_field!(
        /// The user, and optionally, group to run the exec process inside the container.
        user: UserOpt => "User"
    );

    impl_str_field!(
        /// The working directory for the exec process inside the container.
        working_dir: D => "WorkingDir"
    );
}

impl_opts_builder!(json =>
    /// Adjust how an exec instance is started inside of a running container.
    ExecStart
);

impl ExecStartOptsBuilder {
    impl_field!(
        /// Detach from the command.
        detach: bool => "Detach"
    );

    impl_field!(
        /// Height of the TTY session in characters. Tty must be set to true to use it.
        height: usize => "h"
    );

    impl_field!(
        /// Allocate a pseudo-TTY.
        tty: bool => "Tty"
    );

    impl_field!(
        /// Width of the TTY session in characters. Tty must be set to true to use it.
        width: usize => "w"
    );
}

impl_opts_builder!(url =>
    /// Adjust how to wait for a container.
    ContainerWait
);

impl ContainerWaitOptsBuilder {
    pub fn conditions<I>(mut self, conditions: I) -> Self
    where
        I: IntoIterator<Item = models::ContainerStatus>,
    {
        let joined = conditions
            .into_iter()
            .map(|it| format!("\"{}\"", it.as_ref()))
            .collect::<Vec<_>>()
            .join(",");
        self.params.insert("condition", format!("[{}]", joined));
        self
    }

    impl_url_str_field!(
        /// Time Interval to wait before polling for completion. Example: `250ms`, `2s`
        interval: I => "interval"
    );
}
