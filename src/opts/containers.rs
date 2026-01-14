use crate::models;
use crate::opts::ImageOpt;
use containers_api::opts::{Filter, FilterItem};
use containers_api::{
    impl_field, impl_filter_func, impl_map_field, impl_opts_builder, impl_str_enum_field,
    impl_str_field, impl_url_bool_field, impl_url_field, impl_url_str_field, impl_url_vec_field,
    impl_vec_field,
};
use std::fmt;

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
    /// Container with key label.
    LabelKey(String),
    /// Container with key-value label.
    LabelKeyVal(String, String),
    /// Container without key label.
    NoLabelKey(String),
    /// Container without key-value label.
    NoLabelKeyVal(String, String),
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
    fn query_item(&self) -> FilterItem {
        use ContainerListFilter::*;
        match &self {
            Ancestor(ancestor) => FilterItem::new("ancestor", ancestor.to_string()),
            Before(container) => FilterItem::new("before", container.clone()),
            Expose(port) => FilterItem::new("expose", port.clone()),
            Exited(code) => FilterItem::new("exited", code.to_string()),
            Health(health) => FilterItem::new("health", health.as_ref().to_string()),
            Id(id) => FilterItem::new("id", id.to_string()),
            IsTask(is_task) => FilterItem::new("is-task", is_task.to_string()),
            LabelKey(key) => FilterItem::new("label", key.clone()),
            LabelKeyVal(key, val) => FilterItem::new("label", format!("{key}={val}")),
            NoLabelKey(key) => FilterItem::new("label!", key.clone()),
            NoLabelKeyVal(key, val) => FilterItem::new("label!", format!("{key}={val}",)),
            Name(name) => FilterItem::new("name", name.clone()),
            Network(net) => FilterItem::new("network", net.clone()),
            Pod(pod) => FilterItem::new("pod", pod.clone()),
            Publish(port) => FilterItem::new("publish", port.clone()),
            Since(container) => FilterItem::new("since", container.clone()),
            Status(status) => FilterItem::new("status", status.as_ref().to_string()),
            Volume(vol) => FilterItem::new("volume", vol.clone()),
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

    impl_url_field!(
        /// Delete associated volumes.
        timeout: usize => "timeout"
    );
}

impl_opts_builder!(url =>
    /// Adjust the way a container is checkpointed.
    ContainerCheckpoint
);

impl ContainerCheckpointOpts {
    pub(crate) fn for_export(&self) -> Self {
        let mut new = self.clone();
        new.params.insert("export", true.to_string());
        new
    }
}

impl ContainerCheckpointOptsBuilder {
    impl_url_bool_field!(
        /// Export the checkpoint image to a tar.gz
        export => "export"
    );

    impl_url_bool_field!(
        /// Checkpoint a container with filelocks`
        file_locks => "fileLocks"
    );

    impl_url_bool_field!(
        /// Do not include root file-system changes when exporting
        ignore_root_fs => "ignoreRootFS"
    );

    impl_url_bool_field!(
        /// Do not include associated volumes. can only be used with export
        ignore_volumes => "ignoreVolumes"
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
        /// Dump the container's memory information only, leaving the container running. only works on runc 1.0-rc or higher
        pre_checkpoint => "preCheckpoint"
    );

    impl_url_bool_field!(
        /// Add checkpoint statistics to the returned CheckpointReport
        print_stats => "printStats"
    );

    impl_url_bool_field!(
        /// Checkpoint a container with established TCP connections
        tcp_established => "tcpEstablished"
    );

    impl_url_bool_field!(
        /// Check out the container with previous criu image files in pre-dump. only works on runc 1.0-rc or higher
        with_previous => "withPrevious"
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

impl_opts_builder!(url =>
    /// Adjust how to wait for a container.
    ContainerWait
);

impl ContainerWaitOptsBuilder {
    pub fn conditions(
        mut self,
        conditions: impl IntoIterator<Item = models::ContainerStatus>,
    ) -> Self {
        self.vec_params.insert(
            "condition",
            conditions.into_iter().map(|c| c.as_ref().into()).collect(),
        );
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
#[derive(Default)]
pub enum ImageVolumeMode {
    /// Do not create
    Ignore,
    /// Create a tmpfs
    Tmpfs,
    /// Create as anonymous volumes
    #[default]
    Anonymous,
}

impl AsRef<str> for ImageVolumeMode {
    fn as_ref(&self) -> &str {
        match self {
            ImageVolumeMode::Ignore => "ignore",
            ImageVolumeMode::Tmpfs => "tmpfs",
            ImageVolumeMode::Anonymous => "anonymous",
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
    Conmon,
    /// Unset `NOTIFY_SOCKET`
    Ignore,
}

impl AsRef<str> for SocketNotifyMode {
    fn as_ref(&self) -> &str {
        match self {
            SocketNotifyMode::Container => "container",
            SocketNotifyMode::Conmon => "conmon",
            SocketNotifyMode::Ignore => "ignore",
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
#[derive(Default)]
pub enum SeccompPolicy {
    Empty,
    #[default]
    Default,
    Image,
}

impl AsRef<str> for SeccompPolicy {
    fn as_ref(&self) -> &str {
        match self {
            SeccompPolicy::Empty => "empty",
            SeccompPolicy::Default => "default",
            SeccompPolicy::Image => "image",
        }
    }
}

impl fmt::Display for SeccompPolicy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Used with [`ContainerCreateOptsBuilder::systemd`](ContainerCreateOptsBuilder::systemd).
#[derive(Default)]
pub enum SystemdEnabled {
    True,
    #[default]
    False,
    Always,
}

impl AsRef<str> for SystemdEnabled {
    fn as_ref(&self) -> &str {
        match self {
            SystemdEnabled::True => "true",
            SystemdEnabled::False => "false",
            SystemdEnabled::Always => "always",
        }
    }
}

impl fmt::Display for SystemdEnabled {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Used with
/// [`ContainerCreateOptsBuilder::restart_policy`](ContainerCreateOptsBuilder::restart_policy).
#[derive(Default)]
pub enum ContainerRestartPolicy {
    Always,
    #[default]
    No,
    OnFailure,
    UnlessStopped,
}

impl AsRef<str> for ContainerRestartPolicy {
    fn as_ref(&self) -> &str {
        match self {
            ContainerRestartPolicy::Always => "always",
            ContainerRestartPolicy::No => "no",
            ContainerRestartPolicy::OnFailure => "on-failure",
            ContainerRestartPolicy::UnlessStopped => "unless-stopped",
        }
    }
}

impl fmt::Display for ContainerRestartPolicy {
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
        /// Additional set of directories that need to be treated as root directories.
        /// Standard bind mounts will be mounted into paths relative to these directories.
        chroot_directories => "chroot_directories"
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

    impl_vec_field!(
        /// DeviceCgroupRule are device cgroup rules that allow containers to use additional types of devices.
        device_cgroup_rule : models::LinuxDeviceCgroup => "device_cgroup_rule"
    );

    impl_vec_field!(
        /// Devices are devices that will be added to the container.
        devices : models::LinuxDevice => "devices"
    );

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

    impl_vec_field!(
        /// Takes the specified environment variables from image and preprocess them before injecting them into the container.
        envmerge => "envmerge"
    );

    impl_vec_field!(
        /// Groups are a list of supplemental groups the container's user will be granted access to.
        groups => "groups"
    );

    impl_field!(
        /// Defines how Podman reacts when a container's health status turns unhealthy.
        health_check_on_failure_action: i64 => "health_check_on_failure_action"
    );

    impl_field!(
        /// Health config which holds configuration settings for the HEALTHCHECK feature, from
        /// docker/docker/api/types/container.
        health_config: models::Schema2HealthConfig => "healthconfig"
    );

    impl_vec_field!(
        /// The bits have the same definition on all systems, so that information about files can be moved from one system to another portably.
        /// Not all bits apply to all systems. The only required bit is ModeDir for directories.
        host_device_list : models::LinuxDevice => "host_device_list"
    );

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

    impl_vec_field!(
        /// List of host usernames or UIDs to add to the container etc/passwd file.
        hostusers => "hostusers"
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

    impl_str_field!(
        /// User-specified image architecture
        image_arch => "image_arch"
    );

    impl_str_field!(
        /// User-specified image OS
        image_os => "image_os"
    );

    impl_str_field!(
        /// User-specified image variant
        image_variant => "image_variant"
    );

    impl_str_enum_field!(
        /// Indicates how image volumes will be created. The default if unset is
        /// [`anonymous`](ImageVolumeMode::Anonymous).
        image_volume_mode: ImageVolumeMode => "image_volume_mode"
    );

    impl_vec_field!(
        /// Image volumes bind-mount a container-image mount into the container.
        image_volumes : models::ImageVolume => "image_volumes"
    );

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

    impl_field!(
        /// Container run option that determines if we are validating users/groups before
        /// running the container.
        manage_password: bool => "manage_password"
    );

    impl_vec_field!(
        /// The path we want to mask in the container. This masks the paths given in addition to
        /// the default list.
        mask => "mask"
    );

    impl_vec_field!(
        /// Mounts that will be added to the container. These will supersede
        /// [`image_volumes`](ContainerCreateOptsBuilder::image_volumes) and
        /// [`volumes_from`](ContainerCreateOptsBuilder::volumes_from) volumes where there
        /// are conflicts.
        mounts: models::ContainerMount => "mounts"
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

    impl_map_field!(json
        /// Map of networks names or ids that the container should join.
        /// You can request additional settings for each network, you can set network aliases,
        /// static ips, static mac address and the network interface name for this container on the specific network.
        /// If the map is empty and the bridge network mode is set the container will be joined to the default network.
        networks => "Networks"
    );

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

    impl_vec_field!(
        /// Overlay volumes are named volumes that will be added to the container.
        overlay_volumes : models::OverlayVolume => "overlay_volumes"
    );

    impl_str_field!(
        /// Specifies arbitrary data to append to a file.
        passwd_entry => "passwd_entry"
    );

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

    impl_vec_field!(
        /// Set of ports to map into the container. Only available if NetNS is set to bridge or
        /// slirp.
        portmappings: models::PortMapping => "portmappings"
    );

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

    impl_vec_field!(
        /// Rlimits are POSIX rlimits to apply to the container. Optional.
        r_limits : models::PosixRlimit => "r_limits"
    );

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

    impl_str_enum_field!(
        /// An action which will be taken when the container exits. If not given, the default
        /// policy, which does nothing, will be used.
        restart_policy: ContainerRestartPolicy => "restart_policy"
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

    /*
    // TODO update for podman 4.5/5
    impl_vec_field!(
        /// Secrets are the secrets that will be added to the container.
        secrets :models::Secret => "secrets"
    );
    */

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

    impl_map_field!(json
        /// IO read rate limit per cgroup per device, bytes per second
        throttle_read_bps_device => "throttleReadBpsDevice"
    );

    impl_map_field!(json
        ///IO read rate limit per cgroup per device, IO per second
        throttle_read_iops_device => "throttleReadIOPSDevice"
    );

    impl_map_field!(json
        /// IO write rate limit per cgroup per device, bytes per second
        throttle_write_bps_device => "throttleWriteBpsDevice"
    );

    impl_map_field!(json
        /// IO write rate limit per cgroup per device, IO per second
        throttle_write_iops_device => "throttleWriteIOPSDevice"
    );

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

    impl_vec_field!(
        /// Specifies the container volume to use with this container.
        volumes: models::NamedVolume => "volumes"
    );

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
    /// Adjust how to attach to a running container.
    ContainerAttach
);

impl ContainerAttachOpts {
    pub(crate) fn stream(&self) -> Self {
        let mut new = self.clone();
        new.params.insert("stream", true.to_string());
        new
    }
}

impl ContainerAttachOptsBuilder {
    impl_url_str_field!(
        /// Keys to use for detaching from the container.
        detach_keys => "detachKeys"
    );

    impl_url_bool_field!(
        /// Attach to container STDERR
        stderr => "stderr"
    );

    impl_url_bool_field!(
        /// Attach to container STDIN
        stdin => "stdin"
    );

    impl_url_bool_field!(
        /// Attach to container STDOUT
        stdout => "stdout"
    );
}

impl_opts_builder!(url =>
    /// Adjust how to attach to a running container.
    ContainerLogs
);

impl ContainerLogsOptsBuilder {
    impl_url_bool_field!(
        /// Keep connection after returning logs.
        follow => "follow"
    );

    impl_url_str_field!(
        /// Only return logs since this time, as a UNIX timestamp.
        since => "since"
    );

    impl_url_bool_field!(
        /// Return logs from STDERR.
        stderr => "stderr"
    );

    impl_url_bool_field!(
        /// Return logs from STDOUT.
        stdout => "stdout"
    );

    impl_url_str_field!(
        /// Only return this number of log lines from the end of the logs. Default - `all`
        tail => "tail"
    );

    impl_url_bool_field!(
        /// Add timestamps to every log line.
        timestamps => "timestamps"
    );

    impl_url_str_field!(
        /// Only return logs before this time, as a UNIX timestamp.
        until => "until"
    );
}

impl_opts_builder!(url =>
    /// Adjust how container stats are reported.
    ContainerStats
);

impl ContainerStatsOpts {
    pub(crate) fn oneshot(&self) -> Self {
        let mut new = self.clone();
        new.params.insert("stream", false.to_string());
        new
    }

    pub(crate) fn stream(&self) -> Self {
        let mut new = self.clone();
        new.params.insert("stream", true.to_string());
        new
    }
}

impl ContainerStatsOptsBuilder {
    impl_url_vec_field!(
        /// Names or IDs of containers
        containers => "containers"
    );

    impl_url_field!(
        /// Time in seconds between stats reports
        interval: usize => "interval"
    );
}

impl_opts_builder!(url =>
    /// Adjust how container stats are reported.
    ContainerTop
);

impl ContainerTopOpts {
    pub(crate) fn oneshot(&self) -> Self {
        let mut new = self.clone();
        new.params.insert("stream", false.to_string());
        new
    }

    pub(crate) fn stream(&self) -> Self {
        let mut new = self.clone();
        new.params.insert("stream", true.to_string());
        new
    }
}

impl ContainerTopOptsBuilder {
    impl_url_field!(
        /// if streaming, delay in seconds between updates. Must be >1. (As of version 4.0)
        delay: usize => "delay"
    );

    impl_url_str_field!(
        /// Arguments to pass to ps such as aux. Requires ps(1) to be installed in the container
        /// if no ps(1) compatible AIX descriptors are used.
        ps_args => "ps_args"
    );
}

#[derive(Debug)]
/// Used to filter removed images.
pub enum ContainerPruneFilter {
    /// Prune containers created before this timestamp. The <timestamp> can be Unix timestamps, date
    /// formatted timestamps, or Go duration strings (e.g. 10m, 1h30m) computed relative to the
    /// daemon machine’s time.
    // #TODO: DateTime
    Until(String),
    /// Container with key label.
    LabelKey(String),
    /// Container with key-value label.
    LabelKeyVal(String, String),
    /// Container without key label.
    NoLabelKey(String),
    /// Container without key-value label.
    NoLabelKeyVal(String, String),
}

impl Filter for ContainerPruneFilter {
    fn query_item(&self) -> FilterItem {
        use ContainerPruneFilter::*;
        match &self {
            Until(until) => FilterItem::new("until", until.to_string()),
            LabelKey(key) => FilterItem::new("label", key.clone()),
            LabelKeyVal(key, val) => FilterItem::new("label", format!("{key}={val}")),
            NoLabelKey(key) => FilterItem::new("label!", key.clone()),
            NoLabelKeyVal(key, val) => FilterItem::new("label!", format!("{key}={val}")),
        }
    }
}

impl_opts_builder!(url =>
    /// Adjust how stopped containers are removed.
    ContainerPrune
);

impl ContainerPruneOptsBuilder {
    impl_filter_func!(
        /// Filters to process on the prune list.
        ContainerPruneFilter
    );
}

impl_opts_builder!(url =>
    /// Adjust how a container is restored.
    ContainerRestore
);

impl ContainerRestoreOptsBuilder {
    impl_url_bool_field!(
        /// Do not include root file-system changes when exporting.
        ignore_root_fs => "ignoreRootFS"
    );

    impl_url_bool_field!(
        /// Ignore IP address if set statically.
        ignore_static_ip => "ignoreStaticIP"
    );

    impl_url_bool_field!(
        /// Ignore MAC address if set statically.
        ignore_static_mac => "ignoreStaticMac"
    );

    impl_url_bool_field!(
        /// Import the restore from a checkpoint tar.gz.
        import => "import"
    );

    impl_url_bool_field!(
        /// Keep all temporary checkpoint files.
        keep => "keep"
    );

    impl_url_bool_field!(
        /// Leave the container running after writing checkpoint to disk.
        leave_running => "leaveRunning"
    );

    impl_url_str_field!(
        /// The name of the container when restored from a tar. can only be used with import.
        name => "name"
    );

    impl_url_str_field!(
        /// Pod to restore into.
        pod => "pod"
    );

    impl_url_bool_field!(
        /// Add restore statistics to the response.
        print_stats => "printStats"
    );

    impl_url_bool_field!(
        /// Checkpoint a container with established TCP connections.
        tcp_established => "tcpEstablished"
    );
}
