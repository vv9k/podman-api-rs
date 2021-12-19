//! Options used for configuring the behavior of certain API endpoints

use crate::{api::Filter, models};
use serde::Serialize;
use std::collections::HashMap;
use std::fmt;

pub type EventsConstraint = (String, Vec<String>);

impl_opts_builder!(
    url =>
    /// Used to filter events returned by [Podman::events](crate::Podman::events).
    Events
);

impl EventsOptsBuilder {
    impl_url_str_field!(
        /// Start streaming events from this time
        since => "since"
    );

    impl_url_str_field!(
        /// Stop streaming events later than this
        until => "until"
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
    /// Images that are ancestors.
    Ancestor(ImageOpt),
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
    /// Container key label.
    LabelKey(String),
    /// Container key-value label.
    LabelKeyVal(String, String),
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
            Ancestor(ancestor) => ("ancestor", ancestor.to_string()),
            Before(container) => ("before", container.clone()),
            Expose(port) => ("expose", port.clone()),
            Exited(code) => ("exited", code.to_string()),
            Health(health) => ("health", health.as_ref().to_string()),
            Id(id) => ("id", id.to_string()),
            IsTask(is_task) => ("is-task", is_task.to_string()),
            LabelKey(key) => ("label", key.clone()),
            LabelKeyVal(key, val) => ("label", format!("{}={}", key, val)),
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
        author => "author"
    );

    impl_url_vec_field!(
        /// Instructions to apply while committing in Dockerfile format (i.e. "CMD=/bin/foo")
        changes => "changes"
    );

    impl_url_str_field!(
        /// Commit message
        comment => "comment"
    );

    impl_url_str_field!(
        /// Format of the image manifest and metadata (default "oci")
        format => "format"
    );

    impl_url_bool_field!(
        /// Pause the container before committing it
        pause => "pause"
    );

    impl_url_str_field!(
        /// The repository name for the created image
        repo => "repo"
    );

    impl_url_str_field!(
        /// Tag name for the created image
        tag => "tag"
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

impl fmt::Display for UserOpt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
        command => "Cmd"
    );

    impl_str_field!(
        /// Override the key sequence for detaching a container. Format is a single character [a-Z] or ctrl- where is one of: a-z, @, ^, [, , or _.
        detach_keys => "DetachKeys"
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
        working_dir => "WorkingDir"
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
        interval => "interval"
    );
}

impl_opts_builder!(json =>
    /// Adjust the created container.
    ContainerCreate
);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Mode used to configure image volume with
/// [`image_volume_mode`](ContainerCreateOptsBuilder::image_volume_mode).
pub enum ImageVolumeMode {
    /// Do not create
    Ignore,
    /// Create a tmpfs
    Tmpfs,
    /// Create as anonymous volumes
    Anonymous,
}

impl AsRef<str> for ImageVolumeMode {
    fn as_ref(&self) -> &str {
        match self {
            &ImageVolumeMode::Ignore => "ignore",
            &ImageVolumeMode::Tmpfs => "tmpfs",
            &ImageVolumeMode::Anonymous => "anonymous",
        }
    }
}

impl fmt::Display for ImageVolumeMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// How to handle the `NOTIFY_SOCKET`. Used with
/// [`sdnotify_mode`](ContainerCreateOptsBuilder::sdnotify_mode).
pub enum SocketNotifyMode {
    /// Let the OCI runtime deal with it, advertise conmon's MAINPID.
    Container,
    /// Advertise conmon's MAINPID, send READY when started, don't pass to OCI.
    ConmonOnly,
    /// Unset `NOTIFY_SOCKET`
    Ignore,
}

impl AsRef<str> for SocketNotifyMode {
    fn as_ref(&self) -> &str {
        match self {
            &SocketNotifyMode::Container => "container",
            &SocketNotifyMode::ConmonOnly => "conmon-only",
            &SocketNotifyMode::Ignore => "ignore",
        }
    }
}

impl fmt::Display for SocketNotifyMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Used with [`ContainerCreateOptsBuilder::seccomp_policy`](ContainerCreateOptsBuilder::seccomp_policy).
pub enum SeccompPolicy {
    Empty,
    Default,
    Image,
}

impl AsRef<str> for SeccompPolicy {
    fn as_ref(&self) -> &str {
        match self {
            &SeccompPolicy::Empty => "empty",
            &SeccompPolicy::Default => "default",
            &SeccompPolicy::Image => "image",
        }
    }
}

impl fmt::Display for SeccompPolicy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Used with [`ContainerCreateOptsBuilder::seccomp_policy`](ContainerCreateOptsBuilder::seccomp_policy).
pub enum SystemdEnabled {
    True,
    False,
    Always,
}

impl AsRef<str> for SystemdEnabled {
    fn as_ref(&self) -> &str {
        match self {
            &SystemdEnabled::True => "true",
            &SystemdEnabled::False => "false",
            &SystemdEnabled::Always => "always",
        }
    }
}

impl fmt::Display for SystemdEnabled {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl ContainerCreateOptsBuilder {
    impl_map_field!(json
        /// Annotations are key-value options passed into the container runtime that can be used to
        /// trigger special behavior.
        annotations => "annotations"
    );

    impl_str_field!(
        /// ApparmorProfile is the name of the Apparmor profile the container will use.
        apparmor_profile => "apparmor_profile"
    );

    impl_vec_field!(
        /// Capabilities which will be added to the container. Conflicts with
        /// [`privileged`](ContainerCreateOptsBuilder::privileged).
        add_capabilities => "cap_add"
    );

    impl_vec_field!(
        /// Capabilities which will be removed from the container. Conflicts with
        /// [`privileged`](ContainerCreateOptsBuilder::privileged).
        drop_capabilities => "cap_drop"
    );

    impl_str_field!(
        /// Set the container's CGroup parent. If not set, the default for the current cgroup driver
        /// will be used.
        cgroup_parent => "cgroup_parent"
    );

    impl_field!(
        /// Namespace to use for cgroups.
        cgroup_namespace: models::Namespace => "cgroupns"
    );

    impl_str_field!(
        /// Sets a policy for how cgroups will be created in the container, including the ability
        /// to disable creation entirely.
        cgroup_mode => "cgroups_mode"
    );

    impl_vec_field!(
        /// Command that the container should run. If not given and Image is specified, this will
        /// be populated by the image's configuration.
        command => "command"
    );

    impl_str_field!(
        /// A path at which a PID file for Conmon will be placed. If not given, a default location
        /// will be used.
        common_pid_file => "common_pid_file"
    );

    impl_vec_field!(
        /// The command that was used to create this container. This will be returned when
        /// inspecting the container.
        create_command => "containerCreateCommand"
    );

    impl_field!(
        /// CPU period of the cpuset
        cpu_period: u64 => "cpu_period"
    );

    impl_field!(
        /// CPU quota of the cpuset
        cpu_quota: i64 => "cpu_quota"
    );

    impl_field!(
        /// Create the working directory if it doesn't exist. If unset, it doesn't create it.
        create_working_dir: bool => "create_working_dir"
    );

    impl_vec_field!(
        /// An array of containers this container depends on. Dependency containers must be started
        /// before this container. Dependencies can be specified by name or full/partial ID.
        dependency_containers => "dependencyContainers"
    );

    //TODO: device_cgroup_rule

    //TODO: devices

    impl_vec_field!(
        /// A way to ensure your container inherits device specific information from another container.
        devices_from => "device_from"
    );

    impl_vec_field!(
        /// A set of DNS options that will be used in the container's resolv.conf, replacing the host's
        /// DNS options which are used by default. Conflicts with
        /// [`use_image_resolv_conf`](ContainerCreateOptsBuilder::use_image_resolv_conf).
        dns_option => "dns_option"
    );

    impl_vec_field!(
        /// A set of DNS search domains that will be used in the container's resolv.conf, replacing
        /// the host's DNS search domains which are used by default. Conflicts with
        /// [`use_image_resolv_conf`](ContainerCreateOptsBuilder::use_image_resolv_conf).
        dns_search => "dns_search"
    );

    impl_vec_field!(
        /// A set of DNS servers that will be used in the container's resolv.conf, replacing the
        /// host's DNS Servers which are used by default. Conflicts with
        /// [`use_image_resolv_conf`](ContainerCreateOptsBuilder::use_image_resolv_conf).
        dns_server => "dns_server"
    );

    impl_vec_field!(
        /// Container's entrypoint. If not given and Image is specified, this will be populated by
        /// the image's configuration.
        entrypoint => "entrypoint"
    );

    impl_map_field!(json
        /// A list of environment variables that will be set in the container.
        env => "env"
    );

    impl_field!(
        /// Indicates that the host environment should be added to container.
        env_host: bool => "env_host"
    );

    impl_field!(
        /// Health config which holds configuration settings for the HEALTHCHECK feature, from
        /// docker/docker/api/types/container.
        health_config: models::Schema2HealthConfig => "healthconfig"
    );

    // TODO: host_device_list

    impl_vec_field!(
        /// A set of hosts which will be added to the container's etc/hosts file. Conflicts with
        /// [`use_image_hosts`](ContainerCreateOptsBuilder::use_image_hosts).
        hosts_add => "hostadd"
    );

    impl_str_field!(
        /// If not set, the hostname will not be modified (if UtsNS is not private) or will be set
        /// to the container ID (if UtsNS is private). Conflicts with UtsNS if UtsNS is not set to
        /// private.
        hostname => "hostname"
    );

    impl_field!(
        /// Indicates that the http host proxy environment variables should be added to container.
        http_proxy: bool => "httpproxy"
    );

    impl_field!(
        /// Used for specifying how ID mapping should be set up for a layer or container.
        id_mappings: models::IdMappingOptions => "idmappings"
    );

    impl_str_field!(
        /// Image is the image the container will be based on. The image will be used as the
        /// container's root filesystem, and its environment vars, volumes, and other configuration
        /// will be applied to the container. Conflicts with [`rootfs`](ContainerCreateOptsBuilder::rootfs).
        ///
        /// At least one of [`image`](ContainerCreateOptsBuilder::image) or
        /// [`rootfs`](ContainerCreateOptsBuilder::rootfs) must be specified.
        image => "image"
    );

    impl_str_enum_field!(
        /// Indicates how image volumes will be created. The default if unset is
        /// [`anonymous`](ImageVolumeMode::Anonymous).
        image_volume_mode: ImageVolumeMode => "image_volume_mode"
    );

    // TODO: image_volumes

    impl_field!(
        /// Specifies that an init binary will be mounted into the container, and will be used as
        /// PID1.
        init: bool => "init"
    );

    impl_str_field!(
        /// Describes if this container is an init container and if so, what type: always or once.
        init_container_type => "init_container_type"
    );

    impl_str_field!(
        /// Specifies the path to the init binary that will be added if
        /// [`init`](ContainerCreateOptsBuilder::init) is specified above. If not specified, the
        /// default set in the Libpod config will be used. Ignored if
        /// [`init`](ContainerCreateOptsBuilder::init) is not set.
        init_path => "init_path"
    );

    impl_field!(
        /// Namespace to use for IPC.
        ipc_namespace: models::Namespace => "ipcns"
    );

    impl_map_field!(json
        /// A list of labels that will be assigned to the container.
        labels => "labels"
    );

    impl_field!(
        /// Logging configuration for the container.
        log_configuration: models::LogConfig => "log_configuration"
    );

    impl_vec_field!(
        /// The path we want to mask in the container. This masks the paths given in addition to
        /// the default list.
        mask => "mask"
    );

    impl_str_field!(
        /// The name the container will be given. If no name is provided, one will be randomly
        /// generated.
        name => "name"
    );

    impl_str_field!(
        /// The libpod namespace the container will be placed in.
        namespace => "namespace"
    );

    impl_field!(
        /// Namespace to use for network.
        net_namespace: models::Namespace => "netns"
    );

    impl_map_field!(json
        /// Additional options for each network.
        network_options => "network_options"
    );

    // TODO: networks

    impl_field!(
        /// Whether the container will set the no new privileges flag on create, which disables
        /// gaining additional privileges (e.g. via setuid) in the container.
        no_new_privilages: bool => "no_new_privilages"
    );

    impl_str_field!(
        /// The name of the OCI runtime that will be used to create the container. If not
        /// specified, the default will be used.
        oci_runtime => "oci_runtime"
    );

    impl_field!(
        /// Adjusts the score used by the OOM killer to determine processes to kill for the container's process.
        oom_score_adj: i64 => "oom_score_adj"
    );

    // TODO: overlay_volumes

    impl_field!(
        /// Specify the Linux personality syscall input.
        personality: models::LinuxPersonality => "personality"
    );

    impl_field!(
        /// Namespace to use for pids.
        pid_namespace: models::Namespace => "pidns"
    );

    impl_str_field!(
        /// ID of the pod the container should join.
        pod => "pod"
    );

    // TODO: portmappings

    impl_field!(
        /// Whether the container is privileged. Privileged does the following: Adds all devices on
        /// the system to the container. Adds all capabilities to the container. Disables Seccomp,
        /// SELinux, and Apparmor confinement. (Though SELinux can be manually re-enabled).
        privileged: bool => "privileged"
    );

    impl_vec_field!(
        /// The options used for the proc mount.
        procfs_opts => "procfs_opts"
    );

    impl_field!(
        /// If set to true the ports specified in the image will be published to random unused ports
        /// (guaranteed to be above 1024) on the host. This is based on ports set in Expose below,
        /// and any ports specified by the Image (if one is given). Only available if
        /// [`net_namespace`](ContainerCreateOptsBuilder::net_namespace) is set to Bridge or Slirp.
        publish_image_ports: bool => "publish_image_ports"
    );

    // TODO: r_limits

    impl_str_field!(
        /// The user-specified and unprocessed input referring to a local or a remote image.
        raw_image_name => "raw_image_name"
    );

    impl_field!(
        /// If set to true everything will be mounted as read-only.
        read_only_fs: bool => "read_only_filesystem"
    );

    impl_field!(
        /// If set to true the container will be removed upon exitting.
        remove: bool => "remove"
    );

    impl_field!(
        /// Set the container runtime resource contstraints.
        resource_limits: models::LinuxResources => "resource_limits"
    );

    impl_str_field!(
        /// An action which will be taken when the container exits. If not given, the default
        /// policy, which does nothing, will be used.
        restart_policy => "restart_policy"
    );

    impl_field!(
        /// The number of attempts that will be made to restart the container. Only available
        /// when [`restart_policy`](ContainerCreateOptsBuilder::restart_policy) is set to `on-failure`.
        restart_tries: u64 => "restart_tries"
    );

    impl_str_field!(
        /// The path to a directory that will be used as the container's root filesystem. No
        /// modification will be made to the directory, it will be directly mounted into the
        /// container as root. Conflicts with [`image`](ContainerCreateOptsBuilder::image).
        ///
        /// At least one of [`image`](ContainerCreateOptsBuilder::image) or
        /// [`rootfs`](ContainerCreateOptsBuilder::rootfs) must be specified.
        rootfs => "rootfs"
    );

    impl_field!(
        /// Tells if rootfs is actuall an overlay on top of base path.
        rootfs_overlay: bool => "rootfs_overlay"
    );

    impl_str_field!(
        /// The rootfs propagation mode for the container. If not set, the default of rslave will
        /// be used.
        rootfs_propagation => "rootfs_propagation"
    );

    impl_str_enum_field!(
        /// Determine how to handle `NOTIFY_SOCKET`.
        sdnotify_mode: SocketNotifyMode => "sdnotifyMode"
    );

    impl_str_enum_field!(
        /// Determines which seccomp profile gets applied the container.
        seccomp_policy: SeccompPolicy => "seccomp_policy"
    );

    impl_str_field!(
        /// The path to a JSON file containing the container's Seccomp profile. If not specified,
        /// no Seccomp profile will be used.
        seccomp_profile_path => "seccomp_profile_path"
    );

    impl_map_field!(json
        /// A list of secrets that will be set as environment variables.
        secret_env => "secret_env"
    );

    // TODO: secrets

    impl_vec_field!(
        /// The process label the container will use. if SELinux is enabled and this is not
        /// specified, a label will be automatically generated if not specified.
        selinux_opts => "selinux_opts"
    );

    impl_field!(
        /// The size of the tmpfs to mount in at /dev/shm, in bytes.
        shm_size: i64 => "shm_size"
    );

    impl_field!(
        /// Whether the container should keep it's STDIN open.
        stdin: bool => "stdin"
    );

    impl_field!(
        /// A number describing a process signal.
        stop_signal: i64 => "stop_signal"
    );

    impl_field!(
        /// A timeout between the container's stop signal being sent and SIGKILL being sent. If not
        /// provided, the default will be used. If 0 is used, stop signal will not be sent, and
        /// SIGKILL will be sent instead.
        stop_timeout: u64 => "stop_timeout"
    );

    impl_map_field!(json
        /// A list of container's storage options.
        storage_opts => "storage_opts"
    );

    impl_map_field!(json
        /// A list of kernel parameters to set in the container.
        sysctl => "sysctl"
    );

    impl_str_enum_field!(
        systemd: SystemdEnabled => "systemd"
    );

    impl_field!(
        /// Whether the container will create a PTY.
        terminal: bool => "terminal"
    );

    // TODO: throttleReadBpsDevice
    // TODO: throttleReadIOPSDevice
    // TODO: throttleWriteBpsDevice
    // TODO: throttleWriteIOPSDevice

    impl_field!(
        /// A maximum time in seconds the container will run before main process is sent SIGKILL.
        /// If 0 is used, signal will not be sent.
        timeout: u64 => "timeout"
    );

    impl_str_field!(
        /// The timezone inside the container. Local means it has the same timezone as the host
        /// machine.
        timezone => "timezone"
    );

    impl_str_field!(
        /// The umask the init process of the container will be run with.
        umask => "umask"
    );

    impl_map_field!(json
        /// A list of key-value options passed into the container runtime that are used to
        /// configure cgroup v2.
        unified => "unified"
    );

    impl_vec_field!(
        /// The path we want to unmask in the container. To override all the default paths that are
        /// masked, set unmask=ALL.
        unmask => "unmask"
    );

    impl_vec_field!(
        /// A list of environment variables to unset if specified in the image or from buildin or
        /// containers.conf
        unset_env => "unsetenv"
    );

    impl_field!(
        /// If true all environment variables from the image or from buldin or containers.conf will
        /// get unset.
        unset_env_all: bool => "unsetenvall"
    );

    impl_field!(
        /// Indicates that /etc/hosts should not be managed by Podman, and instead sourced from the image.
        /// Conflicts with [`hosts_add`](ContainerCreateOptsBuilder::hosts_add).
        use_image_hosts: bool => "use_image_hosts"
    );

    impl_field!(
        /// Indicates that /etc/hosts should not be managed by Podman, and instead sourced from the image.
        /// Conflicts with [`dns_server`](ContainerCreateOptsBuilder::dns_server),
        /// [`dns_search`](ContainerCreateOptsBuilder::dns_search),
        /// [`dns_option`](ContainerCreateOptsBuilder::dns_option).
        use_image_resolv_conf: bool => "use_image_resolv_conf"
    );

    impl_str_field!(
        /// The user the container will be run as. Can be given as a UID or a username; if a username,
        /// it will be resolved within the container, using the container's /etc/passwd. If unset, the
        /// container will be run as root.
        user => "user"
    );

    impl_field!(
        /// Namespace to use for users.
        user_namespace: models::Namespace => "userns"
    );

    impl_field!(
        /// Namespace to use for uts.
        uts_namespace: models::Namespace => "utsns"
    );

    impl_field!(
        /// Specifies whether the container storage can be optimized at the cost of not syncing all
        /// the dirty files in memory.
        volatile: bool => "volatile"
    );

    // TODO: volumes

    impl_vec_field!(
        /// Set of containers whose volumes will be added to this container. The name or ID of the
        /// container must be provided, and may optionally be followed by a : and then one or more
        /// comma-separated options. Valid options are 'ro', 'rw', and 'z'. Options will be used
        /// for all volumes sourced from the container.
        volumes_from => "volumes_from"
    );

    impl_field!(
        /// Weight per cgroup per device.
        weight_device: models::LinuxWeightDevice => "weightDevice"
    );

    impl_str_field!(
        /// Override the container's working directory. If unset, the default, `/`, will be used.
        work_dir => "work_dir"
    );
}

impl_opts_builder!(url =>
    /// Adjust how an image is built.
    ImageBuild
);

#[derive(Debug, Clone, PartialEq, Eq)]
/// The networking mode for the run commands during image build.
pub enum NetworkMode {
    /// Limited to containers within a single host, port mapping required for external access.
    Bridge,
    /// No isolation between host and containers on this network.
    Host,
    /// Disable all networking for this container.
    None,
    /// Share networking with given container.
    Container,
    /// Custom network's name.
    Custom(String),
}

impl AsRef<str> for NetworkMode {
    fn as_ref(&self) -> &str {
        match self {
            NetworkMode::Bridge => "bridge",
            NetworkMode::Host => "host",
            NetworkMode::None => "none",
            NetworkMode::Container => "container",
            NetworkMode::Custom(custom) => &custom,
        }
    }
}

impl fmt::Display for NetworkMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

#[derive(Debug, Clone, PartialEq)]
/// Used to set the image platform with [`platform`](ImageBuildOptsBuilder::platform).
pub struct Platform {
    os: String,
    arch: Option<String>,
    version: Option<String>,
}

impl Platform {
    pub fn new(os: impl Into<String>) -> Self {
        Self {
            os: os.into(),
            arch: None,
            version: None,
        }
    }

    pub fn arch(mut self, arch: impl Into<String>) -> Self {
        self.arch = Some(arch.into());
        self
    }

    pub fn version(mut self, version: impl Into<String>) -> Self {
        self.version = Some(version.into());
        self
    }
}

impl fmt::Display for Platform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(arch) = &self.arch {
            if let Some(vers) = &self.version {
                write!(f, "{}/{}/{}", self.os, arch, vers)
            } else {
                write!(f, "{}/{}", self.os, arch)
            }
        } else {
            write!(f, "{}", self.os)
        }
    }
}

impl ImageBuildOptsBuilder {
    impl_map_field!(url
        /// Key-value build time variables.
        build_args => "buildargs"
    );

    /// List of images used to build cache resolution
    pub fn cache_from<I>(mut self, images: impl IntoIterator<Item = I>) -> Self
    where
        I: Into<String>,
    {
        self.params.insert(
            "cachefrom",
            serde_json::to_string(&images.into_iter().map(|i| i.into()).collect::<Vec<_>>())
                .unwrap_or_default(),
        );
        self
    }

    impl_url_field!(
        /// Limits the CPU CFS (Completely Fair Scheduler) period.
        cpu_period: isize => "cpuperiod"
    );

    impl_url_field!(
        /// Limits the CPU CFS (Completely Fair Scheduler) quota.
        cpu_quota: isize => "cpuquota"
    );

    impl_url_field!(
        /// Set CPUs in which to allow execution. Example: `0-1`, `1-3`
        cpu_set_cpus: isize => "cpusetcpus"
    );

    impl_url_field!(
        /// CPU shares - relative weights
        cpu_shares: isize => "cpushares"
    );

    impl_url_str_field!(
        /// Path within the build context to the Dockerfile. This is ignored
        /// if remote is specified and points to an external Dockerfile.
        dockerfile => "Dockerfile"
    );

    impl_url_str_field!(
        /// Extra hosts to add to /etc/hosts.
        extra_hosts => "extrahosts"
    );

    impl_url_bool_field!(
        /// Always remove intermediate containers, even upon failure.
        force_rm => "forcerm"
    );

    impl_url_bool_field!(
        /// Inject http proxy environment variables into container.
        http_proxy => "httpproxy"
    );

    impl_map_field!(url
        /// Key-value pairs to set as labels on the new image.
        labels => "labels"
    );

    impl_url_bool_field!(
        /// Cache intermediate layers during build.
        layers => "layers"
    );

    impl_url_field!(
        /// The upper limit (in bytes) on how much memory running
        /// containers can use.
        memory: usize => "memory"
    );

    impl_url_field!(
        /// Limits the amount of memory and swap together.
        memswap: usize => "memswap"
    );

    impl_url_enum_field!(
        /// Set the networking mode for the run commands during build.
        network_mode: NetworkMode => "networkmode"
    );

    impl_url_bool_field!(
        /// Do not use the cache when building the image.
        no_cache => "nocache"
    );

    impl_url_str_field!(
        /// Output configuration.
        outputs => "outputs"
    );

    pub fn platform(mut self, platform: Platform) -> Self {
        self.params.insert("platform", platform.to_string());
        self
    }

    impl_url_bool_field!(
        /// Attempt to pull the image even if an older image exists locally.
        pull => "pull"
    );

    impl_url_bool_field!(
        /// Suppress verbose build output.
        quiet => "q"
    );

    impl_url_str_field!(
        /// A Git repository URI or HTTP/HTTPS context URI. If the URI points
        /// to a single text file, the file’s contents are placed into a file
        /// called Dockerfile and the image is built from that file. If the URI
        /// points to a tarball, the file is downloaded by the daemon and
        /// the contents therein used as the context for the build. If the URI
        /// points to a tarball and the dockerfile parameter is also specified,
        /// there must be a file with the corresponding path inside the tarball.
        remote => "remote"
    );

    impl_url_bool_field!(
        /// Remove intermediate containers after a successful build.
        remove => "rm"
    );

    impl_url_field!(
        /// Value to use when mounting an shmfs on the container's /dev/shm directory.
        /// Default is 64MB
        shared_mem_size: usize => "shmsize"
    );

    impl_url_bool_field!(
        /// Silently ignored. Squash the resulting images layers into a single layer.
        squash => "squash"
    );

    impl_url_str_field!(
        /// A name and optional tag to apply to the image in the `name:tag` format.
        tag => "t"
    );

    impl_url_str_field!(
        /// Target build stage
        target => "target"
    );
}

impl_opts_builder!(url =>
    /// Adjust how images are listed.
    ImageList
);

#[derive(Debug, Clone)]
/// Used to filter listed images with [`ImagesListFilter`](ImagesListFilter).
pub enum ImageOpt {
    Name(crate::Id),
    Tag(crate::Id, String),
    Digest(crate::Id, String),
}

impl fmt::Display for ImageOpt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ImageOpt::*;
        match self {
            Name(id) => write!(f, "{}", id),
            Tag(id, tag) => write!(f, "{}:{}", id, tag),
            Digest(id, digest) => write!(f, "{}@{}", id, digest),
        }
    }
}

#[derive(Debug)]
/// Used to filter listed images by one of the variants.
pub enum ImageListFilter {
    Before(ImageOpt),
    Dangling(bool),
    /// Image that contains key label.
    LabelKey(String),
    /// Image that contains key-value label.
    LabelKeyVal(String, String),
    /// Image name with optional tag.
    Reference(crate::Id, Option<String>),
    Id(crate::Id),
    Since(ImageOpt),
}

impl Filter for ImageListFilter {
    fn query_key_val(&self) -> (&'static str, String) {
        use ImageListFilter::*;
        match &self {
            Before(image) => ("before", image.to_string()),
            Dangling(dangling) => ("dangling", dangling.to_string()),
            LabelKey(key) => ("label", key.clone()),
            LabelKeyVal(key, val) => ("label", format!("{}={}", key, val)),
            Reference(image, tag) => (
                "reference",
                if let Some(tag) = tag {
                    format!("{}:{}", image, tag)
                } else {
                    image.to_string()
                },
            ),
            Id(id) => ("id", id.to_string()),
            Since(image) => ("since", image.to_string()),
        }
    }
}

impl ImageListOptsBuilder {
    impl_filter_func!(ImageListFilter);

    impl_url_bool_field!(
        /// Show all images. Only images from a final layer (no children) are shown by default.
        all => "all"
    );
}

impl_opts_builder!(url =>
    /// Adjust the way an image is tagged/untagged.
    ImageTag
);

impl ImageTagOptsBuilder {
    impl_url_str_field!(
        repository => "repo"
    );

    impl_url_str_field!(
        tag => "tag"
    );
}

#[derive(Clone, Serialize, Debug)]
#[serde(untagged)]
pub enum RegistryAuth {
    Password {
        username: String,
        password: String,

        #[serde(skip_serializing_if = "Option::is_none")]
        email: Option<String>,

        #[serde(rename = "serveraddress")]
        #[serde(skip_serializing_if = "Option::is_none")]
        server_address: Option<String>,
    },
    Token {
        #[serde(rename = "identitytoken")]
        identity_token: String,
    },
}

impl RegistryAuth {
    /// return a new instance with token authentication
    pub fn token(token: impl Into<String>) -> RegistryAuth {
        RegistryAuth::Token {
            identity_token: token.into(),
        }
    }

    /// return a new instance of a builder for authentication
    pub fn builder() -> RegistryAuthBuilder {
        RegistryAuthBuilder::default()
    }

    /// serialize authentication as JSON in base64
    pub fn serialize(&self) -> String {
        serde_json::to_string(self)
            .map(|c| base64::encode_config(&c, base64::URL_SAFE))
            .unwrap_or_default()
    }
}

#[derive(Default)]
pub struct RegistryAuthBuilder {
    username: Option<String>,
    password: Option<String>,
    email: Option<String>,
    server_address: Option<String>,
}

impl RegistryAuthBuilder {
    /// The username used for authentication.
    pub fn username(&mut self, username: impl Into<String>) -> &mut Self {
        self.username = Some(username.into());
        self
    }

    /// The password used for authentication.
    pub fn password(&mut self, password: impl Into<String>) -> &mut Self {
        self.password = Some(password.into());
        self
    }

    /// The email addres used for authentication.
    pub fn email(&mut self, email: impl Into<String>) -> &mut Self {
        self.email = Some(email.into());
        self
    }

    /// The server address of registry, should be a domain/IP without a protocol.
    /// Example: `10.92.0.1`, `docker.corp.local`
    pub fn server_address(&mut self, server_address: impl Into<String>) -> &mut Self {
        self.server_address = Some(server_address.into());
        self
    }

    /// Create the final authentication object.
    pub fn build(&self) -> RegistryAuth {
        RegistryAuth::Password {
            username: self.username.clone().unwrap_or_default(),
            password: self.password.clone().unwrap_or_default(),
            email: self.email.clone(),
            server_address: self.server_address.clone(),
        }
    }
}

#[derive(Default, Debug)]
pub struct PullOpts {
    auth: Option<RegistryAuth>,
    params: HashMap<&'static str, String>,
}

impl PullOpts {
    /// return a new instance of a builder for Opts
    pub fn builder() -> PullOptsBuilder {
        PullOptsBuilder::default()
    }

    /// serialize Opts as a string. returns None if no Opts are defined
    pub fn serialize(&self) -> Option<String> {
        if self.params.is_empty() {
            None
        } else {
            Some(crate::util::url::encoded_pairs(
                self.params.iter().map(|(k, v)| (k, v)),
            ))
        }
    }

    pub(crate) fn auth_header(&self) -> Option<String> {
        self.auth.clone().map(|a| a.serialize())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// The networking mode for the run commands during image build.
pub enum PullPolicy {
    Always,
    Missing,
    Newer,
    Never,
}

impl AsRef<str> for PullPolicy {
    fn as_ref(&self) -> &str {
        match self {
            PullPolicy::Always => "always",
            PullPolicy::Missing => "missing",
            PullPolicy::Newer => "newer",
            PullPolicy::Never => "never",
        }
    }
}

impl fmt::Display for PullPolicy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

#[derive(Debug, Default)]
pub struct PullOptsBuilder {
    auth: Option<RegistryAuth>,
    params: HashMap<&'static str, String>,
}

impl PullOptsBuilder {
    impl_url_bool_field!(
        /// Pull all tagged images in the repository.
        all_tags => "allTags"
    );

    impl_url_str_field!(
        /// Pull image for the specified architecture.
        arch => "Arch"
    );

    impl_url_str_field!(
        /// username:password for the registry.
        credentials => "credentials"
    );

    impl_url_str_field!(
        /// Pull image for the specified operating system.
        os => "OS"
    );

    impl_url_enum_field!(
        /// Image pull policy.
        policy: PullPolicy => "policy"
    );

    impl_url_bool_field!(
        /// Silences extra stream data on pull.
        quiet => "quiet"
    );

    impl_url_str_field!(
        /// Mandatory reference to the image.
        reference => "referene"
    );

    impl_url_bool_field!(
        /// Require TLS verification.
        tls_verify => "tlsVerify"
    );

    impl_url_str_field!(
        /// Pull image for the specified variant.
        variant => "Variant"
    );

    pub fn auth(&mut self, auth: RegistryAuth) -> &mut Self {
        self.auth = Some(auth);
        self
    }

    pub fn build(&mut self) -> PullOpts {
        PullOpts {
            auth: self.auth.take(),
            params: self.params.clone(),
        }
    }
}
