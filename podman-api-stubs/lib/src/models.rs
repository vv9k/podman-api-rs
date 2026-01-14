#![allow(
    non_snake_case,
    clippy::redundant_field_names,
    clippy::new_without_default,
    clippy::too_many_arguments
)]

use serde::{Deserialize, Serialize};
use serde_json::Value;

use std::collections::HashMap;

use chrono::{DateTime, Utc};

fn deserialize_nonoptional_vec<
    'de,
    D: serde::de::Deserializer<'de>,
    T: serde::de::DeserializeOwned,
>(
    d: D,
) -> Result<Vec<T>, D::Error> {
    serde::de::Deserialize::deserialize(d).map(|x: Option<_>| x.unwrap_or_default())
}

fn deserialize_nonoptional_map<
    'de,
    D: serde::de::Deserializer<'de>,
    T: serde::de::DeserializeOwned,
>(
    d: D,
) -> Result<HashMap<String, T>, D::Error> {
    serde::de::Deserialize::deserialize(d).map(|x: Option<_>| x.unwrap_or_default())
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AccessMode {
    #[serde(rename = "BlockVolume")]
    pub block_volume: Option<TypeBlock>,
    #[serde(rename = "MountVolume")]
    pub mount_volume: Option<TypeMount>,
    #[serde(rename = "Scope")]
    pub scope: Option<String>,
    #[serde(rename = "Sharing")]
    pub sharing: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Address {
    #[serde(rename = "Addr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addr: Option<String>,
    #[serde(rename = "PrefixLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix_length: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AttestationProperties {
    #[serde(rename = "For")]
    pub for_: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// AuthConfig contains authorization information for connecting to a Registry
pub struct AuthConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Email is an optional value associated with the username.
    /// This field is deprecated and will be removed in a later
    /// version of docker.
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// IdentityToken is used to authenticate the user and get
    /// an access token for the registry.
    pub identitytoken: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// RegistryToken is a bearer token to be sent to a registry
    pub registrytoken: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serveraddress: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// AuthReport describes the response for authentication check
pub struct AuthReport {
    #[serde(rename = "IdentityToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_token: Option<String>,
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// AuthenticateOKBody authenticate o k body
pub struct AuthenticateOkBody {
    #[serde(rename = "IdentityToken")]
    /// An opaque token used to authenticate a user after a successful login
    pub identity_token: String,
    #[serde(rename = "Status")]
    /// The status of the authentication
    pub status: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AutoUserNsOptions {
    #[serde(rename = "AdditionalGIDMappings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// AdditionalGIDMappings specified additional GID mappings to include in
    /// the generated user namespace.
    pub additional_gid_mappings: Option<Vec<IdMap>>,
    #[serde(rename = "AdditionalUIDMappings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// AdditionalUIDMappings specified additional UID mappings to include in
    /// the generated user namespace.
    pub additional_uid_mappings: Option<Vec<IdMap>>,
    #[serde(rename = "GroupFile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// GroupFile to use if the container uses a volume.
    pub group_file: Option<String>,
    #[serde(rename = "InitialSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// InitialSize defines the minimum size for the user namespace.
    /// The created user namespace will have at least this size.
    pub initial_size: Option<u32>,
    #[serde(rename = "PasswdFile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// PasswdFile to use if the container uses a volume.
    pub passwd_file: Option<String>,
    #[serde(rename = "Size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Size defines the size for the user namespace.  If it is set to a
    /// value bigger than 0, the user namespace will have exactly this size.
    /// If it is not set, some heuristics will be used to find its size.
    pub size: Option<u32>,
}

pub type Availability = String;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BindOptions {
    #[serde(rename = "CreateMountpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_mountpoint: Option<bool>,
    #[serde(rename = "NonRecursive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_recursive: Option<bool>,
    #[serde(rename = "Propagation")]
    pub propagation: Option<String>,
    #[serde(rename = "ReadOnlyForceRecursive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// ReadOnlyForceRecursive raises an error if the mount cannot be made recursively read-only.
    pub read_only_force_recursive: Option<bool>,
    #[serde(rename = "ReadOnlyNonRecursive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// ReadOnlyNonRecursive makes the mount non-recursively read-only, but still leaves the mount recursive
    /// (unless NonRecursive is set to true in conjunction).
    pub read_only_non_recursive: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CpuUsage {
    #[serde(rename = "idlePercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle_percent: Option<f64>,
    #[serde(rename = "systemPercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_percent: Option<f64>,
    #[serde(rename = "userPercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_percent: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// CapacityRange describes the minimum and maximum capacity a volume should be
/// created with
pub struct CapacityRange {
    #[serde(rename = "LimitBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// LimitBytes specifies that a volume must not be bigger than this. The
    /// value of 0 indicates an unspecified maximum
    pub limit_bytes: Option<i64>,
    #[serde(rename = "RequiredBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// RequiredBytes specifies that a volume must be at least this big. The
    /// value of 0 indicates an unspecified minimum.
    pub required_bytes: Option<i64>,
}

pub type CgroupSpec = String;

/// CgroupnsMode represents the cgroup namespace mode of the container
pub type CgroupnsMode = String;

/// Can be one of:
///
/// `0`: Modified ("C")
/// `1`: Added ("A")
/// `2`: Deleted ("D")
pub type ChangeType = u8;

pub type ClusterOptions = Value;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// ClusterVolume contains options and information specific to, and only present
/// on, Swarm CSI cluster volumes.
pub struct ClusterVolume {
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// ID is the Swarm ID of the volume. Because cluster volumes are Swarm
    /// objects, they have an ID, unlike non-cluster volumes, which only have a
    /// Name. This ID can be used to refer to the cluster volume.
    pub id: Option<String>,
    #[serde(rename = "Info")]
    pub info: Option<Info>,
    #[serde(rename = "PublishStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// PublishStatus contains the status of the volume as it pertains to its
    /// publishing on Nodes.
    pub publish_status: Option<Vec<PublishStatus>>,
    #[serde(rename = "Spec")]
    pub spec: Option<ClusterVolumeSpec>,
    #[serde(rename = "UpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
    #[serde(rename = "Version")]
    pub version: Option<Version>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClusterVolumeSpec {
    #[serde(rename = "AccessMode")]
    pub access_mode: Option<AccessMode>,
    #[serde(rename = "AccessibilityRequirements")]
    pub accessibility_requirements: Option<TopologyRequirement>,
    #[serde(rename = "Availability")]
    pub availability: Option<String>,
    #[serde(rename = "CapacityRange")]
    pub capacity_range: Option<CapacityRange>,
    #[serde(rename = "Group")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Group defines the volume group of this volume. Volumes belonging to the
    /// same group can be referred to by group name when creating Services.
    /// Referring to a volume by group instructs swarm to treat volumes in that
    /// group interchangeably for the purpose of scheduling. Volumes with an
    /// empty string for a group technically all belong to the same, emptystring
    /// group.
    pub group: Option<String>,
    #[serde(rename = "Secrets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Secrets defines Swarm Secrets that are passed to the CSI storage plugin
    /// when operating on this volume.
    pub secrets: Option<Vec<Secret>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComponentVersion {
    #[serde(rename = "Details")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<HashMap<String, String>>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// It should hold only portable information about the container.
/// Here, "portable" means "independent from the host we are running on".
/// Non-portable information *should* appear in HostConfig.
/// All fields added to this struct must be marked `omitempty` to keep getting
/// predictable hashes from the old `v1Compatibility` configuration.
pub struct Config {
    #[serde(rename = "ArgsEscaped")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args_escaped: Option<bool>,
    #[serde(rename = "AttachStderr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach_stderr: Option<bool>,
    #[serde(rename = "AttachStdin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach_stdin: Option<bool>,
    #[serde(rename = "AttachStdout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach_stdout: Option<bool>,
    #[serde(rename = "Cmd")]
    pub cmd: Option<StrSlice>,
    #[serde(rename = "Domainname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domainname: Option<String>,
    #[serde(rename = "Entrypoint")]
    pub entrypoint: Option<StrSlice>,
    #[serde(rename = "Env")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<Vec<String>>,
    #[serde(rename = "ExposedPorts")]
    pub exposed_ports: Option<PortSet>,
    #[serde(rename = "Healthcheck")]
    pub healthcheck: Option<HealthcheckConfig>,
    #[serde(rename = "Hostname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    #[serde(rename = "Image")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(rename = "Labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<HashMap<String, String>>,
    #[serde(rename = "MacAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Mac Address of the container.
    ///
    /// Deprecated: this field is deprecated since API v1.44. Use EndpointSettings.MacAddress instead.
    pub mac_address: Option<String>,
    #[serde(rename = "NetworkDisabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_disabled: Option<bool>,
    #[serde(rename = "OnBuild")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_build: Option<Vec<String>>,
    #[serde(rename = "OpenStdin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_stdin: Option<bool>,
    #[serde(rename = "Shell")]
    pub shell: Option<StrSlice>,
    #[serde(rename = "StdinOnce")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stdin_once: Option<bool>,
    #[serde(rename = "StopSignal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_signal: Option<String>,
    #[serde(rename = "StopTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_timeout: Option<i64>,
    #[serde(rename = "Tty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tty: Option<bool>,
    #[serde(rename = "User")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    #[serde(rename = "Volumes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes: Option<HashMap<String, Value>>,
    #[serde(rename = "WorkingDir")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_dir: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// ConfigReference specifies the source which provides a network's configuration
pub struct ConfigReference {
    #[serde(rename = "Network")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// ConmonInfo describes the conmon executable being used
pub struct ConmonInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// ConnectOptions represents the data to be used to connect a container to the
/// network.
pub struct ConnectOptions {
    #[serde(rename = "Container")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container: Option<String>,
    #[serde(rename = "EndpointConfig")]
    pub endpoint_config: Option<EndpointSettings>,
}

pub type Consistency = String;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Container {
    #[serde(rename = "Command")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,
    #[serde(rename = "Config")]
    pub config: Option<Config>,
    #[serde(rename = "Created")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<i64>,
    #[serde(rename = "DefaultReadOnlyNonRecursive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_read_only_non_recursive: Option<bool>,
    #[serde(rename = "HostConfig")]
    pub host_config: Option<HostConfig>,
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Image")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(rename = "ImageID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
    #[serde(rename = "Labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<HashMap<String, String>>,
    #[serde(rename = "Mounts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mounts: Option<Vec<MountPoint>>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Names")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<Vec<String>>,
    #[serde(rename = "NetworkSettings")]
    pub network_settings: Option<SummaryNetworkSettings>,
    #[serde(rename = "NetworkingConfig")]
    pub networking_config: Option<NetworkingConfig>,
    #[serde(rename = "Platform")]
    pub platform: Option<Platform>,
    #[serde(rename = "Ports")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ports: Option<Vec<Port>>,
    #[serde(rename = "SizeRootFs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_root_fs: Option<i64>,
    #[serde(rename = "SizeRw")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_rw: Option<i64>,
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// no error
pub type ContainerArchive200Response = Vec<u8>;

/// no error
pub type ContainerArchiveLibpod200Response = Vec<u8>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContainerBasicConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Annotations are key-value options passed into the container runtime
    /// that can be used to trigger special behavior.
    /// Optional.
    pub annotations: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Command is the container's command.
    /// If not given and Image is specified, this will be populated by the
    /// image's configuration.
    /// Optional.
    pub command: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// ConmonPidFile is a path at which a PID file for Conmon will be
    /// placed.
    /// If not given, a default location will be used.
    /// Optional.
    pub conmon_pid_file: Option<String>,
    #[serde(rename = "containerCreateCommand")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// ContainerCreateCommand is the command that was used to create this
    /// container.
    /// This will be shown in the output of Inspect() on the container, and
    /// may also be used by some tools that wish to recreate the container
    /// (e.g. `podman generate systemd --new`).
    /// Optional.
    pub container_create_command: Option<Vec<String>>,
    #[serde(rename = "dependencyContainers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// DependencyContainers is an array of containers this container
    /// depends on. Dependency containers must be started before this
    /// container. Dependencies can be specified by name or full/partial ID.
    /// Optional.
    pub dependency_containers: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Entrypoint is the container's entrypoint.
    /// If not given and Image is specified, this will be populated by the
    /// image's configuration.
    /// Optional.
    pub entrypoint: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Env is a set of environment variables that will be set in the
    /// container.
    /// Optional.
    pub env: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// EnvHost indicates that the host environment should be added to container
    /// Optional.
    pub env_host: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// EnvMerge takes the specified environment variables from image and preprocess them before injecting them into the
    /// container.
    /// Optional.
    pub envmerge: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// GroupEntry specifies an arbitrary string to append to the container's /etc/group file.
    /// Optional.
    pub group_entry: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Hostname is the container's hostname. If not set, the hostname will
    /// not be modified (if UtsNS is not private) or will be set to the
    /// container ID (if UtsNS is private).
    /// Conflicts with UtsNS if UtsNS is not set to private.
    /// Optional.
    pub hostname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// HostUsers is a list of host usernames or UIDs to add to the container
    /// etc/passwd file
    pub hostusers: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// EnvHTTPProxy indicates that the http host proxy environment variables
    /// should be added to container
    /// Optional.
    pub httpproxy: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// InitContainerType describes if this container is an init container
    /// and if so, what type: always or once.
    /// Optional.
    pub init_container_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Labels are key-value pairs that are used to add metadata to
    /// containers.
    /// Optional.
    pub labels: Option<HashMap<String, String>>,
    pub log_configuration: Option<LogConfigLibpod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Passwd is a container run option that determines if we are validating users/groups before running the container
    pub manage_password: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Name is the name the container will be given.
    /// If no name is provided, one will be randomly generated.
    /// Optional.
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// OCIRuntime is the name of the OCI runtime that will be used to create
    /// the container.
    /// If not specified, the default will be used.
    /// Optional.
    pub oci_runtime: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// PasswdEntry specifies an arbitrary string to append to the container's /etc/passwd file.
    /// Optional.
    pub passwd_entry: Option<String>,
    pub personality: Option<LinuxPersonality>,
    pub pidns: Option<Namespace>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Pod is the ID of the pod the container will join.
    /// Optional.
    pub pod: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Remove indicates if the container should be removed once it has been started
    /// and exits.
    /// Optional.
    pub remove: Option<bool>,
    #[serde(rename = "removeImage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// RemoveImage indicates that the container should remove the image it
    /// was created from after it exits.
    /// Only allowed if Remove is set to true and Image, not Rootfs, is in
    /// use.
    /// Optional.
    pub remove_image: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// RestartPolicy is the container's restart policy - an action which
    /// will be taken when the container exits.
    /// If not given, the default policy, which does nothing, will be used.
    /// Optional.
    pub restart_policy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// RestartRetries is the number of attempts that will be made to restart
    /// the container.
    /// Only available when RestartPolicy is set to "on-failure".
    /// Optional.
    pub restart_tries: Option<u64>,
    #[serde(rename = "sdnotifyMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Determine how to handle the NOTIFY_SOCKET - do we participate or pass it through
    /// "container" - let the OCI runtime deal with it, advertise conmon's MAINPID
    /// "conmon-only" - advertise conmon's MAINPID, send READY when started, don't pass to OCI
    /// "ignore" - unset NOTIFY_SOCKET
    /// Optional.
    pub sdnotify_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// EnvSecrets are secrets that will be set as environment variables
    /// Optional.
    pub secret_env: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Stdin is whether the container will keep its STDIN open.
    /// Optional.
    pub stdin: Option<bool>,
    pub stop_signal: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// StopTimeout is a timeout between the container's stop signal being
    /// sent and SIGKILL being sent.
    /// If not provided, the default will be used.
    /// If 0 is used, stop signal will not be sent, and SIGKILL will be sent
    /// instead.
    /// Optional.
    pub stop_timeout: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Sysctl sets kernel parameters for the container
    pub sysctl: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Systemd is whether the container will be started in systemd mode.
    /// Valid options are "true", "false", and "always".
    /// "true" enables this mode only if the binary run in the container is
    /// sbin/init or systemd. "always" unconditionally enables systemd mode.
    /// "false" unconditionally disables systemd mode.
    /// If enabled, mounts and stop signal will be modified.
    /// If set to "always" or set to "true" and conditionally triggered,
    /// conflicts with StopSignal.
    /// If not specified, "false" will be assumed.
    /// Optional.
    pub systemd: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Terminal is whether the container will create a PTY.
    /// Optional.
    pub terminal: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Timeout is a maximum time in seconds the container will run before
    /// main process is sent SIGKILL.
    /// If 0 is used, signal will not be sent. Container can run indefinitely
    /// if they do not stop after the default termination signal.
    /// Optional.
    pub timeout: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Timezone is the timezone inside the container.
    /// Local means it has the same timezone as the host machine
    /// Optional.
    pub timezone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// UnsetEnv unsets the specified default environment variables from the image or from built-in or containers.conf
    /// Optional.
    pub unsetenv: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// UnsetEnvAll unsetall default environment variables from the image or from built-in or containers.conf
    /// UnsetEnvAll unsets all default environment variables from the image or from built-in
    /// Optional.
    pub unsetenvall: Option<bool>,
    pub utsns: Option<Namespace>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// ContainerCgroupConfig contains configuration information about a container's
/// cgroups.
pub struct ContainerCgroupConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// CgroupParent is the container's Cgroup parent.
    /// If not set, the default for the current cgroup driver will be used.
    /// Optional.
    pub cgroup_parent: Option<String>,
    pub cgroupns: Option<Namespace>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// CgroupsMode sets a policy for how cgroups will be created for the
    /// container, including the ability to disable creation entirely.
    /// Optional.
    pub cgroups_mode: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// ContainerCreateResponse is the response struct for creating a container
pub struct ContainerCreateResponse {
    #[serde(rename = "Id")]
    /// ID of the container created
    pub id: String,
    #[serde(rename = "Warnings")]
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_nonoptional_vec")]
    /// Warnings during container creation
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContainerExecControlParam {
    #[serde(rename = "AttachStderr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Attach to stderr of the exec command
    pub attach_stderr: Option<bool>,
    #[serde(rename = "AttachStdin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Attach to stdin of the exec command
    pub attach_stdin: Option<bool>,
    #[serde(rename = "AttachStdout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Attach to stdout of the exec command
    pub attach_stdout: Option<bool>,
    #[serde(rename = "Cmd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Command to run, as a string or array of strings.
    pub cmd: Option<Vec<String>>,
    #[serde(rename = "DetachKeys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// "Override the key sequence for detaching a container. Format is a single character [a-Z] or ctrl-<value> where <value> is one of: a-z, @, ^, [, , or _."
    pub detach_keys: Option<String>,
    #[serde(rename = "Env")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A list of environment variables in the form ["VAR=value", ...]
    pub env: Option<Vec<String>>,
    #[serde(rename = "Privileged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Runs the exec process with extended privileges
    pub privileged: Option<bool>,
    #[serde(rename = "Tty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Allocate a pseudo-TTY
    pub tty: Option<bool>,
    #[serde(rename = "User")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// "The user, and optionally, group to run the exec process inside the container. Format is one of: user, user:group, uid, or uid:gid."
    pub user: Option<String>,
    #[serde(rename = "WorkingDir")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The working directory for the exec process inside the container.
    pub working_dir: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContainerExecLibpodControlParam {
    #[serde(rename = "AttachStderr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Attach to stderr of the exec command
    pub attach_stderr: Option<bool>,
    #[serde(rename = "AttachStdin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Attach to stdin of the exec command
    pub attach_stdin: Option<bool>,
    #[serde(rename = "AttachStdout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Attach to stdout of the exec command
    pub attach_stdout: Option<bool>,
    #[serde(rename = "Cmd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Command to run, as a string or array of strings.
    pub cmd: Option<Vec<String>>,
    #[serde(rename = "DetachKeys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// "Override the key sequence for detaching a container. Format is a single character [a-Z] or ctrl-<value> where <value> is one of: a-z, @, ^, [, , or _."
    pub detach_keys: Option<String>,
    #[serde(rename = "Env")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A list of environment variables in the form ["VAR=value", ...]
    pub env: Option<Vec<String>>,
    #[serde(rename = "Privileged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Runs the exec process with extended privileges
    pub privileged: Option<bool>,
    #[serde(rename = "Tty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Allocate a pseudo-TTY
    pub tty: Option<bool>,
    #[serde(rename = "User")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// "The user, and optionally, group to run the exec process inside the container. Format is one of: user, user:group, uid, or uid:gid."
    pub user: Option<String>,
    #[serde(rename = "WorkingDir")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The working directory for the exec process inside the container.
    pub working_dir: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// ContainerHealthCheckConfig describes a container healthcheck with attributes
/// like command, retries, interval, start period, and timeout.
pub struct ContainerHealthCheckConfig {
    #[serde(rename = "healthLogDestination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// HealthLogDestination defines the destination where the log is stored.
    /// TODO (6.0): In next major release convert it to pointer and use omitempty
    pub health_log_destination: Option<String>,
    #[serde(rename = "healthMaxLogCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// HealthMaxLogCount is maximum number of attempts in the HealthCheck log file.
    /// ('0' value means an infinite number of attempts in the log file).
    /// TODO (6.0): In next major release convert it to pointer and use omitempty
    pub health_max_log_count: Option<u64>,
    #[serde(rename = "healthMaxLogSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// HealthMaxLogSize is the maximum length in characters of stored HealthCheck log
    /// ("0" value means an infinite log length).
    /// TODO (6.0): In next major release convert it to pointer and use omitempty
    pub health_max_log_size: Option<u64>,
    pub health_check_on_failure_action: Option<i64>,
    pub healthconfig: Option<Schema2HealthConfig>,
    #[serde(rename = "startupHealthConfig")]
    pub startup_health_config: Option<StartupHealthCheck>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// ContainerJSON is newly used struct along with MountPoint
pub struct ContainerJson {
    #[serde(rename = "AppArmorProfile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_armor_profile: Option<String>,
    #[serde(rename = "Args")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<String>>,
    #[serde(rename = "Config")]
    pub config: Option<Config>,
    #[serde(rename = "Created")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(rename = "Driver")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver: Option<String>,
    #[serde(rename = "ExecIDs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exec_i_ds: Option<Vec<String>>,
    #[serde(rename = "GraphDriver")]
    pub graph_driver: Option<GraphDriverData>,
    #[serde(rename = "HostConfig")]
    pub host_config: Option<HostConfig>,
    #[serde(rename = "HostnamePath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname_path: Option<String>,
    #[serde(rename = "HostsPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosts_path: Option<String>,
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Image")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(rename = "LogPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_path: Option<String>,
    #[serde(rename = "MountLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_label: Option<String>,
    #[serde(rename = "Mounts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mounts: Option<Vec<MountPoint>>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "NetworkSettings")]
    pub network_settings: Option<NetworkSettings>,
    #[serde(rename = "Node")]
    pub node: Option<ContainerNode>,
    #[serde(rename = "Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "Platform")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    #[serde(rename = "ProcessLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_label: Option<String>,
    #[serde(rename = "ResolvConfPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolv_conf_path: Option<String>,
    #[serde(rename = "RestartCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restart_count: Option<i64>,
    #[serde(rename = "SizeRootFs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_root_fs: Option<i64>,
    #[serde(rename = "SizeRw")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_rw: Option<i64>,
    #[serde(rename = "State")]
    pub state: Option<ContainerState>,
}

/// mounted container
pub type ContainerMountLibpod200Response = String;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// ContainerNetworkConfig contains information on a container's network
/// configuration.
pub struct ContainerNetworkConfig {
    #[serde(rename = "Networks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Map of networks names or ids that the container should join.
    /// You can request additional settings for each network, you can
    /// set network aliases, static ips, static mac address  and the
    /// network interface name for this container on the specific network.
    /// If the map is empty and the bridge network mode is set the container
    /// will be joined to the default network.
    /// Optional.
    pub networks: Option<HashMap<String, PerNetworkOptions>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// BaseHostsFile is the base file to create the `/etc/hosts` file inside the container.
    /// This must either be an absolute path to a file on the host system, or one of the
    /// special flags `image` or `none`.
    /// If it is empty it defaults to the base_hosts_file configuration in containers.conf.
    /// Optional.
    pub base_hosts_file: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// CNINetworks is a list of CNI networks to join the container to.
    /// If this list is empty, the default CNI network will be joined
    /// instead. If at least one entry is present, we will not join the
    /// default network (unless it is part of this list).
    /// Only available if NetNS is set to bridge.
    /// Optional.
    /// Deprecated: as of podman 4.0 use "Networks" instead.
    pub cni_networks: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// DNSOptions is a set of DNS options that will be used in the
    /// container's resolv.conf, replacing the host's DNS options which are
    /// used by default.
    /// Conflicts with UseImageResolvConf.
    /// Optional.
    pub dns_option: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// DNSSearch is a set of DNS search domains that will be used in the
    /// container's resolv.conf, replacing the host's DNS search domains
    /// which are used by default.
    /// Conflicts with UseImageResolvConf.
    /// Optional.
    pub dns_search: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// DNSServers is a set of DNS servers that will be used in the
    /// container's resolv.conf, replacing the host's DNS Servers which are
    /// used by default.
    /// Conflicts with UseImageResolvConf.
    /// Optional.
    pub dns_server: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Expose is a number of ports that will be forwarded to the container
    /// if PublishExposedPorts is set.
    /// Expose is a map of uint16 (port number) to a string representing
    /// protocol i.e map[uint16]string. Allowed protocols are "tcp", "udp", and "sctp", or some
    /// combination of the three separated by commas.
    /// If protocol is set to "" we will assume TCP.
    /// Only available if NetNS is set to Bridge or Slirp, and
    /// PublishExposedPorts is set.
    /// Optional.
    pub expose: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// HostAdd is a set of hosts which will be added to the container's
    /// etc/hosts file.
    /// Conflicts with UseImageHosts.
    /// Optional.
    pub hostadd: Option<Vec<String>>,
    pub netns: Option<Namespace>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// NetworkOptions are additional options for each network
    /// Optional.
    pub network_options: Option<HashMap<String, Vec<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// PortBindings is a set of ports to map into the container.
    /// Only available if NetNS is set to bridge, slirp, or pasta.
    /// Optional.
    pub portmappings: Option<Vec<PortMapping>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// PublishExposedPorts will publish ports specified in the image to
    /// random unused ports (guaranteed to be above 1024) on the host.
    /// This is based on ports set in Expose below, and any ports specified
    /// by the Image (if one is given).
    /// Only available if NetNS is set to Bridge or Slirp.
    /// Optional.
    pub publish_image_ports: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// UseImageHostname indicates that /etc/hostname should not be managed by
    /// Podman, and instead sourced from the image.
    /// Optional.
    pub use_image_hostname: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// UseImageHosts indicates that /etc/hosts should not be managed by
    /// Podman, and instead sourced from the image.
    /// Conflicts with HostAdd.
    /// Optional.
    pub use_image_hosts: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// UseImageResolvConf indicates that resolv.conf should not be managed
    /// by Podman, but instead sourced from the image.
    /// Conflicts with DNSServer, DNSSearch, DNSOption.
    /// Optional.
    pub use_image_resolve_conf: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Statistics for an individual container network interface
pub struct ContainerNetworkStats {
    #[serde(rename = "RxBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rx_bytes: Option<u64>,
    #[serde(rename = "RxDropped")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rx_dropped: Option<u64>,
    #[serde(rename = "RxErrors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rx_errors: Option<u64>,
    #[serde(rename = "RxPackets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rx_packets: Option<u64>,
    #[serde(rename = "TxBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_bytes: Option<u64>,
    #[serde(rename = "TxDropped")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_dropped: Option<u64>,
    #[serde(rename = "TxErrors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_errors: Option<u64>,
    #[serde(rename = "TxPackets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_packets: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Deprecated: ContainerNode was used for the classic Docker Swarm standalone API. It will be removed in the next release.
pub struct ContainerNode {
    #[serde(rename = "Addr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addr: Option<String>,
    #[serde(rename = "Cpus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpus: Option<i64>,
    #[serde(rename = "ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "IP")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    #[serde(rename = "Labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<HashMap<String, String>>,
    #[serde(rename = "Memory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<i64>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContainerResourceConfig {
    #[serde(rename = "intelRdt")]
    pub intel_rdt: Option<LinuxIntelRdt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// OOMScoreAdj adjusts the score used by the OOM killer to determine
    /// processes to kill for the container's process.
    /// Optional.
    pub oom_score_adj: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Rlimits are POSIX rlimits to apply to the container.
    /// Optional.
    pub r_limits: Option<Vec<PosixRlimit>>,
    pub resource_limits: Option<LinuxResources>,
    #[serde(rename = "throttleReadBpsDevice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// IO read rate limit per cgroup per device, bytes per second
    pub throttle_read_bps_device: Option<HashMap<String, LinuxThrottleDevice>>,
    #[serde(rename = "throttleReadIOPSDevice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// IO read rate limit per cgroup per device, IO per second
    pub throttle_read_iops_device: Option<HashMap<String, LinuxThrottleDevice>>,
    #[serde(rename = "throttleWriteBpsDevice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// IO write rate limit per cgroup per device, bytes per second
    pub throttle_write_bps_device: Option<HashMap<String, LinuxThrottleDevice>>,
    #[serde(rename = "throttleWriteIOPSDevice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// IO write rate limit per cgroup per device, IO per second
    pub throttle_write_iops_device: Option<HashMap<String, LinuxThrottleDevice>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// CgroupConf are key-value options passed into the container runtime
    /// that are used to configure cgroup v2.
    /// Optional.
    pub unified: Option<HashMap<String, String>>,
    #[serde(rename = "weightDevice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Weight per cgroup per device, can override BlkioWeight
    pub weight_device: Option<HashMap<String, LinuxWeightDevice>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// ContainerSecurityConfig is a container's security features, including
/// SELinux, Apparmor, and Seccomp.
pub struct ContainerSecurityConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// ApparmorProfile is the name of the Apparmor profile the container
    /// will use.
    /// Optional.
    pub apparmor_profile: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// CapAdd are capabilities which will be added to the container.
    /// Conflicts with Privileged.
    /// Optional.
    pub cap_add: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// CapDrop are capabilities which will be removed from the container.
    /// Conflicts with Privileged.
    /// Optional.
    pub cap_drop: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Groups are a list of supplemental groups the container's user will
    /// be granted access to.
    /// Optional.
    pub groups: Option<Vec<String>>,
    pub idmappings: Option<IdMappingOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// LabelNested indicates whether or not the container is allowed to
    /// run fully nested containers including SELinux labelling.
    /// Optional.
    pub label_nested: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Mask is the path we want to mask in the container. This masks the paths
    /// given in addition to the default list.
    /// Optional
    pub mask: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// NoNewPrivileges is whether the container will set the no new
    /// privileges flag on create, which disables gaining additional
    /// privileges (e.g. via setuid) in the container.
    /// Optional.
    pub no_new_privileges: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Privileged is whether the container is privileged.
    /// Privileged does the following:
    /// Adds all devices on the system to the container.
    /// Adds all capabilities to the container.
    /// Disables Seccomp, SELinux, and Apparmor confinement.
    /// (Though SELinux can be manually re-enabled).
    /// TODO: this conflicts with things.
    /// TODO: this does more.
    /// Optional.
    pub privileged: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// ProcOpts are the options used for the proc mount.
    pub procfs_opts: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// ReadOnlyFilesystem indicates that everything will be mounted
    /// as read-only.
    /// Optional.
    pub read_only_filesystem: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// ReadWriteTmpfs indicates that when running with a ReadOnlyFilesystem
    /// mount temporary file systems.
    /// Optional.
    pub read_write_tmpfs: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// SeccompPolicy determines which seccomp profile gets applied
    /// the container. valid values: empty,default,image
    pub seccomp_policy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// SeccompProfilePath is the path to a JSON file containing the
    /// container's Seccomp profile.
    /// If not specified, no Seccomp profile will be used.
    /// Optional.
    pub seccomp_profile_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// SelinuxProcessLabel is the process label the container will use.
    /// If SELinux is enabled and this is not specified, a label will be
    /// automatically generated if not specified.
    /// Optional.
    pub selinux_opts: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Umask is the umask the init process of the container will be run with.
    pub umask: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Unmask a path in the container. Some paths are masked by default,
    /// preventing them from being accessed within the container; this undoes
    /// that masking. If ALL is passed, all paths will be unmasked.
    /// Optional.
    pub unmask: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// User is the user the container will be run as.
    /// Can be given as a UID or a username; if a username, it will be
    /// resolved within the container, using the container's /etc/passwd.
    /// If unset, the container will be run as root.
    /// Optional.
    pub user: Option<String>,
    pub userns: Option<Namespace>,
}

/// mounted containers
pub type ContainerShowMountedLibpod200Response = HashMap<String, String>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// ContainerSize holds the size of the container's root filesystem and top
/// read-write layer.
pub struct ContainerSize {
    #[serde(rename = "rootFsSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_fs_size: Option<i64>,
    #[serde(rename = "rwSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rw_size: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// ContainerState stores container's running state
/// it's part of ContainerJSONBase and will return by "inspect" command
pub struct ContainerState {
    #[serde(rename = "Dead")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dead: Option<bool>,
    #[serde(rename = "Error")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(rename = "ExitCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exit_code: Option<i64>,
    #[serde(rename = "FinishedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finished_at: Option<String>,
    #[serde(rename = "Health")]
    pub health: Option<Health>,
    #[serde(rename = "OOMKilled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oom_killed: Option<bool>,
    #[serde(rename = "Paused")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paused: Option<bool>,
    #[serde(rename = "Pid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pid: Option<i64>,
    #[serde(rename = "Restarting")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restarting: Option<bool>,
    #[serde(rename = "Running")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub running: Option<bool>,
    #[serde(rename = "StartedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<String>,
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// ContainerStats contains the statistics information for a running container
pub struct ContainerStats {
    #[serde(rename = "AvgCPU")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avg_cpu: Option<f64>,
    #[serde(rename = "BlockInput")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_input: Option<u64>,
    #[serde(rename = "BlockOutput")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_output: Option<u64>,
    #[serde(rename = "CPU")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu: Option<f64>,
    #[serde(rename = "CPUNano")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_nano: Option<u64>,
    #[serde(rename = "CPUSystemNano")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_system_nano: Option<u64>,
    #[serde(rename = "ContainerID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_id: Option<String>,
    #[serde(rename = "Duration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<u64>,
    #[serde(rename = "MemLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mem_limit: Option<u64>,
    #[serde(rename = "MemPerc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mem_perc: Option<f64>,
    #[serde(rename = "MemUsage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mem_usage: Option<u64>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Network")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Map of interface name to network statistics for that interface.
    pub network: Option<HashMap<String, ContainerNetworkStats>>,
    #[serde(rename = "PIDs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pi_ds: Option<u64>,
    #[serde(rename = "PerCPU")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_cpu: Option<Vec<u64>>,
    #[serde(rename = "SystemNano")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_nano: Option<u64>,
    #[serde(rename = "UpTime")]
    pub up_time: Option<i64>,
}

/// no error
pub type ContainerStats200Response = Value;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// ContainerStorageConfig contains information on the storage configuration of a
/// container.
pub struct ContainerStorageConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// ChrootDirs is an additional set of directories that need to be
    /// treated as root directories. Standard bind mounts will be mounted
    /// into paths relative to these directories.
    /// Optional.
    pub chroot_directories: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Create the working directory if it doesn't exist.
    /// If unset, it doesn't create it.
    /// Optional.
    pub create_working_dir: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// DeviceCgroupRule are device cgroup rules that allow containers
    /// to use additional types of devices.
    pub device_cgroup_rule: Option<Vec<LinuxDeviceCgroup>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Devices are devices that will be added to the container.
    /// Optional.
    pub devices: Option<Vec<LinuxDevice>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// DevicesFrom specifies that this container will mount the device(s) from other container(s).
    /// Optional.
    pub devices_from: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// HostDeviceList is used to recreate the mounted device on inherited containers
    pub host_device_list: Option<Vec<LinuxDevice>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Image is the image the container will be based on. The image will be
    /// used as the container's root filesystem, and its environment vars,
    /// volumes, and other configuration will be applied to the container.
    /// Conflicts with Rootfs.
    /// At least one of Image or Rootfs must be specified.
    pub image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// ImageArch is the user-specified image architecture.
    /// Used to select a different variant from a manifest list.
    /// Optional.
    pub image_arch: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// ImageOS is the user-specified OS of the image.
    /// Used to select a different variant from a manifest list.
    /// Optional.
    pub image_os: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// ImageVariant is the user-specified image variant.
    /// Used to select a different variant from a manifest list.
    /// Optional.
    pub image_variant: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// ImageVolumeMode indicates how image volumes will be created.
    /// Supported modes are "ignore" (do not create), "tmpfs" (create as
    /// tmpfs), and "anonymous" (create as anonymous volumes).
    /// The default if unset is anonymous.
    /// Optional.
    pub image_volume_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Image volumes bind-mount a container-image mount into the container.
    /// Optional.
    pub image_volumes: Option<Vec<ImageVolume>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Init specifies that an init binary will be mounted into the
    /// container, and will be used as PID1.
    /// Optional.
    pub init: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// InitPath specifies the path to the init binary that will be added if
    /// Init is specified above. If not specified, the default set in the
    /// Libpod config will be used. Ignored if Init above is not set.
    /// Optional.
    pub init_path: Option<String>,
    pub ipcns: Option<Namespace>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Mounts are mounts that will be added to the container.
    /// These will supersede Image Volumes and VolumesFrom volumes where
    /// there are conflicts.
    /// Optional.
    pub mounts: Option<Vec<Mount>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Overlay volumes are named volumes that will be added to the container.
    /// Optional.
    pub overlay_volumes: Option<Vec<OverlayVolume>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// RawImageName is the user-specified and unprocessed input referring
    /// to a local or a remote image.
    /// Optional, but strongly encouraged to be set if Image is set.
    pub raw_image_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Rootfs is the path to a directory that will be used as the
    /// container's root filesystem. No modification will be made to the
    /// directory, it will be directly mounted into the container as root.
    /// Conflicts with Image.
    /// At least one of Image or Rootfs must be specified.
    pub rootfs: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// RootfsMapping specifies if there are UID/GID mappings to apply to the rootfs.
    /// Optional.
    pub rootfs_mapping: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// RootfsOverlay tells if rootfs is actually an overlay on top of base path.
    /// Optional.
    pub rootfs_overlay: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// RootfsPropagation is the rootfs propagation mode for the container.
    /// If not set, the default of rslave will be used.
    /// Optional.
    pub rootfs_propagation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Secrets are the secrets that will be added to the container
    /// Optional.
    pub secrets: Option<Vec<Secret>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// ShmSize is the size of the tmpfs to mount in at /dev/shm, in bytes.
    /// Conflicts with ShmSize if IpcNS is not private.
    /// Optional.
    pub shm_size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// ShmSizeSystemd is the size of systemd-specific tmpfs mounts
    /// specifically /run, /run/lock, /var/log/journal and /tmp.
    /// Optional
    pub shm_size_systemd: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// StorageOpts is the container's storage options
    /// Optional.
    pub storage_opts: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Volatile specifies whether the container storage can be optimized
    /// at the cost of not syncing all the dirty files in memory.
    /// Optional.
    pub volatile: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Volumes are named volumes that will be added to the container.
    /// These will supersede Image Volumes and VolumesFrom volumes where
    /// there are conflicts.
    /// Optional.
    pub volumes: Option<Vec<NamedVolume>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// VolumesFrom is a set of containers whose volumes will be added to
    /// this container. The name or ID of the container must be provided, and
    /// may optionally be followed by a : and then one or more
    /// comma-separated options. Valid options are 'ro', 'rw', and 'z'.
    /// Options will be used for all volumes sourced from the container.
    /// Optional.
    pub volumes_from: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// WorkDir is the container's working directory.
    /// If unset, the default, /, will be used.
    /// Optional.
    pub work_dir: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// ContainerStore describes the quantity of containers in the
/// store by status
pub struct ContainerStore {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paused: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub running: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stopped: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// ContainerTopOKBody OK response to ContainerTop operation
pub struct ContainerTopOkBody {
    #[serde(rename = "Processes")]
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_nonoptional_vec")]
    /// Each process running in the container, where each is process
    /// is an array of values corresponding to the titles.
    pub processes: Vec<Vec<String>>,
    #[serde(rename = "Titles")]
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_nonoptional_vec")]
    /// The ps column titles
    pub titles: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// ContainerUpdateOKBody OK response to ContainerUpdate operation
pub struct ContainerUpdateOkBody {
    #[serde(rename = "Warnings")]
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_nonoptional_vec")]
    /// warnings
    pub warnings: Vec<String>,
}

/// Status code
pub type ContainerWaitLibpod200Response = i32;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContainersPruneReport {
    #[serde(rename = "ContainersDeleted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub containers_deleted: Option<Vec<String>>,
    #[serde(rename = "SpaceReclaimed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub space_reclaimed: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContainersPruneReportLibpod {
    #[serde(rename = "Err")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Error which occurred during prune operation (if any).
    /// This field is optional and may be omitted if no error occurred.
    pub err: Option<String>,
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// CreateContainerConfig used when compatible endpoint creates a container
pub struct CreateContainerConfig {
    #[serde(rename = "ArgsEscaped")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args_escaped: Option<bool>,
    #[serde(rename = "AttachStderr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach_stderr: Option<bool>,
    #[serde(rename = "AttachStdin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach_stdin: Option<bool>,
    #[serde(rename = "AttachStdout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach_stdout: Option<bool>,
    #[serde(rename = "Cmd")]
    pub cmd: Option<StrSlice>,
    #[serde(rename = "Domainname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domainname: Option<String>,
    #[serde(rename = "Entrypoint")]
    pub entrypoint: Option<StrSlice>,
    #[serde(rename = "Env")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<Vec<String>>,
    #[serde(rename = "EnvMerge")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env_merge: Option<Vec<String>>,
    #[serde(rename = "ExposedPorts")]
    pub exposed_ports: Option<PortSet>,
    #[serde(rename = "Healthcheck")]
    pub healthcheck: Option<HealthcheckConfig>,
    #[serde(rename = "HostConfig")]
    pub host_config: Option<HostConfig>,
    #[serde(rename = "Hostname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    #[serde(rename = "Image")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(rename = "Labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<HashMap<String, String>>,
    #[serde(rename = "MacAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Mac Address of the container.
    ///
    /// Deprecated: this field is deprecated since API v1.44. Use EndpointSettings.MacAddress instead.
    pub mac_address: Option<String>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "NetworkDisabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_disabled: Option<bool>,
    #[serde(rename = "NetworkingConfig")]
    pub networking_config: Option<NetworkingConfig>,
    #[serde(rename = "OnBuild")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_build: Option<Vec<String>>,
    #[serde(rename = "OpenStdin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_stdin: Option<bool>,
    #[serde(rename = "Shell")]
    pub shell: Option<StrSlice>,
    #[serde(rename = "StdinOnce")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stdin_once: Option<bool>,
    #[serde(rename = "StopSignal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_signal: Option<String>,
    #[serde(rename = "StopTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_timeout: Option<i64>,
    #[serde(rename = "Tty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tty: Option<bool>,
    #[serde(rename = "UnsetEnv")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unset_env: Option<Vec<String>>,
    #[serde(rename = "UnsetEnvAll")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unset_env_all: Option<bool>,
    #[serde(rename = "User")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    #[serde(rename = "Volumes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes: Option<HashMap<String, Value>>,
    #[serde(rename = "WorkingDir")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_dir: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateOptions {
    #[serde(rename = "Attachable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachable: Option<bool>,
    #[serde(rename = "ConfigFrom")]
    pub config_from: Option<ConfigReference>,
    #[serde(rename = "ConfigOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_only: Option<bool>,
    #[serde(rename = "Driver")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Name of the volume driver to use.
    pub driver: Option<String>,
    #[serde(rename = "EnableIPv6")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_i_pv_6: Option<bool>,
    #[serde(rename = "IPAM")]
    pub ipam: Option<Ipam>,
    #[serde(rename = "Ingress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingress: Option<bool>,
    #[serde(rename = "Internal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal: Option<bool>,
    #[serde(rename = "Labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// User-defined key/value metadata.
    pub labels: Option<HashMap<String, String>>,
    #[serde(rename = "Options")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<HashMap<String, String>>,
    #[serde(rename = "Scope")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateRequest {
    #[serde(rename = "Attachable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachable: Option<bool>,
    #[serde(rename = "CheckDuplicate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Deprecated: CheckDuplicate is deprecated since API v1.44, but it defaults to true when sent by the client
    /// package to older daemons.
    pub check_duplicate: Option<bool>,
    #[serde(rename = "ConfigFrom")]
    pub config_from: Option<ConfigReference>,
    #[serde(rename = "ConfigOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_only: Option<bool>,
    #[serde(rename = "Driver")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver: Option<String>,
    #[serde(rename = "EnableIPv6")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_i_pv_6: Option<bool>,
    #[serde(rename = "IPAM")]
    pub ipam: Option<Ipam>,
    #[serde(rename = "Ingress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingress: Option<bool>,
    #[serde(rename = "Internal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal: Option<bool>,
    #[serde(rename = "Labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<HashMap<String, String>>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Options")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<HashMap<String, String>>,
    #[serde(rename = "Scope")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// OK response to ContainerCreate operation
pub struct CreateResponse {
    #[serde(rename = "Id")]
    /// The ID of the created container
    pub id: String,
    #[serde(rename = "Warnings")]
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_nonoptional_vec")]
    /// Warnings encountered when creating the container
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DnsNetworkInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// DeleteResponse delete response
pub struct DeleteResponse {
    #[serde(rename = "Deleted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The image ID of an image that was deleted
    pub deleted: Option<String>,
    #[serde(rename = "Untagged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The image ID of an image that was untagged
    pub untagged: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// This structure provides `application/vnd.oci.descriptor.v1+json` mediatype
/// when marshalled to JSON.
pub struct Descriptor {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Annotations contains arbitrary metadata relating to the targeted content.
    pub annotations: Option<HashMap<String, String>>,
    #[serde(rename = "artifactType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// ArtifactType is the IANA media type of this artifact.
    pub artifact_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Data is an embedding of the targeted content. This is encoded as a base64
    /// string when marshalled to JSON (automatically, by encoding/json). If
    /// present, Data can be used directly to avoid fetching the targeted content.
    pub data: Option<Vec<u8>>,
    pub digest: Option<String>,
    #[serde(rename = "mediaType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// MediaType is the media type of the object this schema refers to.
    pub media_type: Option<String>,
    pub platform: Option<Platform>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Size specifies the size in bytes of the blob.
    pub size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// URLs specifies a list of URLs from which this object MAY be downloaded
    pub urls: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeviceMapping {
    #[serde(rename = "CgroupPermissions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cgroup_permissions: Option<String>,
    #[serde(rename = "PathInContainer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_in_container: Option<String>,
    #[serde(rename = "PathOnHost")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_on_host: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Used by GPU device drivers.
pub struct DeviceRequest {
    #[serde(rename = "Capabilities")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<Vec<Vec<String>>>,
    #[serde(rename = "Count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[serde(rename = "DeviceIDs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_i_ds: Option<Vec<String>>,
    #[serde(rename = "Driver")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver: Option<String>,
    #[serde(rename = "Options")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<HashMap<String, String>>,
}

/// The following is an example of the contents of Digest types:
///
/// sha256:7173b809ca12ec5dee4506cd86be934c4596dd234ee82c0662eac04a8c2c71dc
///
/// This allows to abstract the digest behind this type and work only in those
/// terms.
pub type Digest = String;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// DisconnectOptions represents the data to be used to disconnect a container
/// from the network.
pub struct DisconnectOptions {
    #[serde(rename = "Container")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container: Option<String>,
    #[serde(rename = "Force")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// DistributionInfo describes the host distribution for libpod
pub struct DistributionInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codename: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distribution: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Driver {
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Options")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// DriverData handles the data for a storage driver
pub struct DriverData {
    #[serde(rename = "Data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<HashMap<String, String>>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// A Duration represents the elapsed time between two instants
/// as an int64 nanosecond count. The representation limits the
/// largest representable duration to approximately 290 years.
pub type Duration = i64;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// EndpointIPAMConfig represents IPAM configurations for the endpoint
pub struct EndpointIpamConfig {
    #[serde(rename = "IPv4Address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_pv_4_address: Option<String>,
    #[serde(rename = "IPv6Address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_pv_6_address: Option<String>,
    #[serde(rename = "LinkLocalIPs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_local_i_ps: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// EndpointResource contains network resources allocated and used for a
/// container in a network.
pub struct EndpointResource {
    #[serde(rename = "EndpointID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_id: Option<String>,
    #[serde(rename = "IPv4Address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_pv_4_address: Option<String>,
    #[serde(rename = "IPv6Address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_pv_6_address: Option<String>,
    #[serde(rename = "MacAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// EndpointSettings stores the network endpoint details
pub struct EndpointSettings {
    #[serde(rename = "Aliases")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Vec<String>>,
    #[serde(rename = "DNSNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// DNSNames holds all the (non fully qualified) DNS names associated to this endpoint. First entry is used to
    /// generate PTR records.
    pub dns_names: Option<Vec<String>>,
    #[serde(rename = "DriverOpts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver_opts: Option<HashMap<String, String>>,
    #[serde(rename = "EndpointID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_id: Option<String>,
    #[serde(rename = "Gateway")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway: Option<String>,
    #[serde(rename = "GlobalIPv6Address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_i_pv_6_address: Option<String>,
    #[serde(rename = "GlobalIPv6PrefixLen")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_i_pv_6_prefix_len: Option<i64>,
    #[serde(rename = "IPAMConfig")]
    pub ipam_config: Option<EndpointIpamConfig>,
    #[serde(rename = "IPAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[serde(rename = "IPPrefixLen")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_prefix_len: Option<i64>,
    #[serde(rename = "IPv6Gateway")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_pv_6_gateway: Option<String>,
    #[serde(rename = "Links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<String>>,
    #[serde(rename = "MacAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// MacAddress may be used to specify a MAC address when the container is created.
    /// Once the container is running, it becomes operational data (it may contain a
    /// generated address).
    pub mac_address: Option<String>,
    #[serde(rename = "NetworkID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Operational data
    pub network_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// ErrorModel is used in remote connections with podman
pub struct ErrorModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// API root cause formatted for automated parsing
    pub cause: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// human error message, formatted for a human to read
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// HTTP response code
    pub response: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponse {
    /// The error message.
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExecStartControlParam {
    #[serde(rename = "Detach")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Detach from the command. Not presently supported.
    pub detach: Option<bool>,
    #[serde(rename = "Tty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Allocate a pseudo-TTY. Presently ignored.
    pub tty: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExecStartLibpodControlParam {
    #[serde(rename = "Detach")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Detach from the command.
    pub detach: Option<bool>,
    #[serde(rename = "Tty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Allocate a pseudo-TTY.
    pub tty: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Height of the TTY session in characters. Tty must be set to true to use it.
    pub h: Option<isize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Width of the TTY session in characters. Tty must be set to true to use it.
    pub w: Option<isize>,
}

/// The bits have the same definition on all systems, so that
/// information about files can be moved from one system
/// to another portably. Not all bits apply to all systems.
/// The only required bit is [ModeDir] for directories.
pub type FileMode = u32;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FilesystemChange {
    #[serde(rename = "Kind")]
    pub kind: u8,
    #[serde(rename = "Path")]
    /// Path to file or directory that has changed.
    pub path: String,
}

/// Kubernetes YAML file describing pod
pub type GenerateKubeLibpod200Response = Vec<u8>;

/// no error
pub type GenerateSystemdLibpod200Response = HashMap<String, String>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// GraphDriverData Information about the storage driver used to store the container's and
/// image's filesystem.
pub struct GraphDriverData {
    #[serde(rename = "Data")]
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_nonoptional_map")]
    /// Low-level storage metadata, provided as key/value pairs.
    ///
    /// This information is driver-specific, and depends on the storage-driver
    /// in use, and should be used for informational purposes only.
    pub data: HashMap<String, String>,
    #[serde(rename = "Name")]
    /// Name of the storage driver.
    pub name: String,
}

pub type HardwareAddr = Vec<u8>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Health stores information about the container's healthcheck results
pub struct Health {
    #[serde(rename = "FailingStreak")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failing_streak: Option<i64>,
    #[serde(rename = "Log")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log: Option<Vec<HealthcheckResult>>,
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// HealthCheckLog describes the results of a single healthcheck
pub struct HealthCheckLog {
    #[serde(rename = "End")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// End time as a string
    pub end: Option<String>,
    #[serde(rename = "ExitCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Exitcode is 0 or 1
    pub exit_code: Option<i64>,
    #[serde(rename = "Output")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Output is the stdout/stderr from the healthcheck command
    pub output: Option<String>,
    #[serde(rename = "Start")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Start time as string
    pub start: Option<String>,
}

/// HealthCheckOnFailureAction defines how Podman reacts when a container's health
/// status turns unhealthy.
pub type HealthCheckOnFailureAction = i64;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// HealthCheckResults describes the results/logs from a healthcheck
pub struct HealthCheckResults {
    #[serde(rename = "FailingStreak")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// FailingStreak is the number of consecutive failed healthchecks
    pub failing_streak: Option<i64>,
    #[serde(rename = "Log")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Log describes healthcheck attempts and results
    pub log: Option<Vec<HealthCheckLog>>,
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Status starting, healthy or unhealthy
    pub status: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HealthcheckConfig {
    #[serde(rename = "Interval")]
    pub interval: Option<i64>,
    #[serde(rename = "Retries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Retries is the number of consecutive failures needed to consider a container as unhealthy.
    /// Zero means inherit.
    pub retries: Option<i64>,
    #[serde(rename = "StartInterval")]
    pub start_interval: Option<i64>,
    #[serde(rename = "StartPeriod")]
    pub start_period: Option<i64>,
    #[serde(rename = "Test")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Test is the test to perform to check that the container is healthy.
    /// An empty slice means to inherit the default.
    /// The options are:
    /// {} : inherit healthcheck
    /// {"NONE"} : disable healthcheck
    /// {"CMD", args...} : exec arguments directly
    /// {"CMD-SHELL", command} : run command with system's default shell
    pub test: Option<Vec<String>>,
    #[serde(rename = "Timeout")]
    pub timeout: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// HealthcheckResult stores information about a single run of a healthcheck probe
pub struct HealthcheckResult {
    #[serde(rename = "End")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<DateTime<Utc>>,
    #[serde(rename = "ExitCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exit_code: Option<i64>,
    #[serde(rename = "Output")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
    #[serde(rename = "Start")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct History {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Author is the author of the build point.
    pub author: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Comment is a custom message set when creating the layer.
    pub comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Created is the combined date and time at which the layer was created, formatted as defined by RFC 3339, section 5.6.
    pub created: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// CreatedBy is the command which created the layer.
    pub created_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// EmptyLayer is used to mark if the history item created a filesystem diff.
    pub empty_layer: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// HistoryResponse provides details on image layers
pub struct HistoryResponse {
    #[serde(rename = "Comment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(rename = "Created")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<i64>,
    #[serde(rename = "CreatedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// HistoryResponseItem individual image layer information in response to ImageHistory operation
pub struct HistoryResponseItem {
    #[serde(rename = "Comment")]
    /// comment
    pub comment: String,
    #[serde(rename = "Created")]
    /// created
    pub created: i64,
    #[serde(rename = "CreatedBy")]
    /// created by
    pub created_by: String,
    #[serde(rename = "Id")]
    /// Id
    pub id: String,
    #[serde(rename = "Size")]
    /// size
    pub size: i64,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_nonoptional_vec")]
    /// tags
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Here, "non-portable" means "dependent of the host we are running on".
/// Portable information *should* appear in Config.
pub struct HostConfig {
    #[serde(rename = "Annotations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<HashMap<String, String>>,
    #[serde(rename = "AutoRemove")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_remove: Option<bool>,
    #[serde(rename = "Binds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Applicable to all platforms
    pub binds: Option<Vec<String>>,
    #[serde(rename = "BlkioDeviceReadBps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blkio_device_read_bps: Option<Vec<ThrottleDevice>>,
    #[serde(rename = "BlkioDeviceReadIOps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blkio_device_read_i_ops: Option<Vec<ThrottleDevice>>,
    #[serde(rename = "BlkioDeviceWriteBps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blkio_device_write_bps: Option<Vec<ThrottleDevice>>,
    #[serde(rename = "BlkioDeviceWriteIOps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blkio_device_write_i_ops: Option<Vec<ThrottleDevice>>,
    #[serde(rename = "BlkioWeight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blkio_weight: Option<u16>,
    #[serde(rename = "BlkioWeightDevice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blkio_weight_device: Option<Vec<WeightDevice>>,
    #[serde(rename = "CapAdd")]
    pub cap_add: Option<StrSlice>,
    #[serde(rename = "CapDrop")]
    pub cap_drop: Option<StrSlice>,
    #[serde(rename = "Cgroup")]
    pub cgroup: Option<String>,
    #[serde(rename = "CgroupParent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Applicable to UNIX platforms
    pub cgroup_parent: Option<String>,
    #[serde(rename = "CgroupnsMode")]
    pub cgroupns_mode: Option<String>,
    #[serde(rename = "ConsoleSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub console_size: Option<Vec<u64>>,
    #[serde(rename = "ContainerIDFile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_id_file: Option<String>,
    #[serde(rename = "CpuCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Applicable to Windows
    pub cpu_count: Option<i64>,
    #[serde(rename = "CpuPercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_percent: Option<i64>,
    #[serde(rename = "CpuPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_period: Option<i64>,
    #[serde(rename = "CpuQuota")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_quota: Option<i64>,
    #[serde(rename = "CpuRealtimePeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_realtime_period: Option<i64>,
    #[serde(rename = "CpuRealtimeRuntime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_realtime_runtime: Option<i64>,
    #[serde(rename = "CpuShares")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Applicable to all platforms
    pub cpu_shares: Option<i64>,
    #[serde(rename = "CpusetCpus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpuset_cpus: Option<String>,
    #[serde(rename = "CpusetMems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpuset_mems: Option<String>,
    #[serde(rename = "DeviceCgroupRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_cgroup_rules: Option<Vec<String>>,
    #[serde(rename = "DeviceRequests")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_requests: Option<Vec<DeviceRequest>>,
    #[serde(rename = "Devices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<DeviceMapping>>,
    #[serde(rename = "Dns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns: Option<Vec<String>>,
    #[serde(rename = "DnsOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_options: Option<Vec<String>>,
    #[serde(rename = "DnsSearch")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_search: Option<Vec<String>>,
    #[serde(rename = "ExtraHosts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_hosts: Option<Vec<String>>,
    #[serde(rename = "GroupAdd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_add: Option<Vec<String>>,
    #[serde(rename = "IOMaximumBandwidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub io_maximum_bandwidth: Option<u64>,
    #[serde(rename = "IOMaximumIOps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub io_maximum_i_ops: Option<u64>,
    #[serde(rename = "Init")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Run a custom init inside the container, if null, use the daemon's configured settings
    pub init: Option<bool>,
    #[serde(rename = "IpcMode")]
    pub ipc_mode: Option<String>,
    #[serde(rename = "Isolation")]
    pub isolation: Option<String>,
    #[serde(rename = "KernelMemory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// KernelMemory specifies the kernel memory limit (in bytes) for the container.
    /// Deprecated: kernel 5.4 deprecated kmem.limit_in_bytes.
    pub kernel_memory: Option<i64>,
    #[serde(rename = "KernelMemoryTCP")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kernel_memory_tcp: Option<i64>,
    #[serde(rename = "Links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<String>>,
    #[serde(rename = "LogConfig")]
    pub log_config: Option<LogConfig>,
    #[serde(rename = "MaskedPaths")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// MaskedPaths is the list of paths to be masked inside the container (this overrides the default set of paths)
    pub masked_paths: Option<Vec<String>>,
    #[serde(rename = "Memory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<i64>,
    #[serde(rename = "MemoryReservation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_reservation: Option<i64>,
    #[serde(rename = "MemorySwap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_swap: Option<i64>,
    #[serde(rename = "MemorySwappiness")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_swappiness: Option<i64>,
    #[serde(rename = "Mounts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Mounts specs used by the container
    pub mounts: Option<Vec<Mount>>,
    #[serde(rename = "NanoCpus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nano_cpus: Option<i64>,
    #[serde(rename = "NetworkMode")]
    pub network_mode: Option<String>,
    #[serde(rename = "OomKillDisable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oom_kill_disable: Option<bool>,
    #[serde(rename = "OomScoreAdj")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oom_score_adj: Option<i64>,
    #[serde(rename = "PidMode")]
    pub pid_mode: Option<String>,
    #[serde(rename = "PidsLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pids_limit: Option<i64>,
    #[serde(rename = "PortBindings")]
    pub port_bindings: Option<PortMap>,
    #[serde(rename = "Privileged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privileged: Option<bool>,
    #[serde(rename = "PublishAllPorts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publish_all_ports: Option<bool>,
    #[serde(rename = "ReadonlyPaths")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// ReadonlyPaths is the list of paths to be set as read-only inside the container (this overrides the default set of paths)
    pub readonly_paths: Option<Vec<String>>,
    #[serde(rename = "ReadonlyRootfs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readonly_rootfs: Option<bool>,
    #[serde(rename = "RestartPolicy")]
    pub restart_policy: Option<RestartPolicy>,
    #[serde(rename = "Runtime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime: Option<String>,
    #[serde(rename = "SecurityOpt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_opt: Option<Vec<String>>,
    #[serde(rename = "ShmSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shm_size: Option<i64>,
    #[serde(rename = "StorageOpt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_opt: Option<HashMap<String, String>>,
    #[serde(rename = "Sysctls")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sysctls: Option<HashMap<String, String>>,
    #[serde(rename = "Tmpfs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tmpfs: Option<HashMap<String, String>>,
    #[serde(rename = "UTSMode")]
    pub uts_mode: Option<String>,
    #[serde(rename = "Ulimits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ulimits: Option<Vec<Ulimit>>,
    #[serde(rename = "UsernsMode")]
    pub userns_mode: Option<String>,
    #[serde(rename = "VolumeDriver")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_driver: Option<String>,
    #[serde(rename = "VolumesFrom")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes_from: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// HostInfo describes the libpod host
pub struct HostInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arch: Option<String>,
    #[serde(rename = "buildahVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buildah_version: Option<String>,
    #[serde(rename = "cgroupControllers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cgroup_controllers: Option<Vec<String>>,
    #[serde(rename = "cgroupManager")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cgroup_manager: Option<String>,
    #[serde(rename = "cgroupVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cgroup_version: Option<String>,
    pub conmon: Option<ConmonInfo>,
    #[serde(rename = "cpuUtilization")]
    pub cpu_utilization: Option<CpuUsage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpus: Option<i64>,
    #[serde(rename = "databaseBackend")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_backend: Option<String>,
    pub distribution: Option<DistributionInfo>,
    #[serde(rename = "eventLogger")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_logger: Option<String>,
    #[serde(rename = "freeLocks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub free_locks: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    #[serde(rename = "idMappings")]
    pub id_mappings: Option<IdMappings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kernel: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linkmode: Option<String>,
    #[serde(rename = "logDriver")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_driver: Option<String>,
    #[serde(rename = "memFree")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mem_free: Option<i64>,
    #[serde(rename = "memTotal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mem_total: Option<i64>,
    #[serde(rename = "networkBackend")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_backend: Option<String>,
    #[serde(rename = "networkBackendInfo")]
    pub network_backend_info: Option<NetworkInfo>,
    #[serde(rename = "ociRuntime")]
    pub oci_runtime: Option<OciRuntimeInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,
    pub pasta: Option<PastaInfo>,
    #[serde(rename = "remoteSocket")]
    pub remote_socket: Option<RemoteSocket>,
    #[serde(rename = "rootlessNetworkCmd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// RootlessNetworkCmd returns the default rootless network command (slirp4netns or pasta)
    pub rootless_network_cmd: Option<String>,
    #[serde(rename = "runtimeInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime_info: Option<Value>,
    pub security: Option<SecurityInfo>,
    #[serde(rename = "serviceIsRemote")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// ServiceIsRemote is true when the podman/libpod service is remote to the client
    pub service_is_remote: Option<bool>,
    #[serde(rename = "slirp4netns")]
    pub slirp_4_netns: Option<SlirpInfo>,
    #[serde(rename = "swapFree")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swap_free: Option<i64>,
    #[serde(rename = "swapTotal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swap_total: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uptime: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// IDMap contains a single entry for user namespace range remapping. An array
/// of IDMap entries represents the structure that will be provided to the Linux
/// kernel for creating a user namespace.
pub struct IdMap {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// IDMappingOptions are used for specifying how ID mapping should be set up for
/// a layer or container.
pub struct IdMappingOptions {
    #[serde(rename = "AutoUserNs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_user_ns: Option<bool>,
    #[serde(rename = "AutoUserNsOpts")]
    pub auto_user_ns_opts: Option<AutoUserNsOptions>,
    #[serde(rename = "GIDMap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gid_map: Option<Vec<IdMap>>,
    #[serde(rename = "HostGIDMapping")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_gid_mapping: Option<bool>,
    #[serde(rename = "HostUIDMapping")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// UIDMap and GIDMap are used for setting up a layer's root filesystem
    /// for use inside of a user namespace where ID mapping is being used.
    /// If HostUIDMapping/HostGIDMapping is true, no mapping of the
    /// respective type will be used.  Otherwise, if UIDMap and/or GIDMap
    /// contain at least one mapping, one or both will be used.  By default,
    /// if neither of those conditions apply, if the layer has a parent
    /// layer, the parent layer's mapping will be used, and if it does not
    /// have a parent layer, the mapping which was passed to the Store
    /// object when it was initialized will be used.
    pub host_uid_mapping: Option<bool>,
    #[serde(rename = "UIDMap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid_map: Option<Vec<IdMap>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// IDMappings describe the GID and UID mappings
pub struct IdMappings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gidmap: Option<Vec<IdMap>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uidmap: Option<Vec<IdMap>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// IPAM represents IP Address Management
pub struct Ipam {
    #[serde(rename = "Config")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<Vec<IpamConfig>>,
    #[serde(rename = "Driver")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver: Option<String>,
    #[serde(rename = "Options")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// IPAMConfig represents IPAM configurations
pub struct IpamConfig {
    #[serde(rename = "AuxiliaryAddresses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auxiliary_addresses: Option<HashMap<String, String>>,
    #[serde(rename = "Gateway")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway: Option<String>,
    #[serde(rename = "IPRange")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_range: Option<String>,
    #[serde(rename = "Subnet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet: Option<String>,
}

/// See type [IPNet] and func [ParseCIDR] for details.
pub type IpMask = Vec<u8>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IpNet {
    #[serde(rename = "IP")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    #[serde(rename = "Mask")]
    pub mask: Option<IpMask>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// IDResponse Response to an API call that returns just an Id
pub struct IdResponse {
    #[serde(rename = "Id")]
    /// The id of the newly created object.
    pub id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// OK (As of version 1.xx)
pub struct ImageBuild200Response {
    /// output from build process
    pub stream: String,
}

pub type ImageBuildInputStreamParam = Vec<u8>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// OK (As of version 1.xx)
pub struct ImageBuildLibpod200Response {
    /// output from build process
    pub stream: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImageConfig {
    #[serde(rename = "ArgsEscaped")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// ArgsEscaped
    ///
    /// Deprecated: This field is present only for legacy compatibility with
    /// Docker and should not be used by new image builders.  It is used by Docker
    /// for Windows images to indicate that the `Entrypoint` or `Cmd` or both,
    /// contains only a single element array, that is a pre-escaped, and combined
    /// into a single string `CommandLine`. If `true` the value in `Entrypoint` or
    /// `Cmd` should be used as-is to avoid double escaping.
    /// https://github.com/opencontainers/image-spec/pull/892
    pub args_escaped: Option<bool>,
    #[serde(rename = "Cmd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Cmd defines the default arguments to the entrypoint of the container.
    pub cmd: Option<Vec<String>>,
    #[serde(rename = "Entrypoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Entrypoint defines a list of arguments to use as the command to execute when the container starts.
    pub entrypoint: Option<Vec<String>>,
    #[serde(rename = "Env")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Env is a list of environment variables to be used in a container.
    pub env: Option<Vec<String>>,
    #[serde(rename = "ExposedPorts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// ExposedPorts a set of ports to expose from a container running this image.
    pub exposed_ports: Option<HashMap<String, Value>>,
    #[serde(rename = "Labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Labels contains arbitrary metadata for the container.
    pub labels: Option<HashMap<String, String>>,
    #[serde(rename = "StopSignal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// StopSignal contains the system call signal that will be sent to the container to exit.
    pub stop_signal: Option<String>,
    #[serde(rename = "User")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// User defines the username or UID which the process in the container should run as.
    pub user: Option<String>,
    #[serde(rename = "Volumes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Volumes is a set of directories describing where the process is likely write data specific to a container instance.
    pub volumes: Option<HashMap<String, Value>>,
    #[serde(rename = "WorkingDir")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// WorkingDir sets the current working directory of the entrypoint process in the container.
    pub working_dir: Option<String>,
}

/// no error
pub type ImageCreate200Response = Vec<u8>;

pub type ImageCreateInputImageParam = Vec<u8>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImageData {
    #[serde(rename = "Annotations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<HashMap<String, String>>,
    #[serde(rename = "Architecture")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,
    #[serde(rename = "Author")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[serde(rename = "Comment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(rename = "Config")]
    pub config: Option<ImageConfig>,
    #[serde(rename = "Created")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<DateTime<Utc>>,
    #[serde(rename = "Digest")]
    pub digest: Option<String>,
    #[serde(rename = "GraphDriver")]
    pub graph_driver: Option<DriverData>,
    #[serde(rename = "Healthcheck")]
    pub healthcheck: Option<Schema2HealthConfig>,
    #[serde(rename = "History")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub history: Option<Vec<History>>,
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<HashMap<String, String>>,
    #[serde(rename = "ManifestType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_type: Option<String>,
    #[serde(rename = "NamesHistory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names_history: Option<Vec<String>>,
    #[serde(rename = "Os")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,
    #[serde(rename = "Parent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,
    #[serde(rename = "RepoDigests")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_digests: Option<Vec<String>>,
    #[serde(rename = "RepoTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_tags: Option<Vec<String>>,
    #[serde(rename = "RootFS")]
    pub root_fs: Option<RootFs>,
    #[serde(rename = "Size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(rename = "User")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "VirtualSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_size: Option<i64>,
}

/// no error
pub type ImageExportLibpod200Response = Vec<u8>;

/// no error
pub type ImageGet200Response = Vec<u8>;

/// no error
pub type ImageGetAll200Response = Vec<u8>;

/// no error
pub type ImageGetLibpod200Response = Vec<u8>;

pub type ImageImportLibpodUploadParam = Vec<u8>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImageImportReport {
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImageInspect {
    #[serde(rename = "Architecture")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Architecture is the hardware CPU architecture that the image runs on.
    pub architecture: Option<String>,
    #[serde(rename = "Author")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Author is the name of the author that was specified when committing the
    /// image, or as specified through MAINTAINER (deprecated) in the Dockerfile.
    pub author: Option<String>,
    #[serde(rename = "Comment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Comment is an optional message that can be set when committing or
    /// importing the image.
    pub comment: Option<String>,
    #[serde(rename = "Config")]
    pub config: Option<Config>,
    #[serde(rename = "Container")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Container is for backwards compat but is basically unused
    pub container: Option<String>,
    #[serde(rename = "ContainerConfig")]
    pub container_config: Option<Config>,
    #[serde(rename = "Created")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Created is the date and time at which the image was created, formatted in
    /// RFC 3339 nano-seconds (time.RFC3339Nano).
    ///
    /// This information is only available if present in the image,
    /// and omitted otherwise.
    pub created: Option<String>,
    #[serde(rename = "DockerVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// DockerVersion is the version of Docker that was used to build the image.
    ///
    /// Depending on how the image was created, this field may be empty.
    pub docker_version: Option<String>,
    #[serde(rename = "GraphDriver")]
    pub graph_driver: Option<GraphDriverData>,
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// ID is the content-addressable ID of an image.
    ///
    /// This identifier is a content-addressable digest calculated from the
    /// image's configuration (which includes the digests of layers used by
    /// the image).
    ///
    /// Note that this digest differs from the `RepoDigests` below, which
    /// holds digests of image manifests that reference the image.
    pub id: Option<String>,
    #[serde(rename = "Metadata")]
    pub metadata: Option<Metadata>,
    #[serde(rename = "Os")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// OS is the Operating System the image is built to run on.
    pub os: Option<String>,
    #[serde(rename = "OsVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// OsVersion is the version of the Operating System the image is built to
    /// run on (especially for Windows).
    pub os_version: Option<String>,
    #[serde(rename = "Parent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Parent is the ID of the parent image.
    ///
    /// Depending on how the image was created, this field may be empty and
    /// is only set for images that were built/created locally. This field
    /// is empty if the image was pulled from an image registry.
    pub parent: Option<String>,
    #[serde(rename = "RepoDigests")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// RepoDigests is a list of content-addressable digests of locally available
    /// image manifests that the image is referenced from. Multiple manifests can
    /// refer to the same image.
    ///
    /// These digests are usually only available if the image was either pulled
    /// from a registry, or if the image was pushed to a registry, which is when
    /// the manifest is generated and its digest calculated.
    pub repo_digests: Option<Vec<String>>,
    #[serde(rename = "RepoTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// RepoTags is a list of image names/tags in the local image cache that
    /// reference this image.
    ///
    /// Multiple image tags can refer to the same image, and this list may be
    /// empty if no tags reference the image, in which case the image is
    /// "untagged", in which case it can still be referenced by its ID.
    pub repo_tags: Option<Vec<String>>,
    #[serde(rename = "RootFS")]
    pub root_fs: Option<RootFs>,
    #[serde(rename = "Size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Size is the total size of the image including all layers it is composed of.
    pub size: Option<i64>,
    #[serde(rename = "Variant")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Variant is the CPU architecture variant (presently ARM-only).
    pub variant: Option<String>,
    #[serde(rename = "VirtualSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// VirtualSize is the total size of the image including all layers it is
    /// composed of.
    ///
    /// Deprecated: this field is omitted in API v1.44, but kept for backward compatibility. Use Size instead.
    pub virtual_size: Option<i64>,
}

pub type ImageLoadLibpodUploadParam = String;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImageLoadReport {
    #[serde(rename = "Names")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<Vec<String>>,
}

pub type ImageLoadRequestParam = String;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImageProperties {
    #[serde(rename = "Containers")]
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_nonoptional_vec")]
    /// Containers is an array containing the IDs of the containers that are
    /// using this image.
    pub containers: Vec<String>,
    #[serde(rename = "Platform")]
    pub platform: Platform,
    #[serde(rename = "Size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<ImagePropertiesSizeInlineItem>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImagePropertiesSizeInlineItem {
    #[serde(rename = "Unpacked")]
    /// Unpacked is the size (in bytes) of the locally unpacked
    /// (uncompressed) image content that's directly usable by the containers
    /// running this image.
    /// It's independent of the distributable content - e.g.
    /// the image might still have an unpacked data that's still used by
    /// some container even when the distributable/compressed content is
    /// already gone.
    pub unpacked: i64,
}

/// no error
pub type ImagePush200Response = Vec<u8>;

/// no error
pub type ImagePushLibpod200Response = Vec<u8>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// ImageStore describes the image store.  Right now only the number
/// of images present
pub struct ImageStore {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImageTreeReport {
    #[serde(rename = "Tree")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tree: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// ImageVolume is a volume based on a container image.  The container image is
/// first mounted on the host and is then bind-mounted into the container.  An
/// ImageVolume is always mounted read-only.
pub struct ImageVolume {
    #[serde(rename = "Destination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Destination is the absolute path of the mount in the container.
    pub destination: Option<String>,
    #[serde(rename = "ReadWrite")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// ReadWrite sets the volume writable.
    pub read_write: Option<bool>,
    #[serde(rename = "Source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Source is the source of the image volume.  The image can be referred
    /// to by name and by ID.
    pub source: Option<String>,
    #[serde(rename = "subPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// SubPath mounts a particular path within the image.
    /// If empty, the whole image is mounted.
    pub sub_path: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Info contains information about the Volume as a whole as provided by
/// the CSI storage plugin.
pub struct Info {
    #[serde(rename = "AccessibleTopology")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// AccessibleTopology is the topology this volume is actually accessible
    /// from.
    pub accessible_topology: Option<Vec<Topology>>,
    #[serde(rename = "CapacityBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// CapacityBytes is the capacity of the volume in bytes. A value of 0
    /// indicates that the capacity is unknown.
    pub capacity_bytes: Option<i64>,
    #[serde(rename = "VolumeContext")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// VolumeContext is the context originating from the CSI storage plugin
    /// when the Volume is created.
    pub volume_context: Option<HashMap<String, String>>,
    #[serde(rename = "VolumeID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// VolumeID is the ID of the Volume as seen by the CSI storage plugin. This
    /// is distinct from the Volume's Swarm ID, which is the ID used by all of
    /// the Docker Engine to refer to the Volume. If this field is blank, then
    /// the Volume has not been successfully created yet.
    pub volume_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Inspect {
    #[serde(rename = "Attachable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachable: Option<bool>,
    #[serde(rename = "ConfigFrom")]
    pub config_from: Option<ConfigReference>,
    #[serde(rename = "ConfigOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_only: Option<bool>,
    #[serde(rename = "Containers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub containers: Option<HashMap<String, EndpointResource>>,
    #[serde(rename = "Created")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<DateTime<Utc>>,
    #[serde(rename = "Driver")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver: Option<String>,
    #[serde(rename = "EnableIPv6")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_i_pv_6: Option<bool>,
    #[serde(rename = "IPAM")]
    pub ipam: Option<Ipam>,
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Ingress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingress: Option<bool>,
    #[serde(rename = "Internal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal: Option<bool>,
    #[serde(rename = "Labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<HashMap<String, String>>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Options")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<HashMap<String, String>>,
    #[serde(rename = "Peers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peers: Option<Vec<PeerInfo>>,
    #[serde(rename = "Scope")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[serde(rename = "Services")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub services: Option<HashMap<String, ServiceInfo>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// InspectAdditionalNetwork holds information about non-default networks the
/// container has been connected to.
/// As with InspectNetworkSettings, many fields are unused and maintained only
/// for compatibility with Docker.
pub struct InspectAdditionalNetwork {
    #[serde(rename = "AdditionalMACAddresses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// AdditionalMacAddresses is a set of additional MAC Addresses beyond
    /// the first. CNI may configure more than one interface for a single
    /// network, which can cause this.
    pub additional_mac_addresses: Option<Vec<String>>,
    #[serde(rename = "Aliases")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Aliases are any network aliases the container has in this network.
    pub aliases: Option<Vec<String>>,
    #[serde(rename = "DriverOpts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// DriverOpts is presently unused and maintained exclusively for
    /// compatibility.
    pub driver_opts: Option<HashMap<String, String>>,
    #[serde(rename = "EndpointID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// EndpointID is unused, maintained exclusively for compatibility.
    pub endpoint_id: Option<String>,
    #[serde(rename = "Gateway")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Gateway is the IP address of the gateway this network will use.
    pub gateway: Option<String>,
    #[serde(rename = "GlobalIPv6Address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// GlobalIPv6Address is the global-scope IPv6 Address for this network.
    pub global_i_pv_6_address: Option<String>,
    #[serde(rename = "GlobalIPv6PrefixLen")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// GlobalIPv6PrefixLen is the length of the subnet mask of this network.
    pub global_i_pv_6_prefix_len: Option<i64>,
    #[serde(rename = "IPAMConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// IPAMConfig is presently unused and maintained exclusively for
    /// compatibility.
    pub ipam_config: Option<HashMap<String, String>>,
    #[serde(rename = "IPAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// IPAddress is the IP address for this network.
    pub ip_address: Option<String>,
    #[serde(rename = "IPPrefixLen")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// IPPrefixLen is the length of the subnet mask of this network.
    pub ip_prefix_len: Option<i64>,
    #[serde(rename = "IPv6Gateway")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// IPv6Gateway is the IPv6 gateway this network will use.
    pub i_pv_6_gateway: Option<String>,
    #[serde(rename = "Links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Links is presently unused and maintained exclusively for
    /// compatibility.
    pub links: Option<Vec<String>>,
    #[serde(rename = "MacAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// MacAddress is the MAC address for the interface in this network.
    pub mac_address: Option<String>,
    #[serde(rename = "NetworkID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Name of the network we're connecting to.
    pub network_id: Option<String>,
    #[serde(rename = "SecondaryIPAddresses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// SecondaryIPAddresses is a list of extra IP Addresses that the
    /// container has been assigned in this network.
    pub secondary_ip_addresses: Option<Vec<Address>>,
    #[serde(rename = "SecondaryIPv6Addresses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// SecondaryIPv6Addresses is a list of extra IPv6 Addresses that the
    /// container has been assigned in this network.
    pub secondary_i_pv_6_addresses: Option<Vec<Address>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// InspectBlkioThrottleDevice holds information about a speed cap for a device
/// node. This cap applies to a specific operation (read, write, etc) on the given
/// node.
pub struct InspectBlkioThrottleDevice {
    #[serde(rename = "Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Path is the path to the device this applies to.
    pub path: Option<String>,
    #[serde(rename = "Rate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Rate is the maximum rate. It is in either bytes per second or iops
    /// per second, determined by where it is used - documentation will
    /// indicate which is appropriate.
    pub rate: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// InspectBlkioWeightDevice holds information about the relative weight
/// of an individual device node. Weights are used in the I/O scheduler to give
/// relative priority to some accesses.
pub struct InspectBlkioWeightDevice {
    #[serde(rename = "Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Path is the path to the device this applies to.
    pub path: Option<String>,
    #[serde(rename = "Weight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Weight is the relative weight the scheduler will use when scheduling
    /// I/O.
    pub weight: Option<u16>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// InspectContainerConfig holds further data about how a container was initially
/// configured.
pub struct InspectContainerConfig {
    #[serde(rename = "Annotations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Container annotations
    pub annotations: Option<HashMap<String, String>>,
    #[serde(rename = "AttachStderr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Unused, at present
    pub attach_stderr: Option<bool>,
    #[serde(rename = "AttachStdin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Unused, at present
    pub attach_stdin: Option<bool>,
    #[serde(rename = "AttachStdout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Unused, at present
    pub attach_stdout: Option<bool>,
    #[serde(rename = "ChrootDirs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// ChrootDirs is an additional set of directories that need to be
    /// treated as root directories. Standard bind mounts will be mounted
    /// into paths relative to these directories.
    pub chroot_dirs: Option<Vec<String>>,
    #[serde(rename = "Cmd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Container command
    pub cmd: Option<Vec<String>>,
    #[serde(rename = "CreateCommand")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// CreateCommand is the full command plus arguments of the process the
    /// container has been created with.
    pub create_command: Option<Vec<String>>,
    #[serde(rename = "Domainname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Container domain name - unused at present
    pub domainname: Option<String>,
    #[serde(rename = "Entrypoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Container entrypoint
    pub entrypoint: Option<Vec<String>>,
    #[serde(rename = "Env")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Container environment variables
    pub env: Option<Vec<String>>,
    #[serde(rename = "ExposedPorts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// ExposedPorts includes ports the container has exposed.
    pub exposed_ports: Option<HashMap<String, Value>>,
    #[serde(rename = "HealthLogDestination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// HealthLogDestination defines the destination where the log is stored
    pub health_log_destination: Option<String>,
    #[serde(rename = "Healthcheck")]
    pub healthcheck: Option<Schema2HealthConfig>,
    #[serde(rename = "HealthcheckMaxLogCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// HealthMaxLogCount is maximum number of attempts in the HealthCheck log file.
    /// ('0' value means an infinite number of attempts in the log file)
    pub healthcheck_max_log_count: Option<u64>,
    #[serde(rename = "HealthcheckMaxLogSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// HealthMaxLogSize is the maximum length in characters of stored HealthCheck log
    /// ("0" value means an infinite log length)
    pub healthcheck_max_log_size: Option<u64>,
    #[serde(rename = "HealthcheckOnFailureAction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// HealthcheckOnFailureAction defines an action to take once the container turns unhealthy.
    pub healthcheck_on_failure_action: Option<String>,
    #[serde(rename = "Hostname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Container hostname
    pub hostname: Option<String>,
    #[serde(rename = "Image")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Container image
    pub image: Option<String>,
    #[serde(rename = "Labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Container labels
    pub labels: Option<HashMap<String, String>>,
    #[serde(rename = "OnBuild")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// On-build arguments - presently unused. More of Buildah's domain.
    pub on_build: Option<String>,
    #[serde(rename = "OpenStdin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether the container leaves STDIN open
    pub open_stdin: Option<bool>,
    #[serde(rename = "Passwd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Passwd determines whether or not podman can add entries to /etc/passwd and /etc/group
    pub passwd: Option<bool>,
    #[serde(rename = "Secrets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Secrets are the secrets mounted in the container
    pub secrets: Option<Vec<InspectSecret>>,
    #[serde(rename = "StartupHealthCheck")]
    pub startup_health_check: Option<StartupHealthCheck>,
    #[serde(rename = "StdinOnce")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether STDIN is only left open once.
    /// Presently not supported by Podman, unused.
    pub stdin_once: Option<bool>,
    #[serde(rename = "StopSignal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Container stop signal
    pub stop_signal: Option<String>,
    #[serde(rename = "StopTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// StopTimeout is time before container is stopped when calling stop
    pub stop_timeout: Option<u64>,
    #[serde(rename = "SystemdMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// SystemdMode is whether the container is running in systemd mode. In
    /// systemd mode, the container configuration is customized to optimize
    /// running systemd in the container.
    pub systemd_mode: Option<bool>,
    #[serde(rename = "Timeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Timeout is time before container is killed by conmon
    pub timeout: Option<u64>,
    #[serde(rename = "Timezone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Timezone is the timezone inside the container.
    /// Local means it has the same timezone as the host machine
    pub timezone: Option<String>,
    #[serde(rename = "Tty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether the container creates a TTY
    pub tty: Option<bool>,
    #[serde(rename = "Umask")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Umask is the umask inside the container.
    pub umask: Option<String>,
    #[serde(rename = "User")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// User the container was launched with
    pub user: Option<String>,
    #[serde(rename = "Volumes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Unused, at present. I've never seen this field populated.
    pub volumes: Option<HashMap<String, Value>>,
    #[serde(rename = "WorkingDir")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Container working directory
    pub working_dir: Option<String>,
    #[serde(rename = "sdNotifyMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// SdNotifyMode is the sd-notify mode of the container.
    pub sd_notify_mode: Option<String>,
    #[serde(rename = "sdNotifySocket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// SdNotifySocket is the NOTIFY_SOCKET in use by/configured for the container.
    pub sd_notify_socket: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// InspectContainerData provides a detailed record of a container's configuration
/// and state as viewed by Libpod.
/// Large portions of this structure are defined such that the output is
/// compatible with `docker inspect` JSON, but additional fields have been added
/// as required to share information not in the original output.
pub struct InspectContainerData {
    #[serde(rename = "AppArmorProfile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_armor_profile: Option<String>,
    #[serde(rename = "Args")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<String>>,
    #[serde(rename = "BoundingCaps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bounding_caps: Option<Vec<String>>,
    #[serde(rename = "Config")]
    pub config: Option<InspectContainerConfig>,
    #[serde(rename = "ConmonPidFile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conmon_pid_file: Option<String>,
    #[serde(rename = "Created")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<DateTime<Utc>>,
    #[serde(rename = "Dependencies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<Vec<String>>,
    #[serde(rename = "Driver")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver: Option<String>,
    #[serde(rename = "EffectiveCaps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_caps: Option<Vec<String>>,
    #[serde(rename = "ExecIDs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exec_i_ds: Option<Vec<String>>,
    #[serde(rename = "GraphDriver")]
    pub graph_driver: Option<DriverData>,
    #[serde(rename = "HostConfig")]
    pub host_config: Option<InspectContainerHostConfig>,
    #[serde(rename = "HostnamePath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname_path: Option<String>,
    #[serde(rename = "HostsPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosts_path: Option<String>,
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Image")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(rename = "ImageDigest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_digest: Option<String>,
    #[serde(rename = "ImageName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_name: Option<String>,
    #[serde(rename = "IsInfra")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_infra: Option<bool>,
    #[serde(rename = "IsService")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_service: Option<bool>,
    #[serde(rename = "KubeExitCodePropagation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kube_exit_code_propagation: Option<String>,
    #[serde(rename = "MountLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_label: Option<String>,
    #[serde(rename = "Mounts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mounts: Option<Vec<InspectMount>>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Namespace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(rename = "NetworkSettings")]
    pub network_settings: Option<InspectNetworkSettings>,
    #[serde(rename = "OCIConfigPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oci_config_path: Option<String>,
    #[serde(rename = "OCIRuntime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oci_runtime: Option<String>,
    #[serde(rename = "Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "PidFile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pid_file: Option<String>,
    #[serde(rename = "Pod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pod: Option<String>,
    #[serde(rename = "ProcessLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_label: Option<String>,
    #[serde(rename = "ResolvConfPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolv_conf_path: Option<String>,
    #[serde(rename = "RestartCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restart_count: Option<i32>,
    #[serde(rename = "Rootfs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rootfs: Option<String>,
    #[serde(rename = "SizeRootFs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_root_fs: Option<i64>,
    #[serde(rename = "SizeRw")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_rw: Option<i64>,
    #[serde(rename = "State")]
    pub state: Option<InspectContainerState>,
    #[serde(rename = "StaticDir")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub static_dir: Option<String>,
    #[serde(rename = "UseImageHostname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_image_hostname: Option<bool>,
    #[serde(rename = "UseImageHosts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_image_hosts: Option<bool>,
    #[serde(rename = "lockNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock_number: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// nolint:revive,stylecheck // Field names are fixed for compatibility and cannot be changed.
pub struct InspectContainerHostConfig {
    #[serde(rename = "Annotations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Annotations are provided to the runtime when the container is
    /// started.
    pub annotations: Option<HashMap<String, String>>,
    #[serde(rename = "AutoRemove")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// AutoRemove is whether the container will be automatically removed on
    /// exiting.
    /// It is not handled directly within libpod and is stored in an
    /// annotation.
    pub auto_remove: Option<bool>,
    #[serde(rename = "AutoRemoveImage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// AutoRemoveImage is whether the container's image will be
    /// automatically removed on exiting.
    /// It is not handled directly within libpod and is stored in an
    /// annotation.
    pub auto_remove_image: Option<bool>,
    #[serde(rename = "Binds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Binds contains an array of user-added mounts.
    /// Both volume mounts and named volumes are included.
    /// Tmpfs mounts are NOT included.
    /// In 'docker inspect' this is separated into 'Binds' and 'Mounts' based
    /// on how a mount was added. We do not make this distinction and do not
    /// include a Mounts field in inspect.
    /// Format: <src>:<destination>[:<comma-separated options>]
    pub binds: Option<Vec<String>>,
    #[serde(rename = "BlkioDeviceReadBps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// BlkioDeviceReadBps is an array of I/O throttle parameters for
    /// individual device nodes.
    /// This specifically sets read rate cap in bytes per second for device
    /// nodes.
    /// As with BlkioWeightDevice, we pull the path from /sys/dev, and we
    /// don't guarantee the path will be identical to the original (though
    /// the node will be).
    pub blkio_device_read_bps: Option<Vec<InspectBlkioThrottleDevice>>,
    #[serde(rename = "BlkioDeviceReadIOps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// BlkioDeviceReadIOps is an array of I/O throttle parameters for
    /// individual device nodes.
    /// This specifically sets the read rate cap in iops per second for
    /// device nodes.
    /// As with BlkioWeightDevice, we pull the path from /sys/dev, and we
    /// don't guarantee the path will be identical to the original (though
    /// the node will be).
    pub blkio_device_read_i_ops: Option<Vec<InspectBlkioThrottleDevice>>,
    #[serde(rename = "BlkioDeviceWriteBps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// BlkioDeviceWriteBps is an array of I/O throttle parameters for
    /// individual device nodes.
    /// this specifically sets write rate cap in bytes per second for device
    /// nodes.
    /// as with BlkioWeightDevice, we pull the path from /sys/dev, and we
    /// don't guarantee the path will be identical to the original (though
    /// the node will be).
    pub blkio_device_write_bps: Option<Vec<InspectBlkioThrottleDevice>>,
    #[serde(rename = "BlkioDeviceWriteIOps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// BlkioDeviceWriteIOps is an array of I/O throttle parameters for
    /// individual device nodes.
    /// This specifically sets the write rate cap in iops per second for
    /// device nodes.
    /// As with BlkioWeightDevice, we pull the path from /sys/dev, and we
    /// don't guarantee the path will be identical to the original (though
    /// the node will be).
    pub blkio_device_write_i_ops: Option<Vec<InspectBlkioThrottleDevice>>,
    #[serde(rename = "BlkioWeight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// BlkioWeight indicates the I/O resources allocated to the container.
    /// It is a relative weight in the scheduler for assigning I/O time
    /// versus other Cgroups.
    pub blkio_weight: Option<u16>,
    #[serde(rename = "BlkioWeightDevice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// BlkioWeightDevice is an array of I/O resource priorities for
    /// individual device nodes.
    /// Unfortunately, the spec only stores the device's Major/Minor numbers
    /// and not the path, which is used here.
    /// Fortunately, the kernel provides an interface for retrieving the path
    /// of a given node by major:minor at /sys/dev/. However, the exact path
    /// in use may not be what was used in the original CLI invocation -
    /// though it is guaranteed that the device node will be the same, and
    /// using the given path will be functionally identical.
    pub blkio_weight_device: Option<Vec<InspectBlkioWeightDevice>>,
    #[serde(rename = "CapAdd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// CapAdd is a list of capabilities added to the container.
    /// It is not directly stored by Libpod, and instead computed from the
    /// capabilities listed in the container's spec, compared against a set
    /// of default capabilities.
    pub cap_add: Option<Vec<String>>,
    #[serde(rename = "CapDrop")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// CapDrop is a list of capabilities removed from the container.
    /// It is not directly stored by libpod, and instead computed from the
    /// capabilities listed in the container's spec, compared against a set
    /// of default capabilities.
    pub cap_drop: Option<Vec<String>>,
    #[serde(rename = "Cgroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Cgroup contains the container's cgroup. It is presently not
    /// populated.
    /// TODO.
    pub cgroup: Option<String>,
    #[serde(rename = "CgroupConf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// CgroupConf is the configuration for cgroup v2.
    pub cgroup_conf: Option<HashMap<String, String>>,
    #[serde(rename = "CgroupManager")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// CgroupManager is the cgroup manager used by the container.
    /// At present, allowed values are either "cgroupfs" or "systemd".
    pub cgroup_manager: Option<String>,
    #[serde(rename = "CgroupMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// CgroupMode is the configuration of the container's cgroup namespace.
    /// Populated as follows:
    /// private - a cgroup namespace has been created
    /// host - No cgroup namespace created
    /// container:<id> - Using another container's cgroup namespace
    /// ns:<path> - A path to a cgroup namespace has been specified
    pub cgroup_mode: Option<String>,
    #[serde(rename = "CgroupParent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// CgroupParent is the Cgroup parent of the container.
    /// Only set if not default.
    pub cgroup_parent: Option<String>,
    #[serde(rename = "Cgroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Cgroups contains the container's Cgroup mode.
    /// Allowed values are "default" (container is creating Cgroups) and
    /// "disabled" (container is not creating Cgroups).
    /// This is Libpod-specific and not included in `docker inspect`.
    pub cgroups: Option<String>,
    #[serde(rename = "ConsoleSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// ConsoleSize is an array of 2 integers showing the size of the
    /// container's console.
    /// It is only set if the container is creating a terminal.
    /// TODO.
    pub console_size: Option<Vec<u64>>,
    #[serde(rename = "ContainerIDFile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// ContainerIDFile is a file created during container creation to hold
    /// the ID of the created container.
    /// This is not handled within libpod and is stored in an annotation.
    pub container_id_file: Option<String>,
    #[serde(rename = "CpuCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// CpuCount is Windows-only and not presently implemented.
    pub cpu_count: Option<u64>,
    #[serde(rename = "CpuPercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// CpuPercent is Windows-only and not presently implemented.
    pub cpu_percent: Option<u64>,
    #[serde(rename = "CpuPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// CpuPeriod is the length of a CPU period in microseconds.
    /// It relates directly to CpuQuota.
    pub cpu_period: Option<u64>,
    #[serde(rename = "CpuQuota")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// CpuPeriod is the amount of time (in microseconds) that a container
    /// can use the CPU in every CpuPeriod.
    pub cpu_quota: Option<i64>,
    #[serde(rename = "CpuRealtimePeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// CpuRealtimePeriod is the length of time (in microseconds) of the CPU
    /// realtime period. If set to 0, no time will be allocated to realtime
    /// tasks.
    pub cpu_realtime_period: Option<u64>,
    #[serde(rename = "CpuRealtimeRuntime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// CpuRealtimeRuntime is the length of time (in microseconds) allocated
    /// for realtime tasks within every CpuRealtimePeriod.
    pub cpu_realtime_runtime: Option<i64>,
    #[serde(rename = "CpuShares")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// CpuShares indicates the CPU resources allocated to the container.
    /// It is a relative weight in the scheduler for assigning CPU time
    /// versus other Cgroups.
    pub cpu_shares: Option<u64>,
    #[serde(rename = "CpusetCpus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// CpusetCpus is the set of CPUs that the container will execute on.
    /// Formatted as `0-3` or `0,2`. Default (if unset) is all CPUs.
    pub cpuset_cpus: Option<String>,
    #[serde(rename = "CpusetMems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// CpusetMems is the set of memory nodes the container will use.
    /// Formatted as `0-3` or `0,2`. Default (if unset) is all memory nodes.
    pub cpuset_mems: Option<String>,
    #[serde(rename = "Devices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Devices is a list of device nodes that will be added to the
    /// container.
    /// These are stored in the OCI spec only as type, major, minor while we
    /// display the host path. We convert this with /sys/dev, but we cannot
    /// guarantee that the host path will be identical - only that the actual
    /// device will be.
    pub devices: Option<Vec<InspectDevice>>,
    #[serde(rename = "DiskQuota")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// DiskQuota is the maximum amount of disk space the container may use
    /// (in bytes).
    /// Presently not populated.
    /// TODO.
    pub disk_quota: Option<u64>,
    #[serde(rename = "Dns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Dns is a list of DNS nameservers that will be added to the
    /// container's resolv.conf
    pub dns: Option<Vec<String>>,
    #[serde(rename = "DnsOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// DnsOptions is a list of DNS options that will be set in the
    /// container's resolv.conf
    pub dns_options: Option<Vec<String>>,
    #[serde(rename = "DnsSearch")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// DnsSearch is a list of DNS search domains that will be set in the
    /// container's resolv.conf
    pub dns_search: Option<Vec<String>>,
    #[serde(rename = "ExtraHosts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// ExtraHosts contains hosts that will be added to the container's
    /// etc/hosts.
    pub extra_hosts: Option<Vec<String>>,
    #[serde(rename = "GroupAdd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// GroupAdd contains groups that the user inside the container will be
    /// added to.
    pub group_add: Option<Vec<String>>,
    #[serde(rename = "HostsFile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// HostsFile is the base file to create the `/etc/hosts` file inside the container.
    pub hosts_file: Option<String>,
    #[serde(rename = "IDMappings")]
    pub id_mappings: Option<InspectIdMappings>,
    #[serde(rename = "IOMaximumBandwidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// IOMaximumBandwidth is Windows-only and not presently implemented.
    pub io_maximum_bandwidth: Option<u64>,
    #[serde(rename = "IOMaximumIOps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// IOMaximumIOps is Windows-only and not presently implemented.
    pub io_maximum_i_ops: Option<u64>,
    #[serde(rename = "Init")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Init indicates whether the container has an init mounted into it.
    pub init: Option<bool>,
    #[serde(rename = "IntelRdtClosID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// IntelRdtClosID defines the Intel RDT CAT Class Of Service (COS) that
    /// all processes of the container should run in.
    pub intel_rdt_clos_id: Option<String>,
    #[serde(rename = "IpcMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// IpcMode represents the configuration of the container's IPC
    /// namespace.
    /// Populated as follows:
    /// "" (empty string) - Default, an IPC namespace will be created
    /// host - No IPC namespace created
    /// container:<id> - Using another container's IPC namespace
    /// ns:<path> - A path to an IPC namespace has been specified
    pub ipc_mode: Option<String>,
    #[serde(rename = "Isolation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Isolation is presently unused and provided solely for Docker
    /// compatibility.
    pub isolation: Option<String>,
    #[serde(rename = "KernelMemory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// KernelMemory is the maximum amount of memory the kernel will devote
    /// to the container.
    pub kernel_memory: Option<i64>,
    #[serde(rename = "Links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Links is unused, and provided purely for Docker compatibility.
    pub links: Option<Vec<String>>,
    #[serde(rename = "LogConfig")]
    pub log_config: Option<InspectLogConfig>,
    #[serde(rename = "Memory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Memory indicates the memory resources allocated to the container.
    /// This is the limit (in bytes) of RAM the container may use.
    pub memory: Option<i64>,
    #[serde(rename = "MemoryReservation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// MemoryReservation is the reservation (soft limit) of memory available
    /// to the container. Soft limits are warnings only and can be exceeded.
    pub memory_reservation: Option<i64>,
    #[serde(rename = "MemorySwap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// MemorySwap is the total limit for all memory available to the
    /// container, including swap. 0 indicates that there is no limit to the
    /// amount of memory available.
    pub memory_swap: Option<i64>,
    #[serde(rename = "MemorySwappiness")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// MemorySwappiness is the willingness of the kernel to page container
    /// memory to swap. It is an integer from 0 to 100, with low numbers
    /// being more likely to be put into swap.
    /// 1, the default, will not set swappiness and use the system defaults.
    pub memory_swappiness: Option<i64>,
    #[serde(rename = "NanoCpus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// NanoCpus indicates number of CPUs allocated to the container.
    /// It is an integer where one full CPU is indicated by 1000000000 (one
    /// billion).
    /// Thus, 2.5 CPUs (fractional portions of CPUs are allowed) would be
    /// 2500000000 (2.5 billion).
    /// In 'docker inspect' this is set exclusively of two further options in
    /// the output (CpuPeriod and CpuQuota) which are both used to implement
    /// this functionality.
    /// We can't distinguish here, so if CpuQuota is set to the default of
    /// 100000, we will set both CpuQuota, CpuPeriod, and NanoCpus. If
    /// CpuQuota is not the default, we will not set NanoCpus.
    pub nano_cpus: Option<i64>,
    #[serde(rename = "NetworkMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// NetworkMode is the configuration of the container's network
    /// namespace.
    /// Populated as follows:
    /// default - A network namespace is being created and configured via CNI
    /// none - A network namespace is being created, not configured via CNI
    /// host - No network namespace created
    /// container:<id> - Using another container's network namespace
    /// ns:<path> - A path to a network namespace has been specified
    pub network_mode: Option<String>,
    #[serde(rename = "OomKillDisable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// OomKillDisable indicates whether the kernel OOM killer is disabled
    /// for the container.
    pub oom_kill_disable: Option<bool>,
    #[serde(rename = "OomScoreAdj")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// OOMScoreAdj is an adjustment that will be made to the container's OOM
    /// score.
    pub oom_score_adj: Option<i64>,
    #[serde(rename = "PidMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// PidMode represents the configuration of the container's PID
    /// namespace.
    /// Populated as follows:
    /// "" (empty string) - Default, a PID namespace will be created
    /// host - No PID namespace created
    /// container:<id> - Using another container's PID namespace
    /// ns:<path> - A path to a PID namespace has been specified
    pub pid_mode: Option<String>,
    #[serde(rename = "PidsLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// PidsLimit is the maximum number of PIDs that may be created within
    /// the container. 0, the default, indicates no limit.
    pub pids_limit: Option<i64>,
    #[serde(rename = "PortBindings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// PortBindings contains the container's port bindings.
    /// It is formatted as map[string][]InspectHostPort.
    /// The string key here is formatted as <integer port number>/<protocol>
    /// and represents the container port. A single container port may be
    /// bound to multiple host ports (on different IPs).
    pub port_bindings: Option<HashMap<String, Option<Vec<InspectHostPort>>>>,
    #[serde(rename = "Privileged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Privileged indicates whether the container is running with elevated
    /// privileges.
    /// This has a very specific meaning in the Docker sense, so it's very
    /// difficult to decode from the spec and config, and so is stored as an
    /// annotation.
    pub privileged: Option<bool>,
    #[serde(rename = "PublishAllPorts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// PublishAllPorts indicates whether image ports are being published.
    /// This is not directly stored in libpod and is saved as an annotation.
    pub publish_all_ports: Option<bool>,
    #[serde(rename = "ReadonlyRootfs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// ReadonlyRootfs is whether the container will be mounted read-only.
    pub readonly_rootfs: Option<bool>,
    #[serde(rename = "RestartPolicy")]
    pub restart_policy: Option<InspectRestartPolicy>,
    #[serde(rename = "Runtime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Runtime is provided purely for Docker compatibility.
    /// It is set unconditionally to "oci" as Podman does not presently
    /// support non-OCI runtimes.
    pub runtime: Option<String>,
    #[serde(rename = "SecurityOpt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// SecurityOpt is a list of security-related options that are set in the
    /// container.
    pub security_opt: Option<Vec<String>>,
    #[serde(rename = "ShmSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shm_size: Option<i64>,
    #[serde(rename = "Tmpfs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Tmpfs is a list of tmpfs filesystems that will be mounted into the
    /// container.
    /// It is a map of destination path to options for the mount.
    pub tmpfs: Option<HashMap<String, String>>,
    #[serde(rename = "UTSMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// UTSMode represents the configuration of the container's UID
    /// namespace.
    /// Populated as follows:
    /// "" (empty string) - Default, a UTS namespace will be created
    /// host - no UTS namespace created
    /// container:<id> - Using another container's UTS namespace
    /// ns:<path> - A path to a UTS namespace has been specified
    pub uts_mode: Option<String>,
    #[serde(rename = "Ulimits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Ulimits is a set of ulimits that will be set within the container.
    pub ulimits: Option<Vec<InspectUlimit>>,
    #[serde(rename = "UsernsMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// UsernsMode represents the configuration of the container's user
    /// namespace.
    /// When running rootless, a user namespace is created outside of libpod
    /// to allow some privileged operations. This will not be reflected here.
    /// Populated as follows:
    /// "" (empty string) - No user namespace will be created
    /// private - The container will be run in a user namespace
    /// container:<id> - Using another container's user namespace
    /// ns:<path> - A path to a user namespace has been specified
    /// TODO Rootless has an additional 'keep-id' option, presently not
    /// reflected here.
    pub userns_mode: Option<String>,
    #[serde(rename = "VolumeDriver")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// VolumeDriver is presently unused and is retained for Docker
    /// compatibility.
    pub volume_driver: Option<String>,
    #[serde(rename = "VolumesFrom")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// VolumesFrom is a list of containers which this container uses volumes
    /// from. This is not handled directly within libpod and is stored in an
    /// annotation.
    /// It is formatted as an array of container names and IDs.
    pub volumes_from: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// InspectContainerState provides a detailed record of a container's current
/// state. It is returned as part of InspectContainerData.
/// As with InspectContainerData, many portions of this struct are matched to
/// Docker, but here we see more fields that are unused (nonsensical in the
/// context of Libpod).
pub struct InspectContainerState {
    #[serde(rename = "CgroupPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cgroup_path: Option<String>,
    #[serde(rename = "CheckpointLog")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkpoint_log: Option<String>,
    #[serde(rename = "CheckpointPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkpoint_path: Option<String>,
    #[serde(rename = "Checkpointed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkpointed: Option<bool>,
    #[serde(rename = "CheckpointedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkpointed_at: Option<DateTime<Utc>>,
    #[serde(rename = "ConmonPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conmon_pid: Option<i64>,
    #[serde(rename = "Dead")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dead: Option<bool>,
    #[serde(rename = "Error")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(rename = "ExitCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exit_code: Option<i32>,
    #[serde(rename = "FinishedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finished_at: Option<DateTime<Utc>>,
    #[serde(rename = "Health")]
    pub health: Option<HealthCheckResults>,
    #[serde(rename = "OOMKilled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oom_killed: Option<bool>,
    #[serde(rename = "OciVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oci_version: Option<String>,
    #[serde(rename = "Paused")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paused: Option<bool>,
    #[serde(rename = "Pid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pid: Option<i64>,
    #[serde(rename = "Restarting")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restarting: Option<bool>,
    #[serde(rename = "RestoreLog")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_log: Option<String>,
    #[serde(rename = "Restored")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restored: Option<bool>,
    #[serde(rename = "RestoredAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restored_at: Option<DateTime<Utc>>,
    #[serde(rename = "Running")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub running: Option<bool>,
    #[serde(rename = "StartedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<DateTime<Utc>>,
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StoppedByUser")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stopped_by_user: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InspectDevice {
    #[serde(rename = "CgroupPermissions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// CgroupPermissions is the permissions of the mounted device.
    /// Presently not populated.
    /// TODO.
    pub cgroup_permissions: Option<String>,
    #[serde(rename = "PathInContainer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// PathInContainer is the path of the device within the container.
    pub path_in_container: Option<String>,
    #[serde(rename = "PathOnHost")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// PathOnHost is the path of the device on the host.
    pub path_on_host: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// InspectExecProcess contains information about the process in a given exec
/// session.
pub struct InspectExecProcess {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Arguments are the arguments to the entrypoint command of the exec
    /// session.
    pub arguments: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Entrypoint is the entrypoint for the exec session (the command that
    /// will be executed in the container).
    pub entrypoint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Privileged is whether the exec session will be started with elevated
    /// privileges.
    pub privileged: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Tty is whether the exec session created a terminal.
    pub tty: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// User is the user the exec session was started as.
    pub user: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InspectExecSession {
    #[serde(rename = "CanRemove")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// CanRemove is legacy and used purely for compatibility reasons.
    /// Will always be set to true, unless the exec session is running.
    pub can_remove: Option<bool>,
    #[serde(rename = "ContainerID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// ContainerID is the ID of the container this exec session is attached
    /// to.
    pub container_id: Option<String>,
    #[serde(rename = "DetachKeys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// DetachKeys are the detach keys used by the exec session.
    /// If set to "" the default keys are being used.
    /// Will show "<none>" if no detach keys are set.
    pub detach_keys: Option<String>,
    #[serde(rename = "ExitCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// ExitCode is the exit code of the exec session. Will be set to 0 if
    /// the exec session has not yet exited.
    pub exit_code: Option<i64>,
    #[serde(rename = "ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// ID is the ID of the exec session.
    pub id: Option<String>,
    #[serde(rename = "OpenStderr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// OpenStderr is whether the container's STDERR stream will be attached.
    /// Always set to true if the exec session created a TTY.
    pub open_stderr: Option<bool>,
    #[serde(rename = "OpenStdin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// OpenStdin is whether the container's STDIN stream will be attached
    /// to.
    pub open_stdin: Option<bool>,
    #[serde(rename = "OpenStdout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// OpenStdout is whether the container's STDOUT stream will be attached.
    /// Always set to true if the exec session created a TTY.
    pub open_stdout: Option<bool>,
    #[serde(rename = "Pid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Pid is the PID of the exec session's process.
    /// Will be set to 0 if the exec session is not running.
    pub pid: Option<i64>,
    #[serde(rename = "ProcessConfig")]
    pub process_config: Option<InspectExecProcess>,
    #[serde(rename = "Running")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Running is whether the exec session is running.
    pub running: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// InspectHostPort provides information on a port on the host that a container's
/// port is bound to.
pub struct InspectHostPort {
    #[serde(rename = "HostIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// IP on the host we are bound to. "" if not specified (binding to all
    /// IPs).
    pub host_ip: Option<String>,
    #[serde(rename = "HostPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Port on the host we are bound to. No special formatting - just an
    /// integer stuffed into a string.
    pub host_port: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InspectIdMappings {
    #[serde(rename = "GidMap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gid_map: Option<Vec<String>>,
    #[serde(rename = "UidMap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid_map: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// InspectLogConfig holds information about a container's configured log driver
pub struct InspectLogConfig {
    #[serde(rename = "Config")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<HashMap<String, String>>,
    #[serde(rename = "Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Path specifies a path to the log file
    pub path: Option<String>,
    #[serde(rename = "Size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Size specifies a maximum size of the container log
    pub size: Option<String>,
    #[serde(rename = "Tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Tag specifies a custom log tag for the container
    pub tag: Option<String>,
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// InspectMount provides a record of a single mount in a container. It contains
/// fields for both named and normal volumes. Only user-specified volumes will be
/// included, and tmpfs volumes are not included even if the user specified them.
pub struct InspectMount {
    #[serde(rename = "Destination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The destination directory for the volume. Specified as a path within
    /// the container, as it would be passed into the OCI runtime.
    pub destination: Option<String>,
    #[serde(rename = "Driver")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The driver used for the named volume. Empty for bind mounts.
    pub driver: Option<String>,
    #[serde(rename = "Mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Contains SELinux :z/:Z mount options. Unclear what, if anything, else
    /// goes in here.
    pub mode: Option<String>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The name of the volume. Empty for bind mounts.
    pub name: Option<String>,
    #[serde(rename = "Options")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// All remaining mount options. Additional data, not present in the
    /// original output.
    pub options: Option<Vec<String>>,
    #[serde(rename = "Propagation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Mount propagation for the mount. Can be empty if not specified, but
    /// is always printed - no omitempty.
    pub propagation: Option<String>,
    #[serde(rename = "RW")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether the volume is read-write
    pub rw: Option<bool>,
    #[serde(rename = "Source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The source directory for the volume.
    pub source: Option<String>,
    #[serde(rename = "SubPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// SubPath object from the volume. Specified as a path within
    /// the source volume to be mounted at the Destination.
    pub sub_path: Option<String>,
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether the mount is a volume or bind mount. Allowed values are
    /// "volume" and "bind".
    pub type_: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// InspectNetworkSettings holds information about the network settings of the
/// container.
/// Many fields are maintained only for compatibility with `docker inspect` and
/// are unused within Libpod.
pub struct InspectNetworkSettings {
    #[serde(rename = "AdditionalMACAddresses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// AdditionalMacAddresses is a set of additional MAC Addresses beyond
    /// the first. CNI may configure more than one interface for a single
    /// network, which can cause this.
    pub additional_mac_addresses: Option<Vec<String>>,
    #[serde(rename = "Bridge")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bridge: Option<String>,
    #[serde(rename = "EndpointID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// EndpointID is unused, maintained exclusively for compatibility.
    pub endpoint_id: Option<String>,
    #[serde(rename = "Gateway")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Gateway is the IP address of the gateway this network will use.
    pub gateway: Option<String>,
    #[serde(rename = "GlobalIPv6Address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// GlobalIPv6Address is the global-scope IPv6 Address for this network.
    pub global_i_pv_6_address: Option<String>,
    #[serde(rename = "GlobalIPv6PrefixLen")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// GlobalIPv6PrefixLen is the length of the subnet mask of this network.
    pub global_i_pv_6_prefix_len: Option<i64>,
    #[serde(rename = "HairpinMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hairpin_mode: Option<bool>,
    #[serde(rename = "IPAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// IPAddress is the IP address for this network.
    pub ip_address: Option<String>,
    #[serde(rename = "IPPrefixLen")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// IPPrefixLen is the length of the subnet mask of this network.
    pub ip_prefix_len: Option<i64>,
    #[serde(rename = "IPv6Gateway")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// IPv6Gateway is the IPv6 gateway this network will use.
    pub i_pv_6_gateway: Option<String>,
    #[serde(rename = "LinkLocalIPv6Address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_local_i_pv_6_address: Option<String>,
    #[serde(rename = "LinkLocalIPv6PrefixLen")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_local_i_pv_6_prefix_len: Option<i64>,
    #[serde(rename = "MacAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// MacAddress is the MAC address for the interface in this network.
    pub mac_address: Option<String>,
    #[serde(rename = "Networks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Networks contains information on non-default networks this
    /// container has joined.
    /// It is a map of network name to network information.
    pub networks: Option<HashMap<String, InspectAdditionalNetwork>>,
    #[serde(rename = "Ports")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ports: Option<HashMap<String, Option<Vec<InspectHostPort>>>>,
    #[serde(rename = "SandboxID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sandbox_id: Option<String>,
    #[serde(rename = "SandboxKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sandbox_key: Option<String>,
    #[serde(rename = "SecondaryIPAddresses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// SecondaryIPAddresses is a list of extra IP Addresses that the
    /// container has been assigned in this network.
    pub secondary_ip_addresses: Option<Vec<Address>>,
    #[serde(rename = "SecondaryIPv6Addresses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// SecondaryIPv6Addresses is a list of extra IPv6 Addresses that the
    /// container has been assigned in this network.
    pub secondary_i_pv_6_addresses: Option<Vec<Address>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InspectPodContainerInfo {
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// ID is the ID of the container.
    pub id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Name is the name of the container.
    pub name: Option<String>,
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// State is the current status of the container.
    pub state: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// InspectPodData contains detailed information on a pod's configuration and
/// state. It is used as the output of Inspect on pods.
pub struct InspectPodData {
    #[serde(rename = "CgroupParent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// CgroupParent is the parent of the pod's Cgroup.
    pub cgroup_parent: Option<String>,
    #[serde(rename = "CgroupPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// CgroupPath is the path to the pod's Cgroup.
    pub cgroup_path: Option<String>,
    #[serde(rename = "Containers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Containers gives a brief summary of all containers in the pod and
    /// their current status.
    pub containers: Option<Vec<InspectPodContainerInfo>>,
    #[serde(rename = "CreateCgroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// CreateCgroup is whether this pod will create its own Cgroup to group
    /// containers under.
    pub create_cgroup: Option<bool>,
    #[serde(rename = "CreateCommand")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// CreateCommand is the full command plus arguments of the process the
    /// container has been created with.
    pub create_command: Option<Vec<String>>,
    #[serde(rename = "CreateInfra")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// CreateInfra is whether this pod will create an infra container to
    /// share namespaces.
    pub create_infra: Option<bool>,
    #[serde(rename = "Created")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Created is the time when the pod was created.
    pub created: Option<DateTime<Utc>>,
    #[serde(rename = "ExitPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// ExitPolicy of the pod.
    pub exit_policy: Option<String>,
    #[serde(rename = "Hostname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Hostname is the hostname that the pod will set.
    pub hostname: Option<String>,
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// ID is the ID of the pod.
    pub id: Option<String>,
    #[serde(rename = "InfraConfig")]
    pub infra_config: Option<InspectPodInfraConfig>,
    #[serde(rename = "InfraContainerID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// InfraContainerID is the ID of the pod's infra container, if one is
    /// present.
    pub infra_container_id: Option<String>,
    #[serde(rename = "Labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Labels is a set of key-value labels that have been applied to the
    /// pod.
    pub labels: Option<HashMap<String, String>>,
    #[serde(rename = "LockNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Number of the pod's Libpod lock.
    pub lock_number: Option<u32>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Name is the name of the pod.
    pub name: Option<String>,
    #[serde(rename = "Namespace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Namespace is the Libpod namespace the pod is placed in.
    pub namespace: Option<String>,
    #[serde(rename = "NumContainers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// NumContainers is the number of containers in the pod, including the
    /// infra container.
    pub num_containers: Option<u64>,
    #[serde(rename = "RestartPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// RestartPolicy of the pod.
    pub restart_policy: Option<String>,
    #[serde(rename = "SharedNamespaces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// SharedNamespaces contains a list of namespaces that will be shared by
    /// containers within the pod. Can only be set if CreateInfra is true.
    pub shared_namespaces: Option<Vec<String>>,
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// State represents the current state of the pod.
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// BlkioWeight contains the blkio weight limit for the pod
    pub blkio_weight: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// BlkioWeightDevice contains the blkio weight device limits for the pod
    pub blkio_weight_device: Option<Vec<InspectBlkioWeightDevice>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// CPUPeriod contains the CPU period of the pod
    pub cpu_period: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// CPUQuota contains the CPU quota of the pod
    pub cpu_quota: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// CPUShares contains the cpu shares for the pod
    pub cpu_shares: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// CPUSetCPUs contains linux specific CPU data for the pod
    pub cpuset_cpus: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// CPUSetMems contains linux specific CPU data for the pod
    pub cpuset_mems: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// BlkioDeviceReadBps contains the Read/Access limit for the pod's devices
    pub device_read_bps: Option<Vec<InspectBlkioThrottleDevice>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// BlkioDeviceReadBps contains the Read/Access limit for the pod's devices
    pub device_write_bps: Option<Vec<InspectBlkioThrottleDevice>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Devices contains the specified host devices
    pub devices: Option<Vec<InspectDevice>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// MemoryLimit contains the specified cgroup memory limit for the pod
    pub memory_limit: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// MemorySwap contains the specified memory swap limit for the pod
    pub memory_swap: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Mounts contains volume related information for the pod
    pub mounts: Option<Vec<InspectMount>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// SecurityOpt contains the specified security labels and related SELinux information
    pub security_opt: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// VolumesFrom contains the containers that the pod inherits mounts from
    pub volumes_from: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// InspectPodInfraConfig contains the configuration of the pod's infra
/// container.
pub struct InspectPodInfraConfig {
    #[serde(rename = "DNSOption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// DNSOption is a set of DNS options that will be used by the infra
    /// container's resolv.conf and shared with the remainder of the pod.
    pub dns_option: Option<Vec<String>>,
    #[serde(rename = "DNSSearch")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// DNSSearch is a set of DNS search domains that will be used by the
    /// infra container's resolv.conf and shared with the remainder of the
    /// pod.
    pub dns_search: Option<Vec<String>>,
    #[serde(rename = "DNSServer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// DNSServer is a set of DNS Servers that will be used by the infra
    /// container's resolv.conf and shared with the remainder of the pod.
    pub dns_server: Option<Vec<String>>,
    #[serde(rename = "HostAdd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// HostAdd adds a number of hosts to the infra container's resolv.conf
    /// which will be shared with the rest of the pod.
    pub host_add: Option<Vec<String>>,
    #[serde(rename = "HostNetwork")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// HostNetwork is whether the infra container (and thus the whole pod)
    /// will use the host's network and not create a network namespace.
    pub host_network: Option<bool>,
    #[serde(rename = "HostsFile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// HostsFile is the base file to create the `/etc/hosts` file inside the infra container
    /// which will be shared with the rest of the pod.
    pub hosts_file: Option<String>,
    #[serde(rename = "NetworkOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// NetworkOptions are additional options for each network
    pub network_options: Option<HashMap<String, Vec<String>>>,
    #[serde(rename = "Networks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Networks is a list of networks the pod will join.
    pub networks: Option<Vec<String>>,
    #[serde(rename = "NoManageHostname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// NoManageHostname indicates that the pod will not manage /etc/hostname
    /// and instead each container will handle their own.
    pub no_manage_hostname: Option<bool>,
    #[serde(rename = "NoManageHosts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// NoManageHosts indicates that the pod will not manage /etc/hosts and
    /// instead each container will handle their own.
    pub no_manage_hosts: Option<bool>,
    #[serde(rename = "NoManageResolvConf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// NoManageResolvConf indicates that the pod will not manage resolv.conf
    /// and instead each container will handle their own.
    pub no_manage_resolv_conf: Option<bool>,
    #[serde(rename = "PortBindings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// PortBindings are ports that will be forwarded to the infra container
    /// and then shared with the pod.
    pub port_bindings: Option<HashMap<String, Option<Vec<InspectHostPort>>>>,
    #[serde(rename = "StaticIP")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// StaticIP is a static IPv4 that will be assigned to the infra
    /// container and then used by the pod.
    pub static_ip: Option<String>,
    #[serde(rename = "StaticMAC")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// StaticMAC is a static MAC address that will be assigned to the infra
    /// container and then used by the pod.
    pub static_mac: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// CPUPeriod contains the CPU period of the pod
    pub cpu_period: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// CPUQuota contains the CPU quota of the pod
    pub cpu_quota: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// CPUSetCPUs contains linux specific CPU data for the container
    pub cpuset_cpus: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Pid is the PID namespace mode of the pod's infra container
    pub pid_ns: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// UserNS is the usernamespace that all the containers in the pod will join.
    pub userns: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// UtsNS is the uts namespace that all containers in the pod will join
    pub uts_ns: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InspectRestartPolicy {
    #[serde(rename = "MaximumRetryCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// MaximumRetryCount is the maximum number of retries allowed if the
    /// "on-failure" restart policy is in use. Not used if "on-failure" is
    /// not set.
    pub maximum_retry_count: Option<u64>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Name contains the container's restart policy.
    /// Allowable values are "no" or "" (take no action),
    /// "on-failure" (restart on non-zero exit code, with an optional max
    /// retry count), and "always" (always restart on container stop, unless
    /// explicitly requested by API).
    /// Note that this is NOT actually a name of any sort - the poor naming
    /// is for Docker compatibility.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// InspectSecret contains information on secrets mounted inside the container
pub struct InspectSecret {
    #[serde(rename = "GID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// ID is the GID of the mounted secret file
    pub gid: Option<u32>,
    #[serde(rename = "ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// ID is the ID of the secret
    pub id: Option<String>,
    #[serde(rename = "Mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// ID is the ID of the mode of the mounted secret file
    pub mode: Option<u32>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Name is the name of the secret
    pub name: Option<String>,
    #[serde(rename = "UID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// ID is the UID of the mounted secret file
    pub uid: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InspectUlimit {
    #[serde(rename = "Hard")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Hard is the hard limit that will be applied.
    pub hard: Option<i64>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Name is the name (type) of the ulimit.
    pub name: Option<String>,
    #[serde(rename = "Soft")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Soft is the soft limit that will be applied.
    pub soft: Option<i64>,
}

pub type IpcMode = String;

/// Isolation represents the isolation technology of a container. The supported
/// values are platform specific
pub type Isolation = String;

/// Kubernetes YAML file successfully deployed to cluster
pub type KubeApplyLibpod200Response = Vec<u8>;

pub type KubeApplyLibpodRequestParam = String;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LeaseRange {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// EndIP last IP in the subnet which should be used to assign ips.
    pub end_ip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// StartIP first IP in the subnet which should be used to assign ips.
    pub start_ip: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LibpodContainersRmReport {
    #[serde(rename = "Err")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Error which occurred during Rm operation (if any).
    /// This field is optional and may be omitted if no error occurred.
    pub err: Option<String>,
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LibpodImageSummary {
    #[serde(rename = "Arch")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Podman extensions
    pub arch: Option<String>,
    #[serde(rename = "Containers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub containers: Option<i64>,
    #[serde(rename = "Created")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<i64>,
    #[serde(rename = "Dangling")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dangling: Option<bool>,
    #[serde(rename = "Digest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
    #[serde(rename = "History")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub history: Option<Vec<String>>,
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "IsManifestList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// IsManifestList is a ptr so we can distinguish between a true
    /// json empty response and false.  the docker compat side needs to return
    /// empty; where as the libpod side needs a value of true or false
    pub is_manifest_list: Option<bool>,
    #[serde(rename = "Labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<HashMap<String, String>>,
    #[serde(rename = "Names")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<Vec<String>>,
    #[serde(rename = "Os")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,
    #[serde(rename = "ParentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    #[serde(rename = "ReadOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    #[serde(rename = "RepoDigests")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_digests: Option<Vec<String>>,
    #[serde(rename = "RepoTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_tags: Option<Vec<String>>,
    #[serde(rename = "SharedSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_size: Option<i64>,
    #[serde(rename = "Size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(rename = "VirtualSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_size: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LibpodImagesPullReport {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Error contains text of errors from c/image
    pub error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// ID contains image id (retained for backwards compatibility)
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Images contains the ID's of the images pulled
    pub images: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Stream used to provide output from c/image
    pub stream: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// LibpodImagesRemoveReport is the return type for image removal via the rest
/// api.
pub struct LibpodImagesRemoveReport {
    #[serde(rename = "Deleted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Deleted images.
    pub deleted: Option<Vec<String>>,
    #[serde(rename = "Errors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Image removal requires is to return data and an error.
    pub errors: Option<Vec<String>>,
    #[serde(rename = "ExitCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// ExitCode describes the exit codes as described in the `podman rmi`
    /// man page.
    pub exit_code: Option<i64>,
    #[serde(rename = "Untagged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Untagged images. Can be longer than Deleted.
    pub untagged: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Info is the overall struct that describes the host system
/// running libpod/podman
pub struct LibpodInfo {
    pub host: Option<HostInfo>,
    pub plugins: Option<Plugins>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registries: Option<Value>,
    pub store: Option<StoreInfo>,
    pub version: Option<Version>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// LinuxBlockIO for Linux cgroup 'blkio' resource management
pub struct LinuxBlockIo {
    #[serde(rename = "leafWeight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Specifies tasks' weight in the given cgroup while competing with the cgroup's child cgroups, CFQ scheduler only
    pub leaf_weight: Option<u16>,
    #[serde(rename = "throttleReadBpsDevice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// IO read rate limit per cgroup per device, bytes per second
    pub throttle_read_bps_device: Option<Vec<LinuxThrottleDevice>>,
    #[serde(rename = "throttleReadIOPSDevice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// IO read rate limit per cgroup per device, IO per second
    pub throttle_read_iops_device: Option<Vec<LinuxThrottleDevice>>,
    #[serde(rename = "throttleWriteBpsDevice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// IO write rate limit per cgroup per device, bytes per second
    pub throttle_write_bps_device: Option<Vec<LinuxThrottleDevice>>,
    #[serde(rename = "throttleWriteIOPSDevice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// IO write rate limit per cgroup per device, IO per second
    pub throttle_write_iops_device: Option<Vec<LinuxThrottleDevice>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Specifies per cgroup weight
    pub weight: Option<u16>,
    #[serde(rename = "weightDevice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Weight per cgroup per device, can override BlkioWeight
    pub weight_device: Option<Vec<LinuxWeightDevice>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// LinuxBlockIODevice holds major:minor format supported in blkio cgroup
pub struct LinuxBlockIoDevice {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Major is the device's major number.
    pub major: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Minor is the device's minor number.
    pub minor: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// LinuxCPU for Linux cgroup 'cpu' resource management
pub struct LinuxCpu {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// CPU hardcap burst limit (in usecs). Allowed accumulated cpu time additionally for burst in a
    /// given period.
    pub burst: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// CPUs to use within the cpuset. Default is to use any CPU available.
    pub cpus: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// cgroups are configured with minimum weight, 0: default behavior, 1: SCHED_IDLE.
    pub idle: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// List of memory nodes in the cpuset. Default is to use any available memory node.
    pub mems: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// CPU period to be used for hardcapping (in usecs).
    pub period: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// CPU hardcap limit (in usecs). Allowed cpu time in a given period.
    pub quota: Option<i64>,
    #[serde(rename = "realtimePeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// CPU period to be used for realtime scheduling (in usecs).
    pub realtime_period: Option<u64>,
    #[serde(rename = "realtimeRuntime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// How much time realtime scheduling may use (in usecs).
    pub realtime_runtime: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// CPU shares (relative weight (ratio) vs. other cgroups with cpu shares).
    pub shares: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// LinuxDevice represents the mknod information for a Linux special device file
pub struct LinuxDevice {
    #[serde(rename = "fileMode")]
    pub file_mode: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Gid of the device.
    pub gid: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Major is the device's major number.
    pub major: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Minor is the device's minor number.
    pub minor: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Path to the device.
    pub path: Option<String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Device type, block, char, etc.
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// UID of the device.
    pub uid: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// LinuxDeviceCgroup represents a device rule for the devices specified to
/// the device controller
pub struct LinuxDeviceCgroup {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Cgroup access permissions format, rwm.
    pub access: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Allow or deny
    pub allow: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Major is the device's major number.
    pub major: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Minor is the device's minor number.
    pub minor: Option<i64>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Device type, block, char, etc.
    pub type_: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Default to reservation limits if supported. Otherwise fallback to page fault limits.
pub struct LinuxHugepageLimit {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Limit is the limit of "hugepagesize" hugetlb reservations (if supported) or usage.
    pub limit: Option<u64>,
    #[serde(rename = "pageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Pagesize is the hugepage size.
    /// Format: "<size><unit-prefix>B' (e.g. 64KB, 2MB, 1GB, etc.).
    pub page_size: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// LinuxIDMapping specifies UID/GID mappings
pub struct LinuxIdMapping {
    #[serde(rename = "containerID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// ContainerID is the starting UID/GID in the container
    pub container_id: Option<u32>,
    #[serde(rename = "hostID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// HostID is the starting UID/GID on the host to be mapped to 'ContainerID'
    pub host_id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Size is the number of IDs to be mapped
    pub size: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// LinuxIntelRdt has container runtime resource constraints for Intel RDT CAT and MBA
/// features and flags enabling Intel RDT CMT and MBM features.
/// Intel RDT features are available in Linux 4.14 and newer kernel versions.
pub struct LinuxIntelRdt {
    #[serde(rename = "closID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The identity for RDT Class of Service
    pub clos_id: Option<String>,
    #[serde(rename = "enableCMT")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// EnableCMT is the flag to indicate if the Intel RDT CMT is enabled. CMT (Cache Monitoring Technology) supports monitoring of
    /// the last-level cache (LLC) occupancy for the container.
    pub enable_cmt: Option<bool>,
    #[serde(rename = "enableMBM")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// EnableMBM is the flag to indicate if the Intel RDT MBM is enabled. MBM (Memory Bandwidth Monitoring) supports monitoring of
    /// total and local memory bandwidth for the container.
    pub enable_mbm: Option<bool>,
    #[serde(rename = "l3CacheSchema")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The schema for L3 cache id and capacity bitmask (CBM)
    /// Format: "L3:<cache_id0>=<cbm0>;<cache_id1>=<cbm1>;..."
    pub l_3_cache_schema: Option<String>,
    #[serde(rename = "memBwSchema")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The schema of memory bandwidth per L3 cache id
    /// Format: "MB:<cache_id0>=bandwidth0;<cache_id1>=bandwidth1;..."
    /// The unit of memory bandwidth is specified in "percentages" by
    /// default, and in "MBps" if MBA Software Controller is enabled.
    pub mem_bw_schema: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// LinuxInterfacePriority for network interfaces
pub struct LinuxInterfacePriority {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Name is the name of the network interface
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Priority for the interface
    pub priority: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// LinuxMemory for Linux cgroup 'memory' resource management
pub struct LinuxMemory {
    #[serde(rename = "checkBeforeUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// CheckBeforeUpdate enables checking if a new memory limit is lower
    /// than the current usage during update, and if so, rejecting the new
    /// limit.
    pub check_before_update: Option<bool>,
    #[serde(rename = "disableOOMKiller")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// DisableOOMKiller disables the OOM killer for out of memory conditions
    pub disable_oom_killer: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Kernel memory limit (in bytes).
    ///
    /// Deprecated: kernel-memory limits are not supported in cgroups v2, and
    /// were obsoleted in [kernel v5.4]. This field should no longer be used,
    /// as it may be ignored by runtimes.
    ///
    /// [kernel v5.4]: https://github.com/torvalds/linux/commit/0158115f702b0ba208ab0
    pub kernel: Option<i64>,
    #[serde(rename = "kernelTCP")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Kernel memory limit for tcp (in bytes)
    pub kernel_tcp: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Memory limit (in bytes).
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Memory reservation or soft_limit (in bytes).
    pub reservation: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Total memory limit (memory + swap).
    pub swap: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// How aggressive the kernel will swap memory pages.
    pub swappiness: Option<u64>,
    #[serde(rename = "useHierarchy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Enables hierarchical memory accounting
    pub use_hierarchy: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// LinuxNetwork identification and priority configuration
pub struct LinuxNetwork {
    #[serde(rename = "classID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Set class identifier for container's network packets
    pub class_id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Set priority of network traffic for container
    pub priorities: Option<Vec<LinuxInterfacePriority>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// LinuxPersonality represents the Linux personality syscall input
pub struct LinuxPersonality {
    pub domain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Additional flags
    pub flags: Option<Vec<String>>,
}

pub type LinuxPersonalityDomain = String;

pub type LinuxPersonalityFlag = String;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// LinuxPids for Linux cgroup 'pids' resource management (Linux 4.3)
pub struct LinuxPids {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Maximum number of PIDs. Default is "no limit".
    pub limit: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// LinuxRdma for Linux cgroup 'rdma' resource management (Linux 4.11)
pub struct LinuxRdma {
    #[serde(rename = "hcaHandles")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Maximum number of HCA handles that can be opened. Default is "no limit".
    pub hca_handles: Option<u32>,
    #[serde(rename = "hcaObjects")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Maximum number of HCA objects that can be created. Default is "no limit".
    pub hca_objects: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// LinuxResources has container runtime resource constraints
pub struct LinuxResources {
    #[serde(rename = "blockIO")]
    pub block_io: Option<LinuxBlockIo>,
    pub cpu: Option<LinuxCpu>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Devices configures the device allowlist.
    pub devices: Option<Vec<LinuxDeviceCgroup>>,
    #[serde(rename = "hugepageLimits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Hugetlb limits (in bytes). Default to reservation limits if supported.
    pub hugepage_limits: Option<Vec<LinuxHugepageLimit>>,
    pub memory: Option<LinuxMemory>,
    pub network: Option<LinuxNetwork>,
    pub pids: Option<LinuxPids>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Rdma resource restriction configuration.
    /// Limits are a set of key value pairs that define RDMA resource limits,
    /// where the key is device name and value is resource limits.
    pub rdma: Option<HashMap<String, LinuxRdma>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Unified resources.
    pub unified: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// LinuxThrottleDevice struct holds a `major:minor rate_per_second` pair
pub struct LinuxThrottleDevice {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Major is the device's major number.
    pub major: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Minor is the device's minor number.
    pub minor: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Rate is the IO rate limit per cgroup per device
    pub rate: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// LinuxWeightDevice struct holds a `major:minor weight` pair for weightDevice
pub struct LinuxWeightDevice {
    #[serde(rename = "leafWeight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// LeafWeight is the bandwidth rate for the device while competing with the cgroup's child cgroups, CFQ scheduler only
    pub leaf_weight: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Major is the device's major number.
    pub major: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Minor is the device's minor number.
    pub minor: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Weight is the bandwidth rate for the device.
    pub weight: Option<u16>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// ListContainer describes a container suitable for listing
pub struct ListContainer {
    #[serde(rename = "AutoRemove")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// AutoRemove
    pub auto_remove: Option<bool>,
    #[serde(rename = "CIDFile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// CIDFile specified at creation time.
    pub cid_file: Option<String>,
    #[serde(rename = "Command")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Container command
    pub command: Option<Vec<String>>,
    #[serde(rename = "Created")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Container creation time
    pub created: Option<DateTime<Utc>>,
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Human-readable container creation time.
    pub created_at: Option<String>,
    #[serde(rename = "ExitCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// If container has exited, the return code from the command
    pub exit_code: Option<i32>,
    #[serde(rename = "Exited")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// If container has exited/stopped
    pub exited: Option<bool>,
    #[serde(rename = "ExitedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Time container exited
    pub exited_at: Option<i64>,
    #[serde(rename = "ExposedPorts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// ExposedPorts contains the ports that are exposed but not forwarded,
    /// see Ports for forwarded ports.
    /// The key is the port number and the string slice contains the protocols,
    /// i.e. "tcp", "udp" and "sctp".
    pub exposed_ports: Option<Value>,
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The unique identifier for the container
    pub id: Option<String>,
    #[serde(rename = "Image")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Container image
    pub image: Option<String>,
    #[serde(rename = "ImageID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Container image ID
    pub image_id: Option<String>,
    #[serde(rename = "IsInfra")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// If this container is a Pod infra container
    pub is_infra: Option<bool>,
    #[serde(rename = "Labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Labels for container
    pub labels: Option<HashMap<String, String>>,
    #[serde(rename = "Mounts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// User volume mounts
    pub mounts: Option<Vec<String>>,
    #[serde(rename = "Names")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The names assigned to the container
    pub names: Option<Vec<String>>,
    #[serde(rename = "Namespaces")]
    pub namespaces: Option<ListContainerNamespaces>,
    #[serde(rename = "Networks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The network names assigned to the container
    pub networks: Option<Vec<String>>,
    #[serde(rename = "Pid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The process id of the container
    pub pid: Option<i64>,
    #[serde(rename = "Pod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// If the container is part of Pod, the Pod ID. Requires the pod
    /// boolean to be set
    pub pod: Option<String>,
    #[serde(rename = "PodName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// If the container is part of Pod, the Pod name. Requires the pod
    /// boolean to be set
    pub pod_name: Option<String>,
    #[serde(rename = "Ports")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Port mappings
    pub ports: Option<Vec<PortMapping>>,
    #[serde(rename = "Restarts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Restarts is how many times the container was restarted by its
    /// restart policy. This is NOT incremented by normal container restarts
    /// (only by restart policy).
    pub restarts: Option<u64>,
    #[serde(rename = "Size")]
    pub size: Option<ContainerSize>,
    #[serde(rename = "StartedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Time when container started
    pub started_at: Option<i64>,
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// State of container
    pub state: Option<String>,
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Status is a human-readable approximation of a duration for json output
    pub status: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// ListContainerNamespaces contains the identifiers of the container's Linux namespaces
pub struct ListContainerNamespaces {
    #[serde(rename = "Cgroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Cgroup namespace
    pub cgroup: Option<String>,
    #[serde(rename = "Ipc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// IPC namespace
    pub ipc: Option<String>,
    #[serde(rename = "Mnt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Mount namespace
    pub mnt: Option<String>,
    #[serde(rename = "Net")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Network namespace
    pub net: Option<String>,
    #[serde(rename = "Pidns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// PID namespace
    pub pidns: Option<String>,
    #[serde(rename = "User")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// User namespace
    pub user: Option<String>,
    #[serde(rename = "Uts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// UTS namespace
    pub uts: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListPodContainer {
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Names")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<String>,
    #[serde(rename = "RestartCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restart_count: Option<u64>,
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListPodsReport {
    #[serde(rename = "Cgroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cgroup: Option<String>,
    #[serde(rename = "Containers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub containers: Option<Vec<ListPodContainer>>,
    #[serde(rename = "Created")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<DateTime<Utc>>,
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "InfraId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infra_id: Option<String>,
    #[serde(rename = "Labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<HashMap<String, String>>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Namespace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(rename = "Networks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Network names connected to infra container
    pub networks: Option<Vec<String>>,
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Volume list response
pub struct ListResponse {
    #[serde(rename = "Volumes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// List of volumes
    pub volumes: Option<Vec<Volume>>,
    #[serde(rename = "Warnings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Warnings that occurred when fetching the list of volumes.
    pub warnings: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LogConfig {
    #[serde(rename = "Config")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<HashMap<String, String>>,
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// LogConfig describes the logging characteristics for a container
pub struct LogConfigLibpod {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// LogDriver is the container's log driver.
    /// Optional.
    pub driver: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A set of options to accompany the log driver.
    /// Optional.
    pub options: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// LogPath is the path the container's logs will be stored at.
    /// Only available if LogDriver is set to "json-file" or "k8s-file".
    /// Optional.
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Size is the maximum size of the log file
    /// Optional.
    pub size: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// ManifestAddArtifactOptions provides the model for creating artifact manifests
/// for files and adding those manifests to a manifest list
pub struct ManifestAddArtifactOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Annotation to add to the item in the manifest list
    pub annotation: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Annotations to add to the item in the manifest list by a map which is preferred over Annotation
    pub annotations: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Arch overrides the architecture for the item in the manifest list
    pub arch: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact_annotations: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact_config: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact_config_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact_exclude_titles: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact_files: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact_layer_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact_subject: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Note to future maintainers: keep these fields synchronized with ManifestModifyOptions!
    pub artifact_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Feature list for the item in the manifest list
    pub features: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// IndexAnnotation is a slice of key=value annotations to add to the manifest list itself
    pub index_annotation: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// IndexAnnotations is a map of key:value annotations to add to the manifest list itself, by a map which is preferred over IndexAnnotation
    pub index_annotations: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// OS overrides the operating system for the item in the manifest list
    pub os: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// OS features for the item in the manifest list
    pub os_features: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// OSVersion overrides the operating system for the item in the manifest list
    pub os_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// IndexSubject is a subject value to set in the manifest list itself
    pub subject: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Variant for the item in the manifest list
    pub variant: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// ManifestAddOptions provides model for adding digests to manifest list
pub struct ManifestAddOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// True when operating on a list to include all images
    pub all: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Annotation to add to the item in the manifest list
    pub annotation: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Annotations to add to the item in the manifest list by a map which is preferred over Annotation
    pub annotations: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Arch overrides the architecture for the item in the manifest list
    pub arch: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Feature list for the item in the manifest list
    pub features: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Images is an optional list of image references to add to manifest list
    pub images: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// IndexAnnotation is a slice of key=value annotations to add to the manifest list itself
    pub index_annotation: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// IndexAnnotations is a map of key:value annotations to add to the manifest list itself, by a map which is preferred over IndexAnnotation
    pub index_annotations: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// OS overrides the operating system for the item in the manifest list
    pub os: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// OS features for the item in the manifest list
    pub os_features: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// OSVersion overrides the operating system for the item in the manifest list
    pub os_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// IndexSubject is a subject value to set in the manifest list itself
    pub subject: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Variant for the item in the manifest list
    pub variant: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// ManifestAnnotateOptions provides model for annotating manifest list
pub struct ManifestAnnotateOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Annotation to add to the item in the manifest list
    pub annotation: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Annotations to add to the item in the manifest list by a map which is preferred over Annotation
    pub annotations: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Arch overrides the architecture for the item in the manifest list
    pub arch: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Feature list for the item in the manifest list
    pub features: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// IndexAnnotation is a slice of key=value annotations to add to the manifest list itself
    pub index_annotation: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// IndexAnnotations is a map of key:value annotations to add to the manifest list itself, by a map which is preferred over IndexAnnotation
    pub index_annotations: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// OS overrides the operating system for the item in the manifest list
    pub os: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// OS features for the item in the manifest list
    pub os_features: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// OSVersion overrides the operating system for the item in the manifest list
    pub os_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// IndexSubject is a subject value to set in the manifest list itself
    pub subject: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Variant for the item in the manifest list
    pub variant: Option<String>,
}

pub type ManifestKind = String;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// swagger 2.0 does not support oneOf for schema validation.
///
/// Operation "update" uses all fields.
/// Operation "remove" uses fields: Operation and Images
/// Operation "annotate" uses fields: Operation and Annotations
pub struct ManifestModifyOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// True when operating on a list to include all images
    pub all: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Annotation to add to the item in the manifest list
    pub annotation: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Annotations to add to the item in the manifest list by a map which is preferred over Annotation
    pub annotations: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Arch overrides the architecture for the item in the manifest list
    pub arch: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact_annotations: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact_config: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact_config_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact_exclude_titles: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact_files: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact_layer_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact_subject: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The following are all of the fields from ManifestAddArtifactOptions.
    /// We can't just embed the whole structure because it embeds a
    /// ManifestAnnotateOptions, which would conflict with the one that
    /// ManifestAddOptions embeds.
    pub artifact_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Feature list for the item in the manifest list
    pub features: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Images is an optional list of image references to add to manifest list
    pub images: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// IndexAnnotation is a slice of key=value annotations to add to the manifest list itself
    pub index_annotation: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// IndexAnnotations is a map of key:value annotations to add to the manifest list itself, by a map which is preferred over IndexAnnotation
    pub index_annotations: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// OS overrides the operating system for the item in the manifest list
    pub os: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// OS features for the item in the manifest list
    pub os_features: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// OSVersion overrides the operating system for the item in the manifest list
    pub os_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// IndexSubject is a subject value to set in the manifest list itself
    pub subject: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Variant for the item in the manifest list
    pub variant: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ManifestModifyReport {
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Manifest List ID
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Errors associated with operation
    pub errors: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Files added to manifest list, otherwise not provided.
    pub files: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Images added to or removed from manifest list, otherwise not provided.
    pub images: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ManifestPushReport {
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// ID of the pushed manifest
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Error contains text of errors from pushing
    pub error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Stream used to provide push progress
    pub stream: Option<String>,
}

/// ManifestRemoveOptions provides the model for removing digests from a manifest
pub type ManifestRemoveOptions = Value;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ManifestRemoveReport {
    #[serde(rename = "Deleted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Deleted manifest list.
    pub deleted: Option<Vec<String>>,
    #[serde(rename = "Errors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Errors associated with operation
    pub errors: Option<Vec<String>>,
    #[serde(rename = "ExitCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// ExitCode describes the exit codes as described in the `podman rmi`
    /// man page.
    pub exit_code: Option<i64>,
    #[serde(rename = "Untagged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Untagged images. Can be longer than Deleted.
    pub untagged: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ManifestSummary {
    #[serde(rename = "AttestationData")]
    pub attestation_data: Option<AttestationProperties>,
    #[serde(rename = "Available")]
    /// Indicates whether all the child content (image config, layers) is
    /// fully available locally
    pub available: bool,
    #[serde(rename = "Descriptor")]
    pub descriptor: Descriptor,
    #[serde(rename = "ID")]
    /// ID is the content-addressable ID of an image and is the same as the
    /// digest of the image manifest.
    pub id: String,
    #[serde(rename = "ImageData")]
    pub image_data: Option<ImageProperties>,
    #[serde(rename = "Kind")]
    pub kind: String,
    #[serde(rename = "Size")]
    /// Size is the size information of the content related to this manifest.
    /// Note: These sizes only take the locally available content into account.
    pub size: ManifestSummarySizeInlineItem,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Size is the size information of the content related to this manifest.
/// Note: These sizes only take the locally available content into account.
pub struct ManifestSummarySizeInlineItem {
    #[serde(rename = "Content")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Content is the size (in bytes) of all the locally present
    /// content in the content store (e.g. image config, layers)
    /// referenced by this manifest and its children.
    /// This only includes blobs in the content store.
    pub content: Option<i64>,
    #[serde(rename = "Total")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Total is the total size (in bytes) of all the locally present
    /// data (both distributable and non-distributable) that's related to
    /// this manifest and its children.
    /// This equal to the sum of [Content] size AND all the sizes in the
    /// [Size] struct present in the Kind-specific data struct.
    /// For example, for an image kind (Kind == ManifestKindImage),
    /// this would include the size of the image content and unpacked
    /// image snapshots ([Size.Content] + [ImageData.Size.Unpacked]).
    pub total: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Meta {
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "UpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
    #[serde(rename = "Version")]
    pub version: Option<Version>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Metadata {
    #[serde(rename = "LastTagTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// LastTagTime is the date and time at which the image was last tagged.
    pub last_tag_time: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Mount {
    #[serde(rename = "BindOptions")]
    pub bind_options: Option<BindOptions>,
    #[serde(rename = "ClusterOptions")]
    pub cluster_options: Option<ClusterOptions>,
    #[serde(rename = "Consistency")]
    pub consistency: Option<String>,
    #[serde(rename = "ReadOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    #[serde(rename = "Source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Source specifies the name of the mount. Depending on mount type, this
    /// may be a volume name or a host path, or even ignored.
    /// Source is not supported for tmpfs (must be an empty value)
    pub source: Option<String>,
    #[serde(rename = "Target")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(rename = "TmpfsOptions")]
    pub tmpfs_options: Option<TmpfsOptions>,
    #[serde(rename = "Type")]
    pub type_: Option<String>,
    #[serde(rename = "VolumeOptions")]
    pub volume_options: Option<VolumeOptions>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// This is used for reporting the mountpoints in use by a container.
pub struct MountPoint {
    #[serde(rename = "Destination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Destination is the path relative to the container root (`/`) where the
    /// Source is mounted inside the container.
    pub destination: Option<String>,
    #[serde(rename = "Driver")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Driver is the volume driver used to create the volume (if it is a volume).
    pub driver: Option<String>,
    #[serde(rename = "Mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Mode is a comma separated list of options supplied by the user when
    /// creating the bind/volume mount.
    ///
    /// The default is platform-specific (`"z"` on Linux, empty on Windows).
    pub mode: Option<String>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Name is the name reference to the underlying data defined by `Source`
    /// e.g., the volume name.
    pub name: Option<String>,
    #[serde(rename = "Propagation")]
    pub propagation: Option<String>,
    #[serde(rename = "RW")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// RW indicates whether the mount is mounted writable (read-write).
    pub rw: Option<bool>,
    #[serde(rename = "Source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Source is the source location of the mount.
    ///
    /// For volumes, this contains the storage location of the volume (within
    /// `/var/lib/docker/volumes/`). For bind-mounts, and `npipe`, this contains
    /// the source (host) part of the bind-mount. For `tmpfs` mount points, this
    /// field is empty.
    pub source: Option<String>,
    #[serde(rename = "Type")]
    pub type_: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// NamedVolume holds information about a named volume that will be mounted into
/// the container.
pub struct NamedVolume {
    #[serde(rename = "Dest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Destination to mount the named volume within the container. Must be
    /// an absolute path. Path will be created if it does not exist.
    pub dest: Option<String>,
    #[serde(rename = "IsAnonymous")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// IsAnonymous sets the named volume as anonymous even if it has a name
    /// This is used for emptyDir volumes from a kube yaml
    pub is_anonymous: Option<bool>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Name is the name of the named volume to be mounted. May be empty.
    /// If empty, a new named volume with a pseudorandomly generated name
    /// will be mounted at the given destination.
    pub name: Option<String>,
    #[serde(rename = "Options")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Options are options that the named volume will be mounted with.
    pub options: Option<Vec<String>>,
    #[serde(rename = "SubPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// SubPath stores the sub directory of the named volume to be mounted in the container
    pub sub_path: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Namespace describes the namespace
pub struct Namespace {
    pub nsmode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

pub type NamespaceMode = String;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NetAddress {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Gateway for the network. This can be empty if there is no gateway, e.g. internal network.
    pub gateway: Option<String>,
    pub ipnet: Option<IpNet>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NetInterface {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// MacAddress for this Interface.
    pub mac_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Subnets list of assigned subnets with their gateway.
    pub subnets: Option<Vec<NetAddress>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// NetOptions reflect the shared network options between
/// pods and containers
pub struct NetOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_option: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_search: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_server: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostadd: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosts_file: Option<String>,
    pub netns: Option<Namespace>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_alias: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// NetworkOptions are additional options for each network
    pub network_options: Option<HashMap<String, Vec<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub networks: Option<HashMap<String, PerNetworkOptions>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_manage_hostname: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_manage_hosts: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_manage_resolv_conf: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portmappings: Option<Vec<PortMapping>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Network {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Created contains the timestamp when this network was created.
    pub created: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// DNSEnabled is whether name resolution is active for container on
    /// this Network. Only supported with the bridge driver.
    pub dns_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Driver for this Network, e.g. bridge, macvlan...
    pub driver: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// ID of the Network.
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Internal is whether the Network should not have external routes
    /// to public or other Networks.
    pub internal: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// IPAMOptions contains options used for the ip assignment.
    pub ipam_options: Option<HashMap<String, String>>,
    #[serde(rename = "ipv6_enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// IPv6Enabled if set to true an ipv6 subnet should be created for this net.
    pub ipv_6_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Labels is a set of key-value labels that have been applied to the
    /// Network.
    pub labels: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Name of the Network.
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// List of custom DNS server for podman's DNS resolver at network level,
    /// all the containers attached to this network will consider resolvers
    /// configured at network level.
    pub network_dns_servers: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// NetworkInterface is the network interface name on the host.
    pub network_interface: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Options is a set of key-value options that have been applied to
    /// the Network.
    pub options: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Routes to use for this network.
    pub routes: Option<Vec<Route>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Subnets to use for this network.
    pub subnets: Option<Vec<Subnet>>,
}

pub type NetworkBackend = String;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// NetworkConnectOptions describes options for connecting
/// a container to a network
pub struct NetworkConnectOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Aliases contains a list of names which the dns server should resolve
    /// to this container. Should only be set when DNSEnabled is true on the Network.
    /// If aliases are set but there is no dns support for this network the
    /// network interface implementation should ignore this and NOT error.
    /// Optional.
    pub aliases: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// InterfaceName for this container. Required in the backend.
    /// Optional in the frontend. Will be filled with ethX (where X is a integer) when empty.
    pub interface_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Driver-specific options for this container.
    pub options: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// StaticIPs for this container. Optional.
    pub static_ips: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// StaticMac for this container. Optional.
    pub static_mac: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NetworkContainerInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Interfaces configured for this container with their addresses
    pub interfaces: Option<HashMap<String, NetInterface>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Name of the container
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// network created
pub struct NetworkCreate201Response {
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Warning")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warning: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NetworkInfo {
    pub backend: Option<String>,
    pub dns: Option<DnsNetworkInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NetworkInspectReport {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub containers: Option<HashMap<String, NetworkContainerInfo>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Created contains the timestamp when this network was created.
    pub created: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// DNSEnabled is whether name resolution is active for container on
    /// this Network. Only supported with the bridge driver.
    pub dns_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Driver for this Network, e.g. bridge, macvlan...
    pub driver: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// ID of the Network.
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Internal is whether the Network should not have external routes
    /// to public or other Networks.
    pub internal: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// IPAMOptions contains options used for the ip assignment.
    pub ipam_options: Option<HashMap<String, String>>,
    #[serde(rename = "ipv6_enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// IPv6Enabled if set to true an ipv6 subnet should be created for this net.
    pub ipv_6_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Labels is a set of key-value labels that have been applied to the
    /// Network.
    pub labels: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Name of the Network.
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// List of custom DNS server for podman's DNS resolver at network level,
    /// all the containers attached to this network will consider resolvers
    /// configured at network level.
    pub network_dns_servers: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// NetworkInterface is the network interface name on the host.
    pub network_interface: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Options is a set of key-value options that have been applied to
    /// the Network.
    pub options: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Routes to use for this network.
    pub routes: Option<Vec<Route>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Subnets to use for this network.
    pub subnets: Option<Vec<Subnet>>,
}

pub type NetworkMode = String;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// OK
pub struct NetworkPrune200Response {
    #[serde(rename = "NetworksDeleted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub networks_deleted: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// NetworkPruneReport containers the name of network and an error
/// associated in its pruning (removal)
pub struct NetworkPruneReport {
    #[serde(rename = "Error")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// NetworkRmReport describes the results of network removal
pub struct NetworkRmReport {
    #[serde(rename = "Err")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub err: Option<String>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// NetworkSettings exposes the network settings in the api
pub struct NetworkSettings {
    #[serde(rename = "Bridge")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bridge: Option<String>,
    #[serde(rename = "EndpointID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_id: Option<String>,
    #[serde(rename = "Gateway")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway: Option<String>,
    #[serde(rename = "GlobalIPv6Address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_i_pv_6_address: Option<String>,
    #[serde(rename = "GlobalIPv6PrefixLen")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_i_pv_6_prefix_len: Option<i64>,
    #[serde(rename = "HairpinMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// HairpinMode specifies if hairpin NAT should be enabled on the virtual interface
    ///
    /// Deprecated: This field is never set and will be removed in a future release.
    pub hairpin_mode: Option<bool>,
    #[serde(rename = "IPAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[serde(rename = "IPPrefixLen")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_prefix_len: Option<i64>,
    #[serde(rename = "IPv6Gateway")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_pv_6_gateway: Option<String>,
    #[serde(rename = "LinkLocalIPv6Address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// LinkLocalIPv6Address is an IPv6 unicast address using the link-local prefix
    ///
    /// Deprecated: This field is never set and will be removed in a future release.
    pub link_local_i_pv_6_address: Option<String>,
    #[serde(rename = "LinkLocalIPv6PrefixLen")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// LinkLocalIPv6PrefixLen is the prefix length of an IPv6 unicast address
    ///
    /// Deprecated: This field is never set and will be removed in a future release.
    pub link_local_i_pv_6_prefix_len: Option<i64>,
    #[serde(rename = "MacAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>,
    #[serde(rename = "Networks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub networks: Option<HashMap<String, EndpointSettings>>,
    #[serde(rename = "Ports")]
    pub ports: Option<PortMap>,
    #[serde(rename = "SandboxID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sandbox_id: Option<String>,
    #[serde(rename = "SandboxKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sandbox_key: Option<String>,
    #[serde(rename = "SecondaryIPAddresses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_ip_addresses: Option<Vec<Address>>,
    #[serde(rename = "SecondaryIPv6Addresses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_i_pv_6_addresses: Option<Vec<Address>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// NetworkUpdateOptions describes options to update a network
pub struct NetworkUpdateOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adddnsservers: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub removednsservers: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// NetworkingConfig represents the container's networking configuration for each of its interfaces
/// Carries the networking configs specified in the `docker run` and `docker network connect` commands
pub struct NetworkingConfig {
    #[serde(rename = "EndpointsConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoints_config: Option<HashMap<String, EndpointSettings>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// No such secret
pub struct NoSuchSecret {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// API root cause formatted for automated parsing
    pub cause: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// human error message, formatted for a human to read
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// HTTP response code
    pub response: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// OCIRuntimeInfo describes the runtime (crun or runc) being
/// used with podman
pub struct OciRuntimeInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// OverlayVolume holds information about an overlay volume that will be mounted into
/// the container.
pub struct OverlayVolume {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Destination is the absolute path where the mount will be placed in the container.
    pub destination: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Options holds overlay volume options.
    pub options: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Source specifies the source path of the mount.
    pub source: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// POSIXRlimit type and restrictions
pub struct PosixRlimit {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Hard is the hard limit for the specified type
    pub hard: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Soft is the soft limit for the specified type
    pub soft: Option<u64>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Type of the rlimit to set
    pub type_: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// PastaInfo describes the pasta executable that is being used
pub struct PastaInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub executable: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// PeerInfo represents one peer of an overlay network
pub struct PeerInfo {
    #[serde(rename = "IP")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PerNetworkOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Aliases contains a list of names which the dns server should resolve
    /// to this container. Should only be set when DNSEnabled is true on the Network.
    /// If aliases are set but there is no dns support for this network the
    /// network interface implementation should ignore this and NOT error.
    /// Optional.
    pub aliases: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// InterfaceName for this container. Required in the backend.
    /// Optional in the frontend. Will be filled with ethX (where X is a integer) when empty.
    pub interface_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Driver-specific options for this container.
    pub options: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// StaticIPs for this container. Optional.
    pub static_ips: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// StaticMac for this container. Optional.
    pub static_mac: Option<String>,
}

pub type PidMode = String;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Platform {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Architecture field specifies the CPU architecture, for example
    /// `amd64` or `ppc64le`.
    pub architecture: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// OS specifies the operating system, for example `linux` or `windows`.
    pub os: Option<String>,
    #[serde(rename = "os.features")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// OSFeatures is an optional field specifying an array of strings,
    /// each listing a required OS feature (for example on Windows `win32k`).
    pub os_features: Option<Vec<String>>,
    #[serde(rename = "os.version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// OSVersion is an optional field specifying the operating system
    /// version, for example on Windows `10.0.14393.1066`.
    pub os_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Variant is an optional field specifying a variant of the CPU, for
    /// example `v7` to specify ARMv7 when architecture is `arm`.
    pub variant: Option<String>,
}

pub type PlayKubeLibpodRequestParam = String;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayKubePod {
    #[serde(rename = "ContainerErrors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// ContainerErrors - any errors that occurred while starting containers
    /// in the pod.
    pub container_errors: Option<Vec<String>>,
    #[serde(rename = "Containers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Containers - the IDs of the containers running in the created pod.
    pub containers: Option<Vec<String>>,
    #[serde(rename = "ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// ID - ID of the pod created as a result of play kube.
    pub id: Option<String>,
    #[serde(rename = "InitContainers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// InitContainers - the IDs of the init containers to be run in the created pod.
    pub init_containers: Option<Vec<String>>,
    #[serde(rename = "Logs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Logs - non-fatal errors and log messages while processing.
    pub logs: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayKubeReport {
    #[serde(rename = "ExitCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// If set, exit with the specified exit code.
    pub exit_code: Option<i32>,
    #[serde(rename = "Pods")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Pods - pods created by play kube.
    pub pods: Option<Vec<PlayKubePod>>,
    #[serde(rename = "RmReport")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rm_report: Option<Vec<PodRmReport>>,
    #[serde(rename = "SecretRmReport")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_rm_report: Option<Vec<SecretRmReport>>,
    #[serde(rename = "Secrets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Secrets - secrets created by play kube
    pub secrets: Option<Vec<PlaySecret>>,
    #[serde(rename = "ServiceContainerID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// ServiceContainerID - ID of the service container if one is created
    pub service_container_id: Option<String>,
    #[serde(rename = "StopReport")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_report: Option<Vec<PodStopReport>>,
    #[serde(rename = "VolumeRmReport")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_rm_report: Option<Vec<VolumeRmReport>>,
    #[serde(rename = "Volumes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Volumes - volumes created by play kube.
    pub volumes: Option<Vec<PlayKubeVolume>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayKubeVolume {
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Name - Name of the volume created by play kube.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlaySecret {
    #[serde(rename = "CreateReport")]
    pub create_report: Option<SecretCreateReport>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Plugin A plugin for the Engine API
pub struct Plugin {
    #[serde(rename = "Config")]
    pub config: PluginConfig,
    #[serde(rename = "Enabled")]
    /// True if the plugin is running. False if the plugin is not running, only installed.
    pub enabled: bool,
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Id
    pub id: Option<String>,
    #[serde(rename = "Name")]
    /// name
    pub name: String,
    #[serde(rename = "PluginReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// plugin remote reference used to push/pull the plugin
    pub plugin_reference: Option<String>,
    #[serde(rename = "Settings")]
    pub settings: PluginSettings,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PluginConfig {
    #[serde(rename = "Args")]
    pub args: PluginConfigArgs,
    #[serde(rename = "Description")]
    /// description
    pub description: String,
    #[serde(rename = "DockerVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Docker Version used to create the plugin
    pub docker_version: Option<String>,
    #[serde(rename = "Documentation")]
    /// documentation
    pub documentation: String,
    #[serde(rename = "Entrypoint")]
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_nonoptional_vec")]
    /// entrypoint
    pub entrypoint: Vec<String>,
    #[serde(rename = "Env")]
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_nonoptional_vec")]
    /// env
    pub env: Vec<PluginEnv>,
    #[serde(rename = "Interface")]
    pub interface: PluginConfigInterface,
    #[serde(rename = "IpcHost")]
    /// ipc host
    pub ipc_host: bool,
    #[serde(rename = "Linux")]
    pub linux: PluginConfigLinux,
    #[serde(rename = "Mounts")]
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_nonoptional_vec")]
    /// mounts
    pub mounts: Vec<PluginMount>,
    #[serde(rename = "Network")]
    pub network: PluginConfigNetwork,
    #[serde(rename = "PidHost")]
    /// pid host
    pub pid_host: bool,
    #[serde(rename = "PropagatedMount")]
    /// propagated mount
    pub propagated_mount: String,
    #[serde(rename = "User")]
    pub user: Option<PluginConfigUser>,
    #[serde(rename = "WorkDir")]
    /// work dir
    pub work_dir: String,
    pub rootfs: Option<PluginConfigRootfs>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// PluginConfigArgs plugin config args
pub struct PluginConfigArgs {
    #[serde(rename = "Description")]
    /// description
    pub description: String,
    #[serde(rename = "Name")]
    /// name
    pub name: String,
    #[serde(rename = "Settable")]
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_nonoptional_vec")]
    /// settable
    pub settable: Vec<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_nonoptional_vec")]
    /// value
    pub value: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// PluginConfigInterface The interface between Docker and the plugin
pub struct PluginConfigInterface {
    #[serde(rename = "ProtocolScheme")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Protocol to use for clients connecting to the plugin.
    pub protocol_scheme: Option<String>,
    #[serde(rename = "Socket")]
    /// socket
    pub socket: String,
    #[serde(rename = "Types")]
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_nonoptional_vec")]
    /// types
    pub types: Vec<PluginInterfaceType>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// PluginConfigLinux plugin config linux
pub struct PluginConfigLinux {
    #[serde(rename = "AllowAllDevices")]
    /// allow all devices
    pub allow_all_devices: bool,
    #[serde(rename = "Capabilities")]
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_nonoptional_vec")]
    /// capabilities
    pub capabilities: Vec<String>,
    #[serde(rename = "Devices")]
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_nonoptional_vec")]
    /// devices
    pub devices: Vec<PluginDevice>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// PluginConfigNetwork plugin config network
pub struct PluginConfigNetwork {
    #[serde(rename = "Type")]
    /// type
    pub type_: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// PluginConfigRootfs plugin config rootfs
pub struct PluginConfigRootfs {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// diff ids
    pub diff_ids: Option<Vec<String>>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// type
    pub type_: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// PluginConfigUser plugin config user
pub struct PluginConfigUser {
    #[serde(rename = "GID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// g ID
    pub gid: Option<u32>,
    #[serde(rename = "UID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// UID
    pub uid: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// PluginDevice plugin device
pub struct PluginDevice {
    #[serde(rename = "Description")]
    /// description
    pub description: String,
    #[serde(rename = "Name")]
    /// name
    pub name: String,
    #[serde(rename = "Path")]
    /// path
    pub path: String,
    #[serde(rename = "Settable")]
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_nonoptional_vec")]
    /// settable
    pub settable: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// PluginEnv plugin env
pub struct PluginEnv {
    #[serde(rename = "Description")]
    /// description
    pub description: String,
    #[serde(rename = "Name")]
    /// name
    pub name: String,
    #[serde(rename = "Settable")]
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_nonoptional_vec")]
    /// settable
    pub settable: Vec<String>,
    #[serde(rename = "Value")]
    /// value
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// PluginInterfaceType plugin interface type
pub struct PluginInterfaceType {
    #[serde(rename = "Capability")]
    /// capability
    pub capability: String,
    #[serde(rename = "Prefix")]
    /// prefix
    pub prefix: String,
    #[serde(rename = "Version")]
    /// version
    pub version: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// PluginMount plugin mount
pub struct PluginMount {
    #[serde(rename = "Description")]
    /// description
    pub description: String,
    #[serde(rename = "Destination")]
    /// destination
    pub destination: String,
    #[serde(rename = "Name")]
    /// name
    pub name: String,
    #[serde(rename = "Options")]
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_nonoptional_vec")]
    /// options
    pub options: Vec<String>,
    #[serde(rename = "Settable")]
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_nonoptional_vec")]
    /// settable
    pub settable: Vec<String>,
    #[serde(rename = "Source")]
    /// source
    pub source: String,
    #[serde(rename = "Type")]
    /// type
    pub type_: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PluginSettings {
    #[serde(rename = "Args")]
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_nonoptional_vec")]
    /// args
    pub args: Vec<String>,
    #[serde(rename = "Devices")]
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_nonoptional_vec")]
    /// devices
    pub devices: Vec<PluginDevice>,
    #[serde(rename = "Env")]
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_nonoptional_vec")]
    /// env
    pub env: Vec<String>,
    #[serde(rename = "Mounts")]
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_nonoptional_vec")]
    /// mounts
    pub mounts: Vec<PluginMount>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Plugins {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Authorization is provided for compatibility, will always be nil as Podman has no daemon
    pub authorization: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PodBasicConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// ExitPolicy determines the pod's exit and stop behaviour.
    pub exit_policy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Hostname is the pod's hostname. If not set, the name of the pod will
    /// be used (if a name was not provided here, the name auto-generated for
    /// the pod will be used). This will be used by the infra container and
    /// all containers in the pod as long as the UTS namespace is shared.
    /// Optional.
    pub hostname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// InfraCommand sets the command that will be used to start the infra
    /// container.
    /// If not set, the default set in the Libpod configuration file will be
    /// used.
    /// Conflicts with NoInfra=true.
    /// Optional.
    pub infra_command: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// InfraConmonPidFile is a custom path to store the infra container's
    /// conmon PID.
    pub infra_conmon_pid_file: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// InfraImage is the image that will be used for the infra container.
    /// If not set, the default set in the Libpod configuration file will be
    /// used.
    /// Conflicts with NoInfra=true.
    /// Optional.
    pub infra_image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// InfraName is the name that will be used for the infra container.
    /// If not set, the default set in the Libpod configuration file will be
    /// used.
    /// Conflicts with NoInfra=true.
    /// Optional.
    pub infra_name: Option<String>,
    pub ipcns: Option<Namespace>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Labels are key-value pairs that are used to add metadata to pods.
    /// Optional.
    pub labels: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Name is the name of the pod.
    /// If not provided, a name will be generated when the pod is created.
    /// Optional.
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// NoInfra tells the pod not to create an infra container. If this is
    /// done, many networking-related options will become unavailable.
    /// Conflicts with setting any options in PodNetworkConfig, and the
    /// InfraCommand and InfraImages in this struct.
    /// Optional.
    pub no_infra: Option<bool>,
    pub pidns: Option<Namespace>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pod_create_command: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Devices contains user specified Devices to be added to the Pod
    pub pod_devices: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// RestartPolicy is the pod's restart policy - an action which
    /// will be taken when one or all the containers in the pod exits.
    /// If not given, the default policy will be set to Always, which
    /// restarts the containers in the pod when they exit indefinitely.
    /// Optional.
    pub restart_policy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// RestartRetries is the number of attempts that will be made to restart
    /// the container.
    /// Only available when RestartPolicy is set to "on-failure".
    /// Optional.
    pub restart_tries: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// PodCreateCommand is the command used to create this pod.
    /// This will be shown in the output of Inspect() on the pod, and may
    /// also be used by some tools that wish to recreate the pod
    /// (e.g. `podman generate systemd --new`).
    /// Optional.
    /// ShareParent determines if all containers in the pod will share the pod's cgroup as the cgroup parent
    pub share_parent: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// SharedNamespaces instructs the pod to share a set of namespaces.
    /// Shared namespaces will be joined (by default) by every container
    /// which joins the pod.
    /// If not set and NoInfra is false, the pod will set a default set of
    /// namespaces to share.
    /// Conflicts with NoInfra=true.
    /// Optional.
    pub shared_namespaces: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Sysctl sets kernel parameters for the pod
    pub sysctl: Option<HashMap<String, String>>,
    pub userns: Option<Namespace>,
    pub utsns: Option<Namespace>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// This will be expanded in future updates to pods.
pub struct PodCgroupConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// CgroupParent is the parent for the Cgroup that the pod will create.
    /// This pod cgroup will, in turn, be the default cgroup parent for all
    /// containers in the pod.
    /// Optional.
    pub cgroup_parent: Option<String>,
}

/// status conflict
pub type PodCreateLibpod409Response = String;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// The JSON tags below are made to match the respective field in ContainerCreateOptions for the purpose of mapping.
pub struct PodCreateOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cgroup_parent: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_command: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_conmon_pidfile: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpus: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpuset_cpus: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_command: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_read_bps: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exit_policy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infra: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infra_image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub net: Option<NetOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restart: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_opt: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_parent: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sysctl: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uts: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes_from: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PodKillReport {
    #[serde(rename = "Errs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errs: Option<Vec<String>>,
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PodNetworkConfig {
    #[serde(rename = "Networks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Map of networks names to ids the container should join to.
    /// You can request additional settings for each network, you can
    /// set network aliases, static ips, static mac address  and the
    /// network interface name for this container on the specific network.
    /// If the map is empty and the bridge network mode is set the container
    /// will be joined to the default network.
    pub networks: Option<HashMap<String, PerNetworkOptions>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// CNINetworks is a list of CNI networks to join the container to.
    /// If this list is empty, the default CNI network will be joined
    /// instead. If at least one entry is present, we will not join the
    /// default network (unless it is part of this list).
    /// Only available if NetNS is set to bridge.
    /// Optional.
    /// Deprecated: as of podman 4.0 use "Networks" instead.
    pub cni_networks: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// DNSOption is a set of DNS options that will be used in the infra
    /// container's resolv.conf, which will, by default, be shared with all
    /// containers in the pod.
    /// Conflicts with NoInfra=true.
    /// Optional.
    pub dns_option: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// DNSSearch is a set of DNS search domains that will be used in the
    /// infra container's resolv.conf, which will, by default, be shared with
    /// all containers in the pod.
    /// If not provided, DNS search domains from the host's resolv.conf will
    /// be used.
    /// Conflicts with NoInfra=true.
    /// Optional.
    pub dns_search: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// DNSServer is a set of DNS servers that will be used in the infra
    /// container's resolv.conf, which will, by default, be shared with all
    /// containers in the pod.
    /// If not provided, the host's DNS servers will be used, unless the only
    /// server set is a localhost address. As the container cannot connect to
    /// the host's localhost, a default server will instead be set.
    /// Conflicts with NoInfra=true.
    /// Optional.
    pub dns_server: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// HostAdd is a set of hosts that will be added to the infra container's
    /// etc/hosts that will, by default, be shared with all containers in
    /// the pod.
    /// Conflicts with NoInfra=true and NoManageHosts.
    /// Optional.
    pub hostadd: Option<Vec<String>>,
    #[serde(rename = "hostsFile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// HostsFile is the base file to create the `/etc/hosts` file inside the infra container.
    /// This must either be an absolute path to a file on the host system, or one of the
    /// special flags `image` or `none`.
    /// If it is empty it defaults to the base_hosts_file configuration in containers.conf.
    /// Conflicts with NoInfra=true and NoManageHosts.
    /// Optional.
    pub hosts_file: Option<String>,
    pub netns: Option<Namespace>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// NetworkOptions are additional options for each network
    /// Optional.
    pub network_options: Option<HashMap<String, Vec<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// NoManageHostname indicates that /etc/hostname should not be managed
    /// by the pod. Instead, each container will create a separate
    /// etc/hostname as they would if not in a pod.
    pub no_manage_hostname: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// NoManageHosts indicates that /etc/hosts should not be managed by the
    /// pod. Instead, each container will create a separate /etc/hosts as
    /// they would if not in a pod.
    /// Conflicts with HostAdd.
    pub no_manage_hosts: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// NoManageResolvConf indicates that /etc/resolv.conf should not be
    /// managed by the pod. Instead, each container will create and manage a
    /// separate resolv.conf as if they had not joined a pod.
    /// Conflicts with NoInfra=true and DNSServer, DNSSearch, DNSOption.
    /// Optional.
    pub no_manage_resolv_conf: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// PortMappings is a set of ports to map into the infra container.
    /// As, by default, containers share their network with the infra
    /// container, this will forward the ports to the entire pod.
    /// Only available if NetNS is set to Bridge, Slirp, or Pasta.
    /// Optional.
    pub portmappings: Option<Vec<PortMapping>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PodPauseReport {
    #[serde(rename = "Errs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errs: Option<Vec<String>>,
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PodPruneReport {
    #[serde(rename = "Err")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub err: Option<String>,
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PodResourceConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// CPU period of the cpuset, determined by --cpus
    pub cpu_period: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// CPU quota of the cpuset, determined by --cpus
    pub cpu_quota: Option<i64>,
    pub resource_limits: Option<LinuxResources>,
    #[serde(rename = "throttleReadBpsDevice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// ThrottleReadBpsDevice contains the rate at which the devices in the pod can be read from/accessed
    pub throttle_read_bps_device: Option<HashMap<String, LinuxThrottleDevice>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PodRestartReport {
    #[serde(rename = "Errs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errs: Option<Vec<String>>,
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PodRmReport {
    #[serde(rename = "Err")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub err: Option<String>,
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "RemovedCtrs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub removed_ctrs: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PodSecurityConfig {
    pub idmappings: Option<IdMappingOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_opt: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// PodSpecGenerator describes options to create a pod
pub struct PodSpecGenerator {
    #[serde(rename = "Networks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Map of networks names to ids the container should join to.
    /// You can request additional settings for each network, you can
    /// set network aliases, static ips, static mac address  and the
    /// network interface name for this container on the specific network.
    /// If the map is empty and the bridge network mode is set the container
    /// will be joined to the default network.
    pub networks: Option<HashMap<String, PerNetworkOptions>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// CgroupParent is the parent for the Cgroup that the pod will create.
    /// This pod cgroup will, in turn, be the default cgroup parent for all
    /// containers in the pod.
    /// Optional.
    pub cgroup_parent: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// CNINetworks is a list of CNI networks to join the container to.
    /// If this list is empty, the default CNI network will be joined
    /// instead. If at least one entry is present, we will not join the
    /// default network (unless it is part of this list).
    /// Only available if NetNS is set to bridge.
    /// Optional.
    /// Deprecated: as of podman 4.0 use "Networks" instead.
    pub cni_networks: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// CPU period of the cpuset, determined by --cpus
    pub cpu_period: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// CPU quota of the cpuset, determined by --cpus
    pub cpu_quota: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// DNSOption is a set of DNS options that will be used in the infra
    /// container's resolv.conf, which will, by default, be shared with all
    /// containers in the pod.
    /// Conflicts with NoInfra=true.
    /// Optional.
    pub dns_option: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// DNSSearch is a set of DNS search domains that will be used in the
    /// infra container's resolv.conf, which will, by default, be shared with
    /// all containers in the pod.
    /// If not provided, DNS search domains from the host's resolv.conf will
    /// be used.
    /// Conflicts with NoInfra=true.
    /// Optional.
    pub dns_search: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// DNSServer is a set of DNS servers that will be used in the infra
    /// container's resolv.conf, which will, by default, be shared with all
    /// containers in the pod.
    /// If not provided, the host's DNS servers will be used, unless the only
    /// server set is a localhost address. As the container cannot connect to
    /// the host's localhost, a default server will instead be set.
    /// Conflicts with NoInfra=true.
    /// Optional.
    pub dns_server: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// ExitPolicy determines the pod's exit and stop behaviour.
    pub exit_policy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// HostAdd is a set of hosts that will be added to the infra container's
    /// etc/hosts that will, by default, be shared with all containers in
    /// the pod.
    /// Conflicts with NoInfra=true and NoManageHosts.
    /// Optional.
    pub hostadd: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Hostname is the pod's hostname. If not set, the name of the pod will
    /// be used (if a name was not provided here, the name auto-generated for
    /// the pod will be used). This will be used by the infra container and
    /// all containers in the pod as long as the UTS namespace is shared.
    /// Optional.
    pub hostname: Option<String>,
    #[serde(rename = "hostsFile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// HostsFile is the base file to create the `/etc/hosts` file inside the infra container.
    /// This must either be an absolute path to a file on the host system, or one of the
    /// special flags `image` or `none`.
    /// If it is empty it defaults to the base_hosts_file configuration in containers.conf.
    /// Conflicts with NoInfra=true and NoManageHosts.
    /// Optional.
    pub hosts_file: Option<String>,
    pub idmappings: Option<IdMappingOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Image volumes bind-mount a container-image mount into the pod's infra container.
    /// Optional.
    pub image_volumes: Option<Vec<ImageVolume>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// InfraCommand sets the command that will be used to start the infra
    /// container.
    /// If not set, the default set in the Libpod configuration file will be
    /// used.
    /// Conflicts with NoInfra=true.
    /// Optional.
    pub infra_command: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// InfraConmonPidFile is a custom path to store the infra container's
    /// conmon PID.
    pub infra_conmon_pid_file: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// InfraImage is the image that will be used for the infra container.
    /// If not set, the default set in the Libpod configuration file will be
    /// used.
    /// Conflicts with NoInfra=true.
    /// Optional.
    pub infra_image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// InfraName is the name that will be used for the infra container.
    /// If not set, the default set in the Libpod configuration file will be
    /// used.
    /// Conflicts with NoInfra=true.
    /// Optional.
    pub infra_name: Option<String>,
    pub ipcns: Option<Namespace>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Labels are key-value pairs that are used to add metadata to pods.
    /// Optional.
    pub labels: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Mounts are mounts that will be added to the pod.
    /// These will supersede Image Volumes and VolumesFrom volumes where
    /// there are conflicts.
    /// Optional.
    pub mounts: Option<Vec<Mount>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Name is the name of the pod.
    /// If not provided, a name will be generated when the pod is created.
    /// Optional.
    pub name: Option<String>,
    pub netns: Option<Namespace>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// NetworkOptions are additional options for each network
    /// Optional.
    pub network_options: Option<HashMap<String, Vec<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// NoInfra tells the pod not to create an infra container. If this is
    /// done, many networking-related options will become unavailable.
    /// Conflicts with setting any options in PodNetworkConfig, and the
    /// InfraCommand and InfraImages in this struct.
    /// Optional.
    pub no_infra: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// NoManageHostname indicates that /etc/hostname should not be managed
    /// by the pod. Instead, each container will create a separate
    /// etc/hostname as they would if not in a pod.
    pub no_manage_hostname: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// NoManageHosts indicates that /etc/hosts should not be managed by the
    /// pod. Instead, each container will create a separate /etc/hosts as
    /// they would if not in a pod.
    /// Conflicts with HostAdd.
    pub no_manage_hosts: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// NoManageResolvConf indicates that /etc/resolv.conf should not be
    /// managed by the pod. Instead, each container will create and manage a
    /// separate resolv.conf as if they had not joined a pod.
    /// Conflicts with NoInfra=true and DNSServer, DNSSearch, DNSOption.
    /// Optional.
    pub no_manage_resolv_conf: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Overlay volumes are named volumes that will be added to the pod.
    /// Optional.
    pub overlay_volumes: Option<Vec<OverlayVolume>>,
    pub pidns: Option<Namespace>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pod_create_command: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Devices contains user specified Devices to be added to the Pod
    pub pod_devices: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// PortMappings is a set of ports to map into the infra container.
    /// As, by default, containers share their network with the infra
    /// container, this will forward the ports to the entire pod.
    /// Only available if NetNS is set to Bridge, Slirp, or Pasta.
    /// Optional.
    pub portmappings: Option<Vec<PortMapping>>,
    pub resource_limits: Option<LinuxResources>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// RestartPolicy is the pod's restart policy - an action which
    /// will be taken when one or all the containers in the pod exits.
    /// If not given, the default policy will be set to Always, which
    /// restarts the containers in the pod when they exit indefinitely.
    /// Optional.
    pub restart_policy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// RestartRetries is the number of attempts that will be made to restart
    /// the container.
    /// Only available when RestartPolicy is set to "on-failure".
    /// Optional.
    pub restart_tries: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_opt: Option<Vec<String>>,
    #[serde(rename = "serviceContainerID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The ID of the pod's service container.
    pub service_container_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// PodCreateCommand is the command used to create this pod.
    /// This will be shown in the output of Inspect() on the pod, and may
    /// also be used by some tools that wish to recreate the pod
    /// (e.g. `podman generate systemd --new`).
    /// Optional.
    /// ShareParent determines if all containers in the pod will share the pod's cgroup as the cgroup parent
    pub share_parent: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// SharedNamespaces instructs the pod to share a set of namespaces.
    /// Shared namespaces will be joined (by default) by every container
    /// which joins the pod.
    /// If not set and NoInfra is false, the pod will set a default set of
    /// namespaces to share.
    /// Conflicts with NoInfra=true.
    /// Optional.
    pub shared_namespaces: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// ShmSize is the size of the tmpfs to mount in at /dev/shm, in bytes.
    /// Conflicts with ShmSize if IpcNS is not private.
    /// Optional.
    pub shm_size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// ShmSizeSystemd is the size of systemd-specific tmpfs mounts
    /// specifically /run, /run/lock, /var/log/journal and /tmp.
    /// Optional
    pub shm_size_systemd: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Sysctl sets kernel parameters for the pod
    pub sysctl: Option<HashMap<String, String>>,
    #[serde(rename = "throttleReadBpsDevice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// ThrottleReadBpsDevice contains the rate at which the devices in the pod can be read from/accessed
    pub throttle_read_bps_device: Option<HashMap<String, LinuxThrottleDevice>>,
    pub userns: Option<Namespace>,
    pub utsns: Option<Namespace>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Volumes are named volumes that will be added to the pod.
    /// These will supersede Image Volumes and VolumesFrom  volumes where
    /// there are conflicts.
    /// Optional.
    pub volumes: Option<Vec<NamedVolume>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// VolumesFrom is a set of containers whose volumes will be added to
    /// this pod. The name or ID of the container must be provided, and
    /// may optionally be followed by a : and then one or more
    /// comma-separated options. Valid options are 'ro', 'rw', and 'z'.
    /// Options will be used for all volumes sourced from the container.
    pub volumes_from: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PodStartReport {
    #[serde(rename = "Errs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errs: Option<Vec<String>>,
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "RawInput")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_input: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PodStatsReport {
    #[serde(rename = "BlockIO")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Humanized disk usage read + write
    pub block_io: Option<String>,
    #[serde(rename = "CID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Container ID
    pub cid: Option<String>,
    #[serde(rename = "CPU")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Percentage of CPU utilized by pod
    pub cpu: Option<String>,
    #[serde(rename = "Mem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Percentage of Memory utilized by pod
    pub mem: Option<String>,
    #[serde(rename = "MemUsage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Humanized Memory usage and maximum
    pub mem_usage: Option<String>,
    #[serde(rename = "MemUsageBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Memory usage and maximum in bytes
    pub mem_usage_bytes: Option<String>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Pod Name
    pub name: Option<String>,
    #[serde(rename = "NetIO")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Network usage inbound + outbound
    pub net_io: Option<String>,
    #[serde(rename = "PIDS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Container PID
    pub pids: Option<String>,
    #[serde(rename = "Pod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Pod ID
    pub pod: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PodStopReport {
    #[serde(rename = "Errs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errs: Option<Vec<String>>,
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "RawInput")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_input: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PodStorageConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Image volumes bind-mount a container-image mount into the pod's infra container.
    /// Optional.
    pub image_volumes: Option<Vec<ImageVolume>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Mounts are mounts that will be added to the pod.
    /// These will supersede Image Volumes and VolumesFrom volumes where
    /// there are conflicts.
    /// Optional.
    pub mounts: Option<Vec<Mount>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Overlay volumes are named volumes that will be added to the pod.
    /// Optional.
    pub overlay_volumes: Option<Vec<OverlayVolume>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// ShmSize is the size of the tmpfs to mount in at /dev/shm, in bytes.
    /// Conflicts with ShmSize if IpcNS is not private.
    /// Optional.
    pub shm_size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// ShmSizeSystemd is the size of systemd-specific tmpfs mounts
    /// specifically /run, /run/lock, /var/log/journal and /tmp.
    /// Optional
    pub shm_size_systemd: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Volumes are named volumes that will be added to the pod.
    /// These will supersede Image Volumes and VolumesFrom  volumes where
    /// there are conflicts.
    /// Optional.
    pub volumes: Option<Vec<NamedVolume>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// VolumesFrom is a set of containers whose volumes will be added to
    /// this pod. The name or ID of the container must be provided, and
    /// may optionally be followed by a : and then one or more
    /// comma-separated options. Valid options are 'ro', 'rw', and 'z'.
    /// Options will be used for all volumes sourced from the container.
    pub volumes_from: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PodTopOkBody {
    #[serde(rename = "Processes")]
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_nonoptional_vec")]
    /// Each process running in the container, where each is process
    /// is an array of values corresponding to the titles.
    pub processes: Vec<Vec<String>>,
    #[serde(rename = "Titles")]
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_nonoptional_vec")]
    /// The ps column titles
    pub titles: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PodUnpauseReport {
    #[serde(rename = "Errs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errs: Option<Vec<String>>,
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Port An open port on a container
pub struct Port {
    #[serde(rename = "IP")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Host IP address that the container's port is mapped to
    pub ip: Option<String>,
    #[serde(rename = "PrivatePort")]
    /// Port on the container
    pub private_port: u16,
    #[serde(rename = "PublicPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Port exposed on the host
    pub public_port: Option<u16>,
    #[serde(rename = "Type")]
    /// type
    pub type_: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// PortBinding represents a binding between a Host IP address and a Host Port
pub struct PortBinding {
    #[serde(rename = "HostIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// HostIP is the host IP Address
    pub host_ip: Option<String>,
    #[serde(rename = "HostPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// HostPort is the host port number
    pub host_port: Option<String>,
}

/// PortMap is a collection of PortBinding indexed by Port
pub type PortMap = HashMap<String, Option<Vec<PortBinding>>>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PortMapping {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// ContainerPort is the port number that will be exposed from the
    /// container.
    /// Mandatory.
    pub container_port: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// HostIP is the IP that we will bind to on the host.
    /// If unset, assumed to be 0.0.0.0 (all interfaces).
    pub host_ip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// HostPort is the port number that will be forwarded from the host into
    /// the container.
    /// If omitted, a random port on the host (guaranteed to be over 1024)
    /// will be assigned.
    pub host_port: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Protocol is the protocol forward.
    /// Must be either "tcp", "udp", and "sctp", or some combination of these
    /// separated by commas.
    /// If unset, assumed to be TCP.
    pub protocol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Range is the number of ports that will be forwarded, starting at
    /// HostPort and ContainerPort and counting up.
    /// This is 1-indexed, so 1 is assumed to be a single port (only the
    /// Hostport:Containerport mapping will be added), 2 is two ports (both
    /// Hostport:Containerport and Hostport+1:Containerport+1), etc.
    /// If unset, assumed to be 1 (a single port).
    /// Both hostport + range and containerport + range must be less than
    /// 65536.
    pub range: Option<u16>,
}

/// PortSet is a collection of structs indexed by Port
pub type PortSet = HashMap<String, Value>;

pub type Propagation = String;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// POST "/volumes/prune"
pub struct PruneReport {
    #[serde(rename = "Err")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub err: Option<String>,
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<u64>,
}

/// PublishState represents the state of a Volume as it pertains to its
/// use on a particular Node.
pub type PublishState = String;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// PublishStatus represents the status of the volume as published to an
/// individual node
pub struct PublishStatus {
    #[serde(rename = "NodeID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// NodeID is the ID of the swarm node this Volume is published to.
    pub node_id: Option<String>,
    #[serde(rename = "PublishContext")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// PublishContext is the PublishContext returned by the CSI plugin when
    /// a volume is published.
    pub publish_context: Option<HashMap<String, String>>,
    #[serde(rename = "State")]
    pub state: Option<String>,
}

pub type PutContainerArchiveLibpodRequestParam = String;

pub type PutContainerArchiveRequestParam = String;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// RemoteSocket describes information about the API socket
pub struct RemoteSocket {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exists: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Resources contains container's resources (cgroups config, ulimits...)
pub struct Resources {
    #[serde(rename = "BlkioDeviceReadBps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blkio_device_read_bps: Option<Vec<ThrottleDevice>>,
    #[serde(rename = "BlkioDeviceReadIOps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blkio_device_read_i_ops: Option<Vec<ThrottleDevice>>,
    #[serde(rename = "BlkioDeviceWriteBps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blkio_device_write_bps: Option<Vec<ThrottleDevice>>,
    #[serde(rename = "BlkioDeviceWriteIOps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blkio_device_write_i_ops: Option<Vec<ThrottleDevice>>,
    #[serde(rename = "BlkioWeight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blkio_weight: Option<u16>,
    #[serde(rename = "BlkioWeightDevice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blkio_weight_device: Option<Vec<WeightDevice>>,
    #[serde(rename = "CgroupParent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Applicable to UNIX platforms
    pub cgroup_parent: Option<String>,
    #[serde(rename = "CpuCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Applicable to Windows
    pub cpu_count: Option<i64>,
    #[serde(rename = "CpuPercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_percent: Option<i64>,
    #[serde(rename = "CpuPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_period: Option<i64>,
    #[serde(rename = "CpuQuota")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_quota: Option<i64>,
    #[serde(rename = "CpuRealtimePeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_realtime_period: Option<i64>,
    #[serde(rename = "CpuRealtimeRuntime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_realtime_runtime: Option<i64>,
    #[serde(rename = "CpuShares")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Applicable to all platforms
    pub cpu_shares: Option<i64>,
    #[serde(rename = "CpusetCpus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpuset_cpus: Option<String>,
    #[serde(rename = "CpusetMems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpuset_mems: Option<String>,
    #[serde(rename = "DeviceCgroupRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_cgroup_rules: Option<Vec<String>>,
    #[serde(rename = "DeviceRequests")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_requests: Option<Vec<DeviceRequest>>,
    #[serde(rename = "Devices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<DeviceMapping>>,
    #[serde(rename = "IOMaximumBandwidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub io_maximum_bandwidth: Option<u64>,
    #[serde(rename = "IOMaximumIOps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub io_maximum_i_ops: Option<u64>,
    #[serde(rename = "KernelMemory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// KernelMemory specifies the kernel memory limit (in bytes) for the container.
    /// Deprecated: kernel 5.4 deprecated kmem.limit_in_bytes.
    pub kernel_memory: Option<i64>,
    #[serde(rename = "KernelMemoryTCP")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kernel_memory_tcp: Option<i64>,
    #[serde(rename = "Memory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<i64>,
    #[serde(rename = "MemoryReservation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_reservation: Option<i64>,
    #[serde(rename = "MemorySwap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_swap: Option<i64>,
    #[serde(rename = "MemorySwappiness")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_swappiness: Option<i64>,
    #[serde(rename = "NanoCpus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nano_cpus: Option<i64>,
    #[serde(rename = "OomKillDisable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oom_kill_disable: Option<bool>,
    #[serde(rename = "PidsLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pids_limit: Option<i64>,
    #[serde(rename = "Ulimits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ulimits: Option<Vec<Ulimit>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RestartPolicy {
    #[serde(rename = "MaximumRetryCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_retry_count: Option<i64>,
    #[serde(rename = "Name")]
    pub name: Option<String>,
}

pub type RestartPolicyMode = String;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RootFs {
    #[serde(rename = "Layers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layers: Option<Vec<String>>,
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Route {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Destination for this route in CIDR form.
    pub destination: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Gateway IP for this route.
    pub gateway: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Metric for this route. Optional.
    pub metric: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Schema2HealthConfig is a HealthConfig, which holds configuration settings
/// for the HEALTHCHECK feature, from docker/docker/api/types/container.
pub struct Schema2HealthConfig {
    #[serde(rename = "Interval")]
    pub interval: Option<i64>,
    #[serde(rename = "Retries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Retries is the number of consecutive failures needed to consider a container as unhealthy.
    /// Zero means inherit.
    pub retries: Option<i64>,
    #[serde(rename = "StartInterval")]
    pub start_interval: Option<i64>,
    #[serde(rename = "StartPeriod")]
    pub start_period: Option<i64>,
    #[serde(rename = "Test")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Test is the test to perform to check that the container is healthy.
    /// An empty slice means to inherit the default.
    /// The options are:
    /// {} : inherit healthcheck
    /// {"NONE"} : disable healthcheck
    /// {"CMD", args...} : exec arguments directly
    /// {"CMD-SHELL", command} : run command with system's default shell
    pub test: Option<Vec<String>>,
    #[serde(rename = "Timeout")]
    pub timeout: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// This is publicly visible as c/image/manifest.Schema2List.
/// Internal users should usually use Schema2List instead.
pub struct Schema2ListPublic {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifests: Option<Vec<Schema2ManifestDescriptor>>,
    #[serde(rename = "mediaType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_type: Option<String>,
    #[serde(rename = "schemaVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_version: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// This is publicly visible as c/image/manifest.Schema2ManifestDescriptor.
pub struct Schema2ManifestDescriptor {
    pub digest: Option<String>,
    #[serde(rename = "mediaType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_type: Option<String>,
    pub platform: Option<Schema2PlatformSpec>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urls: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Schema2PlatformSpec describes the platform which a particular manifest is
/// specialized for.
/// This is publicly visible as c/image/manifest.Schema2PlatformSpec.
pub struct Schema2PlatformSpec {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,
    #[serde(rename = "os.features")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_features: Option<Vec<String>>,
    #[serde(rename = "os.version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant: Option<String>,
}

/// Scope defines the Scope of a Cluster Volume. This is how many nodes a
/// Volume can be accessed simultaneously on.
pub type Scope = String;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScpReport {
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Secret represents a Swarm Secret value that must be passed to the CSI
/// storage plugin when operating on this Volume. It represents one key-value
/// pair of possibly many.
pub struct Secret {
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Key is the name of the key of the key-value pair passed to the plugin.
    pub key: Option<String>,
    #[serde(rename = "Secret")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Secret is the swarm Secret object from which to read data. This can be a
    /// Secret name or ID. The Secret data is retrieved by Swarm and used as the
    /// value of the key-value pair passed to the plugin.
    pub secret: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SecretCreate {
    #[serde(rename = "Data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Base64-url-safe-encoded (RFC 4648) data to store as secret.
    pub data: Option<String>,
    #[serde(rename = "Driver")]
    pub driver: Option<SecretDriverSpec>,
    #[serde(rename = "Labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Labels are labels on the secret
    pub labels: Option<HashMap<String, String>>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// User-defined name of the secret.
    pub name: Option<String>,
}

pub type SecretCreateLibpodRequestParam = String;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SecretCreateReport {
    #[serde(rename = "ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Secret create response
pub struct SecretCreateResponse {
    #[serde(rename = "ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SecretDriverSpec {
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Options")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Secret in use
pub struct SecretInUse {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// API root cause formatted for automated parsing
    pub cause: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// human error message, formatted for a human to read
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// HTTP response code
    pub response: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SecretInfoReport {
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "SecretData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_data: Option<String>,
    #[serde(rename = "Spec")]
    pub spec: Option<SecretSpec>,
    #[serde(rename = "UpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SecretInfoReportCompat {
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "SecretData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_data: Option<String>,
    #[serde(rename = "Spec")]
    pub spec: Option<SecretSpec>,
    #[serde(rename = "UpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
    #[serde(rename = "Version")]
    pub version: Option<SecretVersion>,
}

/// Secret list response
pub type SecretListCompatResponse = Vec<SecretInfoReportCompat>;

/// Secret list response
pub type SecretListResponse = Vec<SecretInfoReport>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SecretRmReport {
    #[serde(rename = "Err")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub err: Option<String>,
    #[serde(rename = "ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SecretSpec {
    #[serde(rename = "Driver")]
    pub driver: Option<SecretDriverSpec>,
    #[serde(rename = "Labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<HashMap<String, String>>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SecretVersion {
    #[serde(rename = "Index")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// SecurityInfo describes the libpod host
pub struct SecurityInfo {
    #[serde(rename = "apparmorEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apparmor_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rootless: Option<bool>,
    #[serde(rename = "seccompEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seccomp_enabled: Option<bool>,
    #[serde(rename = "seccompProfilePath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seccomp_profile_path: Option<String>,
    #[serde(rename = "selinuxEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selinux_enabled: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// ServiceCreateResponse contains the information returned to a client on the
/// creation of a new service.
pub struct ServiceCreateResponse {
    #[serde(rename = "ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The ID of the created service.
    pub id: Option<String>,
    #[serde(rename = "Warnings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Optional warning message.
    ///
    /// FIXME(thaJeztah): this should have "omitempty" in the generated type.
    pub warnings: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// ServiceInfo represents service parameters with the list of service's tasks
pub struct ServiceInfo {
    #[serde(rename = "LocalLBIndex")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_lb_index: Option<i64>,
    #[serde(rename = "Ports")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ports: Option<Vec<String>>,
    #[serde(rename = "Tasks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tasks: Option<Vec<Task>>,
    #[serde(rename = "VIP")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vip: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// ServiceUpdateResponse service update response
pub struct ServiceUpdateResponse {
    #[serde(rename = "Warnings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Optional warning messages
    pub warnings: Option<Vec<String>>,
}

/// SharingMode defines the Sharing of a Cluster Volume. This is how Tasks using a
/// Volume at the same time can use it.
pub type SharingMode = String;

/// It implements the [os.Signal] interface.
pub type Signal = i64;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// SlirpInfo describes the slirp executable that is being used
pub struct SlirpInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub executable: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// SpecGenerator creates an OCI spec and Libpod configuration options to create
/// a container based on the given configuration.
pub struct SpecGenerator {
    #[serde(rename = "Networks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Map of networks names or ids that the container should join.
    /// You can request additional settings for each network, you can
    /// set network aliases, static ips, static mac address  and the
    /// network interface name for this container on the specific network.
    /// If the map is empty and the bridge network mode is set the container
    /// will be joined to the default network.
    /// Optional.
    pub networks: Option<HashMap<String, PerNetworkOptions>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Annotations are key-value options passed into the container runtime
    /// that can be used to trigger special behavior.
    /// Optional.
    pub annotations: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// ApparmorProfile is the name of the Apparmor profile the container
    /// will use.
    /// Optional.
    pub apparmor_profile: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// BaseHostsFile is the base file to create the `/etc/hosts` file inside the container.
    /// This must either be an absolute path to a file on the host system, or one of the
    /// special flags `image` or `none`.
    /// If it is empty it defaults to the base_hosts_file configuration in containers.conf.
    /// Optional.
    pub base_hosts_file: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// CapAdd are capabilities which will be added to the container.
    /// Conflicts with Privileged.
    /// Optional.
    pub cap_add: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// CapDrop are capabilities which will be removed from the container.
    /// Conflicts with Privileged.
    /// Optional.
    pub cap_drop: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// CgroupParent is the container's Cgroup parent.
    /// If not set, the default for the current cgroup driver will be used.
    /// Optional.
    pub cgroup_parent: Option<String>,
    pub cgroupns: Option<Namespace>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// CgroupsMode sets a policy for how cgroups will be created for the
    /// container, including the ability to disable creation entirely.
    /// Optional.
    pub cgroups_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// ChrootDirs is an additional set of directories that need to be
    /// treated as root directories. Standard bind mounts will be mounted
    /// into paths relative to these directories.
    /// Optional.
    pub chroot_directories: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// CNINetworks is a list of CNI networks to join the container to.
    /// If this list is empty, the default CNI network will be joined
    /// instead. If at least one entry is present, we will not join the
    /// default network (unless it is part of this list).
    /// Only available if NetNS is set to bridge.
    /// Optional.
    /// Deprecated: as of podman 4.0 use "Networks" instead.
    pub cni_networks: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Command is the container's command.
    /// If not given and Image is specified, this will be populated by the
    /// image's configuration.
    /// Optional.
    pub command: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// ConmonPidFile is a path at which a PID file for Conmon will be
    /// placed.
    /// If not given, a default location will be used.
    /// Optional.
    pub conmon_pid_file: Option<String>,
    #[serde(rename = "containerCreateCommand")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// ContainerCreateCommand is the command that was used to create this
    /// container.
    /// This will be shown in the output of Inspect() on the container, and
    /// may also be used by some tools that wish to recreate the container
    /// (e.g. `podman generate systemd --new`).
    /// Optional.
    pub container_create_command: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Create the working directory if it doesn't exist.
    /// If unset, it doesn't create it.
    /// Optional.
    pub create_working_dir: Option<bool>,
    #[serde(rename = "dependencyContainers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// DependencyContainers is an array of containers this container
    /// depends on. Dependency containers must be started before this
    /// container. Dependencies can be specified by name or full/partial ID.
    /// Optional.
    pub dependency_containers: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// DeviceCgroupRule are device cgroup rules that allow containers
    /// to use additional types of devices.
    pub device_cgroup_rule: Option<Vec<LinuxDeviceCgroup>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Devices are devices that will be added to the container.
    /// Optional.
    pub devices: Option<Vec<LinuxDevice>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// DevicesFrom specifies that this container will mount the device(s) from other container(s).
    /// Optional.
    pub devices_from: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// DNSOptions is a set of DNS options that will be used in the
    /// container's resolv.conf, replacing the host's DNS options which are
    /// used by default.
    /// Conflicts with UseImageResolvConf.
    /// Optional.
    pub dns_option: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// DNSSearch is a set of DNS search domains that will be used in the
    /// container's resolv.conf, replacing the host's DNS search domains
    /// which are used by default.
    /// Conflicts with UseImageResolvConf.
    /// Optional.
    pub dns_search: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// DNSServers is a set of DNS servers that will be used in the
    /// container's resolv.conf, replacing the host's DNS Servers which are
    /// used by default.
    /// Conflicts with UseImageResolvConf.
    /// Optional.
    pub dns_server: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Entrypoint is the container's entrypoint.
    /// If not given and Image is specified, this will be populated by the
    /// image's configuration.
    /// Optional.
    pub entrypoint: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Env is a set of environment variables that will be set in the
    /// container.
    /// Optional.
    pub env: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// EnvHost indicates that the host environment should be added to container
    /// Optional.
    pub env_host: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// EnvMerge takes the specified environment variables from image and preprocess them before injecting them into the
    /// container.
    /// Optional.
    pub envmerge: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Expose is a number of ports that will be forwarded to the container
    /// if PublishExposedPorts is set.
    /// Expose is a map of uint16 (port number) to a string representing
    /// protocol i.e map[uint16]string. Allowed protocols are "tcp", "udp", and "sctp", or some
    /// combination of the three separated by commas.
    /// If protocol is set to "" we will assume TCP.
    /// Only available if NetNS is set to Bridge or Slirp, and
    /// PublishExposedPorts is set.
    /// Optional.
    pub expose: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// GroupEntry specifies an arbitrary string to append to the container's /etc/group file.
    /// Optional.
    pub group_entry: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Groups are a list of supplemental groups the container's user will
    /// be granted access to.
    /// Optional.
    pub groups: Option<Vec<String>>,
    #[serde(rename = "healthLogDestination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// HealthLogDestination defines the destination where the log is stored.
    /// TODO (6.0): In next major release convert it to pointer and use omitempty
    pub health_log_destination: Option<String>,
    #[serde(rename = "healthMaxLogCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// HealthMaxLogCount is maximum number of attempts in the HealthCheck log file.
    /// ('0' value means an infinite number of attempts in the log file).
    /// TODO (6.0): In next major release convert it to pointer and use omitempty
    pub health_max_log_count: Option<u64>,
    #[serde(rename = "healthMaxLogSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// HealthMaxLogSize is the maximum length in characters of stored HealthCheck log
    /// ("0" value means an infinite log length).
    /// TODO (6.0): In next major release convert it to pointer and use omitempty
    pub health_max_log_size: Option<u64>,
    pub health_check_on_failure_action: Option<i64>,
    pub healthconfig: Option<Schema2HealthConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// HostDeviceList is used to recreate the mounted device on inherited containers
    pub host_device_list: Option<Vec<LinuxDevice>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// HostAdd is a set of hosts which will be added to the container's
    /// etc/hosts file.
    /// Conflicts with UseImageHosts.
    /// Optional.
    pub hostadd: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Hostname is the container's hostname. If not set, the hostname will
    /// not be modified (if UtsNS is not private) or will be set to the
    /// container ID (if UtsNS is private).
    /// Conflicts with UtsNS if UtsNS is not set to private.
    /// Optional.
    pub hostname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// HostUsers is a list of host usernames or UIDs to add to the container
    /// etc/passwd file
    pub hostusers: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// EnvHTTPProxy indicates that the http host proxy environment variables
    /// should be added to container
    /// Optional.
    pub httpproxy: Option<bool>,
    pub idmappings: Option<IdMappingOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Image is the image the container will be based on. The image will be
    /// used as the container's root filesystem, and its environment vars,
    /// volumes, and other configuration will be applied to the container.
    /// Conflicts with Rootfs.
    /// At least one of Image or Rootfs must be specified.
    pub image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// ImageArch is the user-specified image architecture.
    /// Used to select a different variant from a manifest list.
    /// Optional.
    pub image_arch: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// ImageOS is the user-specified OS of the image.
    /// Used to select a different variant from a manifest list.
    /// Optional.
    pub image_os: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// ImageVariant is the user-specified image variant.
    /// Used to select a different variant from a manifest list.
    /// Optional.
    pub image_variant: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// ImageVolumeMode indicates how image volumes will be created.
    /// Supported modes are "ignore" (do not create), "tmpfs" (create as
    /// tmpfs), and "anonymous" (create as anonymous volumes).
    /// The default if unset is anonymous.
    /// Optional.
    pub image_volume_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Image volumes bind-mount a container-image mount into the container.
    /// Optional.
    pub image_volumes: Option<Vec<ImageVolume>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Init specifies that an init binary will be mounted into the
    /// container, and will be used as PID1.
    /// Optional.
    pub init: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// InitContainerType describes if this container is an init container
    /// and if so, what type: always or once.
    /// Optional.
    pub init_container_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// InitPath specifies the path to the init binary that will be added if
    /// Init is specified above. If not specified, the default set in the
    /// Libpod config will be used. Ignored if Init above is not set.
    /// Optional.
    pub init_path: Option<String>,
    #[serde(rename = "intelRdt")]
    pub intel_rdt: Option<LinuxIntelRdt>,
    pub ipcns: Option<Namespace>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// LabelNested indicates whether or not the container is allowed to
    /// run fully nested containers including SELinux labelling.
    /// Optional.
    pub label_nested: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Labels are key-value pairs that are used to add metadata to
    /// containers.
    /// Optional.
    pub labels: Option<HashMap<String, String>>,
    pub log_configuration: Option<LogConfigLibpod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Passwd is a container run option that determines if we are validating users/groups before running the container
    pub manage_password: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Mask is the path we want to mask in the container. This masks the paths
    /// given in addition to the default list.
    /// Optional
    pub mask: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Mounts are mounts that will be added to the container.
    /// These will supersede Image Volumes and VolumesFrom volumes where
    /// there are conflicts.
    /// Optional.
    pub mounts: Option<Vec<Mount>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Name is the name the container will be given.
    /// If no name is provided, one will be randomly generated.
    /// Optional.
    pub name: Option<String>,
    pub netns: Option<Namespace>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// NetworkOptions are additional options for each network
    /// Optional.
    pub network_options: Option<HashMap<String, Vec<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// NoNewPrivileges is whether the container will set the no new
    /// privileges flag on create, which disables gaining additional
    /// privileges (e.g. via setuid) in the container.
    /// Optional.
    pub no_new_privileges: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// OCIRuntime is the name of the OCI runtime that will be used to create
    /// the container.
    /// If not specified, the default will be used.
    /// Optional.
    pub oci_runtime: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// OOMScoreAdj adjusts the score used by the OOM killer to determine
    /// processes to kill for the container's process.
    /// Optional.
    pub oom_score_adj: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Overlay volumes are named volumes that will be added to the container.
    /// Optional.
    pub overlay_volumes: Option<Vec<OverlayVolume>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// PasswdEntry specifies an arbitrary string to append to the container's /etc/passwd file.
    /// Optional.
    pub passwd_entry: Option<String>,
    pub personality: Option<LinuxPersonality>,
    pub pidns: Option<Namespace>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Pod is the ID of the pod the container will join.
    /// Optional.
    pub pod: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// PortBindings is a set of ports to map into the container.
    /// Only available if NetNS is set to bridge, slirp, or pasta.
    /// Optional.
    pub portmappings: Option<Vec<PortMapping>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Privileged is whether the container is privileged.
    /// Privileged does the following:
    /// Adds all devices on the system to the container.
    /// Adds all capabilities to the container.
    /// Disables Seccomp, SELinux, and Apparmor confinement.
    /// (Though SELinux can be manually re-enabled).
    /// TODO: this conflicts with things.
    /// TODO: this does more.
    /// Optional.
    pub privileged: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// ProcOpts are the options used for the proc mount.
    pub procfs_opts: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// PublishExposedPorts will publish ports specified in the image to
    /// random unused ports (guaranteed to be above 1024) on the host.
    /// This is based on ports set in Expose below, and any ports specified
    /// by the Image (if one is given).
    /// Only available if NetNS is set to Bridge or Slirp.
    /// Optional.
    pub publish_image_ports: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Rlimits are POSIX rlimits to apply to the container.
    /// Optional.
    pub r_limits: Option<Vec<PosixRlimit>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// RawImageName is the user-specified and unprocessed input referring
    /// to a local or a remote image.
    /// Optional, but strongly encouraged to be set if Image is set.
    pub raw_image_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// ReadOnlyFilesystem indicates that everything will be mounted
    /// as read-only.
    /// Optional.
    pub read_only_filesystem: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// ReadWriteTmpfs indicates that when running with a ReadOnlyFilesystem
    /// mount temporary file systems.
    /// Optional.
    pub read_write_tmpfs: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Remove indicates if the container should be removed once it has been started
    /// and exits.
    /// Optional.
    pub remove: Option<bool>,
    #[serde(rename = "removeImage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// RemoveImage indicates that the container should remove the image it
    /// was created from after it exits.
    /// Only allowed if Remove is set to true and Image, not Rootfs, is in
    /// use.
    /// Optional.
    pub remove_image: Option<bool>,
    pub resource_limits: Option<LinuxResources>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// RestartPolicy is the container's restart policy - an action which
    /// will be taken when the container exits.
    /// If not given, the default policy, which does nothing, will be used.
    /// Optional.
    pub restart_policy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// RestartRetries is the number of attempts that will be made to restart
    /// the container.
    /// Only available when RestartPolicy is set to "on-failure".
    /// Optional.
    pub restart_tries: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Rootfs is the path to a directory that will be used as the
    /// container's root filesystem. No modification will be made to the
    /// directory, it will be directly mounted into the container as root.
    /// Conflicts with Image.
    /// At least one of Image or Rootfs must be specified.
    pub rootfs: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// RootfsMapping specifies if there are UID/GID mappings to apply to the rootfs.
    /// Optional.
    pub rootfs_mapping: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// RootfsOverlay tells if rootfs is actually an overlay on top of base path.
    /// Optional.
    pub rootfs_overlay: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// RootfsPropagation is the rootfs propagation mode for the container.
    /// If not set, the default of rslave will be used.
    /// Optional.
    pub rootfs_propagation: Option<String>,
    #[serde(rename = "sdnotifyMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Determine how to handle the NOTIFY_SOCKET - do we participate or pass it through
    /// "container" - let the OCI runtime deal with it, advertise conmon's MAINPID
    /// "conmon-only" - advertise conmon's MAINPID, send READY when started, don't pass to OCI
    /// "ignore" - unset NOTIFY_SOCKET
    /// Optional.
    pub sdnotify_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// SeccompPolicy determines which seccomp profile gets applied
    /// the container. valid values: empty,default,image
    pub seccomp_policy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// SeccompProfilePath is the path to a JSON file containing the
    /// container's Seccomp profile.
    /// If not specified, no Seccomp profile will be used.
    /// Optional.
    pub seccomp_profile_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// EnvSecrets are secrets that will be set as environment variables
    /// Optional.
    pub secret_env: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Secrets are the secrets that will be added to the container
    /// Optional.
    pub secrets: Option<Vec<Secret>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// SelinuxProcessLabel is the process label the container will use.
    /// If SELinux is enabled and this is not specified, a label will be
    /// automatically generated if not specified.
    /// Optional.
    pub selinux_opts: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// ShmSize is the size of the tmpfs to mount in at /dev/shm, in bytes.
    /// Conflicts with ShmSize if IpcNS is not private.
    /// Optional.
    pub shm_size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// ShmSizeSystemd is the size of systemd-specific tmpfs mounts
    /// specifically /run, /run/lock, /var/log/journal and /tmp.
    /// Optional
    pub shm_size_systemd: Option<i64>,
    #[serde(rename = "startupHealthConfig")]
    pub startup_health_config: Option<StartupHealthCheck>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Stdin is whether the container will keep its STDIN open.
    /// Optional.
    pub stdin: Option<bool>,
    pub stop_signal: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// StopTimeout is a timeout between the container's stop signal being
    /// sent and SIGKILL being sent.
    /// If not provided, the default will be used.
    /// If 0 is used, stop signal will not be sent, and SIGKILL will be sent
    /// instead.
    /// Optional.
    pub stop_timeout: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// StorageOpts is the container's storage options
    /// Optional.
    pub storage_opts: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Sysctl sets kernel parameters for the container
    pub sysctl: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Systemd is whether the container will be started in systemd mode.
    /// Valid options are "true", "false", and "always".
    /// "true" enables this mode only if the binary run in the container is
    /// sbin/init or systemd. "always" unconditionally enables systemd mode.
    /// "false" unconditionally disables systemd mode.
    /// If enabled, mounts and stop signal will be modified.
    /// If set to "always" or set to "true" and conditionally triggered,
    /// conflicts with StopSignal.
    /// If not specified, "false" will be assumed.
    /// Optional.
    pub systemd: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Terminal is whether the container will create a PTY.
    /// Optional.
    pub terminal: Option<bool>,
    #[serde(rename = "throttleReadBpsDevice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// IO read rate limit per cgroup per device, bytes per second
    pub throttle_read_bps_device: Option<HashMap<String, LinuxThrottleDevice>>,
    #[serde(rename = "throttleReadIOPSDevice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// IO read rate limit per cgroup per device, IO per second
    pub throttle_read_iops_device: Option<HashMap<String, LinuxThrottleDevice>>,
    #[serde(rename = "throttleWriteBpsDevice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// IO write rate limit per cgroup per device, bytes per second
    pub throttle_write_bps_device: Option<HashMap<String, LinuxThrottleDevice>>,
    #[serde(rename = "throttleWriteIOPSDevice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// IO write rate limit per cgroup per device, IO per second
    pub throttle_write_iops_device: Option<HashMap<String, LinuxThrottleDevice>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Timeout is a maximum time in seconds the container will run before
    /// main process is sent SIGKILL.
    /// If 0 is used, signal will not be sent. Container can run indefinitely
    /// if they do not stop after the default termination signal.
    /// Optional.
    pub timeout: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Timezone is the timezone inside the container.
    /// Local means it has the same timezone as the host machine
    /// Optional.
    pub timezone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Umask is the umask the init process of the container will be run with.
    pub umask: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// CgroupConf are key-value options passed into the container runtime
    /// that are used to configure cgroup v2.
    /// Optional.
    pub unified: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Unmask a path in the container. Some paths are masked by default,
    /// preventing them from being accessed within the container; this undoes
    /// that masking. If ALL is passed, all paths will be unmasked.
    /// Optional.
    pub unmask: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// UnsetEnv unsets the specified default environment variables from the image or from built-in or containers.conf
    /// Optional.
    pub unsetenv: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// UnsetEnvAll unsetall default environment variables from the image or from built-in or containers.conf
    /// UnsetEnvAll unsets all default environment variables from the image or from built-in
    /// Optional.
    pub unsetenvall: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// UseImageHostname indicates that /etc/hostname should not be managed by
    /// Podman, and instead sourced from the image.
    /// Optional.
    pub use_image_hostname: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// UseImageHosts indicates that /etc/hosts should not be managed by
    /// Podman, and instead sourced from the image.
    /// Conflicts with HostAdd.
    /// Optional.
    pub use_image_hosts: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// UseImageResolvConf indicates that resolv.conf should not be managed
    /// by Podman, but instead sourced from the image.
    /// Conflicts with DNSServer, DNSSearch, DNSOption.
    /// Optional.
    pub use_image_resolve_conf: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// User is the user the container will be run as.
    /// Can be given as a UID or a username; if a username, it will be
    /// resolved within the container, using the container's /etc/passwd.
    /// If unset, the container will be run as root.
    /// Optional.
    pub user: Option<String>,
    pub userns: Option<Namespace>,
    pub utsns: Option<Namespace>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Volatile specifies whether the container storage can be optimized
    /// at the cost of not syncing all the dirty files in memory.
    /// Optional.
    pub volatile: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Volumes are named volumes that will be added to the container.
    /// These will supersede Image Volumes and VolumesFrom volumes where
    /// there are conflicts.
    /// Optional.
    pub volumes: Option<Vec<NamedVolume>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// VolumesFrom is a set of containers whose volumes will be added to
    /// this container. The name or ID of the container must be provided, and
    /// may optionally be followed by a : and then one or more
    /// comma-separated options. Valid options are 'ro', 'rw', and 'z'.
    /// Options will be used for all volumes sourced from the container.
    /// Optional.
    pub volumes_from: Option<Vec<String>>,
    #[serde(rename = "weightDevice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Weight per cgroup per device, can override BlkioWeight
    pub weight_device: Option<HashMap<String, LinuxWeightDevice>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// WorkDir is the container's working directory.
    /// If unset, the default, /, will be used.
    /// Optional.
    pub work_dir: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StartupHealthCheck {
    #[serde(rename = "Interval")]
    pub interval: Option<i64>,
    #[serde(rename = "Retries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Retries is the number of consecutive failures needed to consider a container as unhealthy.
    /// Zero means inherit.
    pub retries: Option<i64>,
    #[serde(rename = "StartInterval")]
    pub start_interval: Option<i64>,
    #[serde(rename = "StartPeriod")]
    pub start_period: Option<i64>,
    #[serde(rename = "Successes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Successes are the number of successes required to mark the startup HC
    /// as passed.
    /// If set to 0, a single success will mark the HC as passed.
    pub successes: Option<i64>,
    #[serde(rename = "Test")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Test is the test to perform to check that the container is healthy.
    /// An empty slice means to inherit the default.
    /// The options are:
    /// {} : inherit healthcheck
    /// {"NONE"} : disable healthcheck
    /// {"CMD", args...} : exec arguments directly
    /// {"CMD-SHELL", command} : run command with system's default shell
    pub test: Option<Vec<String>>,
    #[serde(rename = "Timeout")]
    pub timeout: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// StoreInfo describes the container storage and its
/// attributes
pub struct StoreInfo {
    #[serde(rename = "configFile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_file: Option<String>,
    #[serde(rename = "containerStore")]
    pub container_store: Option<ContainerStore>,
    #[serde(rename = "graphDriverName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graph_driver_name: Option<String>,
    #[serde(rename = "graphOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graph_options: Option<Value>,
    #[serde(rename = "graphRoot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graph_root: Option<String>,
    #[serde(rename = "graphRootAllocated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// GraphRootAllocated is how much space the graphroot has in bytes
    pub graph_root_allocated: Option<u64>,
    #[serde(rename = "graphRootUsed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// GraphRootUsed is how much of graphroot is used in bytes
    pub graph_root_used: Option<u64>,
    #[serde(rename = "graphStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graph_status: Option<HashMap<String, String>>,
    #[serde(rename = "imageCopyTmpDir")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_copy_tmp_dir: Option<String>,
    #[serde(rename = "imageStore")]
    pub image_store: Option<ImageStore>,
    #[serde(rename = "runRoot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_root: Option<String>,
    #[serde(rename = "transientStore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transient_store: Option<bool>,
    #[serde(rename = "volumePath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_path: Option<String>,
}

/// We need to override the json decoder to accept both options.
pub type StrSlice = Vec<String>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Subnet {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Gateway IP for this Network.
    pub gateway: Option<String>,
    pub lease_range: Option<LeaseRange>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Subnet for this Network in CIDR form.
    pub subnet: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Summary {
    #[serde(rename = "Containers")]
    /// Number of containers using this image. Includes both stopped and running
    /// containers.
    ///
    /// This size is not calculated by default, and depends on which API endpoint
    /// is used. `-1` indicates that the value has not been set / calculated.
    pub containers: i64,
    #[serde(rename = "Created")]
    /// Date and time at which the image was created as a Unix timestamp
    /// (number of seconds since EPOCH).
    pub created: i64,
    #[serde(rename = "Id")]
    /// ID is the content-addressable ID of an image.
    ///
    /// This identifier is a content-addressable digest calculated from the
    /// image's configuration (which includes the digests of layers used by
    /// the image).
    ///
    /// Note that this digest differs from the `RepoDigests` below, which
    /// holds digests of image manifests that reference the image.
    pub id: String,
    #[serde(rename = "Labels")]
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_nonoptional_map")]
    /// User-defined key/value metadata.
    pub labels: HashMap<String, String>,
    #[serde(rename = "Manifests")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Manifests is a list of image manifests available in this image.  It
    /// provides a more detailed view of the platform-specific image manifests or
    /// other image-attached data like build attestations.
    ///
    /// WARNING: This is experimental and may change at any time without any backward
    /// compatibility.
    pub manifests: Option<Vec<ManifestSummary>>,
    #[serde(rename = "ParentId")]
    /// ID of the parent image.
    ///
    /// Depending on how the image was created, this field may be empty and
    /// is only set for images that were built/created locally. This field
    /// is empty if the image was pulled from an image registry.
    pub parent_id: String,
    #[serde(rename = "RepoDigests")]
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_nonoptional_vec")]
    /// List of content-addressable digests of locally available image manifests
    /// that the image is referenced from. Multiple manifests can refer to the
    /// same image.
    ///
    /// These digests are usually only available if the image was either pulled
    /// from a registry, or if the image was pushed to a registry, which is when
    /// the manifest is generated and its digest calculated.
    pub repo_digests: Vec<String>,
    #[serde(rename = "RepoTags")]
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_nonoptional_vec")]
    /// List of image names/tags in the local image cache that reference this
    /// image.
    ///
    /// Multiple image tags can refer to the same image, and this list may be
    /// empty if no tags reference the image, in which case the image is
    /// "untagged", in which case it can still be referenced by its ID.
    pub repo_tags: Vec<String>,
    #[serde(rename = "SharedSize")]
    /// Total size of image layers that are shared between this image and other
    /// images.
    ///
    /// This size is not calculated by default. `-1` indicates that the value
    /// has not been set / calculated.
    pub shared_size: i64,
    #[serde(rename = "Size")]
    /// Total size of the image including all layers it is composed of.
    pub size: i64,
    #[serde(rename = "VirtualSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Total size of the image including all layers it is composed of.
    ///
    /// Deprecated: this field is omitted in API v1.44, but kept for backward compatibility. Use Size instead.
    pub virtual_size: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// SummaryNetworkSettings provides a summary of container's networks
/// in /containers/json
pub struct SummaryNetworkSettings {
    #[serde(rename = "Networks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub networks: Option<HashMap<String, EndpointSettings>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// SystemCheckReport provides a report of what a storage consistency check
/// found, and if we removed anything that was damaged, what we removed.
pub struct SystemCheckReport {
    #[serde(rename = "Containers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub containers: Option<HashMap<String, Vec<String>>>,
    #[serde(rename = "Errors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<bool>,
    #[serde(rename = "Images")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<HashMap<String, Vec<String>>>,
    #[serde(rename = "Layers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layers: Option<HashMap<String, Vec<String>>>,
    #[serde(rename = "ROImages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ro_images: Option<HashMap<String, Vec<String>>>,
    #[serde(rename = "ROLayers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ro_layers: Option<HashMap<String, Vec<String>>>,
    #[serde(rename = "RemovedContainers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub removed_containers: Option<HashMap<String, String>>,
    #[serde(rename = "RemovedImages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub removed_images: Option<HashMap<String, Vec<String>>>,
    #[serde(rename = "RemovedLayers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub removed_layers: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// SystemComponentVersion is the type used by pkg/domain/entities
pub struct SystemComponentVersion {
    #[serde(rename = "ApiVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,
    #[serde(rename = "Arch")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arch: Option<String>,
    #[serde(rename = "BuildTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_time: Option<String>,
    #[serde(rename = "Components")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<ComponentVersion>>,
    #[serde(rename = "Experimental")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experimental: Option<bool>,
    #[serde(rename = "GitCommit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub git_commit: Option<String>,
    #[serde(rename = "GoVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub go_version: Option<String>,
    #[serde(rename = "KernelVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kernel_version: Option<String>,
    #[serde(rename = "MinAPIVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_api_version: Option<String>,
    #[serde(rename = "Os")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,
    #[serde(rename = "Platform")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<SystemComponentVersionPlatformInlineItem>,
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SystemComponentVersionPlatformInlineItem {
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// SystemDfContainerReport describes a container for use with df
pub struct SystemDfContainerReport {
    #[serde(rename = "Command")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<Vec<String>>,
    #[serde(rename = "ContainerID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_id: Option<String>,
    #[serde(rename = "Created")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<DateTime<Utc>>,
    #[serde(rename = "Image")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(rename = "LocalVolumes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_volumes: Option<i64>,
    #[serde(rename = "Names")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<String>,
    #[serde(rename = "RWSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rw_size: Option<i64>,
    #[serde(rename = "Size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// SystemDfImageReport describes an image for use with df
pub struct SystemDfImageReport {
    #[serde(rename = "Containers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub containers: Option<i64>,
    #[serde(rename = "Created")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<DateTime<Utc>>,
    #[serde(rename = "ImageID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
    #[serde(rename = "Repository")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<String>,
    #[serde(rename = "SharedSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_size: Option<i64>,
    #[serde(rename = "Size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(rename = "Tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(rename = "UniqueSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_size: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// SystemDfReport describes the response for df information
pub struct SystemDfReport {
    #[serde(rename = "Containers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub containers: Option<Vec<SystemDfContainerReport>>,
    #[serde(rename = "Images")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<SystemDfImageReport>>,
    #[serde(rename = "ImagesSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images_size: Option<i64>,
    #[serde(rename = "Volumes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes: Option<Vec<SystemDfVolumeReport>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// SystemDfVolumeReport describes a volume and its size
pub struct SystemDfVolumeReport {
    #[serde(rename = "Links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<i64>,
    #[serde(rename = "ReclaimableSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reclaimable_size: Option<i64>,
    #[serde(rename = "Size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(rename = "VolumeName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_name: Option<String>,
}

/// Success
pub type SystemPing200Response = String;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SystemPruneReport {
    #[serde(rename = "ContainerPruneReports")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_prune_reports: Option<Vec<PruneReport>>,
    #[serde(rename = "ImagePruneReports")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_prune_reports: Option<Vec<PruneReport>>,
    #[serde(rename = "NetworkPruneReports")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_prune_reports: Option<Vec<NetworkPruneReport>>,
    #[serde(rename = "PodPruneReport")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pod_prune_report: Option<Vec<PodPruneReport>>,
    #[serde(rename = "ReclaimedSpace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reclaimed_space: Option<u64>,
    #[serde(rename = "VolumePruneReports")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_prune_reports: Option<Vec<PruneReport>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Task carries the information about one backend task
pub struct Task {
    #[serde(rename = "EndpointID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_id: Option<String>,
    #[serde(rename = "EndpointIP")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_ip: Option<String>,
    #[serde(rename = "Info")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub info: Option<HashMap<String, String>>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// ThrottleDevice is a structure that holds device:rate_per_second pair
pub struct ThrottleDevice {
    #[serde(rename = "Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "Rate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TmpfsOptions {
    #[serde(rename = "Mode")]
    pub mode: Option<u32>,
    #[serde(rename = "Options")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Options to be passed to the tmpfs mount. An array of arrays. Flag
    /// options should be provided as 1-length arrays. Other types should be
    /// provided as 2-length arrays, where the first item is the key and the
    /// second the value.
    pub options: Option<Vec<Vec<String>>>,
    #[serde(rename = "SizeBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Size sets the size of the tmpfs, in bytes.
    ///
    /// This will be converted to an operating system specific value
    /// depending on the host. For example, on linux, it will be converted to
    /// use a 'k', 'm' or 'g' syntax. BSD, though not widely supported with
    /// docker, uses a straight byte value.
    ///
    /// Percentages are not supported.
    pub size_bytes: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// This description is taken verbatim from the CSI Spec:
///
/// A topological domain is a sub-division of a cluster, like "region",
/// "zone", "rack", etc.
/// A topological segment is a specific instance of a topological domain,
/// like "zone3", "rack3", etc.
/// For example {"com.company/zone": "Z1", "com.company/rack": "R3"}
/// Valid keys have two segments: an OPTIONAL prefix and name, separated
/// by a slash (/), for example: "com.company.example/zone".
/// The key name segment is REQUIRED. The prefix is OPTIONAL.
/// The key name MUST be 63 characters or less, begin and end with an
/// alphanumeric character ([a-z0-9A-Z]), and contain only dashes (-),
/// underscores (_), dots (.), or alphanumerics in between, for example
/// "zone".
/// The key prefix MUST be 63 characters or less, begin and end with a
/// lower-case alphanumeric character ([a-z0-9]), contain only
/// dashes (-), dots (.), or lower-case alphanumerics in between, and
/// follow domain name notation format
/// (https://tools.ietf.org/html/rfc1035#section-2.3.1).
/// The key prefix SHOULD include the plugin's host company name and/or
/// the plugin name, to minimize the possibility of collisions with keys
/// from other plugins.
/// If a key prefix is specified, it MUST be identical across all
/// topology keys returned by the SP (across all RPCs).
/// Keys MUST be case-insensitive. Meaning the keys "Zone" and "zone"
/// MUST not both exist.
/// Each value (topological segment) MUST contain 1 or more strings.
/// Each string MUST be 63 characters or less and begin and end with an
/// alphanumeric character with '-', '_', '.', or alphanumerics in
/// between.
pub struct Topology {
    #[serde(rename = "Segments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segments: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// TopologyRequirement expresses the user's requirements for a volume's
/// accessible topology.
pub struct TopologyRequirement {
    #[serde(rename = "Preferred")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Preferred is a list of Topologies that the volume should attempt to be
    /// provisioned in.
    ///
    /// Taken from the CSI spec:
    ///
    /// Specifies the list of topologies the CO would prefer the volume to
    /// be provisioned in.
    ///
    /// This field is OPTIONAL. If TopologyRequirement is specified either
    /// requisite or preferred or both MUST be specified.
    ///
    /// An SP MUST attempt to make the provisioned volume available using
    /// the preferred topologies in order from first to last.
    ///
    /// If requisite is specified, all topologies in preferred list MUST
    /// also be present in the list of requisite topologies.
    ///
    /// If the SP is unable to make the provisioned volume available
    /// from any of the preferred topologies, the SP MAY choose a topology
    /// from the list of requisite topologies.
    /// If the list of requisite topologies is not specified, then the SP
    /// MAY choose from the list of all possible topologies.
    /// If the list of requisite topologies is specified and the SP is
    /// unable to make the provisioned volume available from any of the
    /// requisite topologies it MUST fail the CreateVolume call.
    ///
    /// Example 1:
    /// Given a volume should be accessible from a single zone, and
    /// requisite =
    /// {"region": "R1", "zone": "Z2"},
    /// {"region": "R1", "zone": "Z3"}
    /// preferred =
    /// {"region": "R1", "zone": "Z3"}
    /// then the SP SHOULD first attempt to make the provisioned volume
    /// available from "zone" "Z3" in the "region" "R1" and fall back to
    /// "zone" "Z2" in the "region" "R1" if that is not possible.
    ///
    /// Example 2:
    /// Given a volume should be accessible from a single zone, and
    /// requisite =
    /// {"region": "R1", "zone": "Z2"},
    /// {"region": "R1", "zone": "Z3"},
    /// {"region": "R1", "zone": "Z4"},
    /// {"region": "R1", "zone": "Z5"}
    /// preferred =
    /// {"region": "R1", "zone": "Z4"},
    /// {"region": "R1", "zone": "Z2"}
    /// then the SP SHOULD first attempt to make the provisioned volume
    /// accessible from "zone" "Z4" in the "region" "R1" and fall back to
    /// "zone" "Z2" in the "region" "R1" if that is not possible. If that
    /// is not possible, the SP may choose between either the "zone"
    /// "Z3" or "Z5" in the "region" "R1".
    ///
    /// Example 3:
    /// Given a volume should be accessible from TWO zones (because an
    /// opaque parameter in CreateVolumeRequest, for example, specifies
    /// the volume is accessible from two zones, aka synchronously
    /// replicated), and
    /// requisite =
    /// {"region": "R1", "zone": "Z2"},
    /// {"region": "R1", "zone": "Z3"},
    /// {"region": "R1", "zone": "Z4"},
    /// {"region": "R1", "zone": "Z5"}
    /// preferred =
    /// {"region": "R1", "zone": "Z5"},
    /// {"region": "R1", "zone": "Z3"}
    /// then the SP SHOULD first attempt to make the provisioned volume
    /// accessible from the combination of the two "zones" "Z5" and "Z3" in
    /// the "region" "R1". If that's not possible, it should fall back to
    /// a combination of "Z5" and other possibilities from the list of
    /// requisite. If that's not possible, it should fall back  to a
    /// combination of "Z3" and other possibilities from the list of
    /// requisite. If that's not possible, it should fall back  to a
    /// combination of other possibilities from the list of requisite.
    pub preferred: Option<Vec<Topology>>,
    #[serde(rename = "Requisite")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Requisite specifies a list of Topologies, at least one of which the
    /// volume must be accessible from.
    ///
    /// Taken verbatim from the CSI Spec:
    ///
    /// Specifies the list of topologies the provisioned volume MUST be
    /// accessible from.
    /// This field is OPTIONAL. If TopologyRequirement is specified either
    /// requisite or preferred or both MUST be specified.
    ///
    /// If requisite is specified, the provisioned volume MUST be
    /// accessible from at least one of the requisite topologies.
    ///
    /// Given
    /// x = number of topologies provisioned volume is accessible from
    /// n = number of requisite topologies
    /// The CO MUST ensure n >= 1. The SP MUST ensure x >= 1
    /// If x==n, then the SP MUST make the provisioned volume available to
    /// all topologies from the list of requisite topologies. If it is
    /// unable to do so, the SP MUST fail the CreateVolume call.
    /// For example, if a volume should be accessible from a single zone,
    /// and requisite =
    /// {"region": "R1", "zone": "Z2"}
    /// then the provisioned volume MUST be accessible from the "region"
    /// "R1" and the "zone" "Z2".
    /// Similarly, if a volume should be accessible from two zones, and
    /// requisite =
    /// {"region": "R1", "zone": "Z2"},
    /// {"region": "R1", "zone": "Z3"}
    /// then the provisioned volume MUST be accessible from the "region"
    /// "R1" and both "zone" "Z2" and "zone" "Z3".
    ///
    /// If x<n, then the SP SHALL choose x unique topologies from the list
    /// of requisite topologies. If it is unable to do so, the SP MUST fail
    /// the CreateVolume call.
    /// For example, if a volume should be accessible from a single zone,
    /// and requisite =
    /// {"region": "R1", "zone": "Z2"},
    /// {"region": "R1", "zone": "Z3"}
    /// then the SP may choose to make the provisioned volume available in
    /// either the "zone" "Z2" or the "zone" "Z3" in the "region" "R1".
    /// Similarly, if a volume should be accessible from two zones, and
    /// requisite =
    /// {"region": "R1", "zone": "Z2"},
    /// {"region": "R1", "zone": "Z3"},
    /// {"region": "R1", "zone": "Z4"}
    /// then the provisioned volume MUST be accessible from any combination
    /// of two unique topologies: e.g. "R1/Z2" and "R1/Z3", or "R1/Z2" and
    /// "R1/Z4", or "R1/Z3" and "R1/Z4".
    ///
    /// If x>n, then the SP MUST make the provisioned volume available from
    /// all topologies from the list of requisite topologies and MAY choose
    /// the remaining x-n unique topologies from the list of all possible
    /// topologies. If it is unable to do so, the SP MUST fail the
    /// CreateVolume call.
    /// For example, if a volume should be accessible from two zones, and
    /// requisite =
    /// {"region": "R1", "zone": "Z2"}
    /// then the provisioned volume MUST be accessible from the "region"
    /// "R1" and the "zone" "Z2" and the SP may select the second zone
    /// independently, e.g. "R1/Z4".
    pub requisite: Option<Vec<Topology>>,
}

pub type Type = String;

/// Intentionally empty.
pub type TypeBlock = Value;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// TypeMount contains options for using a volume as a Mount-type
/// volume.
pub struct TypeMount {
    #[serde(rename = "FsType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// FsType specifies the filesystem type for the mount volume. Optional.
    pub fs_type: Option<String>,
    #[serde(rename = "MountFlags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// MountFlags defines flags to pass when mounting the volume. Optional.
    pub mount_flags: Option<Vec<String>>,
}

pub type UtsMode = String;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ulimit {
    #[serde(rename = "Hard")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hard: Option<i64>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Soft")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub soft: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Those attributes can be updated at runtime.
pub struct UpdateConfig {
    #[serde(rename = "BlkioDeviceReadBps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blkio_device_read_bps: Option<Vec<ThrottleDevice>>,
    #[serde(rename = "BlkioDeviceReadIOps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blkio_device_read_i_ops: Option<Vec<ThrottleDevice>>,
    #[serde(rename = "BlkioDeviceWriteBps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blkio_device_write_bps: Option<Vec<ThrottleDevice>>,
    #[serde(rename = "BlkioDeviceWriteIOps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blkio_device_write_i_ops: Option<Vec<ThrottleDevice>>,
    #[serde(rename = "BlkioWeight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blkio_weight: Option<u16>,
    #[serde(rename = "BlkioWeightDevice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blkio_weight_device: Option<Vec<WeightDevice>>,
    #[serde(rename = "CgroupParent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Applicable to UNIX platforms
    pub cgroup_parent: Option<String>,
    #[serde(rename = "CpuCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Applicable to Windows
    pub cpu_count: Option<i64>,
    #[serde(rename = "CpuPercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_percent: Option<i64>,
    #[serde(rename = "CpuPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_period: Option<i64>,
    #[serde(rename = "CpuQuota")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_quota: Option<i64>,
    #[serde(rename = "CpuRealtimePeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_realtime_period: Option<i64>,
    #[serde(rename = "CpuRealtimeRuntime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_realtime_runtime: Option<i64>,
    #[serde(rename = "CpuShares")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Applicable to all platforms
    pub cpu_shares: Option<i64>,
    #[serde(rename = "CpusetCpus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpuset_cpus: Option<String>,
    #[serde(rename = "CpusetMems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpuset_mems: Option<String>,
    #[serde(rename = "DeviceCgroupRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_cgroup_rules: Option<Vec<String>>,
    #[serde(rename = "DeviceRequests")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_requests: Option<Vec<DeviceRequest>>,
    #[serde(rename = "Devices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<DeviceMapping>>,
    #[serde(rename = "IOMaximumBandwidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub io_maximum_bandwidth: Option<u64>,
    #[serde(rename = "IOMaximumIOps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub io_maximum_i_ops: Option<u64>,
    #[serde(rename = "KernelMemory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// KernelMemory specifies the kernel memory limit (in bytes) for the container.
    /// Deprecated: kernel 5.4 deprecated kmem.limit_in_bytes.
    pub kernel_memory: Option<i64>,
    #[serde(rename = "KernelMemoryTCP")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kernel_memory_tcp: Option<i64>,
    #[serde(rename = "Memory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<i64>,
    #[serde(rename = "MemoryReservation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_reservation: Option<i64>,
    #[serde(rename = "MemorySwap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_swap: Option<i64>,
    #[serde(rename = "MemorySwappiness")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_swappiness: Option<i64>,
    #[serde(rename = "NanoCpus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nano_cpus: Option<i64>,
    #[serde(rename = "OomKillDisable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oom_kill_disable: Option<bool>,
    #[serde(rename = "PidsLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pids_limit: Option<i64>,
    #[serde(rename = "RestartPolicy")]
    pub restart_policy: Option<RestartPolicy>,
    #[serde(rename = "Ulimits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ulimits: Option<Vec<Ulimit>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateContainerDevicesLimits {
    #[serde(rename = "BlkIOWeightDevice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Block IO weight (relative device weight) in the form:
    /// ```[{"Path": "device_path", "Weight": weight}]```
    pub blk_io_weight_device: Option<Vec<WeightDevice>>,
    #[serde(rename = "DeviceReadBPs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Limit read rate (bytes per second) from a device, in the form:
    /// ```[{"Path": "device_path", "Rate": rate}]```
    pub device_read_b_ps: Option<Vec<ThrottleDevice>>,
    #[serde(rename = "DeviceReadIOPs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Limit read rate (IO per second) from a device, in the form:
    /// ```[{"Path": "device_path", "Rate": rate}]```
    pub device_read_io_ps: Option<Vec<ThrottleDevice>>,
    #[serde(rename = "DeviceWriteBPs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Limit write rate (bytes per second) to a device, in the form:
    /// ```[{"Path": "device_path", "Rate": rate}]```
    pub device_write_b_ps: Option<Vec<ThrottleDevice>>,
    #[serde(rename = "DeviceWriteIOPs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Limit write rate (IO per second) to a device, in the form:
    /// ```[{"Path": "device_path", "Rate": rate}]```
    pub device_write_io_ps: Option<Vec<ThrottleDevice>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// UpdateEntities used to wrap the oci resource spec in a swagger model
pub struct UpdateEntities {
    #[serde(rename = "BlkIOWeightDevice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Block IO weight (relative device weight) in the form:
    /// ```[{"Path": "device_path", "Weight": weight}]```
    pub blk_io_weight_device: Option<Vec<WeightDevice>>,
    #[serde(rename = "DeviceReadBPs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Limit read rate (bytes per second) from a device, in the form:
    /// ```[{"Path": "device_path", "Rate": rate}]```
    pub device_read_b_ps: Option<Vec<ThrottleDevice>>,
    #[serde(rename = "DeviceReadIOPs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Limit read rate (IO per second) from a device, in the form:
    /// ```[{"Path": "device_path", "Rate": rate}]```
    pub device_read_io_ps: Option<Vec<ThrottleDevice>>,
    #[serde(rename = "DeviceWriteBPs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Limit write rate (bytes per second) to a device, in the form:
    /// ```[{"Path": "device_path", "Rate": rate}]```
    pub device_write_b_ps: Option<Vec<ThrottleDevice>>,
    #[serde(rename = "DeviceWriteIOPs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Limit write rate (IO per second) to a device, in the form:
    /// ```[{"Path": "device_path", "Rate": rate}]```
    pub device_write_io_ps: Option<Vec<ThrottleDevice>>,
    #[serde(rename = "blockIO")]
    pub block_io: Option<LinuxBlockIo>,
    pub cpu: Option<LinuxCpu>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Devices configures the device allowlist.
    pub devices: Option<Vec<LinuxDeviceCgroup>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// HealthCmd set a healthcheck command for the container. ('none' disables the existing healthcheck)
    pub health_cmd: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// HealthInterval set an interval for the healthcheck.
    /// (a value of disable results in no automatic timer setup) Changing this setting resets timer.
    pub health_interval: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// HealthLogDestination set the destination of the HealthCheck log.
    /// Directory path, local or events_logger (local use container state file)
    /// Warning: Changing this setting may cause the loss of previous logs!
    pub health_log_destination: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// HealthMaxLogCount set maximum number of attempts in the HealthCheck log file.
    /// ('0' value means an infinite number of attempts in the log file)
    pub health_max_log_count: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// HealthMaxLogSize set maximum length in characters of stored HealthCheck log.
    /// ('0' value means an infinite log length)
    pub health_max_log_size: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// HealthOnFailure set the action to take once the container turns unhealthy.
    pub health_on_failure: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// HealthRetries set the number of retries allowed before a healthcheck is considered to be unhealthy.
    pub health_retries: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// HealthStartPeriod set the initialization time needed for a container to bootstrap.
    pub health_start_period: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// HealthStartupCmd set a startup healthcheck command for the container.
    pub health_startup_cmd: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// HealthStartupInterval set an interval for the startup healthcheck.
    /// Changing this setting resets the timer, depending on the state of the container.
    pub health_startup_interval: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// HealthStartupRetries set the maximum number of retries before the startup healthcheck will restart the container.
    pub health_startup_retries: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// HealthStartupSuccess set the number of consecutive successes before the startup healthcheck is marked as successful
    /// and the normal healthcheck begins (0 indicates any success will start the regular healthcheck)
    pub health_startup_success: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// HealthStartupTimeout set the maximum amount of time that the startup healthcheck may take before it is considered failed.
    pub health_startup_timeout: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// HealthTimeout set the maximum time allowed to complete the healthcheck before an interval is considered failed.
    pub health_timeout: Option<String>,
    #[serde(rename = "hugepageLimits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Hugetlb limits (in bytes). Default to reservation limits if supported.
    pub hugepage_limits: Option<Vec<LinuxHugepageLimit>>,
    pub memory: Option<LinuxMemory>,
    pub network: Option<LinuxNetwork>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Disable healthchecks on container.
    pub no_healthcheck: Option<bool>,
    pub pids: Option<LinuxPids>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Rdma resource restriction configuration.
    /// Limits are a set of key value pairs that define RDMA resource limits,
    /// where the key is device name and value is resource limits.
    pub rdma: Option<HashMap<String, LinuxRdma>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Unified resources.
    pub unified: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateHealthCheckConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// HealthCmd set a healthcheck command for the container. ('none' disables the existing healthcheck)
    pub health_cmd: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// HealthInterval set an interval for the healthcheck.
    /// (a value of disable results in no automatic timer setup) Changing this setting resets timer.
    pub health_interval: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// HealthLogDestination set the destination of the HealthCheck log.
    /// Directory path, local or events_logger (local use container state file)
    /// Warning: Changing this setting may cause the loss of previous logs!
    pub health_log_destination: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// HealthMaxLogCount set maximum number of attempts in the HealthCheck log file.
    /// ('0' value means an infinite number of attempts in the log file)
    pub health_max_log_count: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// HealthMaxLogSize set maximum length in characters of stored HealthCheck log.
    /// ('0' value means an infinite log length)
    pub health_max_log_size: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// HealthOnFailure set the action to take once the container turns unhealthy.
    pub health_on_failure: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// HealthRetries set the number of retries allowed before a healthcheck is considered to be unhealthy.
    pub health_retries: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// HealthStartPeriod set the initialization time needed for a container to bootstrap.
    pub health_start_period: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// HealthStartupCmd set a startup healthcheck command for the container.
    pub health_startup_cmd: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// HealthStartupInterval set an interval for the startup healthcheck.
    /// Changing this setting resets the timer, depending on the state of the container.
    pub health_startup_interval: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// HealthStartupRetries set the maximum number of retries before the startup healthcheck will restart the container.
    pub health_startup_retries: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// HealthStartupSuccess set the number of consecutive successes before the startup healthcheck is marked as successful
    /// and the normal healthcheck begins (0 indicates any success will start the regular healthcheck)
    pub health_startup_success: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// HealthStartupTimeout set the maximum amount of time that the startup healthcheck may take before it is considered failed.
    pub health_startup_timeout: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// HealthTimeout set the maximum time allowed to complete the healthcheck before an interval is considered failed.
    pub health_timeout: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Disable healthchecks on container.
    pub no_healthcheck: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// UsageData Usage details about the volume. This information is used by the
/// `GET /system/df` endpoint, and omitted in other endpoints.
pub struct UsageData {
    #[serde(rename = "RefCount")]
    /// The number of containers referencing this volume. This field
    /// is set to `-1` if the reference-count is not available.
    pub ref_count: i64,
    #[serde(rename = "Size")]
    /// Amount of disk space used by the volume (in bytes). This information
    /// is only available for volumes created with the `"local"` volume
    /// driver. For volumes created with other volume drivers, this field
    /// is set to `-1` ("not available")
    pub size: i64,
}

pub type UsernsMode = String;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Version {
    #[serde(rename = "Index")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Volume volume
pub struct Volume {
    #[serde(rename = "ClusterVolume")]
    pub cluster_volume: Option<ClusterVolume>,
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Date/Time the volume was created.
    pub created_at: Option<String>,
    #[serde(rename = "Driver")]
    /// Name of the volume driver used by the volume.
    pub driver: String,
    #[serde(rename = "Labels")]
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_nonoptional_map")]
    /// User-defined key/value metadata.
    pub labels: HashMap<String, String>,
    #[serde(rename = "Mountpoint")]
    /// Mount path of the volume on the host.
    pub mountpoint: String,
    #[serde(rename = "Name")]
    /// Name of the volume.
    pub name: String,
    #[serde(rename = "Options")]
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_nonoptional_map")]
    /// The driver specific options used when creating the volume.
    pub options: HashMap<String, String>,
    #[serde(rename = "Scope")]
    /// The level at which the volume exists. Either `global` for cluster-wide,
    /// or `local` for machine level.
    pub scope: String,
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Low-level details about the volume, provided by the volume driver.
    /// Details are returned as a map with key/value pairs:
    /// `{"key":"value","key2":"value2"}`.
    ///
    /// The `Status` field is optional, and is omitted if the volume driver
    /// does not support this feature.
    pub status: Option<Value>,
    #[serde(rename = "UsageData")]
    pub usage_data: Option<UsageData>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VolumeConfigResponse {
    #[serde(rename = "Anonymous")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Anonymous indicates that the volume was created as an anonymous
    /// volume for a specific container, and will be removed when any
    /// container using it is removed.
    pub anonymous: Option<bool>,
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// CreatedAt is the date and time the volume was created at. This is not
    /// stored for older Libpod volumes; if so, it will be omitted.
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "Driver")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Driver is the driver used to create the volume.
    /// If set to "local" or "", the Local driver (Podman built-in code) is
    /// used to service the volume; otherwise, a volume plugin with the given
    /// name is used to mount and manage the volume.
    pub driver: Option<String>,
    #[serde(rename = "GID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// GID is the GID that the volume was created with.
    pub gid: Option<i64>,
    #[serde(rename = "Labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Labels includes the volume's configured labels, key:value pairs that
    /// can be passed during volume creation to provide information for third
    /// party tools.
    pub labels: Option<HashMap<String, String>>,
    #[serde(rename = "LockNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// LockNumber is the number of the volume's Libpod lock.
    pub lock_number: Option<u32>,
    #[serde(rename = "MountCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// MountCount is the number of times this volume has been mounted.
    pub mount_count: Option<u64>,
    #[serde(rename = "Mountpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Mountpoint is the path on the host where the volume is mounted.
    pub mountpoint: Option<String>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Name is the name of the volume.
    pub name: Option<String>,
    #[serde(rename = "NeedsChown")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// NeedsChown indicates that the next time the volume is mounted into
    /// a container, the container will chown the volume to the container process
    /// UID/GID.
    pub needs_chown: Option<bool>,
    #[serde(rename = "NeedsCopyUp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// NeedsCopyUp indicates that the next time the volume is mounted into
    pub needs_copy_up: Option<bool>,
    #[serde(rename = "Options")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Options is a set of options that were used when creating the volume.
    /// For the Local driver, these are mount options that will be used to
    /// determine how a local filesystem is mounted; they are handled as
    /// parameters to Mount in a manner described in the volume create
    /// manpage.
    /// For non-local drivers, these are passed as-is to the volume plugin.
    pub options: Option<HashMap<String, String>>,
    #[serde(rename = "Scope")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Scope is unused and provided solely for Docker compatibility. It is
    /// unconditionally set to "local".
    pub scope: Option<String>,
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Status is used to return information on the volume's current state,
    /// if the volume was created using a volume plugin (uses a Driver that
    /// is not the local driver).
    /// Status is provided to us by an external program, so no guarantees are
    /// made about its format or contents. Further, it is an optional field,
    /// so it may not be set even in cases where a volume plugin is in use.
    pub status: Option<Value>,
    #[serde(rename = "StorageID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// StorageID is the ID of the container backing the volume in c/storage.
    /// Only used with Image Volumes.
    pub storage_id: Option<String>,
    #[serde(rename = "Timeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Timeout is the specified driver timeout if given
    pub timeout: Option<u64>,
    #[serde(rename = "UID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// UID is the UID that the volume was created with.
    pub uid: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VolumeCreateOptions {
    #[serde(rename = "Driver")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Volume driver to use
    pub driver: Option<String>,
    #[serde(rename = "IgnoreIfExists")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Ignore existing volumes
    pub ignore_if_exists: Option<bool>,
    #[serde(rename = "Label")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// User-defined key/value metadata. Provided for compatibility
    pub label: Option<HashMap<String, String>>,
    #[serde(rename = "Labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// User-defined key/value metadata. Preferred field, will override Label
    pub labels: Option<HashMap<String, String>>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// New volume's name. Can be left blank
    pub name: Option<String>,
    #[serde(rename = "Options")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Mapping of driver options and values.
    pub options: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VolumeOptions {
    #[serde(rename = "DriverConfig")]
    pub driver_config: Option<Driver>,
    #[serde(rename = "Labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<HashMap<String, String>>,
    #[serde(rename = "NoCopy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_copy: Option<bool>,
    #[serde(rename = "Subpath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subpath: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VolumeRmReport {
    #[serde(rename = "Err")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub err: Option<String>,
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// WaitExitError container waiting error, if any
pub struct WaitExitError {
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Details of an error
    pub message: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// OK response to ContainerWait operation
pub struct WaitResponse {
    #[serde(rename = "Error")]
    pub error: Option<WaitExitError>,
    #[serde(rename = "StatusCode")]
    /// Exit code of the container
    pub status_code: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// WeightDevice is a structure that holds device:weight pair
pub struct WeightDevice {
    #[serde(rename = "Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "Weight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<u16>,
}

pub type CacheLibImage = Value;

/// Remove Containers
pub type ContainerRemoveLibpod = Vec<LibpodContainersRmReport>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Container update
pub struct ContainerUpdateRequest {
    #[serde(rename = "BlkioDeviceReadBps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blkio_device_read_bps: Option<Vec<ThrottleDevice>>,
    #[serde(rename = "BlkioDeviceReadIOps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blkio_device_read_i_ops: Option<Vec<ThrottleDevice>>,
    #[serde(rename = "BlkioDeviceWriteBps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blkio_device_write_bps: Option<Vec<ThrottleDevice>>,
    #[serde(rename = "BlkioDeviceWriteIOps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blkio_device_write_i_ops: Option<Vec<ThrottleDevice>>,
    #[serde(rename = "BlkioWeight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blkio_weight: Option<u16>,
    #[serde(rename = "BlkioWeightDevice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blkio_weight_device: Option<Vec<WeightDevice>>,
    #[serde(rename = "CgroupParent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Applicable to UNIX platforms
    pub cgroup_parent: Option<String>,
    #[serde(rename = "CpuCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Applicable to Windows
    pub cpu_count: Option<i64>,
    #[serde(rename = "CpuPercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_percent: Option<i64>,
    #[serde(rename = "CpuPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_period: Option<i64>,
    #[serde(rename = "CpuQuota")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_quota: Option<i64>,
    #[serde(rename = "CpuRealtimePeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_realtime_period: Option<i64>,
    #[serde(rename = "CpuRealtimeRuntime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_realtime_runtime: Option<i64>,
    #[serde(rename = "CpuShares")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Applicable to all platforms
    pub cpu_shares: Option<i64>,
    #[serde(rename = "CpusetCpus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpuset_cpus: Option<String>,
    #[serde(rename = "CpusetMems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpuset_mems: Option<String>,
    #[serde(rename = "DeviceCgroupRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_cgroup_rules: Option<Vec<String>>,
    #[serde(rename = "DeviceRequests")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_requests: Option<Vec<DeviceRequest>>,
    #[serde(rename = "Devices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<DeviceMapping>>,
    #[serde(rename = "IOMaximumBandwidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub io_maximum_bandwidth: Option<u64>,
    #[serde(rename = "IOMaximumIOps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub io_maximum_i_ops: Option<u64>,
    #[serde(rename = "KernelMemory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// KernelMemory specifies the kernel memory limit (in bytes) for the container.
    /// Deprecated: kernel 5.4 deprecated kmem.limit_in_bytes.
    pub kernel_memory: Option<i64>,
    #[serde(rename = "KernelMemoryTCP")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kernel_memory_tcp: Option<i64>,
    #[serde(rename = "Memory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<i64>,
    #[serde(rename = "MemoryReservation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_reservation: Option<i64>,
    #[serde(rename = "MemorySwap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_swap: Option<i64>,
    #[serde(rename = "MemorySwappiness")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_swappiness: Option<i64>,
    #[serde(rename = "NanoCpus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nano_cpus: Option<i64>,
    #[serde(rename = "OomKillDisable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oom_kill_disable: Option<bool>,
    #[serde(rename = "PidsLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pids_limit: Option<i64>,
    #[serde(rename = "RestartPolicy")]
    pub restart_policy: Option<RestartPolicy>,
    #[serde(rename = "Ulimits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ulimits: Option<Vec<Ulimit>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Update container
pub struct ContainerUpdateResponse {
    #[serde(rename = "ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Wait container
pub struct ContainerWaitResponse {
    #[serde(rename = "Error")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ContainerWaitResponseErrorInlineItem>,
    #[serde(rename = "StatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// container exit code
    pub status_code: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContainerWaitResponseErrorInlineItem {
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// List Containers
pub type ContainersList = Vec<Container>;

/// List Containers
pub type ContainersListLibpod = Vec<ListContainer>;

/// Prune Containers
pub type ContainersPrune = Vec<ContainersPruneReport>;

/// Prune Containers
pub type ContainersPruneLibpod = Vec<ContainersPruneReportLibpod>;

/// Image Delete
pub type ImageDeleteResponse = Vec<ImageDeleteResponseInlineItem>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImageDeleteResponseInlineItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub untagged: Option<Vec<String>>,
}

/// Image summary for compat API
pub type ImageList = Vec<Summary>;

/// Image summary for libpod API
pub type ImageListLibpod = Vec<LibpodImageSummary>;

/// Image Prune
pub type ImagesPruneLibpod = Vec<PruneReport>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Network create
pub struct NetworkCreateLibpod {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Created contains the timestamp when this network was created.
    pub created: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// DNSEnabled is whether name resolution is active for container on
    /// this Network. Only supported with the bridge driver.
    pub dns_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Driver for this Network, e.g. bridge, macvlan...
    pub driver: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// ID of the Network.
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Internal is whether the Network should not have external routes
    /// to public or other Networks.
    pub internal: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// IPAMOptions contains options used for the ip assignment.
    pub ipam_options: Option<HashMap<String, String>>,
    #[serde(rename = "ipv6_enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// IPv6Enabled if set to true an ipv6 subnet should be created for this net.
    pub ipv_6_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Labels is a set of key-value labels that have been applied to the
    /// Network.
    pub labels: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Name of the Network.
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// List of custom DNS server for podman's DNS resolver at network level,
    /// all the containers attached to this network will consider resolvers
    /// configured at network level.
    pub network_dns_servers: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// NetworkInterface is the network interface name on the host.
    pub network_interface: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Options is a set of key-value options that have been applied to
    /// the Network.
    pub options: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Routes to use for this network.
    pub routes: Option<Vec<Route>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Subnets to use for this network.
    pub subnets: Option<Vec<Subnet>>,
}

/// Network list
pub type NetworkListCompat = Vec<Inspect>;

/// Network list
pub type NetworkListLibpod = Vec<Network>;

/// Network prune
pub type NetworkPruneResponse = Vec<NetworkPruneReport>;

/// Network Delete
pub type NetworkRmResponse = Vec<NetworkRmReport>;

/// Success
pub type Ok = Value;

/// Pod Statistics
pub type PodStatsResponse = Vec<PodStatsReport>;

/// List pods
pub type PodsListResponse = Vec<ListPodsReport>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Registry Search
pub struct RegistrySearchResponse {
    #[serde(rename = "Automated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Automated indicates if the image was created by an automated build.
    pub automated: Option<String>,
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Description of the image.
    pub description: Option<String>,
    #[serde(rename = "Index")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Index is the image index
    pub index: Option<String>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Name is the canonical name of the image
    pub name: Option<String>,
    #[serde(rename = "Official")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Official indicates if it's an official image.
    pub official: Option<String>,
    #[serde(rename = "Stars")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Stars is the number of stars of the image.
    pub stars: Option<i64>,
    #[serde(rename = "Tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Tag is the image tag
    pub tag: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Details for creating a volume
pub struct VolumeCreate {
    #[serde(rename = "Driver")]
    /// Name of the volume driver to use.
    pub driver: String,
    #[serde(rename = "DriverOpts")]
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_nonoptional_map")]
    /// A mapping of driver options and values. These options are
    /// passed directly to the driver and are driver specific.
    pub driver_opts: HashMap<String, String>,
    #[serde(rename = "Labels")]
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_nonoptional_map")]
    /// User-defined key/value metadata.
    pub labels: HashMap<String, String>,
    #[serde(rename = "Name")]
    /// The new volume's name. If not specified, Docker generates a name.
    pub name: String,
}

/// Volume list
pub type VolumeListLibpod = Vec<VolumeConfigResponse>;

/// Volume Prune
pub type VolumePruneLibpod = Vec<PruneReport>;

/// CreateContainerConfig used when compatible endpoint creates a container
pub type ContainerCreateBodyParam = CreateContainerConfig;

/// SpecGenerator creates an OCI spec and Libpod configuration options to create
/// a container based on the given configuration.
pub type ContainerCreateLibpodCreateParam = SpecGenerator;

/// UpdateEntities used to wrap the oci resource spec in a swagger model
pub type ContainerUpdateLibpodConfigParam = UpdateEntities;

/// Container update
pub type ContainerUpdateResourcesParam = ContainerUpdateRequest;

/// ManifestAddOptions provides model for adding digests to manifest list
pub type ManifestAddLibpodOptionsParam = ManifestAddOptions;

/// swagger 2.0 does not support oneOf for schema validation.
///
/// Operation "update" uses all fields.
/// Operation "remove" uses fields: Operation and Images
/// Operation "annotate" uses fields: Operation and Annotations
pub type ManifestCreateLibpodOptionsParam = ManifestModifyOptions;

/// swagger 2.0 does not support oneOf for schema validation.
///
/// Operation "update" uses all fields.
/// Operation "remove" uses fields: Operation and Images
/// Operation "annotate" uses fields: Operation and Annotations
pub type ManifestModifyLibpodOptionsParam = ManifestModifyOptions;

/// Network create
pub type NetworkCreateLibpodCreateParam = NetworkCreateLibpod;

/// PodSpecGenerator describes options to create a pod
pub type PodCreateLibpodCreateParam = PodSpecGenerator;

pub type SecretCreateCreateParam = SecretCreate;

pub type SecretInspectCompatResponse = SecretInfoReportCompat;

pub type SecretInspectResponse = SecretInfoReport;

/// AuthConfig contains authorization information for connecting to a Registry
pub type SystemAuthAuthConfigParam = AuthConfig;

/// Details for creating a volume
pub type VolumeCreateCreateParam = VolumeCreate;

pub type VolumeCreateLibpodCreateParam = VolumeCreateOptions;

/// ErrorModel is used in remote connections with podman
pub type BadParamError = ErrorModel;

/// ErrorModel is used in remote connections with podman
pub type ConflictError = ErrorModel;

/// ErrorModel is used in remote connections with podman
pub type ContainerAlreadyStartedError = ErrorModel;

/// ErrorModel is used in remote connections with podman
pub type ContainerAlreadyStoppedError = ErrorModel;

/// ContainerJSON is newly used struct along with MountPoint
pub type ContainerInspectResponse = ContainerJson;

/// InspectContainerData provides a detailed record of a container's configuration
/// and state as viewed by Libpod.
/// Large portions of this structure are defined such that the output is
/// compatible with `docker inspect` JSON, but additional fields have been added
/// as required to share information not in the original output.
pub type ContainerInspectResponseLibpod = InspectContainerData;

/// ErrorModel is used in remote connections with podman
pub type ContainerNotFound = ErrorModel;

/// ContainerTopOKBody OK response to ContainerTop operation
pub type ContainerTopResponse = ContainerTopOkBody;

pub type ExecSessionInspect = InspectExecSession;

/// ErrorModel is used in remote connections with podman
pub type ExecSessionNotFound = ErrorModel;

/// HealthCheckResults describes the results/logs from a healthcheck
pub type HealthCheck = HealthCheckResults;

/// ErrorModel is used in remote connections with podman
pub type ImageNotFound = ErrorModel;

pub type ImagesImportResponseLibpod = ImageImportReport;

pub type ImagesLoadResponseLibpod = ImageLoadReport;

pub type ImagesPullResponseLibpod = LibpodImagesPullReport;

/// LibpodImagesRemoveReport is the return type for image removal via the rest
/// api.
pub type ImagesRemoveResponseLibpod = LibpodImagesRemoveReport;

pub type ImagesScpResponseLibpod = ScpReport;

/// Info is the overall struct that describes the host system
/// running libpod/podman
pub type InfoResponse = LibpodInfo;

pub type InspectImageResponseLibpod = ImageData;

/// ErrorModel is used in remote connections with podman
pub type InternalError = ErrorModel;

/// This is publicly visible as c/image/manifest.Schema2List.
/// Internal users should usually use Schema2List instead.
pub type ManifestInspect = Schema2ListPublic;

/// ErrorModel is used in remote connections with podman
pub type ManifestNotFound = ErrorModel;

/// ConnectOptions represents the data to be used to connect a container to the
/// network.
pub type NetworkConnectRequest = ConnectOptions;

/// NetworkConnectOptions describes options for connecting
/// a container to a network
pub type NetworkConnectRequestLibpod = NetworkConnectOptions;

/// ErrorModel is used in remote connections with podman
pub type NetworkConnectedError = ErrorModel;

pub type NetworkCreate = CreateRequest;

pub type NetworkCreateResponse = Network;

/// DisconnectOptions represents the data to be used to disconnect a container
/// from the network.
pub type NetworkDisconnectRequest = DisconnectOptions;

pub type NetworkInspectCompat = Inspect;

pub type NetworkInspectResponse = NetworkInspectReport;

/// ErrorModel is used in remote connections with podman
pub type NetworkNotFound = ErrorModel;

/// NetworkUpdateOptions describes options to update a network
pub type NetworkUpdateRequestLibpod = NetworkUpdateOptions;

pub type PlayKubeResponseLibpod = PlayKubeReport;

/// ErrorModel is used in remote connections with podman
pub type PodAlreadyStartedError = ErrorModel;

/// ErrorModel is used in remote connections with podman
pub type PodAlreadyStoppedError = ErrorModel;

/// InspectPodData contains detailed information on a pod's configuration and
/// state. It is used as the output of Inspect on pods.
pub type PodInspectResponse = InspectPodData;

pub type PodKillResponse = PodKillReport;

/// ErrorModel is used in remote connections with podman
pub type PodNotFound = ErrorModel;

pub type PodPauseResponse = PodPauseReport;

pub type PodPruneResponse = PodPruneReport;

pub type PodRestartResponse = PodRestartReport;

pub type PodRmResponse = PodRmReport;

pub type PodStartResponse = PodStartReport;

pub type PodStopResponse = PodStopReport;

pub type PodTopResponse = PodTopOkBody;

pub type PodUnpauseResponse = PodUnpauseReport;

/// AuthReport describes the response for authentication check
pub type SystemAuthResponse = AuthReport;

/// SystemCheckReport provides a report of what a storage consistency check
/// found, and if we removed anything that was damaged, what we removed.
pub type SystemCheckResponse = SystemCheckReport;

/// SystemDfReport describes the response for df information
pub type SystemDiskUsage = SystemDfReport;

pub type SystemPruneResponse = SystemPruneReport;

pub type TreeResponse = ImageTreeReport;

/// SystemComponentVersion is the type used by pkg/domain/entities
pub type VersionResponse = SystemComponentVersion;

pub type VolumeCreateResponse = VolumeConfigResponse;

/// Volume volume
pub type VolumeInspect = Volume;

/// Volume list response
pub type VolumeList = ListResponse;

/// ErrorModel is used in remote connections with podman
pub type VolumeNotFound = ErrorModel;

/// POST "/volumes/prune"
pub type VolumePruneResponse = PruneReport;
