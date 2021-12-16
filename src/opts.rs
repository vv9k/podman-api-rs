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
