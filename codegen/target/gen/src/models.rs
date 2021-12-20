#![allow(
    non_snake_case,
    clippy::redundant_field_names,
    clippy::new_without_default,
    clippy::too_many_arguments
)]

use serde::{
    de::{DeserializeOwned, Deserializer},
    Deserialize, Serialize,
};
use serde_json::Value;

use std::collections::HashMap;

use chrono::DateTime;
use chrono::Utc;

fn deserialize_nonoptional_vec<'de, D: Deserializer<'de>, T: DeserializeOwned>(
    d: D,
) -> Result<Vec<T>, D::Error> {
    Deserialize::deserialize(d).map(|x: Option<_>| x.unwrap_or_default())
}

fn deserialize_nonoptional_map<'de, D: Deserializer<'de>, T: DeserializeOwned>(
    d: D,
) -> Result<HashMap<String, T>, D::Error> {
    Deserialize::deserialize(d).map(|x: Option<_>| x.unwrap_or_default())
}

/// Address represents an IP address
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Address {
    #[serde(rename = "Addr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addr: Option<String>,
    #[serde(rename = "PrefixLen")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix_len: Option<i64>,
}

/// AuthConfig contains authorization information for connecting to a Registry
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct AuthConfig {
    #[serde(rename = "auth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth: Option<String>,
    /// Email is an optional value associated with the username. This field is deprecated and will be removed in a later version of docker.
    #[serde(rename = "email")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// IdentityToken is used to authenticate the user and get an access token for the registry.
    #[serde(rename = "identitytoken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identitytoken: Option<String>,
    #[serde(rename = "password")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// RegistryToken is a bearer token to be sent to a registry
    #[serde(rename = "registrytoken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registrytoken: Option<String>,
    #[serde(rename = "serveraddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serveraddress: Option<String>,
    #[serde(rename = "username")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

/// AuthenticateOKBody authenticate o k body
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct AuthenticateOkBody {
    /// An opaque token used to authenticate a user after a successful login
    #[serde(rename = "IdentityToken")]
    pub identity_token: String,
    /// The status of the authentication
    #[serde(rename = "Status")]
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct AutoUserNsOptions {
    /// AdditionalGIDMappings specified additional GID mappings to include in the generated user namespace.
    #[serde(rename = "AdditionalGIDMappings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_gid_mappings: Option<Vec<IdMap>>,
    /// AdditionalUIDMappings specified additional UID mappings to include in the generated user namespace.
    #[serde(rename = "AdditionalUIDMappings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_uid_mappings: Option<Vec<IdMap>>,
    /// GroupFile to use if the container uses a volume.
    #[serde(rename = "GroupFile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_file: Option<String>,
    /// InitialSize defines the minimum size for the user namespace. The created user namespace will have at least this size.
    #[serde(rename = "InitialSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_size: Option<i64>,
    /// PasswdFile to use if the container uses a volume.
    #[serde(rename = "PasswdFile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passwd_file: Option<String>,
    /// Size defines the size for the user namespace.  If it is set to a value bigger than 0, the user namespace will have exactly this size. If it is not set, some heuristics will be used to find its size.
    #[serde(rename = "Size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct BindOptions {
    #[serde(rename = "NonRecursive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_recursive: Option<bool>,
    #[serde(rename = "Propagation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub propagation: Option<Propagation>,
}

pub type CgroupSpec = String;

/// CgroupnsMode represents the cgroup namespace mode of the container
pub type CgroupnsMode = String;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
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

/// It should hold only portable information about the container. Here, \"portable\" means \"independent from the host we are running on\". Non-portable information *should* appear in HostConfig. All fields added to this struct must be marked `omitempty` to keep getting predictable hashes from the old `v1Compatibility` configuration.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cmd: Option<StrSlice>,
    #[serde(rename = "Domainname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domainname: Option<String>,
    #[serde(rename = "Entrypoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entrypoint: Option<StrSlice>,
    #[serde(rename = "Env")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<Vec<String>>,
    #[serde(rename = "ExposedPorts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exposed_ports: Option<PortSet>,
    #[serde(rename = "Healthcheck")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub healthcheck: Option<HealthConfig>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
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

/// ConfigReference specifies the source which provides a network's configuration
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ConfigReference {
    #[serde(rename = "Network")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,
}

/// ConmonInfo describes the conmon executable being used
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ConmonInfo {
    #[serde(rename = "package")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package: Option<String>,
    #[serde(rename = "path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

pub type Consistency = String;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ContainerBasicConfig {
    /// Annotations are key-value options passed into the container runtime that can be used to trigger special behavior. Optional.
    #[serde(rename = "annotations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<HashMap<String, String>>,
    /// Command is the container's command. If not given and Image is specified, this will be populated by the image's configuration. Optional.
    #[serde(rename = "command")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<Vec<String>>,
    /// ConmonPidFile is a path at which a PID file for Conmon will be placed. If not given, a default location will be used. Optional.
    #[serde(rename = "conmon_pid_file")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conmon_pid_file: Option<String>,
    /// ContainerCreateCommand is the command that was used to create this container. This will be shown in the output of Inspect() on the container, and may also be used by some tools that wish to recreate the container (e.g. `podman generate systemd --new`). Optional.
    #[serde(rename = "containerCreateCommand")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_create_command: Option<Vec<String>>,
    /// DependencyContainers is an array of containers this container depends on. Dependency containers must be started before this container. Dependencies can be specified by name or full/partial ID. Optional.
    #[serde(rename = "dependencyContainers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependency_containers: Option<Vec<String>>,
    /// Entrypoint is the container's entrypoint. If not given and Image is specified, this will be populated by the image's configuration. Optional.
    #[serde(rename = "entrypoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entrypoint: Option<Vec<String>>,
    /// Env is a set of environment variables that will be set in the container. Optional.
    #[serde(rename = "env")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<HashMap<String, String>>,
    /// EnvHost indicates that the host environment should be added to container Optional.
    #[serde(rename = "env_host")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env_host: Option<bool>,
    /// Hostname is the container's hostname. If not set, the hostname will not be modified (if UtsNS is not private) or will be set to the container ID (if UtsNS is private). Conflicts with UtsNS if UtsNS is not set to private. Optional.
    #[serde(rename = "hostname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    /// EnvHTTPProxy indicates that the http host proxy environment variables should be added to container Optional.
    #[serde(rename = "httpproxy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub httpproxy: Option<bool>,
    /// InitContainerType describes if this container is an init container and if so, what type: always or once
    #[serde(rename = "init_container_type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub init_container_type: Option<String>,
    /// Labels are key-value pairs that are used to add metadata to containers. Optional.
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<HashMap<String, String>>,
    #[serde(rename = "log_configuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_configuration: Option<LogConfig>,
    /// Name is the name the container will be given. If no name is provided, one will be randomly generated. Optional.
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Namespace is the libpod namespace the container will be placed in. Optional.
    #[serde(rename = "namespace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// OCIRuntime is the name of the OCI runtime that will be used to create the container. If not specified, the default will be used. Optional.
    #[serde(rename = "oci_runtime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oci_runtime: Option<String>,
    #[serde(rename = "personality")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personality: Option<LinuxPersonality>,
    #[serde(rename = "pidns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pidns: Option<Namespace>,
    /// Pod is the ID of the pod the container will join. Optional.
    #[serde(rename = "pod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pod: Option<String>,
    /// RawImageName is the user-specified and unprocessed input referring to a local or a remote image.
    #[serde(rename = "raw_image_name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_image_name: Option<String>,
    /// Remove indicates if the container should be removed once it has been started and exits
    #[serde(rename = "remove")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove: Option<bool>,
    /// RestartPolicy is the container's restart policy - an action which will be taken when the container exits. If not given, the default policy, which does nothing, will be used. Optional.
    #[serde(rename = "restart_policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restart_policy: Option<String>,
    /// RestartRetries is the number of attempts that will be made to restart the container. Only available when RestartPolicy is set to \"on-failure\". Optional.
    #[serde(rename = "restart_tries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restart_tries: Option<i64>,
    /// Determine how to handle the NOTIFY_SOCKET - do we participate or pass it through \"container\" - let the OCI runtime deal with it, advertise conmon's MAINPID \"conmon-only\" - advertise conmon's MAINPID, send READY when started, don't pass to OCI \"ignore\" - unset NOTIFY_SOCKET
    #[serde(rename = "sdnotifyMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sdnotify_mode: Option<String>,
    /// EnvSecrets are secrets that will be set as environment variables Optional.
    #[serde(rename = "secret_env")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_env: Option<HashMap<String, String>>,
    /// Stdin is whether the container will keep its STDIN open.
    #[serde(rename = "stdin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stdin: Option<bool>,
    #[serde(rename = "stop_signal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_signal: Option<Signal>,
    /// StopTimeout is a timeout between the container's stop signal being sent and SIGKILL being sent. If not provided, the default will be used. If 0 is used, stop signal will not be sent, and SIGKILL will be sent instead. Optional.
    #[serde(rename = "stop_timeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_timeout: Option<i64>,
    /// Sysctl sets kernel parameters for the container
    #[serde(rename = "sysctl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sysctl: Option<HashMap<String, String>>,
    /// Systemd is whether the container will be started in systemd mode. Valid options are \"true\", \"false\", and \"always\". \"true\" enables this mode only if the binary run in the container is sbin/init or systemd. \"always\" unconditionally enables systemd mode. \"false\" unconditionally disables systemd mode. If enabled, mounts and stop signal will be modified. If set to \"always\" or set to \"true\" and conditionally triggered, conflicts with StopSignal. If not specified, \"false\" will be assumed. Optional.
    #[serde(rename = "systemd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub systemd: Option<String>,
    /// Terminal is whether the container will create a PTY. Optional.
    #[serde(rename = "terminal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminal: Option<bool>,
    /// Timeout is a maximum time in seconds the container will run before main process is sent SIGKILL. If 0 is used, signal will not be sent. Container can run indefinitely Optional.
    #[serde(rename = "timeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i64>,
    /// Timezone is the timezone inside the container. Local means it has the same timezone as the host machine Optional.
    #[serde(rename = "timezone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    #[serde(rename = "utsns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utsns: Option<Namespace>,
}

/// ContainerCgroupConfig contains configuration information about a container's cgroups.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ContainerCgroupConfig {
    /// CgroupParent is the container's CGroup parent. If not set, the default for the current cgroup driver will be used. Optional.
    #[serde(rename = "cgroup_parent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cgroup_parent: Option<String>,
    #[serde(rename = "cgroupns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cgroupns: Option<Namespace>,
    /// CgroupsMode sets a policy for how cgroups will be created in the container, including the ability to disable creation entirely.
    #[serde(rename = "cgroups_mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cgroups_mode: Option<String>,
}

/// ContainerChangeResponseItem change item in response to ContainerChanges operation
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ContainerChangeResponseItem {
    /// Kind of change
    #[serde(rename = "Kind")]
    pub kind: i64,
    /// Path to file that has changed
    #[serde(rename = "Path")]
    pub path: String,
}

/// ContainerCreateCreatedBody OK response to ContainerCreate operation
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ContainerCreateCreatedBody {
    /// The ID of the created container
    #[serde(rename = "Id")]
    pub id: String,
    /// Warnings encountered when creating the container
    #[serde(rename = "Warnings")]
    #[serde(deserialize_with = "deserialize_nonoptional_vec")]
    pub warnings: Vec<String>,
}

/// ContainerHealthCheckConfig describes a container healthcheck with attributes like command, retries, interval, start period, and timeout.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ContainerHealthCheckConfig {
    #[serde(rename = "healthconfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub healthconfig: Option<Schema2HealthConfig>,
}

/// ContainerNetworkConfig contains information on a container's network configuration.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ContainerNetworkConfig {
    /// Aliases are a list of network-scoped aliases for container Optional
    #[serde(rename = "aliases")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliases: Option<HashMap<String, Vec<String>>>,
    /// CNINetworks is a list of CNI networks to join the container to. If this list is empty, the default CNI network will be joined instead. If at least one entry is present, we will not join the default network (unless it is part of this list). Only available if NetNS is set to bridge. Optional.
    #[serde(rename = "cni_networks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cni_networks: Option<Vec<String>>,
    /// DNSOptions is a set of DNS options that will be used in the container's resolv.conf, replacing the host's DNS options which are used by default. Conflicts with UseImageResolvConf. Optional.
    #[serde(rename = "dns_option")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_option: Option<Vec<String>>,
    /// DNSSearch is a set of DNS search domains that will be used in the container's resolv.conf, replacing the host's DNS search domains which are used by default. Conflicts with UseImageResolvConf. Optional.
    #[serde(rename = "dns_search")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_search: Option<Vec<String>>,
    /// DNSServers is a set of DNS servers that will be used in the container's resolv.conf, replacing the host's DNS Servers which are used by default. Conflicts with UseImageResolvConf. Optional.
    #[serde(rename = "dns_server")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_server: Option<Vec<Ip>>,
    /// Expose is a number of ports that will be forwarded to the container if PublishExposedPorts is set. Expose is a map of uint16 (port number) to a string representing protocol. Allowed protocols are \"tcp\", \"udp\", and \"sctp\", or some combination of the three separated by commas. If protocol is set to \"\" we will assume TCP. Only available if NetNS is set to Bridge or Slirp, and PublishExposedPorts is set. Optional.
    #[serde(rename = "expose")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expose: Option<Value>,
    /// HostAdd is a set of hosts which will be added to the container's etc/hosts file. Conflicts with UseImageHosts. Optional.
    #[serde(rename = "hostadd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostadd: Option<Vec<String>>,
    #[serde(rename = "netns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub netns: Option<Namespace>,
    /// NetworkOptions are additional options for each network Optional.
    #[serde(rename = "network_options")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_options: Option<HashMap<String, Vec<String>>>,
    /// PortBindings is a set of ports to map into the container. Only available if NetNS is set to bridge or slirp. Optional.
    #[serde(rename = "portmappings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portmappings: Option<Vec<PortMapping>>,
    /// PublishExposedPorts will publish ports specified in the image to random unused ports (guaranteed to be above 1024) on the host. This is based on ports set in Expose below, and any ports specified by the Image (if one is given). Only available if NetNS is set to Bridge or Slirp.
    #[serde(rename = "publish_image_ports")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publish_image_ports: Option<bool>,
    #[serde(rename = "static_ip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub static_ip: Option<Ip>,
    #[serde(rename = "static_ipv6")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub static_ipv6: Option<Ip>,
    #[serde(rename = "static_mac")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub static_mac: Option<HardwareAddr>,
    /// UseImageHosts indicates that /etc/hosts should not be managed by Podman, and instead sourced from the image. Conflicts with HostAdd.
    #[serde(rename = "use_image_hosts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_image_hosts: Option<bool>,
    /// UseImageResolvConf indicates that resolv.conf should not be managed by Podman, but instead sourced from the image. Conflicts with DNSServer, DNSSearch, DNSOption.
    #[serde(rename = "use_image_resolve_conf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_image_resolve_conf: Option<bool>,
}

/// ContainerNode stores information about the node that a container is running on.  It's only used by the Docker Swarm standalone API
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ContainerNode {
    #[serde(rename = "Addr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addr: Option<String>,
    #[serde(rename = "Cpus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpus: Option<i64>,
    #[serde(rename = "ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ID: Option<String>,
    #[serde(rename = "IP")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub IP: Option<String>,
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

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ContainerResourceConfig {
    /// CPU period of the cpuset, determined by --cpus
    #[serde(rename = "cpu_period")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_period: Option<i64>,
    /// CPU quota of the cpuset, determined by --cpus
    #[serde(rename = "cpu_quota")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_quota: Option<i64>,
    /// OOMScoreAdj adjusts the score used by the OOM killer to determine processes to kill for the container's process. Optional.
    #[serde(rename = "oom_score_adj")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oom_score_adj: Option<i64>,
    /// Rlimits are POSIX rlimits to apply to the container. Optional.
    #[serde(rename = "r_limits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r_limits: Option<Vec<PosixRlimit>>,
    #[serde(rename = "resource_limits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_limits: Option<LinuxResources>,
    /// IO read rate limit per cgroup per device, bytes per second
    #[serde(rename = "throttleReadBpsDevice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throttle_read_bps_device: Option<HashMap<String, LinuxThrottleDevice>>,
    /// IO read rate limit per cgroup per device, IO per second
    #[serde(rename = "throttleReadIOPSDevice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throttle_read_iops_device: Option<HashMap<String, LinuxThrottleDevice>>,
    /// IO write rate limit per cgroup per device, bytes per second
    #[serde(rename = "throttleWriteBpsDevice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throttle_write_bps_device: Option<HashMap<String, LinuxThrottleDevice>>,
    /// IO write rate limit per cgroup per device, IO per second
    #[serde(rename = "throttleWriteIOPSDevice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throttle_write_iops_device: Option<HashMap<String, LinuxThrottleDevice>>,
    /// CgroupConf are key-value options passed into the container runtime that are used to configure cgroup v2. Optional.
    #[serde(rename = "unified")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unified: Option<HashMap<String, String>>,
    /// Weight per cgroup per device, can override BlkioWeight
    #[serde(rename = "weightDevice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight_device: Option<HashMap<String, LinuxWeightDevice>>,
}

/// ContainerSecurityConfig is a container's security features, including SELinux, Apparmor, and Seccomp.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ContainerSecurityConfig {
    /// ApparmorProfile is the name of the Apparmor profile the container will use. Optional.
    #[serde(rename = "apparmor_profile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apparmor_profile: Option<String>,
    /// CapAdd are capabilities which will be added to the container. Conflicts with Privileged. Optional.
    #[serde(rename = "cap_add")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cap_add: Option<Vec<String>>,
    /// CapDrop are capabilities which will be removed from the container. Conflicts with Privileged. Optional.
    #[serde(rename = "cap_drop")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cap_drop: Option<Vec<String>>,
    /// Groups are a list of supplemental groups the container's user will be granted access to. Optional.
    #[serde(rename = "groups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<String>>,
    #[serde(rename = "idmappings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idmappings: Option<IdMappingOptions>,
    /// Mask is the path we want to mask in the container. This masks the paths given in addition to the default list. Optional
    #[serde(rename = "mask")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask: Option<Vec<String>>,
    /// NoNewPrivileges is whether the container will set the no new privileges flag on create, which disables gaining additional privileges (e.g. via setuid) in the container.
    #[serde(rename = "no_new_privileges")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_new_privileges: Option<bool>,
    /// Privileged is whether the container is privileged. Privileged does the following: Adds all devices on the system to the container. Adds all capabilities to the container. Disables Seccomp, SELinux, and Apparmor confinement. (Though SELinux can be manually re-enabled). TODO: this conflicts with things. TODO: this does more.
    #[serde(rename = "privileged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privileged: Option<bool>,
    /// ProcOpts are the options used for the proc mount.
    #[serde(rename = "procfs_opts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub procfs_opts: Option<Vec<String>>,
    /// ReadOnlyFilesystem indicates that everything will be mounted as read-only
    #[serde(rename = "read_only_filesystem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only_filesystem: Option<bool>,
    /// SeccompPolicy determines which seccomp profile gets applied the container. valid values: empty,default,image
    #[serde(rename = "seccomp_policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seccomp_policy: Option<String>,
    /// SeccompProfilePath is the path to a JSON file containing the container's Seccomp profile. If not specified, no Seccomp profile will be used. Optional.
    #[serde(rename = "seccomp_profile_path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seccomp_profile_path: Option<String>,
    /// SelinuxProcessLabel is the process label the container will use. If SELinux is enabled and this is not specified, a label will be automatically generated if not specified. Optional.
    #[serde(rename = "selinux_opts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selinux_opts: Option<Vec<String>>,
    /// Umask is the umask the init process of the container will be run with.
    #[serde(rename = "umask")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub umask: Option<String>,
    /// Unmask is the path we want to unmask in the container. To override all the default paths that are masked, set unmask=ALL.
    #[serde(rename = "unmask")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unmask: Option<Vec<String>>,
    /// User is the user the container will be run as. Can be given as a UID or a username; if a username, it will be resolved within the container, using the container's /etc/passwd. If unset, the container will be run as root. Optional.
    #[serde(rename = "user")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    #[serde(rename = "userns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub userns: Option<Namespace>,
}

/// ContainerSize holds the size of the container's root filesystem and top read-write layer.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ContainerSize {
    #[serde(rename = "rootFsSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_fs_size: Option<i64>,
    #[serde(rename = "rwSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rw_size: Option<i64>,
}

/// ContainerState stores container's running state it's part of ContainerJSONBase and will return by \"inspect\" command
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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

/// ContainerStorageConfig contains information on the storage configuration of a container.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ContainerStorageConfig {
    /// Create the working directory if it doesn't exist. If unset, it doesn't create it. Optional.
    #[serde(rename = "create_working_dir")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_working_dir: Option<bool>,
    /// DeviceCGroupRule are device cgroup rules that allow containers to use additional types of devices.
    #[serde(rename = "device_cgroup_rule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_cgroup_rule: Option<Vec<LinuxDeviceCgroup>>,
    /// Devices are devices that will be added to the container. Optional.
    #[serde(rename = "devices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<LinuxDevice>>,
    /// Image is the image the container will be based on. The image will be used as the container's root filesystem, and its environment vars, volumes, and other configuration will be applied to the container. Conflicts with Rootfs. At least one of Image or Rootfs must be specified.
    #[serde(rename = "image")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// ImageVolumeMode indicates how image volumes will be created. Supported modes are \"ignore\" (do not create), \"tmpfs\" (create as tmpfs), and \"anonymous\" (create as anonymous volumes). The default if unset is anonymous. Optional.
    #[serde(rename = "image_volume_mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_volume_mode: Option<String>,
    /// Image volumes bind-mount a container-image mount into the container. Optional.
    #[serde(rename = "image_volumes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_volumes: Option<Vec<ImageVolume>>,
    /// Init specifies that an init binary will be mounted into the container, and will be used as PID1.
    #[serde(rename = "init")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub init: Option<bool>,
    /// InitPath specifies the path to the init binary that will be added if Init is specified above. If not specified, the default set in the Libpod config will be used. Ignored if Init above is not set. Optional.
    #[serde(rename = "init_path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub init_path: Option<String>,
    #[serde(rename = "ipcns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipcns: Option<Namespace>,
    /// Mounts are mounts that will be added to the container. These will supersede Image Volumes and VolumesFrom volumes where there are conflicts. Optional.
    #[serde(rename = "mounts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mounts: Option<Vec<Mount>>,
    /// Overlay volumes are named volumes that will be added to the container. Optional.
    #[serde(rename = "overlay_volumes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overlay_volumes: Option<Vec<OverlayVolume>>,
    /// Rootfs is the path to a directory that will be used as the container's root filesystem. No modification will be made to the directory, it will be directly mounted into the container as root. Conflicts with Image. At least one of Image or Rootfs must be specified.
    #[serde(rename = "rootfs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rootfs: Option<String>,
    /// RootfsPropagation is the rootfs propagation mode for the container. If not set, the default of rslave will be used. Optional.
    #[serde(rename = "rootfs_propagation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rootfs_propagation: Option<String>,
    /// Secrets are the secrets that will be added to the container Optional.
    #[serde(rename = "secrets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets: Option<Vec<Secret>>,
    /// ShmSize is the size of the tmpfs to mount in at /dev/shm, in bytes. Conflicts with ShmSize if IpcNS is not private. Optional.
    #[serde(rename = "shm_size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shm_size: Option<i64>,
    /// Volatile specifies whether the container storage can be optimized at the cost of not syncing all the dirty files in memory.
    #[serde(rename = "volatile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volatile: Option<bool>,
    /// Volumes are named volumes that will be added to the container. These will supersede Image Volumes and VolumesFrom volumes where there are conflicts. Optional.
    #[serde(rename = "volumes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes: Option<Vec<NamedVolume>>,
    /// VolumesFrom is a set of containers whose volumes will be added to this container. The name or ID of the container must be provided, and may optionally be followed by a : and then one or more comma-separated options. Valid options are 'ro', 'rw', and 'z'. Options will be used for all volumes sourced from the container.
    #[serde(rename = "volumes_from")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes_from: Option<Vec<String>>,
    /// WorkDir is the container's working directory. If unset, the default, /, will be used. Optional.
    #[serde(rename = "work_dir")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_dir: Option<String>,
}

/// ContainerStore describes the quantity of containers in the store by status
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ContainerStore {
    #[serde(rename = "number")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<i64>,
    #[serde(rename = "paused")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paused: Option<i64>,
    #[serde(rename = "running")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub running: Option<i64>,
    #[serde(rename = "stopped")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stopped: Option<i64>,
}

/// ContainerTopOKBody OK response to ContainerTop operation
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ContainerTopOkBody {
    /// Each process running in the container, where each is process is an array of values corresponding to the titles.
    #[serde(rename = "Processes")]
    #[serde(deserialize_with = "deserialize_nonoptional_vec")]
    pub processes: Vec<Vec<String>>,
    /// The ps column titles
    #[serde(rename = "Titles")]
    #[serde(deserialize_with = "deserialize_nonoptional_vec")]
    pub titles: Vec<String>,
}

/// ContainerUpdateOKBody OK response to ContainerUpdate operation
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ContainerUpdateOkBody {
    /// warnings
    #[serde(rename = "Warnings")]
    #[serde(deserialize_with = "deserialize_nonoptional_vec")]
    pub warnings: Vec<String>,
}

/// ContainerWaitOKBody OK response to ContainerWait operation
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ContainerWaitOkBody {
    #[serde(rename = "Error")]
    pub error: ContainerWaitOkBodyError,
    /// Exit code of the container
    #[serde(rename = "StatusCode")]
    pub status_code: i64,
}

/// ContainerWaitOKBodyError container waiting error, if any
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ContainerWaitOkBodyError {
    /// Details of an error
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ContainersPruneReport {
    #[serde(rename = "ContainersDeleted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub containers_deleted: Option<Vec<String>>,
    #[serde(rename = "SpaceReclaimed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub space_reclaimed: Option<i64>,
}

/// CreateContainerConfig used when compatible endpoint creates a container
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cmd: Option<StrSlice>,
    #[serde(rename = "Domainname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domainname: Option<String>,
    #[serde(rename = "Entrypoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entrypoint: Option<StrSlice>,
    #[serde(rename = "Env")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<Vec<String>>,
    #[serde(rename = "ExposedPorts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exposed_ports: Option<PortSet>,
    #[serde(rename = "Healthcheck")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub healthcheck: Option<HealthConfig>,
    #[serde(rename = "HostConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    pub mac_address: Option<String>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "NetworkDisabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_disabled: Option<bool>,
    #[serde(rename = "NetworkingConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub networking_config: Option<NetworkingConfig>,
    #[serde(rename = "OnBuild")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_build: Option<Vec<String>>,
    #[serde(rename = "OpenStdin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_stdin: Option<bool>,
    #[serde(rename = "Shell")]
    #[serde(skip_serializing_if = "Option::is_none")]
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

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
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

/// Used by GPU device drivers.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
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

/// The following is an example of the contents of Digest types:  sha256:7173b809ca12ec5dee4506cd86be934c4596dd234ee82c0662eac04a8c2c71dc  This allows to abstract the digest behind this type and work only in those terms.
pub type Digest = String;

/// DistributionInfo describes the host distribution for libpod
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct DistributionInfo {
    #[serde(rename = "codename")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codename: Option<String>,
    #[serde(rename = "distribution")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distribution: Option<String>,
    #[serde(rename = "variant")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant: Option<String>,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// DNS contains values interesting for DNS resolvers
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Dns {
    #[serde(rename = "domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(rename = "nameservers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nameservers: Option<Vec<String>>,
    #[serde(rename = "options")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<String>>,
    #[serde(rename = "search")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<Vec<String>>,
}

pub type DockerVolumeCreate = VolumeCreateBody;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Driver {
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Options")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<HashMap<String, String>>,
}

/// DriverData handles the data for a storage driver
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct DriverData {
    #[serde(rename = "Data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<HashMap<String, String>>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// A Duration represents the elapsed time between two instants as an int64 nanosecond count. The representation limits the largest representable duration to approximately 290 years.
pub type Duration = i64;

/// EndpointIPAMConfig represents IPAM configurations for the endpoint
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct EndpointIpamConfig {
    #[serde(rename = "IPv4Address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_pv4_address: Option<String>,
    #[serde(rename = "IPv6Address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_pv6_address: Option<String>,
    #[serde(rename = "LinkLocalIPs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_local_i_ps: Option<Vec<String>>,
}

/// EndpointResource contains network resources allocated and used for a container in a network
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct EndpointResource {
    #[serde(rename = "EndpointID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_id: Option<String>,
    #[serde(rename = "IPv4Address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_pv4_address: Option<String>,
    #[serde(rename = "IPv6Address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_pv6_address: Option<String>,
    #[serde(rename = "MacAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// EndpointSettings stores the network endpoint details
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct EndpointSettings {
    #[serde(rename = "Aliases")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Vec<String>>,
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
    pub global_i_pv6_address: Option<String>,
    #[serde(rename = "GlobalIPv6PrefixLen")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_i_pv6_prefix_len: Option<i64>,
    #[serde(rename = "IPAMConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipam_config: Option<EndpointIpamConfig>,
    #[serde(rename = "IPAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[serde(rename = "IPPrefixLen")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_prefix_len: Option<i64>,
    #[serde(rename = "IPv6Gateway")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_pv6_gateway: Option<String>,
    #[serde(rename = "Links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<String>>,
    #[serde(rename = "MacAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>,
    /// Operational data
    #[serde(rename = "NetworkID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ErrorResponse {
    /// The error message.
    #[serde(rename = "message")]
    pub message: String,
}

/// The bits have the same definition on all systems, so that information about files can be moved from one system to another portably. Not all bits apply to all systems. The only required bit is ModeDir for directories.
pub type FileMode = i32;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GraphDriverData {
    /// data
    #[serde(rename = "Data")]
    #[serde(deserialize_with = "deserialize_nonoptional_map")]
    pub data: HashMap<String, String>,
    /// name
    #[serde(rename = "Name")]
    pub name: String,
}

pub type HardwareAddr = Vec<i32>;

/// Health stores information about the container's healthcheck results
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
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

/// HealthCheckLog describes the results of a single healthcheck
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct HealthCheckLog {
    /// End time as a string
    #[serde(rename = "End")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
    /// Exitcode is 0 or 1
    #[serde(rename = "ExitCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exit_code: Option<i64>,
    /// Output is the stdout/stderr from the healthcheck command
    #[serde(rename = "Output")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
    /// Start time as string
    #[serde(rename = "Start")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,
}

/// HealthCheckResults describes the results/logs from a healthcheck
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct HealthCheckResults {
    /// FailingStreak is the number of consecutive failed healthchecks
    #[serde(rename = "FailingStreak")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failing_streak: Option<i64>,
    /// Log describes healthcheck attempts and results
    #[serde(rename = "Log")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log: Option<Vec<HealthCheckLog>>,
    /// Status healthy or unhealthy
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct HealthConfig {
    #[serde(rename = "Interval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<Duration>,
    /// Retries is the number of consecutive failures needed to consider a container as unhealthy. Zero means inherit.
    #[serde(rename = "Retries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retries: Option<i64>,
    #[serde(rename = "StartPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_period: Option<Duration>,
    /// Test is the test to perform to check that the container is healthy. An empty slice means to inherit the default. The options are: {} : inherit healthcheck {\"NONE\"} : disable healthcheck {\"CMD\", args...} : exec arguments directly {\"CMD-SHELL\", command} : run command with system's default shell
    #[serde(rename = "Test")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test: Option<Vec<String>>,
    #[serde(rename = "Timeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<Duration>,
}

/// HealthcheckResult stores information about a single run of a healthcheck probe
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
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

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct History {
    /// Author is the author of the build point.
    #[serde(rename = "author")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    /// Comment is a custom message set when creating the layer.
    #[serde(rename = "comment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// Created is the combined date and time at which the layer was created, formatted as defined by RFC 3339, section 5.6.
    #[serde(rename = "created")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<DateTime<Utc>>,
    /// CreatedBy is the command which created the layer.
    #[serde(rename = "created_by")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    /// EmptyLayer is used to mark if the history item created a filesystem diff.
    #[serde(rename = "empty_layer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub empty_layer: Option<bool>,
}

/// HistoryResponseItem individual image layer information in response to ImageHistory operation
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct HistoryResponseItem {
    /// comment
    #[serde(rename = "Comment")]
    pub comment: String,
    /// created
    #[serde(rename = "Created")]
    pub created: i64,
    /// created by
    #[serde(rename = "CreatedBy")]
    pub created_by: String,
    /// Id
    #[serde(rename = "Id")]
    pub id: String,
    /// size
    #[serde(rename = "Size")]
    pub size: i64,
    /// tags
    #[serde(rename = "Tags")]
    #[serde(deserialize_with = "deserialize_nonoptional_vec")]
    pub tags: Vec<String>,
}

/// Here, \"non-portable\" means \"dependent of the host we are running on\". Portable information *should* appear in Config.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct HostConfig {
    #[serde(rename = "AutoRemove")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_remove: Option<bool>,
    /// Applicable to all platforms
    #[serde(rename = "Binds")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    pub blkio_weight: Option<i64>,
    #[serde(rename = "BlkioWeightDevice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blkio_weight_device: Option<Vec<WeightDevice>>,
    #[serde(rename = "CapAdd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cap_add: Option<StrSlice>,
    #[serde(rename = "CapDrop")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cap_drop: Option<StrSlice>,
    #[serde(rename = "Cgroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cgroup: Option<CgroupSpec>,
    /// Applicable to UNIX platforms
    #[serde(rename = "CgroupParent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cgroup_parent: Option<String>,
    #[serde(rename = "CgroupnsMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cgroupns_mode: Option<CgroupnsMode>,
    /// Applicable to Windows
    #[serde(rename = "ConsoleSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub console_size: Option<Vec<i64>>,
    #[serde(rename = "ContainerIDFile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_id_file: Option<String>,
    /// Applicable to Windows
    #[serde(rename = "CpuCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    /// Applicable to all platforms
    #[serde(rename = "CpuShares")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    pub io_maximum_bandwidth: Option<i64>,
    #[serde(rename = "IOMaximumIOps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub io_maximum_i_ops: Option<i64>,
    /// Run a custom init inside the container, if null, use the daemon's configured settings
    #[serde(rename = "Init")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub init: Option<bool>,
    #[serde(rename = "IpcMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipc_mode: Option<IpcMode>,
    #[serde(rename = "Isolation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isolation: Option<Isolation>,
    #[serde(rename = "KernelMemory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kernel_memory: Option<i64>,
    #[serde(rename = "KernelMemoryTCP")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kernel_memory_tcp: Option<i64>,
    #[serde(rename = "Links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<String>>,
    #[serde(rename = "LogConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_config: Option<LogConfig>,
    /// MaskedPaths is the list of paths to be masked inside the container (this overrides the default set of paths)
    #[serde(rename = "MaskedPaths")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    /// Mounts specs used by the container
    #[serde(rename = "Mounts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mounts: Option<Vec<Mount>>,
    #[serde(rename = "NanoCpus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nano_cpus: Option<i64>,
    #[serde(rename = "NetworkMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_mode: Option<NetworkMode>,
    #[serde(rename = "OomKillDisable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oom_kill_disable: Option<bool>,
    #[serde(rename = "OomScoreAdj")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oom_score_adj: Option<i64>,
    #[serde(rename = "PidMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pid_mode: Option<PidMode>,
    #[serde(rename = "PidsLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pids_limit: Option<i64>,
    #[serde(rename = "PortBindings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_bindings: Option<PortMap>,
    #[serde(rename = "Privileged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privileged: Option<bool>,
    #[serde(rename = "PublishAllPorts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publish_all_ports: Option<bool>,
    /// ReadonlyPaths is the list of paths to be set as read-only inside the container (this overrides the default set of paths)
    #[serde(rename = "ReadonlyPaths")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readonly_paths: Option<Vec<String>>,
    #[serde(rename = "ReadonlyRootfs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readonly_rootfs: Option<bool>,
    #[serde(rename = "RestartPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uts_mode: Option<UtsMode>,
    #[serde(rename = "Ulimits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ulimits: Option<Vec<Ulimit>>,
    #[serde(rename = "UsernsMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub userns_mode: Option<UsernsMode>,
    #[serde(rename = "VolumeDriver")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_driver: Option<String>,
    #[serde(rename = "VolumesFrom")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes_from: Option<Vec<String>>,
}

/// HostInfo describes the libpod host
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct HostInfo {
    #[serde(rename = "arch")]
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
    #[serde(rename = "conmon")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conmon: Option<ConmonInfo>,
    #[serde(rename = "cpus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpus: Option<i64>,
    #[serde(rename = "distribution")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distribution: Option<DistributionInfo>,
    #[serde(rename = "eventLogger")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_logger: Option<String>,
    #[serde(rename = "hostname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    #[serde(rename = "idMappings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_mappings: Option<IdMappings>,
    #[serde(rename = "kernel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kernel: Option<String>,
    #[serde(rename = "linkmode")]
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
    #[serde(rename = "ociRuntime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oci_runtime: Option<OciRuntimeInfo>,
    #[serde(rename = "os")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,
    #[serde(rename = "remoteSocket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_socket: Option<RemoteSocket>,
    #[serde(rename = "runtimeInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime_info: Option<HashMap<String, Value>>,
    #[serde(rename = "security")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security: Option<SecurityInfo>,
    /// ServiceIsRemote is true when the podman/libpod service is remote to the client
    #[serde(rename = "serviceIsRemote")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_is_remote: Option<bool>,
    #[serde(rename = "slirp4netns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slirp4netns: Option<SlirpInfo>,
    #[serde(rename = "swapFree")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swap_free: Option<i64>,
    #[serde(rename = "swapTotal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swap_total: Option<i64>,
    #[serde(rename = "uptime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uptime: Option<String>,
}

/// IDMap contains a single entry for user namespace range remapping. An array of IDMap entries represents the structure that will be provided to the Linux kernel for creating a user namespace.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct IdMap {
    #[serde(rename = "container_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_id: Option<i64>,
    #[serde(rename = "host_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_id: Option<i64>,
    #[serde(rename = "size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
}

/// IDMappingOptions are used for specifying how ID mapping should be set up for a layer or container.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct IdMappingOptions {
    #[serde(rename = "AutoUserNs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_user_ns: Option<bool>,
    #[serde(rename = "AutoUserNsOpts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_user_ns_opts: Option<AutoUserNsOptions>,
    #[serde(rename = "GIDMap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gid_map: Option<Vec<IdMap>>,
    #[serde(rename = "HostGIDMapping")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_gid_mapping: Option<bool>,
    /// UIDMap and GIDMap are used for setting up a layer's root filesystem for use inside of a user namespace where ID mapping is being used. If HostUIDMapping/HostGIDMapping is true, no mapping of the respective type will be used.  Otherwise, if UIDMap and/or GIDMap contain at least one mapping, one or both will be used.  By default, if neither of those conditions apply, if the layer has a parent layer, the parent layer's mapping will be used, and if it does not have a parent layer, the mapping which was passed to the Store object when it was initialized will be used.
    #[serde(rename = "HostUIDMapping")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_uid_mapping: Option<bool>,
    #[serde(rename = "UIDMap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid_map: Option<Vec<IdMap>>,
}

/// IDMappings describe the GID and UID mappings
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct IdMappings {
    #[serde(rename = "gidmap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gidmap: Option<Vec<IdMap>>,
    #[serde(rename = "uidmap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uidmap: Option<Vec<IdMap>>,
}

/// IDResponse Response to an API call that returns just an Id
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct IdResponse {
    /// The id of the newly created object.
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct IdStartBody {
    /// Detach from the command. Not presently supported.
    #[serde(rename = "Detach")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detach: Option<bool>,
    /// Allocate a pseudo-TTY. Presently ignored.
    #[serde(rename = "Tty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tty: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct IdStartBody1 {
    /// Detach from the command. Not presently supported.
    #[serde(rename = "Detach")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detach: Option<bool>,
    /// Allocate a pseudo-TTY. Presently ignored.
    #[serde(rename = "Tty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tty: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct IdStartBody2 {
    /// Detach from the command.
    #[serde(rename = "Detach")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detach: Option<bool>,
    /// Allocate a pseudo-TTY.
    #[serde(rename = "Tty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tty: Option<bool>,
    /// Height of the TTY session in characters. Tty must be set to true to use it.
    #[serde(rename = "h")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h: Option<i64>,
    /// Width of the TTY session in characters. Tty must be set to true to use it.
    #[serde(rename = "w")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub w: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct IdStartBody3 {
    /// Detach from the command.
    #[serde(rename = "Detach")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detach: Option<bool>,
    /// Allocate a pseudo-TTY.
    #[serde(rename = "Tty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tty: Option<bool>,
    /// Height of the TTY session in characters. Tty must be set to true to use it.
    #[serde(rename = "h")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h: Option<i64>,
    /// Width of the TTY session in characters. Tty must be set to true to use it.
    #[serde(rename = "w")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub w: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ImageConfig {
    /// Cmd defines the default arguments to the entrypoint of the container.
    #[serde(rename = "Cmd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cmd: Option<Vec<String>>,
    /// Entrypoint defines a list of arguments to use as the command to execute when the container starts.
    #[serde(rename = "Entrypoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entrypoint: Option<Vec<String>>,
    /// Env is a list of environment variables to be used in a container.
    #[serde(rename = "Env")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<Vec<String>>,
    /// ExposedPorts a set of ports to expose from a container running this image.
    #[serde(rename = "ExposedPorts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exposed_ports: Option<HashMap<String, Value>>,
    /// Labels contains arbitrary metadata for the container.
    #[serde(rename = "Labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<HashMap<String, String>>,
    /// StopSignal contains the system call signal that will be sent to the container to exit.
    #[serde(rename = "StopSignal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_signal: Option<String>,
    /// User defines the username or UID which the process in the container should run as.
    #[serde(rename = "User")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    /// Volumes is a set of directories describing where the process is likely write data specific to a container instance.
    #[serde(rename = "Volumes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes: Option<HashMap<String, Value>>,
    /// WorkingDir sets the current working directory of the entrypoint process in the container.
    #[serde(rename = "WorkingDir")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_dir: Option<String>,
}

/// ImageDeleteResponseItem image delete response item
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ImageDeleteResponseItem {
    /// The image ID of an image that was deleted
    #[serde(rename = "Deleted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted: Option<String>,
    /// The image ID of an image that was untagged
    #[serde(rename = "Untagged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub untagged: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ImageImportReport {
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ImageLoadReport {
    #[serde(rename = "Names")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<Vec<String>>,
}

/// ImageMetadata contains engine-local data about the image
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ImageMetadata {
    #[serde(rename = "LastTagTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_tag_time: Option<DateTime<Utc>>,
}

/// ImageStore describes the image store.  Right now only the number of images present
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ImageStore {
    #[serde(rename = "number")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<i64>,
}

/// ImageSummary image summary
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ImageSummary {
    /// containers
    #[serde(rename = "Containers")]
    pub containers: i64,
    /// created
    #[serde(rename = "Created")]
    pub created: i64,
    /// Id
    #[serde(rename = "Id")]
    pub id: String,
    /// labels
    #[serde(rename = "Labels")]
    #[serde(deserialize_with = "deserialize_nonoptional_map")]
    pub labels: HashMap<String, String>,
    /// parent Id
    #[serde(rename = "ParentId")]
    pub parent_id: String,
    /// repo digests
    #[serde(rename = "RepoDigests")]
    #[serde(deserialize_with = "deserialize_nonoptional_vec")]
    pub repo_digests: Vec<String>,
    /// repo tags
    #[serde(rename = "RepoTags")]
    #[serde(deserialize_with = "deserialize_nonoptional_vec")]
    pub repo_tags: Vec<String>,
    /// shared size
    #[serde(rename = "SharedSize")]
    pub shared_size: i64,
    /// size
    #[serde(rename = "Size")]
    pub size: i64,
    /// virtual size
    #[serde(rename = "VirtualSize")]
    pub virtual_size: i64,
}

/// ImageVolume is a volume based on a container image.  The container image is first mounted on the host and is then bind-mounted into the container.  An ImageVolume is always mounted read only.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ImageVolume {
    /// Destination is the absolute path of the mount in the container.
    #[serde(rename = "Destination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,
    /// ReadWrite sets the volume writable.
    #[serde(rename = "ReadWrite")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_write: Option<bool>,
    /// Source is the source of the image volume.  The image can be referred to by name and by ID.
    #[serde(rename = "Source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

/// Info is the overall struct that describes the host system running libpod/podman
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Info {
    #[serde(rename = "host")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<HostInfo>,
    #[serde(rename = "plugins")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plugins: Option<Plugins>,
    #[serde(rename = "registries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registries: Option<HashMap<String, Value>>,
    #[serde(rename = "store")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store: Option<StoreInfo>,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<Version>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct InlineResponse200 {
    #[serde(rename = "IdentityToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_token: Option<String>,
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct DockerImageBuildResponse {
    /// output from build process
    #[serde(rename = "stream")]
    pub stream: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct LibpodContainerHealthcheckResponse {
    /// FailingStreak is the number of consecutive failed healthchecks
    #[serde(rename = "FailingStreak")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failing_streak: Option<i64>,
    /// Log describes healthcheck attempts and results
    #[serde(rename = "Log")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log: Option<Vec<HealthCheckLog>>,
    /// Status healthy or unhealthy
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct LibpodContainerInspectResponse {
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(rename = "ExitCommand")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exit_command: Option<Vec<String>>,
    #[serde(rename = "GraphDriver")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graph_driver: Option<DriverData>,
    #[serde(rename = "HostConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(rename = "ImageName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_name: Option<String>,
    #[serde(rename = "IsInfra")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_infra: Option<bool>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    pub restart_count: Option<i64>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<InspectContainerState>,
    #[serde(rename = "StaticDir")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub static_dir: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct LibpodContainerStatsResponse {
    #[serde(rename = "AvgCPU")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avg_cpu: Option<f32>,
    #[serde(rename = "BlockInput")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_input: Option<i64>,
    #[serde(rename = "BlockOutput")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_output: Option<i64>,
    #[serde(rename = "CPU")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub CPU: Option<f32>,
    #[serde(rename = "CPUNano")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_nano: Option<i64>,
    #[serde(rename = "CPUSystemNano")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_system_nano: Option<i64>,
    #[serde(rename = "ContainerID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_id: Option<String>,
    #[serde(rename = "DataPoints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_points: Option<i64>,
    #[serde(rename = "Duration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    #[serde(rename = "MemLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mem_limit: Option<i64>,
    #[serde(rename = "MemPerc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mem_perc: Option<f32>,
    #[serde(rename = "MemUsage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mem_usage: Option<i64>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "NetInput")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_input: Option<i64>,
    #[serde(rename = "NetOutput")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_output: Option<i64>,
    #[serde(rename = "PIDs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pi_ds: Option<i64>,
    #[serde(rename = "PerCPU")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_cpu: Option<Vec<i64>>,
    #[serde(rename = "SystemNano")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_nano: Option<i64>,
    #[serde(rename = "UpTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub up_time: Option<Duration>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct LibpodImageInspectResponse {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<ImageConfig>,
    #[serde(rename = "Created")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<DateTime<Utc>>,
    #[serde(rename = "Digest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest: Option<Digest>,
    #[serde(rename = "GraphDriver")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graph_driver: Option<DriverData>,
    #[serde(rename = "Healthcheck")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct LibpodImageTreeResponse {
    #[serde(rename = "Tree")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tree: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct LibpodPodInspectResponse {
    /// CgroupParent is the parent of the pod's CGroup.
    #[serde(rename = "CgroupParent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cgroup_parent: Option<String>,
    /// CgroupPath is the path to the pod's CGroup.
    #[serde(rename = "CgroupPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cgroup_path: Option<String>,
    /// Containers gives a brief summary of all containers in the pod and their current status.
    #[serde(rename = "Containers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub containers: Option<Vec<InspectPodContainerInfo>>,
    /// CreateCgroup is whether this pod will create its own CGroup to group containers under.
    #[serde(rename = "CreateCgroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_cgroup: Option<bool>,
    /// CreateCommand is the full command plus arguments of the process the container has been created with.
    #[serde(rename = "CreateCommand")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_command: Option<Vec<String>>,
    /// CreateInfra is whether this pod will create an infra container to share namespaces.
    #[serde(rename = "CreateInfra")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_infra: Option<bool>,
    /// Created is the time when the pod was created.
    #[serde(rename = "Created")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<DateTime<Utc>>,
    /// Hostname is the hostname that the pod will set.
    #[serde(rename = "Hostname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    /// ID is the ID of the pod.
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "InfraConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infra_config: Option<InspectPodInfraConfig>,
    /// InfraContainerID is the ID of the pod's infra container, if one is present.
    #[serde(rename = "InfraContainerID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infra_container_id: Option<String>,
    /// Labels is a set of key-value labels that have been applied to the pod.
    #[serde(rename = "Labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<HashMap<String, String>>,
    /// Name is the name of the pod.
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Namespace is the Libpod namespace the pod is placed in.
    #[serde(rename = "Namespace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// NumContainers is the number of containers in the pod, including the infra container.
    #[serde(rename = "NumContainers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_containers: Option<i64>,
    /// SharedNamespaces contains a list of namespaces that will be shared by containers within the pod. Can only be set if CreateInfra is true.
    #[serde(rename = "SharedNamespaces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_namespaces: Option<Vec<String>>,
    /// State represents the current state of the pod.
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct LibpodDataUsageResponse {
    #[serde(rename = "Containers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub containers: Option<Vec<SystemDfContainerReport>>,
    #[serde(rename = "Images")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<SystemDfImageReport>>,
    #[serde(rename = "Volumes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes: Option<Vec<SystemDfVolumeReport>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct LibpodSystemPruneResponse {
    #[serde(rename = "ContainerPruneReports")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_prune_reports: Option<Vec<PruneReport>>,
    #[serde(rename = "ImagePruneReports")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_prune_reports: Option<Vec<PruneReport>>,
    #[serde(rename = "PodPruneReport")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pod_prune_report: Option<Vec<PodPruneReport>>,
    #[serde(rename = "ReclaimedSpace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reclaimed_space: Option<i64>,
    #[serde(rename = "VolumePruneReports")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_prune_reports: Option<Vec<PruneReport>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct LibpodVersionResponse {
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
    pub platform: Option<Value>,
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct LibpodVolumeInspectResponse {
    /// Anonymous indicates that the volume was created as an anonymous volume for a specific container, and will be be removed when any container using it is removed.
    #[serde(rename = "Anonymous")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anonymous: Option<bool>,
    /// CreatedAt is the date and time the volume was created at. This is not stored for older Libpod volumes; if so, it will be omitted.
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
    /// Driver is the driver used to create the volume. If set to \"local\" or \"\", the Local driver (Podman built-in code) is used to service the volume; otherwise, a volume plugin with the given name is used to mount and manage the volume.
    #[serde(rename = "Driver")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver: Option<String>,
    /// GID is the GID that the volume was created with.
    #[serde(rename = "GID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub GID: Option<i64>,
    /// Labels includes the volume's configured labels, key:value pairs that can be passed during volume creation to provide information for third party tools.
    #[serde(rename = "Labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<HashMap<String, String>>,
    /// Mountpoint is the path on the host where the volume is mounted.
    #[serde(rename = "Mountpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mountpoint: Option<String>,
    /// Name is the name of the volume.
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Options is a set of options that were used when creating the volume. For the Local driver, these are mount options that will be used to determine how a local filesystem is mounted; they are handled as parameters to Mount in a manner described in the volume create manpage. For non-local drivers, these are passed as-is to the volume plugin.
    #[serde(rename = "Options")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<HashMap<String, String>>,
    /// Scope is unused and provided solely for Docker compatibility. It is unconditionally set to \"local\".
    #[serde(rename = "Scope")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    /// Status is used to return information on the volume's current state, if the volume was created using a volume plugin (uses a Driver that is not the local driver). Status is provided to us by an external program, so no guarantees are made about its format or contents. Further, it is an optional field, so it may not be set even in cases where a volume plugin is in use.
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<HashMap<String, Value>>,
    /// UID is the UID that the volume was created with.
    #[serde(rename = "UID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub UID: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct InlineResponse2002 {
    #[serde(rename = "AppArmorProfile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_armor_profile: Option<String>,
    #[serde(rename = "Args")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<String>>,
    #[serde(rename = "Config")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graph_driver: Option<GraphDriverData>,
    #[serde(rename = "HostConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_settings: Option<NetworkSettings>,
    #[serde(rename = "Node")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<ContainerState>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct LibpodNetworkCreateResponse {
    #[serde(rename = "Attachable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachable: Option<bool>,
    /// Check for networks with duplicate names. Network is primarily keyed based on a random ID and not on the name. Network name is strictly a user-friendly alias to the network which is uniquely identified using ID. And there is no guaranteed way to check for duplicates. Option CheckDuplicate is there to provide a best effort checking of any networks which has the same name but it is not guaranteed to catch all name collisions.
    #[serde(rename = "CheckDuplicate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_duplicate: Option<bool>,
    #[serde(rename = "ConfigFrom")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_from: Option<ConfigReference>,
    #[serde(rename = "ConfigOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_only: Option<bool>,
    #[serde(rename = "Driver")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver: Option<String>,
    #[serde(rename = "EnableIPv6")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_i_pv6: Option<bool>,
    #[serde(rename = "IPAM")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub IPAM: Option<Ipam>,
    #[serde(rename = "Ingress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingress: Option<bool>,
    #[serde(rename = "Internal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal: Option<bool>,
    #[serde(rename = "Labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<HashMap<String, String>>,
    #[serde(rename = "Options")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<HashMap<String, String>>,
    #[serde(rename = "Scope")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct LibpodNetworksPruneResponse {
    #[serde(rename = "NetworksDeleted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub networks_deleted: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct LibpodVolumeListResponse {
    #[serde(rename = "Volumes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes: Option<Vec<VolumeListOkBody>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct DockerVolumeInfoResponse {
    /// Date/Time the volume was created.
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Name of the volume driver used by the volume.
    #[serde(rename = "Driver")]
    pub driver: String,
    /// User-defined key/value metadata.
    #[serde(rename = "Labels")]
    #[serde(deserialize_with = "deserialize_nonoptional_map")]
    pub labels: HashMap<String, String>,
    /// Mount path of the volume on the host.
    #[serde(rename = "Mountpoint")]
    pub mountpoint: String,
    /// Name of the volume.
    #[serde(rename = "Name")]
    pub name: String,
    /// The driver specific options used when creating the volume.
    #[serde(rename = "Options")]
    #[serde(deserialize_with = "deserialize_nonoptional_map")]
    pub options: HashMap<String, String>,
    /// The level at which the volume exists. Either `global` for cluster-wide, or `local` for machine level.
    #[serde(rename = "Scope")]
    pub scope: String,
    /// Low-level details about the volume, provided by the volume driver. Details are returned as a map with key/value pairs: `{\"key\":\"value\",\"key2\":\"value2\"}`.  The `Status` field is optional, and is omitted if the volume driver does not support this feature.
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<HashMap<String, Value>>,
    #[serde(rename = "UsageData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_data: Option<VolumeUsageData>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct DockerVolumePruneResponse {
    #[serde(rename = "SpaceReclaimed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub space_reclaimed: Option<i64>,
    #[serde(rename = "VolumesDeleted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes_deleted: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct InlineResponse2003 {
    /// Each process running in the container, where each is process is an array of values corresponding to the titles.
    #[serde(rename = "Processes")]
    #[serde(deserialize_with = "deserialize_nonoptional_vec")]
    pub processes: Vec<Vec<String>>,
    /// The ps column titles
    #[serde(rename = "Titles")]
    #[serde(deserialize_with = "deserialize_nonoptional_vec")]
    pub titles: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct InlineResponse2004 {
    #[serde(rename = "Error")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<InlineResponse2004Error>,
    /// container exit code
    #[serde(rename = "StatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct InlineResponse2004Error {
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct InlineResponse2005 {
    #[serde(rename = "deleted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted: Option<String>,
    #[serde(rename = "untagged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub untagged: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct InlineResponse2006 {
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

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct InlineResponse2007 {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<Config>,
    #[serde(rename = "Container")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container: Option<String>,
    #[serde(rename = "ContainerConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_config: Option<Config>,
    #[serde(rename = "Created")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(rename = "DockerVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub docker_version: Option<String>,
    #[serde(rename = "GraphDriver")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graph_driver: Option<GraphDriverData>,
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Metadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ImageMetadata>,
    #[serde(rename = "Os")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,
    #[serde(rename = "OsVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_version: Option<String>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_fs: Option<RootFs>,
    #[serde(rename = "Size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(rename = "Variant")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant: Option<String>,
    #[serde(rename = "VirtualSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_size: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct InlineResponse2008 {
    /// Automated indicates if the image was created by an automated build.
    #[serde(rename = "Automated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automated: Option<String>,
    /// Description of the image.
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Index is the image index (e.g., \"docker.io\" or \"quay.io\")
    #[serde(rename = "Index")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<String>,
    /// Name is the canonical name of the image (e.g., \"docker.io/library/alpine\").
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Official indicates if it's an official image.
    #[serde(rename = "Official")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub official: Option<String>,
    /// Stars is the number of stars of the image.
    #[serde(rename = "Stars")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stars: Option<i64>,
    /// Tag is the image tag
    #[serde(rename = "Tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct LibpodImageBuildResponse {
    /// output from build process
    #[serde(rename = "stream")]
    pub stream: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct InlineResponse201 {
    /// ID of the container created
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Warnings during container creation
    #[serde(rename = "Warnings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct LibpodSecretCreateResponse {
    #[serde(rename = "ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ID: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct InlineResponse500 {
    /// API root cause formatted for automated parsing
    #[serde(rename = "cause")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
    /// human error message, formatted for a human to read
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// http response code
    #[serde(rename = "response")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response: Option<i64>,
}

/// InspectAdditionalNetwork holds information about non-default CNI networks the container has been connected to. As with InspectNetworkSettings, many fields are unused and maintained only for compatibility with Docker.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct InspectAdditionalNetwork {
    /// AdditionalMacAddresses is a set of additional MAC Addresses beyond the first. CNI may configure more than one interface for a single network, which can cause this.
    #[serde(rename = "AdditionalMACAddresses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_mac_addresses: Option<Vec<String>>,
    /// Aliases are any network aliases the container has in this network.
    #[serde(rename = "Aliases")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Vec<String>>,
    /// DriverOpts is presently unused and maintained exclusively for compatibility.
    #[serde(rename = "DriverOpts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver_opts: Option<HashMap<String, String>>,
    /// EndpointID is unused, maintained exclusively for compatibility.
    #[serde(rename = "EndpointID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_id: Option<String>,
    /// Gateway is the IP address of the gateway this network will use.
    #[serde(rename = "Gateway")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway: Option<String>,
    /// GlobalIPv6Address is the global-scope IPv6 Address for this network.
    #[serde(rename = "GlobalIPv6Address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_i_pv6_address: Option<String>,
    /// GlobalIPv6PrefixLen is the length of the subnet mask of this network.
    #[serde(rename = "GlobalIPv6PrefixLen")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_i_pv6_prefix_len: Option<i64>,
    /// IPAMConfig is presently unused and maintained exclusively for compatibility.
    #[serde(rename = "IPAMConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipam_config: Option<HashMap<String, String>>,
    /// IPAddress is the IP address for this network.
    #[serde(rename = "IPAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    /// IPPrefixLen is the length of the subnet mask of this network.
    #[serde(rename = "IPPrefixLen")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_prefix_len: Option<i64>,
    /// IPv6Gateway is the IPv6 gateway this network will use.
    #[serde(rename = "IPv6Gateway")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_pv6_gateway: Option<String>,
    /// Links is presently unused and maintained exclusively for compatibility.
    #[serde(rename = "Links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<String>>,
    /// MacAddress is the MAC address for the interface in this network.
    #[serde(rename = "MacAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>,
    /// Name of the network we're connecting to.
    #[serde(rename = "NetworkID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_id: Option<String>,
    /// SecondaryIPAddresses is a list of extra IP Addresses that the container has been assigned in this network.
    #[serde(rename = "SecondaryIPAddresses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_ip_addresses: Option<Vec<String>>,
    /// SecondaryIPv6Addresses is a list of extra IPv6 Addresses that the container has been assigned in this network.
    #[serde(rename = "SecondaryIPv6Addresses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_i_pv6_addresses: Option<Vec<String>>,
}

/// InspectBlkioThrottleDevice holds information about a speed cap for a device node. This cap applies to a specific operation (read, write, etc) on the given node.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct InspectBlkioThrottleDevice {
    /// Path is the path to the device this applies to.
    #[serde(rename = "Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// Rate is the maximum rate. It is in either bytes per second or iops per second, determined by where it is used - documentation will indicate which is appropriate.
    #[serde(rename = "Rate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate: Option<i64>,
}

/// InspectBlkioWeightDevice holds information about the relative weight of an individual device node. Weights are used in the I/O scheduler to give relative priority to some accesses.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct InspectBlkioWeightDevice {
    /// Path is the path to the device this applies to.
    #[serde(rename = "Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// Weight is the relative weight the scheduler will use when scheduling I/O.
    #[serde(rename = "Weight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<i64>,
}

/// InspectContainerConfig holds further data about how a container was initially configured.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct InspectContainerConfig {
    /// Container annotations
    #[serde(rename = "Annotations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<HashMap<String, String>>,
    /// Unused, at present
    #[serde(rename = "AttachStderr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach_stderr: Option<bool>,
    /// Unused, at present
    #[serde(rename = "AttachStdin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach_stdin: Option<bool>,
    /// Unused, at present
    #[serde(rename = "AttachStdout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach_stdout: Option<bool>,
    /// Container command
    #[serde(rename = "Cmd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cmd: Option<Vec<String>>,
    /// CreateCommand is the full command plus arguments of the process the container has been created with.
    #[serde(rename = "CreateCommand")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_command: Option<Vec<String>>,
    /// Container domain name - unused at present
    #[serde(rename = "Domainname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domainname: Option<String>,
    /// Container entrypoint
    #[serde(rename = "Entrypoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entrypoint: Option<String>,
    /// Container environment variables
    #[serde(rename = "Env")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<Vec<String>>,
    #[serde(rename = "Healthcheck")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub healthcheck: Option<Schema2HealthConfig>,
    /// Container hostname
    #[serde(rename = "Hostname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    /// Container image
    #[serde(rename = "Image")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// Container labels
    #[serde(rename = "Labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<HashMap<String, String>>,
    /// On-build arguments - presently unused. More of Buildah's domain.
    #[serde(rename = "OnBuild")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_build: Option<String>,
    /// Whether the container leaves STDIN open
    #[serde(rename = "OpenStdin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_stdin: Option<bool>,
    /// Secrets are the secrets mounted in the container
    #[serde(rename = "Secrets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets: Option<Vec<InspectSecret>>,
    /// Whether STDIN is only left open once. Presently not supported by Podman, unused.
    #[serde(rename = "StdinOnce")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stdin_once: Option<bool>,
    /// Container stop signal
    #[serde(rename = "StopSignal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_signal: Option<i64>,
    /// StopTimeout is time before container is stopped when calling stop
    #[serde(rename = "StopTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_timeout: Option<i64>,
    /// SystemdMode is whether the container is running in systemd mode. In systemd mode, the container configuration is customized to optimize running systemd in the container.
    #[serde(rename = "SystemdMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub systemd_mode: Option<bool>,
    /// Timeout is time before container is killed by conmon
    #[serde(rename = "Timeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i64>,
    /// Timezone is the timezone inside the container. Local means it has the same timezone as the host machine
    #[serde(rename = "Timezone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    /// Whether the container creates a TTY
    #[serde(rename = "Tty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tty: Option<bool>,
    /// Umask is the umask inside the container.
    #[serde(rename = "Umask")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub umask: Option<String>,
    /// User the container was launched with
    #[serde(rename = "User")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    /// Unused, at present. I've never seen this field populated.
    #[serde(rename = "Volumes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes: Option<HashMap<String, Value>>,
    /// Container working directory
    #[serde(rename = "WorkingDir")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_dir: Option<String>,
}

/// InspectContainerHostConfig holds information used when the container was created. It's very much a Docker-specific struct, retained (mostly) as-is for compatibility. We fill individual fields as best as we can, inferring as much as possible from the spec and container config. Some things cannot be inferred. These will be populated by spec annotations (if available). Field names are fixed for compatibility and cannot be changed. As such, silence lint warnings about them. nolint
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct InspectContainerHostConfig {
    /// AutoRemove is whether the container will be automatically removed on exiting. It is not handled directly within libpod and is stored in an annotation.
    #[serde(rename = "AutoRemove")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_remove: Option<bool>,
    /// Binds contains an array of user-added mounts. Both volume mounts and named volumes are included. Tmpfs mounts are NOT included. In 'docker inspect' this is separated into 'Binds' and 'Mounts' based on how a mount was added. We do not make this distinction and do not include a Mounts field in inspect. Format: <src>:<destination>[:<comma-separated options>]
    #[serde(rename = "Binds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binds: Option<Vec<String>>,
    /// BlkioDeviceReadBps is an array of I/O throttle parameters for individual device nodes. This specifically sets read rate cap in bytes per second for device nodes. As with BlkioWeightDevice, we pull the path from /sys/dev, and we don't guarantee the path will be identical to the original (though the node will be).
    #[serde(rename = "BlkioDeviceReadBps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blkio_device_read_bps: Option<Vec<InspectBlkioThrottleDevice>>,
    /// BlkioDeviceReadIOps is an array of I/O throttle parameters for individual device nodes. This specifically sets the read rate cap in iops per second for device nodes. As with BlkioWeightDevice, we pull the path from /sys/dev, and we don't guarantee the path will be identical to the original (though the node will be).
    #[serde(rename = "BlkioDeviceReadIOps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blkio_device_read_i_ops: Option<Vec<InspectBlkioThrottleDevice>>,
    /// BlkioDeviceWriteBps is an array of I/O throttle parameters for individual device nodes. this specifically sets write rate cap in bytes per second for device nodes. as with BlkioWeightDevice, we pull the path from /sys/dev, and we don't guarantee the path will be identical to the original (though the node will be).
    #[serde(rename = "BlkioDeviceWriteBps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blkio_device_write_bps: Option<Vec<InspectBlkioThrottleDevice>>,
    /// BlkioDeviceWriteIOps is an array of I/O throttle parameters for individual device nodes. This specifically sets the write rate cap in iops per second for device nodes. As with BlkioWeightDevice, we pull the path from /sys/dev, and we don't guarantee the path will be identical to the original (though the node will be).
    #[serde(rename = "BlkioDeviceWriteIOps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blkio_device_write_i_ops: Option<Vec<InspectBlkioThrottleDevice>>,
    /// BlkioWeight indicates the I/O resources allocated to the container. It is a relative weight in the scheduler for assigning I/O time versus other CGroups.
    #[serde(rename = "BlkioWeight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blkio_weight: Option<i64>,
    /// BlkioWeightDevice is an array of I/O resource priorities for individual device nodes. Unfortunately, the spec only stores the device's Major/Minor numbers and not the path, which is used here. Fortunately, the kernel provides an interface for retrieving the path of a given node by major:minor at /sys/dev/. However, the exact path in use may not be what was used in the original CLI invocation - though it is guaranteed that the device node will be the same, and using the given path will be functionally identical.
    #[serde(rename = "BlkioWeightDevice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blkio_weight_device: Option<Vec<InspectBlkioWeightDevice>>,
    /// CapAdd is a list of capabilities added to the container. It is not directly stored by Libpod, and instead computed from the capabilities listed in the container's spec, compared against a set of default capabilities.
    #[serde(rename = "CapAdd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cap_add: Option<Vec<String>>,
    /// CapDrop is a list of capabilities removed from the container. It is not directly stored by libpod, and instead computed from the capabilities listed in the container's spec, compared against a set of default capabilities.
    #[serde(rename = "CapDrop")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cap_drop: Option<Vec<String>>,
    /// Cgroup contains the container's cgroup. It is presently not populated. TODO.
    #[serde(rename = "Cgroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cgroup: Option<String>,
    /// CgroupConf is the configuration for cgroup v2.
    #[serde(rename = "CgroupConf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cgroup_conf: Option<HashMap<String, String>>,
    /// CgroupManager is the cgroup manager used by the container. At present, allowed values are either \"cgroupfs\" or \"systemd\".
    #[serde(rename = "CgroupManager")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cgroup_manager: Option<String>,
    /// CgroupMode is the configuration of the container's cgroup namespace. Populated as follows: private - a cgroup namespace has been created host - No cgroup namespace created container:<id> - Using another container's cgroup namespace ns:<path> - A path to a cgroup namespace has been specified
    #[serde(rename = "CgroupMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cgroup_mode: Option<String>,
    /// CgroupParent is the CGroup parent of the container. Only set if not default.
    #[serde(rename = "CgroupParent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cgroup_parent: Option<String>,
    /// Cgroups contains the container's CGroup mode. Allowed values are \"default\" (container is creating CGroups) and \"disabled\" (container is not creating CGroups). This is Libpod-specific and not included in `docker inspect`.
    #[serde(rename = "Cgroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cgroups: Option<String>,
    /// ConsoleSize is an array of 2 integers showing the size of the container's console. It is only set if the container is creating a terminal. TODO.
    #[serde(rename = "ConsoleSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub console_size: Option<Vec<i64>>,
    /// ContainerIDFile is a file created during container creation to hold the ID of the created container. This is not handled within libpod and is stored in an annotation.
    #[serde(rename = "ContainerIDFile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_id_file: Option<String>,
    /// CpuCount is Windows-only and not presently implemented.
    #[serde(rename = "CpuCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_count: Option<i64>,
    /// CpuPercent is Windows-only and not presently implemented.
    #[serde(rename = "CpuPercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_percent: Option<i64>,
    /// CpuPeriod is the length of a CPU period in microseconds. It relates directly to CpuQuota.
    #[serde(rename = "CpuPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_period: Option<i64>,
    /// CpuPeriod is the amount of time (in microseconds) that a container can use the CPU in every CpuPeriod.
    #[serde(rename = "CpuQuota")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_quota: Option<i64>,
    /// CpuRealtimePeriod is the length of time (in microseconds) of the CPU realtime period. If set to 0, no time will be allocated to realtime tasks.
    #[serde(rename = "CpuRealtimePeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_realtime_period: Option<i64>,
    /// CpuRealtimeRuntime is the length of time (in microseconds) allocated for realtime tasks within every CpuRealtimePeriod.
    #[serde(rename = "CpuRealtimeRuntime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_realtime_runtime: Option<i64>,
    /// CpuShares indicates the CPU resources allocated to the container. It is a relative weight in the scheduler for assigning CPU time versus other CGroups.
    #[serde(rename = "CpuShares")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_shares: Option<i64>,
    /// CpusetCpus is the is the set of CPUs that the container will execute on. Formatted as `0-3` or `0,2`. Default (if unset) is all CPUs.
    #[serde(rename = "CpusetCpus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpuset_cpus: Option<String>,
    /// CpusetMems is the set of memory nodes the container will use. Formatted as `0-3` or `0,2`. Default (if unset) is all memory nodes.
    #[serde(rename = "CpusetMems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpuset_mems: Option<String>,
    /// Devices is a list of device nodes that will be added to the container. These are stored in the OCI spec only as type, major, minor while we display the host path. We convert this with /sys/dev, but we cannot guarantee that the host path will be identical - only that the actual device will be.
    #[serde(rename = "Devices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<InspectDevice>>,
    /// DiskQuota is the maximum amount of disk space the container may use (in bytes). Presently not populated. TODO.
    #[serde(rename = "DiskQuota")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_quota: Option<i64>,
    /// Dns is a list of DNS nameservers that will be added to the container's resolv.conf
    #[serde(rename = "Dns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns: Option<Vec<String>>,
    /// DnsOptions is a list of DNS options that will be set in the container's resolv.conf
    #[serde(rename = "DnsOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_options: Option<Vec<String>>,
    /// DnsSearch is a list of DNS search domains that will be set in the container's resolv.conf
    #[serde(rename = "DnsSearch")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_search: Option<Vec<String>>,
    /// ExtraHosts contains hosts that will be aded to the container's etc/hosts.
    #[serde(rename = "ExtraHosts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_hosts: Option<Vec<String>>,
    /// GroupAdd contains groups that the user inside the container will be added to.
    #[serde(rename = "GroupAdd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_add: Option<Vec<String>>,
    /// IOMaximumBandwidth is Windows-only and not presently implemented.
    #[serde(rename = "IOMaximumBandwidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub io_maximum_bandwidth: Option<i64>,
    /// IOMaximumIOps is Windows-only and not presently implemented.
    #[serde(rename = "IOMaximumIOps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub io_maximum_i_ops: Option<i64>,
    /// Init indicates whether the container has an init mounted into it.
    #[serde(rename = "Init")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub init: Option<bool>,
    /// IpcMode represents the configuration of the container's IPC namespace. Populated as follows: \"\" (empty string) - Default, an IPC namespace will be created host - No IPC namespace created container:<id> - Using another container's IPC namespace ns:<path> - A path to an IPC namespace has been specified
    #[serde(rename = "IpcMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipc_mode: Option<String>,
    /// Isolation is presently unused and provided solely for Docker compatibility.
    #[serde(rename = "Isolation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isolation: Option<String>,
    /// KernelMemory is the maximum amount of memory the kernel will devote to the container.
    #[serde(rename = "KernelMemory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kernel_memory: Option<i64>,
    /// Links is unused, and provided purely for Docker compatibility.
    #[serde(rename = "Links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<String>>,
    #[serde(rename = "LogConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_config: Option<InspectLogConfig>,
    /// Memory indicates the memory resources allocated to the container. This is the limit (in bytes) of RAM the container may use.
    #[serde(rename = "Memory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<i64>,
    /// MemoryReservation is the reservation (soft limit) of memory available to the container. Soft limits are warnings only and can be exceeded.
    #[serde(rename = "MemoryReservation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_reservation: Option<i64>,
    /// MemorySwap is the total limit for all memory available to the container, including swap. 0 indicates that there is no limit to the amount of memory available.
    #[serde(rename = "MemorySwap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_swap: Option<i64>,
    /// MemorySwappiness is the willingness of the kernel to page container memory to swap. It is an integer from 0 to 100, with low numbers being more likely to be put into swap. 1, the default, will not set swappiness and use the system defaults.
    #[serde(rename = "MemorySwappiness")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_swappiness: Option<i64>,
    /// NanoCpus indicates number of CPUs allocated to the container. It is an integer where one full CPU is indicated by 1000000000 (one billion). Thus, 2.5 CPUs (fractional portions of CPUs are allowed) would be 2500000000 (2.5 billion). In 'docker inspect' this is set exclusively of two further options in the output (CpuPeriod and CpuQuota) which are both used to implement this functionality. We can't distinguish here, so if CpuQuota is set to the default of 100000, we will set both CpuQuota, CpuPeriod, and NanoCpus. If CpuQuota is not the default, we will not set NanoCpus.
    #[serde(rename = "NanoCpus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nano_cpus: Option<i64>,
    /// NetworkMode is the configuration of the container's network namespace. Populated as follows: default - A network namespace is being created and configured via CNI none - A network namespace is being created, not configured via CNI host - No network namespace created container:<id> - Using another container's network namespace ns:<path> - A path to a network namespace has been specified
    #[serde(rename = "NetworkMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_mode: Option<String>,
    /// OomKillDisable indicates whether the kernel OOM killer is disabled for the container.
    #[serde(rename = "OomKillDisable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oom_kill_disable: Option<bool>,
    /// OOMScoreAdj is an adjustment that will be made to the container's OOM score.
    #[serde(rename = "OomScoreAdj")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oom_score_adj: Option<i64>,
    /// PidMode represents the configuration of the container's PID namespace. Populated as follows: \"\" (empty string) - Default, a PID namespace will be created host - No PID namespace created container:<id> - Using another container's PID namespace ns:<path> - A path to a PID namespace has been specified
    #[serde(rename = "PidMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pid_mode: Option<String>,
    /// PidsLimit is the maximum number of PIDs what may be created within the container. 0, the default, indicates no limit.
    #[serde(rename = "PidsLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pids_limit: Option<i64>,
    /// PortBindings contains the container's port bindings. It is formatted as map[string][]InspectHostPort. The string key here is formatted as <integer port number>/<protocol> and represents the container port. A single container port may be bound to multiple host ports (on different IPs).
    #[serde(rename = "PortBindings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_bindings: Option<HashMap<String, Vec<InspectHostPort>>>,
    /// Privileged indicates whether the container is running with elevated privileges. This has a very specific meaning in the Docker sense, so it's very difficult to decode from the spec and config, and so is stored as an annotation.
    #[serde(rename = "Privileged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privileged: Option<bool>,
    /// PublishAllPorts indicates whether image ports are being published. This is not directly stored in libpod and is saved as an annotation.
    #[serde(rename = "PublishAllPorts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publish_all_ports: Option<bool>,
    /// ReadonlyRootfs is whether the container will be mounted read-only.
    #[serde(rename = "ReadonlyRootfs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readonly_rootfs: Option<bool>,
    #[serde(rename = "RestartPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restart_policy: Option<InspectRestartPolicy>,
    /// Runtime is provided purely for Docker compatibility. It is set unconditionally to \"oci\" as Podman does not presently support non-OCI runtimes.
    #[serde(rename = "Runtime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime: Option<String>,
    /// SecurityOpt is a list of security-related options that are set in the container.
    #[serde(rename = "SecurityOpt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_opt: Option<Vec<String>>,
    /// ShmSize is the size of the container's SHM device.
    #[serde(rename = "ShmSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shm_size: Option<i64>,
    /// Tmpfs is a list of tmpfs filesystems that will be mounted into the container. It is a map of destination path to options for the mount.
    #[serde(rename = "Tmpfs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tmpfs: Option<HashMap<String, String>>,
    /// UTSMode represents the configuration of the container's UID namespace. Populated as follows: \"\" (empty string) - Default, a UTS namespace will be created host - no UTS namespace created container:<id> - Using another container's UTS namespace ns:<path> - A path to a UTS namespace has been specified
    #[serde(rename = "UTSMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uts_mode: Option<String>,
    /// Ulimits is a set of ulimits that will be set within the container.
    #[serde(rename = "Ulimits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ulimits: Option<Vec<InspectUlimit>>,
    /// UsernsMode represents the configuration of the container's user namespace. When running rootless, a user namespace is created outside of libpod to allow some privileged operations. This will not be reflected here. Populated as follows: \"\" (empty string) - No user namespace will be created private - The container will be run in a user namespace container:<id> - Using another container's user namespace ns:<path> - A path to a user namespace has been specified TODO Rootless has an additional 'keep-id' option, presently not reflected here.
    #[serde(rename = "UsernsMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub userns_mode: Option<String>,
    /// VolumeDriver is presently unused and is retained for Docker compatibility.
    #[serde(rename = "VolumeDriver")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_driver: Option<String>,
    /// VolumesFrom is a list of containers which this container uses volumes from. This is not handled directly within libpod and is stored in an annotation. It is formatted as an array of container names and IDs.
    #[serde(rename = "VolumesFrom")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes_from: Option<Vec<String>>,
}

/// InspectContainerState provides a detailed record of a container's current state. It is returned as part of InspectContainerData. As with InspectContainerData, many portions of this struct are matched to Docker, but here we see more fields that are unused (nonsensical in the context of Libpod).
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct InspectContainerState {
    #[serde(rename = "CgroupPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cgroup_path: Option<String>,
    #[serde(rename = "Checkpointed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkpointed: Option<bool>,
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
    pub exit_code: Option<i64>,
    #[serde(rename = "FinishedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finished_at: Option<DateTime<Utc>>,
    #[serde(rename = "Healthcheck")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub healthcheck: Option<HealthCheckResults>,
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
    #[serde(rename = "Running")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub running: Option<bool>,
    #[serde(rename = "StartedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<DateTime<Utc>>,
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct InspectDevice {
    /// CgroupPermissions is the permissions of the mounted device. Presently not populated. TODO.
    #[serde(rename = "CgroupPermissions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cgroup_permissions: Option<String>,
    /// PathInContainer is the path of the device within the container.
    #[serde(rename = "PathInContainer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_in_container: Option<String>,
    /// PathOnHost is the path of the device on the host.
    #[serde(rename = "PathOnHost")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_on_host: Option<String>,
}

/// InspectHostPort provides information on a port on the host that a container's port is bound to.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct InspectHostPort {
    /// IP on the host we are bound to. \"\" if not specified (binding to all IPs).
    #[serde(rename = "HostIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_ip: Option<String>,
    /// Port on the host we are bound to. No special formatting - just an integer stuffed into a string.
    #[serde(rename = "HostPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_port: Option<String>,
}

/// InspectLogConfig holds information about a container's configured log driver
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct InspectLogConfig {
    #[serde(rename = "Config")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<HashMap<String, String>>,
    /// Path specifies a path to the log file
    #[serde(rename = "Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// Size specifies a maximum size of the container log
    #[serde(rename = "Size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    /// Tag specifies a custom log tag for the container
    #[serde(rename = "Tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
}

/// InspectMount provides a record of a single mount in a container. It contains fields for both named and normal volumes. Only user-specified volumes will be included, and tmpfs volumes are not included even if the user specified them.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct InspectMount {
    /// The destination directory for the volume. Specified as a path within the container, as it would be passed into the OCI runtime.
    #[serde(rename = "Destination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,
    /// The driver used for the named volume. Empty for bind mounts.
    #[serde(rename = "Driver")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver: Option<String>,
    /// Contains SELinux :z/:Z mount options. Unclear what, if anything, else goes in here.
    #[serde(rename = "Mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    /// The name of the volume. Empty for bind mounts.
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// All remaining mount options. Additional data, not present in the original output.
    #[serde(rename = "Options")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<String>>,
    /// Mount propagation for the mount. Can be empty if not specified, but is always printed - no omitempty.
    #[serde(rename = "Propagation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub propagation: Option<String>,
    /// Whether the volume is read-write
    #[serde(rename = "RW")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub RW: Option<bool>,
    /// The source directory for the volume.
    #[serde(rename = "Source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// Whether the mount is a volume or bind mount. Allowed values are \"volume\" and \"bind\".
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
}

/// InspectNetworkSettings holds information about the network settings of the container. Many fields are maintained only for compatibility with `docker inspect` and are unused within Libpod.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct InspectNetworkSettings {
    /// AdditionalMacAddresses is a set of additional MAC Addresses beyond the first. CNI may configure more than one interface for a single network, which can cause this.
    #[serde(rename = "AdditionalMACAddresses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_mac_addresses: Option<Vec<String>>,
    #[serde(rename = "Bridge")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bridge: Option<String>,
    /// EndpointID is unused, maintained exclusively for compatibility.
    #[serde(rename = "EndpointID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_id: Option<String>,
    /// Gateway is the IP address of the gateway this network will use.
    #[serde(rename = "Gateway")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway: Option<String>,
    /// GlobalIPv6Address is the global-scope IPv6 Address for this network.
    #[serde(rename = "GlobalIPv6Address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_i_pv6_address: Option<String>,
    /// GlobalIPv6PrefixLen is the length of the subnet mask of this network.
    #[serde(rename = "GlobalIPv6PrefixLen")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_i_pv6_prefix_len: Option<i64>,
    #[serde(rename = "HairpinMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hairpin_mode: Option<bool>,
    /// IPAddress is the IP address for this network.
    #[serde(rename = "IPAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    /// IPPrefixLen is the length of the subnet mask of this network.
    #[serde(rename = "IPPrefixLen")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_prefix_len: Option<i64>,
    /// IPv6Gateway is the IPv6 gateway this network will use.
    #[serde(rename = "IPv6Gateway")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_pv6_gateway: Option<String>,
    #[serde(rename = "LinkLocalIPv6Address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_local_i_pv6_address: Option<String>,
    #[serde(rename = "LinkLocalIPv6PrefixLen")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_local_i_pv6_prefix_len: Option<i64>,
    /// MacAddress is the MAC address for the interface in this network.
    #[serde(rename = "MacAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>,
    /// Networks contains information on non-default CNI networks this container has joined. It is a map of network name to network information.
    #[serde(rename = "Networks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub networks: Option<HashMap<String, InspectAdditionalNetwork>>,
    #[serde(rename = "Ports")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ports: Option<HashMap<String, Vec<InspectHostPort>>>,
    #[serde(rename = "SandboxID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sandbox_id: Option<String>,
    #[serde(rename = "SandboxKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sandbox_key: Option<String>,
    /// SecondaryIPAddresses is a list of extra IP Addresses that the container has been assigned in this network.
    #[serde(rename = "SecondaryIPAddresses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_ip_addresses: Option<Vec<String>>,
    /// SecondaryIPv6Addresses is a list of extra IPv6 Addresses that the container has been assigned in this network.
    #[serde(rename = "SecondaryIPv6Addresses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_i_pv6_addresses: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct InspectPodContainerInfo {
    /// ID is the ID of the container.
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Name is the name of the container.
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// State is the current status of the container.
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

/// InspectPodInfraConfig contains the configuration of the pod's infra container.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct InspectPodInfraConfig {
    /// DNSOption is a set of DNS options that will be used by the infra container's resolv.conf and shared with the remainder of the pod.
    #[serde(rename = "DNSOption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_option: Option<Vec<String>>,
    /// DNSSearch is a set of DNS search domains that will be used by the infra container's resolv.conf and shared with the remainder of the pod.
    #[serde(rename = "DNSSearch")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_search: Option<Vec<String>>,
    /// DNSServer is a set of DNS Servers that will be used by the infra container's resolv.conf and shared with the remainder of the pod.
    #[serde(rename = "DNSServer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_server: Option<Vec<String>>,
    /// HostAdd adds a number of hosts to the infra container's resolv.conf which will be shared with the rest of the pod.
    #[serde(rename = "HostAdd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_add: Option<Vec<String>>,
    /// HostNetwork is whether the infra container (and thus the whole pod) will use the host's network and not create a network namespace.
    #[serde(rename = "HostNetwork")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_network: Option<bool>,
    /// NetworkOptions are additional options for each network
    #[serde(rename = "NetworkOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_options: Option<HashMap<String, Vec<String>>>,
    /// Networks is a list of CNI networks the pod will join.
    #[serde(rename = "Networks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub networks: Option<Vec<String>>,
    /// NoManageHosts indicates that the pod will not manage /etc/hosts and instead each container will handle their own.
    #[serde(rename = "NoManageHosts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_manage_hosts: Option<bool>,
    /// NoManageResolvConf indicates that the pod will not manage resolv.conf and instead each container will handle their own.
    #[serde(rename = "NoManageResolvConf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_manage_resolv_conf: Option<bool>,
    /// PortBindings are ports that will be forwarded to the infra container and then shared with the pod.
    #[serde(rename = "PortBindings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_bindings: Option<HashMap<String, Vec<InspectHostPort>>>,
    #[serde(rename = "StaticIP")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub static_ip: Option<Ip>,
    /// StaticMAC is a static MAC address that will be assigned to the infra container and then used by the pod.
    #[serde(rename = "StaticMAC")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub static_mac: Option<String>,
    /// Pid is the PID namespace mode of the pod's infra container
    #[serde(rename = "pid_ns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pid_ns: Option<String>,
    /// UserNS is the usernamespace that all the containers in the pod will join.
    #[serde(rename = "userns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub userns: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct InspectRestartPolicy {
    /// MaximumRetryCount is the maximum number of retries allowed if the \"on-failure\" restart policy is in use. Not used if \"on-failure\" is not set.
    #[serde(rename = "MaximumRetryCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_retry_count: Option<i64>,
    /// Name contains the container's restart policy. Allowable values are \"no\" or \"\" (take no action), \"on-failure\" (restart on non-zero exit code, with an optional max retry count), and \"always\" (always restart on container stop, unless explicitly requested by API). Note that this is NOT actually a name of any sort - the poor naming is for Docker compatibility.
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// InspectSecret contains information on secrets mounted inside the container
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct InspectSecret {
    /// ID is the GID of the mounted secret file
    #[serde(rename = "GID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub GID: Option<i64>,
    /// ID is the ID of the secret
    #[serde(rename = "ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ID: Option<String>,
    /// ID is the ID of the mode of the mounted secret file
    #[serde(rename = "Mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<i64>,
    /// Name is the name of the secret
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// ID is the UID of the mounted secret file
    #[serde(rename = "UID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub UID: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct InspectUlimit {
    /// Hard is the hard limit that will be applied.
    #[serde(rename = "Hard")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hard: Option<i64>,
    /// Name is the name (type) of the ulimit.
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Soft is the soft limit that will be applied.
    #[serde(rename = "Soft")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub soft: Option<i64>,
}

/// Note that in this documentation, referring to an IP address as an IPv4 address or an IPv6 address is a semantic property of the address, not just the length of the byte slice: a 16-byte slice can still be an IPv4 address.
pub type Ip = Vec<i32>;

/// See type IPNet and func ParseCIDR for details.
pub type IpMask = Vec<i32>;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct IpNet {
    #[serde(rename = "IP")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub IP: Option<Ip>,
    #[serde(rename = "Mask")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask: Option<IpMask>,
}

/// IPAM represents IP Address Management
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
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

/// IPAMConfig represents IPAM configurations
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
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

pub type IpcMode = String;

/// Isolation represents the isolation technology of a container. The supported values are platform specific
pub type Isolation = String;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct LibpodContainersPruneReport {
    #[serde(rename = "error")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "space")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub space: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct LibpodImageSummary {
    #[serde(rename = "ConfigDigest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_digest: Option<String>,
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
    #[serde(rename = "Labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<HashMap<String, String>>,
    /// Podman extensions
    #[serde(rename = "Names")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<Vec<String>>,
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

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct LibpodImagesPullReport {
    /// Error contains text of errors from c/image
    #[serde(rename = "error")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// ID contains image id (retained for backwards compatibility)
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Images contains the ID's of the images pulled
    #[serde(rename = "images")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<String>>,
    /// Stream used to provide output from c/image
    #[serde(rename = "stream")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<String>,
}

/// LibpodImagesRemoveReport is the return type for image removal via the rest api.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct LibpodImagesRemoveReport {
    /// Deleted images.
    #[serde(rename = "Deleted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted: Option<Vec<String>>,
    /// Image removal requires is to return data and an error.
    #[serde(rename = "Errors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<String>>,
    /// ExitCode describes the exit codes as described in the `podman rmi` man page.
    #[serde(rename = "ExitCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exit_code: Option<i64>,
    /// Untagged images. Can be longer than Deleted.
    #[serde(rename = "Untagged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub untagged: Option<Vec<String>>,
}

/// LinuxBlockIO for Linux cgroup 'blkio' resource management
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct LinuxBlockIo {
    /// Specifies tasks' weight in the given cgroup while competing with the cgroup's child cgroups, CFQ scheduler only
    #[serde(rename = "leafWeight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leaf_weight: Option<i64>,
    /// IO read rate limit per cgroup per device, bytes per second
    #[serde(rename = "throttleReadBpsDevice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throttle_read_bps_device: Option<Vec<LinuxThrottleDevice>>,
    /// IO read rate limit per cgroup per device, IO per second
    #[serde(rename = "throttleReadIOPSDevice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throttle_read_iops_device: Option<Vec<LinuxThrottleDevice>>,
    /// IO write rate limit per cgroup per device, bytes per second
    #[serde(rename = "throttleWriteBpsDevice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throttle_write_bps_device: Option<Vec<LinuxThrottleDevice>>,
    /// IO write rate limit per cgroup per device, IO per second
    #[serde(rename = "throttleWriteIOPSDevice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throttle_write_iops_device: Option<Vec<LinuxThrottleDevice>>,
    /// Specifies per cgroup weight
    #[serde(rename = "weight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<i64>,
    /// Weight per cgroup per device, can override BlkioWeight
    #[serde(rename = "weightDevice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight_device: Option<Vec<LinuxWeightDevice>>,
}

/// linuxBlockIODevice holds major:minor format supported in blkio cgroup
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct LinuxBlockIoDevice {
    /// Major is the device's major number.
    #[serde(rename = "major")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub major: Option<i64>,
    /// Minor is the device's minor number.
    #[serde(rename = "minor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minor: Option<i64>,
}

/// LinuxCPU for Linux cgroup 'cpu' resource management
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct LinuxCpu {
    /// CPUs to use within the cpuset. Default is to use any CPU available.
    #[serde(rename = "cpus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpus: Option<String>,
    /// List of memory nodes in the cpuset. Default is to use any available memory node.
    #[serde(rename = "mems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mems: Option<String>,
    /// CPU period to be used for hardcapping (in usecs).
    #[serde(rename = "period")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<i64>,
    /// CPU hardcap limit (in usecs). Allowed cpu time in a given period.
    #[serde(rename = "quota")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quota: Option<i64>,
    /// CPU period to be used for realtime scheduling (in usecs).
    #[serde(rename = "realtimePeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub realtime_period: Option<i64>,
    /// How much time realtime scheduling may use (in usecs).
    #[serde(rename = "realtimeRuntime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub realtime_runtime: Option<i64>,
    /// CPU shares (relative weight (ratio) vs. other cgroups with cpu shares).
    #[serde(rename = "shares")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shares: Option<i64>,
}

/// LinuxDevice represents the mknod information for a Linux special device file
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct LinuxDevice {
    #[serde(rename = "fileMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_mode: Option<FileMode>,
    /// Gid of the device.
    #[serde(rename = "gid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gid: Option<i64>,
    /// Major is the device's major number.
    #[serde(rename = "major")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub major: Option<i64>,
    /// Minor is the device's minor number.
    #[serde(rename = "minor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minor: Option<i64>,
    /// Path to the device.
    #[serde(rename = "path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// Device type, block, char, etc.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    /// UID of the device.
    #[serde(rename = "uid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<i64>,
}

/// LinuxDeviceCgroup represents a device rule for the devices specified to the device controller
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct LinuxDeviceCgroup {
    /// Cgroup access permissions format, rwm.
    #[serde(rename = "access")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access: Option<String>,
    /// Allow or deny
    #[serde(rename = "allow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow: Option<bool>,
    /// Major is the device's major number.
    #[serde(rename = "major")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub major: Option<i64>,
    /// Minor is the device's minor number.
    #[serde(rename = "minor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minor: Option<i64>,
    /// Device type, block, char, etc.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
}

/// LinuxHugepageLimit structure corresponds to limiting kernel hugepages
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct LinuxHugepageLimit {
    /// Limit is the limit of \"hugepagesize\" hugetlb usage
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Pagesize is the hugepage size Format: \"<size><unit-prefix>B' (e.g. 64KB, 2MB, 1GB, etc.)
    #[serde(rename = "pageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<String>,
}

/// LinuxInterfacePriority for network interfaces
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct LinuxInterfacePriority {
    /// Name is the name of the network interface
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Priority for the interface
    #[serde(rename = "priority")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,
}

/// LinuxMemory for Linux cgroup 'memory' resource management
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct LinuxMemory {
    /// DisableOOMKiller disables the OOM killer for out of memory conditions
    #[serde(rename = "disableOOMKiller")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_oom_killer: Option<bool>,
    /// Kernel memory limit (in bytes).
    #[serde(rename = "kernel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kernel: Option<i64>,
    /// Kernel memory limit for tcp (in bytes)
    #[serde(rename = "kernelTCP")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kernel_tcp: Option<i64>,
    /// Memory limit (in bytes).
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Memory reservation or soft_limit (in bytes).
    #[serde(rename = "reservation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reservation: Option<i64>,
    /// Total memory limit (memory + swap).
    #[serde(rename = "swap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swap: Option<i64>,
    /// How aggressive the kernel will swap memory pages.
    #[serde(rename = "swappiness")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swappiness: Option<i64>,
    /// Enables hierarchical memory accounting
    #[serde(rename = "useHierarchy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_hierarchy: Option<bool>,
}

/// LinuxNetwork identification and priority configuration
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct LinuxNetwork {
    /// Set class identifier for container's network packets
    #[serde(rename = "classID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class_id: Option<i64>,
    /// Set priority of network traffic for container
    #[serde(rename = "priorities")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priorities: Option<Vec<LinuxInterfacePriority>>,
}

/// LinuxPersonality represents the Linux personality syscall input
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct LinuxPersonality {
    #[serde(rename = "domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<LinuxPersonalityDomain>,
    /// Additional flags
    #[serde(rename = "flags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<Vec<LinuxPersonalityFlag>>,
}

pub type LinuxPersonalityDomain = String;

pub type LinuxPersonalityFlag = String;

/// LinuxPids for Linux cgroup 'pids' resource management (Linux 4.3)
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct LinuxPids {
    /// Maximum number of PIDs. Default is \"no limit\".
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

/// LinuxRdma for Linux cgroup 'rdma' resource management (Linux 4.11)
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct LinuxRdma {
    /// Maximum number of HCA handles that can be opened. Default is \"no limit\".
    #[serde(rename = "hcaHandles")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hca_handles: Option<i64>,
    /// Maximum number of HCA objects that can be created. Default is \"no limit\".
    #[serde(rename = "hcaObjects")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hca_objects: Option<i64>,
}

/// LinuxResources has container runtime resource constraints
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct LinuxResources {
    #[serde(rename = "blockIO")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_io: Option<LinuxBlockIo>,
    #[serde(rename = "cpu")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu: Option<LinuxCpu>,
    /// Devices configures the device allowlist.
    #[serde(rename = "devices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<LinuxDeviceCgroup>>,
    /// Hugetlb limit (in bytes)
    #[serde(rename = "hugepageLimits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hugepage_limits: Option<Vec<LinuxHugepageLimit>>,
    #[serde(rename = "memory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<LinuxMemory>,
    #[serde(rename = "network")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<LinuxNetwork>,
    #[serde(rename = "pids")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pids: Option<LinuxPids>,
    /// Rdma resource restriction configuration. Limits are a set of key value pairs that define RDMA resource limits, where the key is device name and value is resource limits.
    #[serde(rename = "rdma")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rdma: Option<HashMap<String, LinuxRdma>>,
    /// Unified resources.
    #[serde(rename = "unified")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unified: Option<HashMap<String, String>>,
}

/// LinuxThrottleDevice struct holds a `major:minor rate_per_second` pair
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct LinuxThrottleDevice {
    /// Major is the device's major number.
    #[serde(rename = "major")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub major: Option<i64>,
    /// Minor is the device's minor number.
    #[serde(rename = "minor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minor: Option<i64>,
    /// Rate is the IO rate limit per cgroup per device
    #[serde(rename = "rate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate: Option<i64>,
}

/// LinuxWeightDevice struct holds a `major:minor weight` pair for weightDevice
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct LinuxWeightDevice {
    /// LeafWeight is the bandwidth rate for the device while competing with the cgroup's child cgroups, CFQ scheduler only
    #[serde(rename = "leafWeight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leaf_weight: Option<i64>,
    /// Major is the device's major number.
    #[serde(rename = "major")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub major: Option<i64>,
    /// Minor is the device's minor number.
    #[serde(rename = "minor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minor: Option<i64>,
    /// Weight is the bandwidth rate for the device.
    #[serde(rename = "weight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<i64>,
}

/// Listcontainer describes a container suitable for listing
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ListContainer {
    /// AutoRemove
    #[serde(rename = "AutoRemove")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_remove: Option<bool>,
    /// Container command
    #[serde(rename = "Command")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<Vec<String>>,
    /// Container creation time
    #[serde(rename = "Created")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<DateTime<Utc>>,
    /// Human readable container creation time.
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// If container has exited, the return code from the command
    #[serde(rename = "ExitCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exit_code: Option<i64>,
    /// If container has exited/stopped
    #[serde(rename = "Exited")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exited: Option<bool>,
    /// Time container exited
    #[serde(rename = "ExitedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exited_at: Option<i64>,
    /// The unique identifier for the container
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Container image
    #[serde(rename = "Image")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// Container image ID
    #[serde(rename = "ImageID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
    /// If this container is a Pod infra container
    #[serde(rename = "IsInfra")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_infra: Option<bool>,
    /// Labels for container
    #[serde(rename = "Labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<HashMap<String, String>>,
    /// User volume mounts
    #[serde(rename = "Mounts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mounts: Option<Vec<String>>,
    /// The names assigned to the container
    #[serde(rename = "Names")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<Vec<String>>,
    #[serde(rename = "Namespaces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespaces: Option<ListContainerNamespaces>,
    /// The network names assigned to the container
    #[serde(rename = "Networks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub networks: Option<Vec<String>>,
    /// The process id of the container
    #[serde(rename = "Pid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pid: Option<i64>,
    /// If the container is part of Pod, the Pod ID. Requires the pod boolean to be set
    #[serde(rename = "Pod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pod: Option<String>,
    /// If the container is part of Pod, the Pod name. Requires the pod boolean to be set
    #[serde(rename = "PodName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pod_name: Option<String>,
    /// Port mappings
    #[serde(rename = "Ports")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ports: Option<Vec<PortMapping>>,
    #[serde(rename = "Size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<ContainerSize>,
    /// Time when container started
    #[serde(rename = "StartedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<i64>,
    /// State of container
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Status is a human-readable approximation of a duration for json output
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// ListContainer Namespaces contains the identifiers of the container's Linux namespaces
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ListContainerNamespaces {
    /// Cgroup namespace
    #[serde(rename = "Cgroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cgroup: Option<String>,
    /// IPC namespace
    #[serde(rename = "Ipc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipc: Option<String>,
    /// Mount namespace
    #[serde(rename = "Mnt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mnt: Option<String>,
    /// Network namespace
    #[serde(rename = "Net")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net: Option<String>,
    /// PID namespace
    #[serde(rename = "Pidns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pidns: Option<String>,
    /// User namespace
    #[serde(rename = "User")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    /// UTS namespace
    #[serde(rename = "Uts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uts: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ListPodContainer {
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Names")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<String>,
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
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
    /// Network names connected to infra container
    #[serde(rename = "Networks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub networks: Option<Vec<String>>,
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct LogConfig {
    #[serde(rename = "Config")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<HashMap<String, String>>,
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ManifestAddOpts {
    #[serde(rename = "all")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all: Option<bool>,
    #[serde(rename = "annotation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotation: Option<Vec<String>>,
    #[serde(rename = "arch")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arch: Option<String>,
    #[serde(rename = "features")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<Vec<String>>,
    #[serde(rename = "images")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<String>>,
    #[serde(rename = "os")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,
    #[serde(rename = "os_version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_version: Option<String>,
    #[serde(rename = "variant")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant: Option<String>,
}

pub type ModelType = String;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Mount {
    #[serde(rename = "BindOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bind_options: Option<BindOptions>,
    #[serde(rename = "Consistency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consistency: Option<Consistency>,
    #[serde(rename = "ReadOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    /// Source specifies the name of the mount. Depending on mount type, this may be a volume name or a host path, or even ignored. Source is not supported for tmpfs (must be an empty value)
    #[serde(rename = "Source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(rename = "Target")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(rename = "TmpfsOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tmpfs_options: Option<TmpfsOptions>,
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<ModelType>,
    #[serde(rename = "VolumeOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_options: Option<VolumeOptions>,
}

/// This is used for reporting the mountpoints in use by a container.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct MountPoint {
    #[serde(rename = "Destination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,
    #[serde(rename = "Driver")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver: Option<String>,
    #[serde(rename = "Mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Propagation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub propagation: Option<Propagation>,
    #[serde(rename = "RW")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub RW: Option<bool>,
    #[serde(rename = "Source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<ModelType>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct NameExecBody {
    /// Attach to stderr of the exec command
    #[serde(rename = "AttachStderr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach_stderr: Option<bool>,
    /// Attach to stdin of the exec command
    #[serde(rename = "AttachStdin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach_stdin: Option<bool>,
    /// Attach to stdout of the exec command
    #[serde(rename = "AttachStdout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach_stdout: Option<bool>,
    /// Command to run, as a string or array of strings.
    #[serde(rename = "Cmd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cmd: Option<Vec<String>>,
    /// \"Override the key sequence for detaching a container. Format is a single character [a-Z] or ctrl-<value> where <value> is one of: a-z, @, ^, [, , or _.\"
    #[serde(rename = "DetachKeys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detach_keys: Option<String>,
    /// A list of environment variables in the form [\"VAR=value\", ...]
    #[serde(rename = "Env")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<Vec<String>>,
    /// Runs the exec process with extended privileges
    #[serde(rename = "Privileged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privileged: Option<bool>,
    /// Allocate a pseudo-TTY
    #[serde(rename = "Tty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tty: Option<bool>,
    /// \"The user, and optionally, group to run the exec process inside the container. Format is one of: user, user:group, uid, or uid:gid.\"
    #[serde(rename = "User")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    /// The working directory for the exec process inside the container.
    #[serde(rename = "WorkingDir")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_dir: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct NameExecBody1 {
    /// Attach to stderr of the exec command
    #[serde(rename = "AttachStderr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach_stderr: Option<bool>,
    /// Attach to stdin of the exec command
    #[serde(rename = "AttachStdin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach_stdin: Option<bool>,
    /// Attach to stdout of the exec command
    #[serde(rename = "AttachStdout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach_stdout: Option<bool>,
    /// Command to run, as a string or array of strings.
    #[serde(rename = "Cmd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cmd: Option<Vec<String>>,
    /// \"Override the key sequence for detaching a container. Format is a single character [a-Z] or ctrl-<value> where <value> is one of: a-z, @, ^, [, , or _.\"
    #[serde(rename = "DetachKeys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detach_keys: Option<String>,
    /// A list of environment variables in the form [\"VAR=value\", ...]
    #[serde(rename = "Env")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<Vec<String>>,
    /// Runs the exec process with extended privileges
    #[serde(rename = "Privileged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privileged: Option<bool>,
    /// Allocate a pseudo-TTY
    #[serde(rename = "Tty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tty: Option<bool>,
    /// \"The user, and optionally, group to run the exec process inside the container. Format is one of: user, user:group, uid, or uid:gid.\"
    #[serde(rename = "User")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    /// The working directory for the exec process inside the container.
    #[serde(rename = "WorkingDir")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_dir: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct NameExecBody2 {
    /// Attach to stderr of the exec command
    #[serde(rename = "AttachStderr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach_stderr: Option<bool>,
    /// Attach to stdin of the exec command
    #[serde(rename = "AttachStdin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach_stdin: Option<bool>,
    /// Attach to stdout of the exec command
    #[serde(rename = "AttachStdout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach_stdout: Option<bool>,
    /// Command to run, as a string or array of strings.
    #[serde(rename = "Cmd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cmd: Option<Vec<String>>,
    /// \"Override the key sequence for detaching a container. Format is a single character [a-Z] or ctrl-<value> where <value> is one of: a-z, @, ^, [, , or _.\"
    #[serde(rename = "DetachKeys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detach_keys: Option<String>,
    /// A list of environment variables in the form [\"VAR=value\", ...]
    #[serde(rename = "Env")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<Vec<String>>,
    /// Runs the exec process with extended privileges
    #[serde(rename = "Privileged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privileged: Option<bool>,
    /// Allocate a pseudo-TTY
    #[serde(rename = "Tty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tty: Option<bool>,
    /// \"The user, and optionally, group to run the exec process inside the container. Format is one of: user, user:group, uid, or uid:gid.\"
    #[serde(rename = "User")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    /// The working directory for the exec process inside the container.
    #[serde(rename = "WorkingDir")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_dir: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct NameExecBody3 {
    /// Attach to stderr of the exec command
    #[serde(rename = "AttachStderr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach_stderr: Option<bool>,
    /// Attach to stdin of the exec command
    #[serde(rename = "AttachStdin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach_stdin: Option<bool>,
    /// Attach to stdout of the exec command
    #[serde(rename = "AttachStdout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach_stdout: Option<bool>,
    /// Command to run, as a string or array of strings.
    #[serde(rename = "Cmd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cmd: Option<Vec<String>>,
    /// \"Override the key sequence for detaching a container. Format is a single character [a-Z] or ctrl-<value> where <value> is one of: a-z, @, ^, [, , or _.\"
    #[serde(rename = "DetachKeys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detach_keys: Option<String>,
    /// A list of environment variables in the form [\"VAR=value\", ...]
    #[serde(rename = "Env")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<Vec<String>>,
    /// Runs the exec process with extended privileges
    #[serde(rename = "Privileged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privileged: Option<bool>,
    /// Allocate a pseudo-TTY
    #[serde(rename = "Tty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tty: Option<bool>,
    /// \"The user, and optionally, group to run the exec process inside the container. Format is one of: user, user:group, uid, or uid:gid.\"
    #[serde(rename = "User")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    /// The working directory for the exec process inside the container.
    #[serde(rename = "WorkingDir")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_dir: Option<String>,
}

/// NamedVolume holds information about a named volume that will be mounted into the container.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct NamedVolume {
    /// Destination to mount the named volume within the container. Must be an absolute path. Path will be created if it does not exist.
    #[serde(rename = "Dest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dest: Option<String>,
    /// Name is the name of the named volume to be mounted. May be empty. If empty, a new named volume with a pseudorandomly generated name will be mounted at the given destination.
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Options are options that the named volume will be mounted with.
    #[serde(rename = "Options")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<String>>,
}

/// Namespace describes the namespace
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Namespace {
    #[serde(rename = "nsmode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsmode: Option<NamespaceMode>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

pub type NamespaceMode = String;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct NetConf {
    #[serde(rename = "capabilities")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<HashMap<String, bool>>,
    #[serde(rename = "cniVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cni_version: Option<String>,
    #[serde(rename = "dns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns: Option<Dns>,
    #[serde(rename = "ipam")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipam: Option<Ipam>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "prevResult")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev_result: Option<HashMap<String, Value>>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct NetOptions {
    #[serde(rename = "cni_networks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cni_networks: Option<Vec<String>>,
    #[serde(rename = "dns_option")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_option: Option<Vec<String>>,
    #[serde(rename = "dns_search")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_search: Option<Vec<String>>,
    #[serde(rename = "dns_server")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_server: Option<Vec<Ip>>,
    #[serde(rename = "hostadd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostadd: Option<Vec<String>>,
    #[serde(rename = "netns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub netns: Option<Namespace>,
    #[serde(rename = "network_alias")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_alias: Option<Vec<String>>,
    /// NetworkOptions are additional options for each network
    #[serde(rename = "network_options")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_options: Option<HashMap<String, Vec<String>>>,
    #[serde(rename = "no_manage_hosts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_manage_hosts: Option<bool>,
    #[serde(rename = "no_manage_resolv_conf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_manage_resolv_conf: Option<bool>,
    #[serde(rename = "portmappings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portmappings: Option<Vec<PortMapping>>,
    #[serde(rename = "static_ip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub static_ip: Option<Ip>,
    #[serde(rename = "static_mac")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub static_mac: Option<HardwareAddr>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct NetworkConfig {
    #[serde(rename = "Bytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes: Option<Vec<i64>>,
    #[serde(rename = "Network")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<NetConf>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct NetworkConfigList {
    #[serde(rename = "Bytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes: Option<Vec<i64>>,
    #[serde(rename = "CNIVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cni_version: Option<String>,
    #[serde(rename = "DisableCheck")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_check: Option<bool>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Plugins")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plugins: Option<Vec<NetworkConfig>>,
}

/// NetworkConnect represents the data to be used to connect a container to the network
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct NetworkConnect {
    #[serde(rename = "Container")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container: Option<String>,
    #[serde(rename = "EndpointConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_config: Option<EndpointSettings>,
}

/// Network disconnect
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct NetworkConnectRequest {
    #[serde(rename = "Body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<NetworkConnectRequestBody>,
}

/// in:body
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct NetworkConnectRequestBody {
    #[serde(rename = "Container")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container: Option<String>,
    #[serde(rename = "EndpointConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_config: Option<EndpointSettings>,
}

/// NetworkCreate is the expected body of the \"create network\" http request message
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct NetworkCreate {
    #[serde(rename = "Attachable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachable: Option<bool>,
    /// Check for networks with duplicate names. Network is primarily keyed based on a random ID and not on the name. Network name is strictly a user-friendly alias to the network which is uniquely identified using ID. And there is no guaranteed way to check for duplicates. Option CheckDuplicate is there to provide a best effort checking of any networks which has the same name but it is not guaranteed to catch all name collisions.
    #[serde(rename = "CheckDuplicate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_duplicate: Option<bool>,
    #[serde(rename = "ConfigFrom")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_from: Option<ConfigReference>,
    #[serde(rename = "ConfigOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_only: Option<bool>,
    #[serde(rename = "Driver")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver: Option<String>,
    #[serde(rename = "EnableIPv6")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_i_pv6: Option<bool>,
    #[serde(rename = "IPAM")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub IPAM: Option<Ipam>,
    #[serde(rename = "Ingress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingress: Option<bool>,
    #[serde(rename = "Internal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal: Option<bool>,
    #[serde(rename = "Labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<HashMap<String, String>>,
    #[serde(rename = "Options")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<HashMap<String, String>>,
    #[serde(rename = "Scope")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
}

/// NetworkCreateOptions describes options to create a network
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct NetworkCreateOptions {
    #[serde(rename = "DisableDNS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_dns: Option<bool>,
    #[serde(rename = "Driver")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver: Option<String>,
    #[serde(rename = "Gateway")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway: Option<Ip>,
    #[serde(rename = "IPv6")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_pv6: Option<bool>,
    #[serde(rename = "Internal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal: Option<bool>,
    #[serde(rename = "Labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<HashMap<String, String>>,
    #[serde(rename = "MacVLAN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac_vlan: Option<String>,
    /// Mapping of driver options and values.
    #[serde(rename = "Options")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<HashMap<String, String>>,
    #[serde(rename = "Range")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<IpNet>,
    #[serde(rename = "Subnet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet: Option<IpNet>,
}

/// NetworkCreateReport describes a created network for the cli
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct NetworkCreateReport {
    #[serde(rename = "Filename")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct NetworkCreateRequest {
    #[serde(rename = "Attachable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachable: Option<bool>,
    /// Check for networks with duplicate names. Network is primarily keyed based on a random ID and not on the name. Network name is strictly a user-friendly alias to the network which is uniquely identified using ID. And there is no guaranteed way to check for duplicates. Option CheckDuplicate is there to provide a best effort checking of any networks which has the same name but it is not guaranteed to catch all name collisions.
    #[serde(rename = "CheckDuplicate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_duplicate: Option<bool>,
    #[serde(rename = "ConfigFrom")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_from: Option<ConfigReference>,
    #[serde(rename = "ConfigOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_only: Option<bool>,
    #[serde(rename = "Driver")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver: Option<String>,
    #[serde(rename = "EnableIPv6")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_i_pv6: Option<bool>,
    #[serde(rename = "IPAM")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub IPAM: Option<Ipam>,
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

/// NetworkDisconnect represents the data to be used to disconnect a container from the network
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct NetworkDisconnect {
    #[serde(rename = "Container")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container: Option<String>,
    #[serde(rename = "Force")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
}

/// Network disconnect
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct NetworkDisconnectRequest {
    #[serde(rename = "Body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<NetworkDisconnectRequestBody>,
}

/// in:body
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct NetworkDisconnectRequestBody {
    #[serde(rename = "Container")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container: Option<String>,
    #[serde(rename = "Force")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
}

/// NetworkListReport describes the results from listing networks
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct NetworkListReport {
    #[serde(rename = "Bytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes: Option<Vec<i64>>,
    #[serde(rename = "CNIVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cni_version: Option<String>,
    #[serde(rename = "DisableCheck")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_check: Option<bool>,
    #[serde(rename = "Labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<HashMap<String, String>>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Plugins")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plugins: Option<Vec<NetworkConfig>>,
}

pub type NetworkMode = String;

/// NetworkPruneReport containers the name of network and an error associated in its pruning (removal)
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct NetworkPruneReport {
    #[serde(rename = "Error")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// NetworkResource is the body of the \"get network\" http response message
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct NetworkResource {
    #[serde(rename = "Attachable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachable: Option<bool>,
    #[serde(rename = "ConfigFrom")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    pub enable_i_pv6: Option<bool>,
    #[serde(rename = "IPAM")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub IPAM: Option<Ipam>,
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

/// NetworkRmReport describes the results of network removal
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct NetworkRmReport {
    #[serde(rename = "Err")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub err: Option<String>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// NetworkSettings exposes the network settings in the api
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
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
    pub global_i_pv6_address: Option<String>,
    #[serde(rename = "GlobalIPv6PrefixLen")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_i_pv6_prefix_len: Option<i64>,
    #[serde(rename = "HairpinMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hairpin_mode: Option<bool>,
    #[serde(rename = "IPAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[serde(rename = "IPPrefixLen")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_prefix_len: Option<i64>,
    #[serde(rename = "IPv6Gateway")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_pv6_gateway: Option<String>,
    #[serde(rename = "LinkLocalIPv6Address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_local_i_pv6_address: Option<String>,
    #[serde(rename = "LinkLocalIPv6PrefixLen")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_local_i_pv6_prefix_len: Option<i64>,
    #[serde(rename = "MacAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>,
    #[serde(rename = "Networks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub networks: Option<HashMap<String, EndpointSettings>>,
    #[serde(rename = "Ports")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    pub secondary_i_pv6_addresses: Option<Vec<Address>>,
}

/// NetworkingConfig represents the container's networking configuration for each of its interfaces Carries the networking configs specified in the `docker run` and `docker network connect` commands
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct NetworkingConfig {
    #[serde(rename = "EndpointsConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoints_config: Option<HashMap<String, EndpointSettings>>,
}

/// OCIRuntimeInfo describes the runtime (crun or runc) being used with podman
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct OciRuntimeInfo {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "package")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package: Option<String>,
    #[serde(rename = "path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// OverlayVolume holds information about a overlay volume that will be mounted into the container.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct OverlayVolume {
    /// Destination is the absolute path where the mount will be placed in the container.
    #[serde(rename = "destination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,
    /// Options holds overlay volume options.
    #[serde(rename = "options")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<String>>,
    /// Source specifies the source path of the mount.
    #[serde(rename = "source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

/// PeerInfo represents one peer of an overlay network
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct PeerInfo {
    #[serde(rename = "IP")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub IP: Option<String>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

pub type PidMode = String;

/// PlayKubePod represents a single pod and associated containers created by play kube
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct PlayKubePod {
    /// ContainerErrors - any errors that occurred while starting containers in the pod.
    #[serde(rename = "ContainerErrors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_errors: Option<Vec<String>>,
    /// Containers - the IDs of the containers running in the created pod.
    #[serde(rename = "Containers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub containers: Option<Vec<String>>,
    /// ID - ID of the pod created as a result of play kube.
    #[serde(rename = "ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ID: Option<String>,
    /// InitContainers - the IDs of the init containers to be run in the created pod.
    #[serde(rename = "InitContainers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub init_containers: Option<Vec<String>>,
    /// Logs - non-fatal errors and log messages while processing.
    #[serde(rename = "Logs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logs: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct PlayKubeReport {
    /// Pods - pods created by play kube.
    #[serde(rename = "Pods")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pods: Option<Vec<PlayKubePod>>,
    #[serde(rename = "RmReport")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rm_report: Option<Vec<PodRmReport>>,
    #[serde(rename = "StopReport")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_report: Option<Vec<PodStopReport>>,
    /// Volumes - volumes created by play kube.
    #[serde(rename = "Volumes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes: Option<Vec<PlayKubeVolume>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct PlayKubeVolume {
    /// Name - Name of the volume created by play kube.
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Plugin A plugin for the Engine API
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Plugin {
    #[serde(rename = "Config")]
    pub config: PluginConfig,
    /// True if the plugin is running. False if the plugin is not running, only installed.
    #[serde(rename = "Enabled")]
    pub enabled: bool,
    /// Id
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// name
    #[serde(rename = "Name")]
    pub name: String,
    /// plugin remote reference used to push/pull the plugin
    #[serde(rename = "PluginReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plugin_reference: Option<String>,
    #[serde(rename = "Settings")]
    pub settings: PluginSettings,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct PluginConfig {
    #[serde(rename = "Args")]
    pub args: PluginConfigArgs,
    /// description
    #[serde(rename = "Description")]
    pub description: String,
    /// Docker Version used to create the plugin
    #[serde(rename = "DockerVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub docker_version: Option<String>,
    /// documentation
    #[serde(rename = "Documentation")]
    pub documentation: String,
    /// entrypoint
    #[serde(rename = "Entrypoint")]
    #[serde(deserialize_with = "deserialize_nonoptional_vec")]
    pub entrypoint: Vec<String>,
    /// env
    #[serde(rename = "Env")]
    #[serde(deserialize_with = "deserialize_nonoptional_vec")]
    pub env: Vec<PluginEnv>,
    #[serde(rename = "Interface")]
    pub interface: PluginConfigInterface,
    /// ipc host
    #[serde(rename = "IpcHost")]
    pub ipc_host: bool,
    #[serde(rename = "Linux")]
    pub linux: PluginConfigLinux,
    /// mounts
    #[serde(rename = "Mounts")]
    #[serde(deserialize_with = "deserialize_nonoptional_vec")]
    pub mounts: Vec<PluginMount>,
    #[serde(rename = "Network")]
    pub network: PluginConfigNetwork,
    /// pid host
    #[serde(rename = "PidHost")]
    pub pid_host: bool,
    /// propagated mount
    #[serde(rename = "PropagatedMount")]
    pub propagated_mount: String,
    #[serde(rename = "User")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<PluginConfigUser>,
    /// work dir
    #[serde(rename = "WorkDir")]
    pub work_dir: String,
    #[serde(rename = "rootfs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rootfs: Option<PluginConfigRootfs>,
}

/// PluginConfigArgs plugin config args
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct PluginConfigArgs {
    /// description
    #[serde(rename = "Description")]
    pub description: String,
    /// name
    #[serde(rename = "Name")]
    pub name: String,
    /// settable
    #[serde(rename = "Settable")]
    #[serde(deserialize_with = "deserialize_nonoptional_vec")]
    pub settable: Vec<String>,
    /// value
    #[serde(rename = "Value")]
    #[serde(deserialize_with = "deserialize_nonoptional_vec")]
    pub value: Vec<String>,
}

/// PluginConfigInterface The interface between Docker and the plugin
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct PluginConfigInterface {
    /// Protocol to use for clients connecting to the plugin.
    #[serde(rename = "ProtocolScheme")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol_scheme: Option<String>,
    /// socket
    #[serde(rename = "Socket")]
    pub socket: String,
    /// types
    #[serde(rename = "Types")]
    #[serde(deserialize_with = "deserialize_nonoptional_vec")]
    pub types: Vec<PluginInterfaceType>,
}

/// PluginConfigLinux plugin config linux
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct PluginConfigLinux {
    /// allow all devices
    #[serde(rename = "AllowAllDevices")]
    pub allow_all_devices: bool,
    /// capabilities
    #[serde(rename = "Capabilities")]
    #[serde(deserialize_with = "deserialize_nonoptional_vec")]
    pub capabilities: Vec<String>,
    /// devices
    #[serde(rename = "Devices")]
    #[serde(deserialize_with = "deserialize_nonoptional_vec")]
    pub devices: Vec<PluginDevice>,
}

/// PluginConfigNetwork plugin config network
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct PluginConfigNetwork {
    /// type
    #[serde(rename = "Type")]
    pub _type: String,
}

/// PluginConfigRootfs plugin config rootfs
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct PluginConfigRootfs {
    /// diff ids
    #[serde(rename = "diff_ids")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diff_ids: Option<Vec<String>>,
    /// type
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
}

/// PluginConfigUser plugin config user
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct PluginConfigUser {
    /// g ID
    #[serde(rename = "GID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub GID: Option<i64>,
    /// UID
    #[serde(rename = "UID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub UID: Option<i64>,
}

/// PluginDevice plugin device
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct PluginDevice {
    /// description
    #[serde(rename = "Description")]
    pub description: String,
    /// name
    #[serde(rename = "Name")]
    pub name: String,
    /// path
    #[serde(rename = "Path")]
    pub path: String,
    /// settable
    #[serde(rename = "Settable")]
    #[serde(deserialize_with = "deserialize_nonoptional_vec")]
    pub settable: Vec<String>,
}

/// PluginEnv plugin env
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct PluginEnv {
    /// description
    #[serde(rename = "Description")]
    pub description: String,
    /// name
    #[serde(rename = "Name")]
    pub name: String,
    /// settable
    #[serde(rename = "Settable")]
    #[serde(deserialize_with = "deserialize_nonoptional_vec")]
    pub settable: Vec<String>,
    /// value
    #[serde(rename = "Value")]
    pub value: String,
}

/// PluginInterfaceType plugin interface type
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct PluginInterfaceType {
    /// capability
    #[serde(rename = "Capability")]
    pub capability: String,
    /// prefix
    #[serde(rename = "Prefix")]
    pub prefix: String,
    /// version
    #[serde(rename = "Version")]
    pub version: String,
}

/// PluginMount plugin mount
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct PluginMount {
    /// description
    #[serde(rename = "Description")]
    pub description: String,
    /// destination
    #[serde(rename = "Destination")]
    pub destination: String,
    /// name
    #[serde(rename = "Name")]
    pub name: String,
    /// options
    #[serde(rename = "Options")]
    #[serde(deserialize_with = "deserialize_nonoptional_vec")]
    pub options: Vec<String>,
    /// settable
    #[serde(rename = "Settable")]
    #[serde(deserialize_with = "deserialize_nonoptional_vec")]
    pub settable: Vec<String>,
    /// source
    #[serde(rename = "Source")]
    pub source: String,
    /// type
    #[serde(rename = "Type")]
    pub _type: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct PluginSettings {
    /// args
    #[serde(rename = "Args")]
    #[serde(deserialize_with = "deserialize_nonoptional_vec")]
    pub args: Vec<String>,
    /// devices
    #[serde(rename = "Devices")]
    #[serde(deserialize_with = "deserialize_nonoptional_vec")]
    pub devices: Vec<PluginDevice>,
    /// env
    #[serde(rename = "Env")]
    #[serde(deserialize_with = "deserialize_nonoptional_vec")]
    pub env: Vec<String>,
    /// mounts
    #[serde(rename = "Mounts")]
    #[serde(deserialize_with = "deserialize_nonoptional_vec")]
    pub mounts: Vec<PluginMount>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Plugins {
    #[serde(rename = "log")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log: Option<Vec<String>>,
    #[serde(rename = "network")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<Vec<String>>,
    #[serde(rename = "volume")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct PodBasicConfig {
    /// Hostname is the pod's hostname. If not set, the name of the pod will be used (if a name was not provided here, the name auto-generated for the pod will be used). This will be used by the infra container and all containers in the pod as long as the UTS namespace is shared. Optional.
    #[serde(rename = "hostname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    /// InfraCommand sets the command that will be used to start the infra container. If not set, the default set in the Libpod configuration file will be used. Conflicts with NoInfra=true. Optional.
    #[serde(rename = "infra_command")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infra_command: Option<Vec<String>>,
    /// InfraConmonPidFile is a custom path to store the infra container's conmon PID.
    #[serde(rename = "infra_conmon_pid_file")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infra_conmon_pid_file: Option<String>,
    /// InfraImage is the image that will be used for the infra container. If not set, the default set in the Libpod configuration file will be used. Conflicts with NoInfra=true. Optional.
    #[serde(rename = "infra_image")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infra_image: Option<String>,
    /// InfraName is the name that will be used for the infra container. If not set, the default set in the Libpod configuration file will be used. Conflicts with NoInfra=true. Optional.
    #[serde(rename = "infra_name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infra_name: Option<String>,
    /// Labels are key-value pairs that are used to add metadata to pods. Optional.
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<HashMap<String, String>>,
    /// Name is the name of the pod. If not provided, a name will be generated when the pod is created. Optional.
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// NoInfra tells the pod not to create an infra container. If this is done, many networking-related options will become unavailable. Conflicts with setting any options in PodNetworkConfig, and the InfraCommand and InfraImages in this struct. Optional.
    #[serde(rename = "no_infra")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_infra: Option<bool>,
    #[serde(rename = "pidns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pidns: Option<Namespace>,
    /// PodCreateCommand is the command used to create this pod. This will be shown in the output of Inspect() on the pod, and may also be used by some tools that wish to recreate the pod (e.g. `podman generate systemd --new`). Optional.
    #[serde(rename = "pod_create_command")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pod_create_command: Option<Vec<String>>,
    /// SharedNamespaces instructs the pod to share a set of namespaces. Shared namespaces will be joined (by default) by every container which joins the pod. If not set and NoInfra is false, the pod will set a default set of namespaces to share. Conflicts with NoInfra=true. Optional.
    #[serde(rename = "shared_namespaces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_namespaces: Option<Vec<String>>,
    #[serde(rename = "userns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub userns: Option<Namespace>,
}

/// This will be expanded in future updates to pods.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct PodCgroupConfig {
    /// CgroupParent is the parent for the CGroup that the pod will create. This pod cgroup will, in turn, be the default cgroup parent for all containers in the pod. Optional.
    #[serde(rename = "cgroup_parent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cgroup_parent: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct PodCreateConfig {
    #[serde(rename = "cgroup-parent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cgroup_parent: Option<String>,
    #[serde(rename = "hostname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    #[serde(rename = "infra")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infra: Option<bool>,
    #[serde(rename = "infra-command")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infra_command: Option<String>,
    #[serde(rename = "infra-image")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infra_image: Option<String>,
    #[serde(rename = "infra-name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infra_name: Option<String>,
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "publish")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publish: Option<Vec<String>>,
    #[serde(rename = "share")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share: Option<String>,
}

/// PodCreateOptions provides all possible options for creating a pod and its infra container
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct PodCreateOptions {
    #[serde(rename = "CGroupParent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_group_parent: Option<String>,
    #[serde(rename = "Cpus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpus: Option<f32>,
    #[serde(rename = "CpusetCpus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpuset_cpus: Option<String>,
    #[serde(rename = "CreateCommand")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_command: Option<Vec<String>>,
    #[serde(rename = "Hostname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    #[serde(rename = "Infra")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infra: Option<bool>,
    #[serde(rename = "InfraCommand")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infra_command: Option<String>,
    #[serde(rename = "InfraConmonPidFile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infra_conmon_pid_file: Option<String>,
    #[serde(rename = "InfraImage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infra_image: Option<String>,
    #[serde(rename = "InfraName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infra_name: Option<String>,
    #[serde(rename = "Labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<HashMap<String, String>>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Net")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net: Option<NetOptions>,
    #[serde(rename = "Pid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pid: Option<String>,
    #[serde(rename = "Share")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share: Option<Vec<String>>,
    #[serde(rename = "Userns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub userns: Option<Namespace>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct PodKillReport {
    #[serde(rename = "Errs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errs: Option<Vec<String>>,
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct PodNetworkConfig {
    /// CNINetworks is a list of CNI networks that the infra container will join. As, by default, containers share their network with the infra container, these networks will effectively be joined by the entire pod. Only available when NetNS is set to Bridge, the default for root. Optional.
    #[serde(rename = "cni_networks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cni_networks: Option<Vec<String>>,
    /// DNSOption is a set of DNS options that will be used in the infra container's resolv.conf, which will, by default, be shared with all containers in the pod. Conflicts with NoInfra=true. Optional.
    #[serde(rename = "dns_option")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_option: Option<Vec<String>>,
    /// DNSSearch is a set of DNS search domains that will be used in the infra container's resolv.conf, which will, by default, be shared with all containers in the pod. If not provided, DNS search domains from the host's resolv.conf will be used. Conflicts with NoInfra=true. Optional.
    #[serde(rename = "dns_search")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_search: Option<Vec<String>>,
    /// DNSServer is a set of DNS servers that will be used in the infra container's resolv.conf, which will, by default, be shared with all containers in the pod. If not provided, the host's DNS servers will be used, unless the only server set is a localhost address. As the container cannot connect to the host's localhost, a default server will instead be set. Conflicts with NoInfra=true. Optional.
    #[serde(rename = "dns_server")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_server: Option<Vec<Ip>>,
    /// HostAdd is a set of hosts that will be added to the infra container's etc/hosts that will, by default, be shared with all containers in the pod. Conflicts with NoInfra=true and NoManageHosts. Optional.
    #[serde(rename = "hostadd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostadd: Option<Vec<String>>,
    #[serde(rename = "netns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub netns: Option<Namespace>,
    /// NetworkOptions are additional options for each network Optional.
    #[serde(rename = "network_options")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_options: Option<HashMap<String, Vec<String>>>,
    /// NoManageHosts indicates that /etc/hosts should not be managed by the pod. Instead, each container will create a separate /etc/hosts as they would if not in a pod. Conflicts with HostAdd.
    #[serde(rename = "no_manage_hosts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_manage_hosts: Option<bool>,
    /// NoManageResolvConf indicates that /etc/resolv.conf should not be managed by the pod. Instead, each container will create and manage a separate resolv.conf as if they had not joined a pod. Conflicts with NoInfra=true and DNSServer, DNSSearch, DNSOption. Optional.
    #[serde(rename = "no_manage_resolv_conf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_manage_resolv_conf: Option<bool>,
    /// PortMappings is a set of ports to map into the infra container. As, by default, containers share their network with the infra container, this will forward the ports to the entire pod. Only available if NetNS is set to Bridge or Slirp. Optional.
    #[serde(rename = "portmappings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portmappings: Option<Vec<PortMapping>>,
    #[serde(rename = "static_ip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub static_ip: Option<Ip>,
    #[serde(rename = "static_mac")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub static_mac: Option<HardwareAddr>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct PodPauseReport {
    #[serde(rename = "Errs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errs: Option<Vec<String>>,
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct PodPruneReport {
    #[serde(rename = "Err")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub err: Option<String>,
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct PodResourceConfig {
    /// CPU period of the cpuset, determined by --cpus
    #[serde(rename = "cpu_period")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_period: Option<i64>,
    /// CPU quota of the cpuset, determined by --cpus
    #[serde(rename = "cpu_quota")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_quota: Option<i64>,
    #[serde(rename = "resource_limits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_limits: Option<LinuxResources>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct PodRestartReport {
    #[serde(rename = "Errs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errs: Option<Vec<String>>,
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct PodRmReport {
    #[serde(rename = "Err")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub err: Option<String>,
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// PodSpecGenerator describes options to create a pod
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct PodSpecGenerator {
    /// CgroupParent is the parent for the CGroup that the pod will create. This pod cgroup will, in turn, be the default cgroup parent for all containers in the pod. Optional.
    #[serde(rename = "cgroup_parent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cgroup_parent: Option<String>,
    /// CNINetworks is a list of CNI networks that the infra container will join. As, by default, containers share their network with the infra container, these networks will effectively be joined by the entire pod. Only available when NetNS is set to Bridge, the default for root. Optional.
    #[serde(rename = "cni_networks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cni_networks: Option<Vec<String>>,
    /// CPU period of the cpuset, determined by --cpus
    #[serde(rename = "cpu_period")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_period: Option<i64>,
    /// CPU quota of the cpuset, determined by --cpus
    #[serde(rename = "cpu_quota")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_quota: Option<i64>,
    /// DNSOption is a set of DNS options that will be used in the infra container's resolv.conf, which will, by default, be shared with all containers in the pod. Conflicts with NoInfra=true. Optional.
    #[serde(rename = "dns_option")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_option: Option<Vec<String>>,
    /// DNSSearch is a set of DNS search domains that will be used in the infra container's resolv.conf, which will, by default, be shared with all containers in the pod. If not provided, DNS search domains from the host's resolv.conf will be used. Conflicts with NoInfra=true. Optional.
    #[serde(rename = "dns_search")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_search: Option<Vec<String>>,
    /// DNSServer is a set of DNS servers that will be used in the infra container's resolv.conf, which will, by default, be shared with all containers in the pod. If not provided, the host's DNS servers will be used, unless the only server set is a localhost address. As the container cannot connect to the host's localhost, a default server will instead be set. Conflicts with NoInfra=true. Optional.
    #[serde(rename = "dns_server")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_server: Option<Vec<Ip>>,
    /// HostAdd is a set of hosts that will be added to the infra container's etc/hosts that will, by default, be shared with all containers in the pod. Conflicts with NoInfra=true and NoManageHosts. Optional.
    #[serde(rename = "hostadd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostadd: Option<Vec<String>>,
    /// Hostname is the pod's hostname. If not set, the name of the pod will be used (if a name was not provided here, the name auto-generated for the pod will be used). This will be used by the infra container and all containers in the pod as long as the UTS namespace is shared. Optional.
    #[serde(rename = "hostname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    /// InfraCommand sets the command that will be used to start the infra container. If not set, the default set in the Libpod configuration file will be used. Conflicts with NoInfra=true. Optional.
    #[serde(rename = "infra_command")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infra_command: Option<Vec<String>>,
    /// InfraConmonPidFile is a custom path to store the infra container's conmon PID.
    #[serde(rename = "infra_conmon_pid_file")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infra_conmon_pid_file: Option<String>,
    /// InfraImage is the image that will be used for the infra container. If not set, the default set in the Libpod configuration file will be used. Conflicts with NoInfra=true. Optional.
    #[serde(rename = "infra_image")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infra_image: Option<String>,
    /// InfraName is the name that will be used for the infra container. If not set, the default set in the Libpod configuration file will be used. Conflicts with NoInfra=true. Optional.
    #[serde(rename = "infra_name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infra_name: Option<String>,
    /// Labels are key-value pairs that are used to add metadata to pods. Optional.
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<HashMap<String, String>>,
    /// Name is the name of the pod. If not provided, a name will be generated when the pod is created. Optional.
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "netns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub netns: Option<Namespace>,
    /// NetworkOptions are additional options for each network Optional.
    #[serde(rename = "network_options")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_options: Option<HashMap<String, Vec<String>>>,
    /// NoInfra tells the pod not to create an infra container. If this is done, many networking-related options will become unavailable. Conflicts with setting any options in PodNetworkConfig, and the InfraCommand and InfraImages in this struct. Optional.
    #[serde(rename = "no_infra")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_infra: Option<bool>,
    /// NoManageHosts indicates that /etc/hosts should not be managed by the pod. Instead, each container will create a separate /etc/hosts as they would if not in a pod. Conflicts with HostAdd.
    #[serde(rename = "no_manage_hosts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_manage_hosts: Option<bool>,
    /// NoManageResolvConf indicates that /etc/resolv.conf should not be managed by the pod. Instead, each container will create and manage a separate resolv.conf as if they had not joined a pod. Conflicts with NoInfra=true and DNSServer, DNSSearch, DNSOption. Optional.
    #[serde(rename = "no_manage_resolv_conf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_manage_resolv_conf: Option<bool>,
    #[serde(rename = "pidns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pidns: Option<Namespace>,
    /// PodCreateCommand is the command used to create this pod. This will be shown in the output of Inspect() on the pod, and may also be used by some tools that wish to recreate the pod (e.g. `podman generate systemd --new`). Optional.
    #[serde(rename = "pod_create_command")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pod_create_command: Option<Vec<String>>,
    /// PortMappings is a set of ports to map into the infra container. As, by default, containers share their network with the infra container, this will forward the ports to the entire pod. Only available if NetNS is set to Bridge or Slirp. Optional.
    #[serde(rename = "portmappings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portmappings: Option<Vec<PortMapping>>,
    #[serde(rename = "resource_limits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_limits: Option<LinuxResources>,
    /// SharedNamespaces instructs the pod to share a set of namespaces. Shared namespaces will be joined (by default) by every container which joins the pod. If not set and NoInfra is false, the pod will set a default set of namespaces to share. Conflicts with NoInfra=true. Optional.
    #[serde(rename = "shared_namespaces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_namespaces: Option<Vec<String>>,
    #[serde(rename = "static_ip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub static_ip: Option<Ip>,
    #[serde(rename = "static_mac")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub static_mac: Option<HardwareAddr>,
    #[serde(rename = "userns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub userns: Option<Namespace>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct PodStartReport {
    #[serde(rename = "Errs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errs: Option<Vec<String>>,
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct PodStopReport {
    #[serde(rename = "Errs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errs: Option<Vec<String>>,
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct PodUnpauseReport {
    #[serde(rename = "Errs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errs: Option<Vec<String>>,
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// Port An open port on a container
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Port {
    /// Host IP address that the container's port is mapped to
    #[serde(rename = "IP")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub IP: Option<String>,
    /// Port on the container
    #[serde(rename = "PrivatePort")]
    pub private_port: i64,
    /// Port exposed on the host
    #[serde(rename = "PublicPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_port: Option<i64>,
    /// type
    #[serde(rename = "Type")]
    pub _type: String,
}

/// PortBinding represents a binding between a Host IP address and a Host Port
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct PortBinding {
    /// HostIP is the host IP Address
    #[serde(rename = "HostIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_ip: Option<String>,
    /// HostPort is the host port number
    #[serde(rename = "HostPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_port: Option<String>,
}

/// PortMap is a collection of PortBinding indexed by Port
pub type PortMap = HashMap<String, String>;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct PortMapping {
    /// ContainerPort is the port number that will be exposed from the container. Mandatory.
    #[serde(rename = "container_port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_port: Option<i64>,
    /// HostIP is the IP that we will bind to on the host. If unset, assumed to be 0.0.0.0 (all interfaces).
    #[serde(rename = "host_ip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_ip: Option<String>,
    /// HostPort is the port number that will be forwarded from the host into the container. If omitted, a random port on the host (guaranteed to be over 1024) will be assigned.
    #[serde(rename = "host_port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_port: Option<i64>,
    /// Protocol is the protocol forward. Must be either \"tcp\", \"udp\", and \"sctp\", or some combination of these separated by commas. If unset, assumed to be TCP.
    #[serde(rename = "protocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    /// Range is the number of ports that will be forwarded, starting at HostPort and ContainerPort and counting up. This is 1-indexed, so 1 is assumed to be a single port (only the Hostport:Containerport mapping will be added), 2 is two ports (both Hostport:Containerport and Hostport+1:Containerport+1), etc. If unset, assumed to be 1 (a single port). Both hostport + range and containerport + range must be less than 65536.
    #[serde(rename = "range")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<i64>,
}

/// PortSet is a collection of structs indexed by Port
pub type PortSet = HashMap<String, String>;

/// POSIXRlimit type and restrictions
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct PosixRlimit {
    /// Hard is the hard limit for the specified type
    #[serde(rename = "hard")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hard: Option<i64>,
    /// Soft is the soft limit for the specified type
    #[serde(rename = "soft")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub soft: Option<i64>,
    /// Type of the rlimit to set
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
}

pub type Propagation = String;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct PruneReport {
    #[serde(rename = "Err")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub err: Option<String>,
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
}

/// RemoteSocket describes information about the API socket
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct RemoteSocket {
    #[serde(rename = "exists")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exists: Option<bool>,
    #[serde(rename = "path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

/// Resources contains container's resources (cgroups config, ulimits...)
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
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
    pub blkio_weight: Option<i64>,
    #[serde(rename = "BlkioWeightDevice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blkio_weight_device: Option<Vec<WeightDevice>>,
    /// Applicable to UNIX platforms
    #[serde(rename = "CgroupParent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cgroup_parent: Option<String>,
    /// Applicable to Windows
    #[serde(rename = "CpuCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    /// Applicable to all platforms
    #[serde(rename = "CpuShares")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    pub io_maximum_bandwidth: Option<i64>,
    #[serde(rename = "IOMaximumIOps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub io_maximum_i_ops: Option<i64>,
    #[serde(rename = "KernelMemory")]
    #[serde(skip_serializing_if = "Option::is_none")]
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

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct RestartPolicy {
    #[serde(rename = "MaximumRetryCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_retry_count: Option<i64>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// RootFS holds the root fs information of an image
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct RootFs {
    #[serde(rename = "Layers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layers: Option<Vec<Digest>>,
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
}

/// Schema2HealthConfig is a HealthConfig, which holds configuration settings for the HEALTHCHECK feature, from docker/docker/api/types/container.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Schema2HealthConfig {
    #[serde(rename = "Interval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<Duration>,
    /// Retries is the number of consecutive failures needed to consider a container as unhealthy. Zero means inherit.
    #[serde(rename = "Retries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retries: Option<i64>,
    #[serde(rename = "StartPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_period: Option<Duration>,
    /// Test is the test to perform to check that the container is healthy. An empty slice means to inherit the default. The options are: {} : inherit healthcheck {\"NONE\"} : disable healthcheck {\"CMD\", args...} : exec arguments directly {\"CMD-SHELL\", command} : run command with system's default shell
    #[serde(rename = "Test")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test: Option<Vec<String>>,
    #[serde(rename = "Timeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<Duration>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Schema2List {
    #[serde(rename = "manifests")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifests: Option<Vec<Schema2ManifestDescriptor>>,
    #[serde(rename = "mediaType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_type: Option<String>,
    #[serde(rename = "schemaVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_version: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Schema2ManifestDescriptor {
    #[serde(rename = "digest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest: Option<Digest>,
    #[serde(rename = "mediaType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_type: Option<String>,
    #[serde(rename = "platform")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<Schema2PlatformSpec>,
    #[serde(rename = "size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(rename = "urls")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urls: Option<Vec<String>>,
}

/// Schema2PlatformSpec describes the platform which a particular manifest is specialized for.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Schema2PlatformSpec {
    #[serde(rename = "architecture")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,
    #[serde(rename = "features")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<Vec<String>>,
    #[serde(rename = "os")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,
    #[serde(rename = "os.features")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_features: Option<Vec<String>>,
    #[serde(rename = "os.version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_version: Option<String>,
    #[serde(rename = "variant")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Secret {
    #[serde(rename = "GID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub GID: Option<i64>,
    #[serde(rename = "Mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<i64>,
    #[serde(rename = "Source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(rename = "Target")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(rename = "UID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub UID: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct SecretCreate {
    /// Base64-url-safe-encoded (RFC 4648) data to store as secret.
    #[serde(rename = "Data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    #[serde(rename = "Driver")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver: Option<SecretDriverSpec>,
    /// User-defined name of the secret.
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct SecretDriverSpec {
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Options")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<HashMap<String, String>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct SecretInfoReport {
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ID: Option<String>,
    #[serde(rename = "Spec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: Option<SecretSpec>,
    #[serde(rename = "UpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct SecretInfoReportCompat {
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ID: Option<String>,
    #[serde(rename = "Spec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: Option<SecretSpec>,
    #[serde(rename = "UpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<SecretVersion>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct SecretSpec {
    #[serde(rename = "Driver")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver: Option<SecretDriverSpec>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct SecretVersion {
    #[serde(rename = "Index")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<i64>,
}

/// HostInfo describes the libpod host
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct SecurityInfo {
    #[serde(rename = "apparmorEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apparmor_enabled: Option<bool>,
    #[serde(rename = "capabilities")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<String>,
    #[serde(rename = "rootless")]
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

/// ServiceInfo represents service parameters with the list of service's tasks
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
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
    pub VIP: Option<String>,
}

/// ServiceUpdateResponse service update response
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ServiceUpdateResponse {
    /// Optional warning messages
    #[serde(rename = "Warnings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<String>>,
}

/// It implements the os.Signal interface.
pub type Signal = i64;

/// SlirpInfo describes the slirp executable that is being being used.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct SlirpInfo {
    #[serde(rename = "executable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub executable: Option<String>,
    #[serde(rename = "package")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package: Option<String>,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// SpecGenerator creates an OCI spec and Libpod configuration options to create a container based on the given configuration.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct SpecGenerator {
    /// Aliases are a list of network-scoped aliases for container Optional
    #[serde(rename = "aliases")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliases: Option<HashMap<String, Vec<String>>>,
    /// Annotations are key-value options passed into the container runtime that can be used to trigger special behavior. Optional.
    #[serde(rename = "annotations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<HashMap<String, String>>,
    /// ApparmorProfile is the name of the Apparmor profile the container will use. Optional.
    #[serde(rename = "apparmor_profile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apparmor_profile: Option<String>,
    /// CapAdd are capabilities which will be added to the container. Conflicts with Privileged. Optional.
    #[serde(rename = "cap_add")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cap_add: Option<Vec<String>>,
    /// CapDrop are capabilities which will be removed from the container. Conflicts with Privileged. Optional.
    #[serde(rename = "cap_drop")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cap_drop: Option<Vec<String>>,
    /// CgroupParent is the container's CGroup parent. If not set, the default for the current cgroup driver will be used. Optional.
    #[serde(rename = "cgroup_parent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cgroup_parent: Option<String>,
    #[serde(rename = "cgroupns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cgroupns: Option<Namespace>,
    /// CgroupsMode sets a policy for how cgroups will be created in the container, including the ability to disable creation entirely.
    #[serde(rename = "cgroups_mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cgroups_mode: Option<String>,
    /// CNINetworks is a list of CNI networks to join the container to. If this list is empty, the default CNI network will be joined instead. If at least one entry is present, we will not join the default network (unless it is part of this list). Only available if NetNS is set to bridge. Optional.
    #[serde(rename = "cni_networks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cni_networks: Option<Vec<String>>,
    /// Command is the container's command. If not given and Image is specified, this will be populated by the image's configuration. Optional.
    #[serde(rename = "command")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<Vec<String>>,
    /// ConmonPidFile is a path at which a PID file for Conmon will be placed. If not given, a default location will be used. Optional.
    #[serde(rename = "conmon_pid_file")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conmon_pid_file: Option<String>,
    /// ContainerCreateCommand is the command that was used to create this container. This will be shown in the output of Inspect() on the container, and may also be used by some tools that wish to recreate the container (e.g. `podman generate systemd --new`). Optional.
    #[serde(rename = "containerCreateCommand")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_create_command: Option<Vec<String>>,
    /// CPU period of the cpuset, determined by --cpus
    #[serde(rename = "cpu_period")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_period: Option<i64>,
    /// CPU quota of the cpuset, determined by --cpus
    #[serde(rename = "cpu_quota")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_quota: Option<i64>,
    /// Create the working directory if it doesn't exist. If unset, it doesn't create it. Optional.
    #[serde(rename = "create_working_dir")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_working_dir: Option<bool>,
    /// DependencyContainers is an array of containers this container depends on. Dependency containers must be started before this container. Dependencies can be specified by name or full/partial ID. Optional.
    #[serde(rename = "dependencyContainers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependency_containers: Option<Vec<String>>,
    /// DeviceCGroupRule are device cgroup rules that allow containers to use additional types of devices.
    #[serde(rename = "device_cgroup_rule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_cgroup_rule: Option<Vec<LinuxDeviceCgroup>>,
    /// Devices are devices that will be added to the container. Optional.
    #[serde(rename = "devices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<LinuxDevice>>,
    /// DNSOptions is a set of DNS options that will be used in the container's resolv.conf, replacing the host's DNS options which are used by default. Conflicts with UseImageResolvConf. Optional.
    #[serde(rename = "dns_option")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_option: Option<Vec<String>>,
    /// DNSSearch is a set of DNS search domains that will be used in the container's resolv.conf, replacing the host's DNS search domains which are used by default. Conflicts with UseImageResolvConf. Optional.
    #[serde(rename = "dns_search")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_search: Option<Vec<String>>,
    /// DNSServers is a set of DNS servers that will be used in the container's resolv.conf, replacing the host's DNS Servers which are used by default. Conflicts with UseImageResolvConf. Optional.
    #[serde(rename = "dns_server")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_server: Option<Vec<Ip>>,
    /// Entrypoint is the container's entrypoint. If not given and Image is specified, this will be populated by the image's configuration. Optional.
    #[serde(rename = "entrypoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entrypoint: Option<Vec<String>>,
    /// Env is a set of environment variables that will be set in the container. Optional.
    #[serde(rename = "env")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<HashMap<String, String>>,
    /// EnvHost indicates that the host environment should be added to container Optional.
    #[serde(rename = "env_host")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env_host: Option<bool>,
    /// Expose is a number of ports that will be forwarded to the container if PublishExposedPorts is set. Expose is a map of uint16 (port number) to a string representing protocol. Allowed protocols are \"tcp\", \"udp\", and \"sctp\", or some combination of the three separated by commas. If protocol is set to \"\" we will assume TCP. Only available if NetNS is set to Bridge or Slirp, and PublishExposedPorts is set. Optional.
    #[serde(rename = "expose")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expose: Option<Value>,
    /// Groups are a list of supplemental groups the container's user will be granted access to. Optional.
    #[serde(rename = "groups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<String>>,
    #[serde(rename = "healthconfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub healthconfig: Option<Schema2HealthConfig>,
    /// HostAdd is a set of hosts which will be added to the container's etc/hosts file. Conflicts with UseImageHosts. Optional.
    #[serde(rename = "hostadd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostadd: Option<Vec<String>>,
    /// Hostname is the container's hostname. If not set, the hostname will not be modified (if UtsNS is not private) or will be set to the container ID (if UtsNS is private). Conflicts with UtsNS if UtsNS is not set to private. Optional.
    #[serde(rename = "hostname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    /// EnvHTTPProxy indicates that the http host proxy environment variables should be added to container Optional.
    #[serde(rename = "httpproxy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub httpproxy: Option<bool>,
    #[serde(rename = "idmappings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idmappings: Option<IdMappingOptions>,
    /// Image is the image the container will be based on. The image will be used as the container's root filesystem, and its environment vars, volumes, and other configuration will be applied to the container. Conflicts with Rootfs. At least one of Image or Rootfs must be specified.
    #[serde(rename = "image")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// ImageVolumeMode indicates how image volumes will be created. Supported modes are \"ignore\" (do not create), \"tmpfs\" (create as tmpfs), and \"anonymous\" (create as anonymous volumes). The default if unset is anonymous. Optional.
    #[serde(rename = "image_volume_mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_volume_mode: Option<String>,
    /// Image volumes bind-mount a container-image mount into the container. Optional.
    #[serde(rename = "image_volumes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_volumes: Option<Vec<ImageVolume>>,
    /// Init specifies that an init binary will be mounted into the container, and will be used as PID1.
    #[serde(rename = "init")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub init: Option<bool>,
    /// InitContainerType describes if this container is an init container and if so, what type: always or once
    #[serde(rename = "init_container_type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub init_container_type: Option<String>,
    /// InitPath specifies the path to the init binary that will be added if Init is specified above. If not specified, the default set in the Libpod config will be used. Ignored if Init above is not set. Optional.
    #[serde(rename = "init_path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub init_path: Option<String>,
    #[serde(rename = "ipcns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipcns: Option<Namespace>,
    /// Labels are key-value pairs that are used to add metadata to containers. Optional.
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<HashMap<String, String>>,
    #[serde(rename = "log_configuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_configuration: Option<LogConfig>,
    /// Mask is the path we want to mask in the container. This masks the paths given in addition to the default list. Optional
    #[serde(rename = "mask")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask: Option<Vec<String>>,
    /// Mounts are mounts that will be added to the container. These will supersede Image Volumes and VolumesFrom volumes where there are conflicts. Optional.
    #[serde(rename = "mounts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mounts: Option<Vec<Mount>>,
    /// Name is the name the container will be given. If no name is provided, one will be randomly generated. Optional.
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Namespace is the libpod namespace the container will be placed in. Optional.
    #[serde(rename = "namespace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(rename = "netns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub netns: Option<Namespace>,
    /// NetworkOptions are additional options for each network Optional.
    #[serde(rename = "network_options")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_options: Option<HashMap<String, Vec<String>>>,
    /// NoNewPrivileges is whether the container will set the no new privileges flag on create, which disables gaining additional privileges (e.g. via setuid) in the container.
    #[serde(rename = "no_new_privileges")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_new_privileges: Option<bool>,
    /// OCIRuntime is the name of the OCI runtime that will be used to create the container. If not specified, the default will be used. Optional.
    #[serde(rename = "oci_runtime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oci_runtime: Option<String>,
    /// OOMScoreAdj adjusts the score used by the OOM killer to determine processes to kill for the container's process. Optional.
    #[serde(rename = "oom_score_adj")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oom_score_adj: Option<i64>,
    /// Overlay volumes are named volumes that will be added to the container. Optional.
    #[serde(rename = "overlay_volumes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overlay_volumes: Option<Vec<OverlayVolume>>,
    #[serde(rename = "personality")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personality: Option<LinuxPersonality>,
    #[serde(rename = "pidns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pidns: Option<Namespace>,
    /// Pod is the ID of the pod the container will join. Optional.
    #[serde(rename = "pod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pod: Option<String>,
    /// PortBindings is a set of ports to map into the container. Only available if NetNS is set to bridge or slirp. Optional.
    #[serde(rename = "portmappings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portmappings: Option<Vec<PortMapping>>,
    /// Privileged is whether the container is privileged. Privileged does the following: Adds all devices on the system to the container. Adds all capabilities to the container. Disables Seccomp, SELinux, and Apparmor confinement. (Though SELinux can be manually re-enabled). TODO: this conflicts with things. TODO: this does more.
    #[serde(rename = "privileged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privileged: Option<bool>,
    /// ProcOpts are the options used for the proc mount.
    #[serde(rename = "procfs_opts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub procfs_opts: Option<Vec<String>>,
    /// PublishExposedPorts will publish ports specified in the image to random unused ports (guaranteed to be above 1024) on the host. This is based on ports set in Expose below, and any ports specified by the Image (if one is given). Only available if NetNS is set to Bridge or Slirp.
    #[serde(rename = "publish_image_ports")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publish_image_ports: Option<bool>,
    /// Rlimits are POSIX rlimits to apply to the container. Optional.
    #[serde(rename = "r_limits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r_limits: Option<Vec<PosixRlimit>>,
    /// RawImageName is the user-specified and unprocessed input referring to a local or a remote image.
    #[serde(rename = "raw_image_name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_image_name: Option<String>,
    /// ReadOnlyFilesystem indicates that everything will be mounted as read-only
    #[serde(rename = "read_only_filesystem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only_filesystem: Option<bool>,
    /// Remove indicates if the container should be removed once it has been started and exits
    #[serde(rename = "remove")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove: Option<bool>,
    #[serde(rename = "resource_limits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_limits: Option<LinuxResources>,
    /// RestartPolicy is the container's restart policy - an action which will be taken when the container exits. If not given, the default policy, which does nothing, will be used. Optional.
    #[serde(rename = "restart_policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restart_policy: Option<String>,
    /// RestartRetries is the number of attempts that will be made to restart the container. Only available when RestartPolicy is set to \"on-failure\". Optional.
    #[serde(rename = "restart_tries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restart_tries: Option<i64>,
    /// Rootfs is the path to a directory that will be used as the container's root filesystem. No modification will be made to the directory, it will be directly mounted into the container as root. Conflicts with Image. At least one of Image or Rootfs must be specified.
    #[serde(rename = "rootfs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rootfs: Option<String>,
    /// RootfsPropagation is the rootfs propagation mode for the container. If not set, the default of rslave will be used. Optional.
    #[serde(rename = "rootfs_propagation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rootfs_propagation: Option<String>,
    /// Determine how to handle the NOTIFY_SOCKET - do we participate or pass it through \"container\" - let the OCI runtime deal with it, advertise conmon's MAINPID \"conmon-only\" - advertise conmon's MAINPID, send READY when started, don't pass to OCI \"ignore\" - unset NOTIFY_SOCKET
    #[serde(rename = "sdnotifyMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sdnotify_mode: Option<String>,
    /// SeccompPolicy determines which seccomp profile gets applied the container. valid values: empty,default,image
    #[serde(rename = "seccomp_policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seccomp_policy: Option<String>,
    /// SeccompProfilePath is the path to a JSON file containing the container's Seccomp profile. If not specified, no Seccomp profile will be used. Optional.
    #[serde(rename = "seccomp_profile_path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seccomp_profile_path: Option<String>,
    /// EnvSecrets are secrets that will be set as environment variables Optional.
    #[serde(rename = "secret_env")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_env: Option<HashMap<String, String>>,
    /// Secrets are the secrets that will be added to the container Optional.
    #[serde(rename = "secrets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets: Option<Vec<Secret>>,
    /// SelinuxProcessLabel is the process label the container will use. If SELinux is enabled and this is not specified, a label will be automatically generated if not specified. Optional.
    #[serde(rename = "selinux_opts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selinux_opts: Option<Vec<String>>,
    /// ShmSize is the size of the tmpfs to mount in at /dev/shm, in bytes. Conflicts with ShmSize if IpcNS is not private. Optional.
    #[serde(rename = "shm_size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shm_size: Option<i64>,
    #[serde(rename = "static_ip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub static_ip: Option<Ip>,
    #[serde(rename = "static_ipv6")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub static_ipv6: Option<Ip>,
    #[serde(rename = "static_mac")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub static_mac: Option<HardwareAddr>,
    /// Stdin is whether the container will keep its STDIN open.
    #[serde(rename = "stdin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stdin: Option<bool>,
    #[serde(rename = "stop_signal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_signal: Option<Signal>,
    /// StopTimeout is a timeout between the container's stop signal being sent and SIGKILL being sent. If not provided, the default will be used. If 0 is used, stop signal will not be sent, and SIGKILL will be sent instead. Optional.
    #[serde(rename = "stop_timeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_timeout: Option<i64>,
    /// Sysctl sets kernel parameters for the container
    #[serde(rename = "sysctl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sysctl: Option<HashMap<String, String>>,
    /// Systemd is whether the container will be started in systemd mode. Valid options are \"true\", \"false\", and \"always\". \"true\" enables this mode only if the binary run in the container is sbin/init or systemd. \"always\" unconditionally enables systemd mode. \"false\" unconditionally disables systemd mode. If enabled, mounts and stop signal will be modified. If set to \"always\" or set to \"true\" and conditionally triggered, conflicts with StopSignal. If not specified, \"false\" will be assumed. Optional.
    #[serde(rename = "systemd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub systemd: Option<String>,
    /// Terminal is whether the container will create a PTY. Optional.
    #[serde(rename = "terminal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminal: Option<bool>,
    /// IO read rate limit per cgroup per device, bytes per second
    #[serde(rename = "throttleReadBpsDevice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throttle_read_bps_device: Option<HashMap<String, LinuxThrottleDevice>>,
    /// IO read rate limit per cgroup per device, IO per second
    #[serde(rename = "throttleReadIOPSDevice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throttle_read_iops_device: Option<HashMap<String, LinuxThrottleDevice>>,
    /// IO write rate limit per cgroup per device, bytes per second
    #[serde(rename = "throttleWriteBpsDevice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throttle_write_bps_device: Option<HashMap<String, LinuxThrottleDevice>>,
    /// IO write rate limit per cgroup per device, IO per second
    #[serde(rename = "throttleWriteIOPSDevice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throttle_write_iops_device: Option<HashMap<String, LinuxThrottleDevice>>,
    /// Timeout is a maximum time in seconds the container will run before main process is sent SIGKILL. If 0 is used, signal will not be sent. Container can run indefinitely Optional.
    #[serde(rename = "timeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i64>,
    /// Timezone is the timezone inside the container. Local means it has the same timezone as the host machine Optional.
    #[serde(rename = "timezone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    /// Umask is the umask the init process of the container will be run with.
    #[serde(rename = "umask")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub umask: Option<String>,
    /// CgroupConf are key-value options passed into the container runtime that are used to configure cgroup v2. Optional.
    #[serde(rename = "unified")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unified: Option<HashMap<String, String>>,
    /// Unmask is the path we want to unmask in the container. To override all the default paths that are masked, set unmask=ALL.
    #[serde(rename = "unmask")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unmask: Option<Vec<String>>,
    /// UseImageHosts indicates that /etc/hosts should not be managed by Podman, and instead sourced from the image. Conflicts with HostAdd.
    #[serde(rename = "use_image_hosts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_image_hosts: Option<bool>,
    /// UseImageResolvConf indicates that resolv.conf should not be managed by Podman, but instead sourced from the image. Conflicts with DNSServer, DNSSearch, DNSOption.
    #[serde(rename = "use_image_resolve_conf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_image_resolve_conf: Option<bool>,
    /// User is the user the container will be run as. Can be given as a UID or a username; if a username, it will be resolved within the container, using the container's /etc/passwd. If unset, the container will be run as root. Optional.
    #[serde(rename = "user")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    #[serde(rename = "userns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub userns: Option<Namespace>,
    #[serde(rename = "utsns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utsns: Option<Namespace>,
    /// Volatile specifies whether the container storage can be optimized at the cost of not syncing all the dirty files in memory.
    #[serde(rename = "volatile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volatile: Option<bool>,
    /// Volumes are named volumes that will be added to the container. These will supersede Image Volumes and VolumesFrom volumes where there are conflicts. Optional.
    #[serde(rename = "volumes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes: Option<Vec<NamedVolume>>,
    /// VolumesFrom is a set of containers whose volumes will be added to this container. The name or ID of the container must be provided, and may optionally be followed by a : and then one or more comma-separated options. Valid options are 'ro', 'rw', and 'z'. Options will be used for all volumes sourced from the container.
    #[serde(rename = "volumes_from")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes_from: Option<Vec<String>>,
    /// Weight per cgroup per device, can override BlkioWeight
    #[serde(rename = "weightDevice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight_device: Option<HashMap<String, LinuxWeightDevice>>,
    /// WorkDir is the container's working directory. If unset, the default, /, will be used. Optional.
    #[serde(rename = "work_dir")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_dir: Option<String>,
}

/// StoreInfo describes the container storage and its attributes
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct StoreInfo {
    #[serde(rename = "configFile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_file: Option<String>,
    #[serde(rename = "containerStore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_store: Option<ContainerStore>,
    #[serde(rename = "graphDriverName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graph_driver_name: Option<String>,
    #[serde(rename = "graphOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graph_options: Option<HashMap<String, Value>>,
    #[serde(rename = "graphRoot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graph_root: Option<String>,
    #[serde(rename = "graphStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graph_status: Option<HashMap<String, String>>,
    #[serde(rename = "imageStore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_store: Option<ImageStore>,
    #[serde(rename = "runRoot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_root: Option<String>,
    #[serde(rename = "volumePath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_path: Option<String>,
}

/// We need to override the json decoder to accept both options.
pub type StrSlice = Vec<String>;

/// SystemDfContainerReport describes a container for use with df
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
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

/// SystemDfImageReport describes an image for use with df
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
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

/// SystemDfVolumeReport describes a volume and its size
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
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

/// Task carries the information about one backend task
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
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

/// ThrottleDevice is a structure that holds device:rate_per_second pair
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ThrottleDevice {
    #[serde(rename = "Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "Rate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct TmpfsOptions {
    #[serde(rename = "Mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<FileMode>,
    /// Size sets the size of the tmpfs, in bytes.  This will be converted to an operating system specific value depending on the host. For example, on linux, it will be converted to use a 'k', 'm' or 'g' syntax. BSD, though not widely supported with docker, uses a straight byte value.  Percentages are not supported.
    #[serde(rename = "SizeBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_bytes: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
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

pub type UsernsMode = String;

pub type UtsMode = String;

/// Version is an output struct for API
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Version {
    #[serde(rename = "APIVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,
    #[serde(rename = "Built")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub built: Option<i64>,
    #[serde(rename = "BuiltTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub built_time: Option<String>,
    #[serde(rename = "GitCommit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub git_commit: Option<String>,
    #[serde(rename = "GoVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub go_version: Option<String>,
    #[serde(rename = "OsArch")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_arch: Option<String>,
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// Volume volume
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Volume {
    /// Date/Time the volume was created.
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Name of the volume driver used by the volume.
    #[serde(rename = "Driver")]
    pub driver: String,
    /// User-defined key/value metadata.
    #[serde(rename = "Labels")]
    #[serde(deserialize_with = "deserialize_nonoptional_map")]
    pub labels: HashMap<String, String>,
    /// Mount path of the volume on the host.
    #[serde(rename = "Mountpoint")]
    pub mountpoint: String,
    /// Name of the volume.
    #[serde(rename = "Name")]
    pub name: String,
    /// The driver specific options used when creating the volume.
    #[serde(rename = "Options")]
    #[serde(deserialize_with = "deserialize_nonoptional_map")]
    pub options: HashMap<String, String>,
    /// The level at which the volume exists. Either `global` for cluster-wide, or `local` for machine level.
    #[serde(rename = "Scope")]
    pub scope: String,
    /// Low-level details about the volume, provided by the volume driver. Details are returned as a map with key/value pairs: `{\"key\":\"value\",\"key2\":\"value2\"}`.  The `Status` field is optional, and is omitted if the volume driver does not support this feature.
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<HashMap<String, Value>>,
    #[serde(rename = "UsageData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_data: Option<VolumeUsageData>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct VolumeCreate {
    /// Volume driver to use
    #[serde(rename = "Driver")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver: Option<String>,
    /// User-defined key/value metadata.
    #[serde(rename = "Label")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<HashMap<String, String>>,
    /// New volume's name. Can be left blank
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Mapping of driver options and values.
    #[serde(rename = "Options")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<HashMap<String, String>>,
}

/// VolumeCreateBody Volume configuration
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct VolumeCreateBody {
    /// Name of the volume driver to use.
    #[serde(rename = "Driver")]
    pub driver: String,
    /// A mapping of driver options and values. These options are passed directly to the driver and are driver specific.
    #[serde(rename = "DriverOpts")]
    #[serde(deserialize_with = "deserialize_nonoptional_map")]
    pub driver_opts: HashMap<String, String>,
    /// User-defined key/value metadata.
    #[serde(rename = "Labels")]
    #[serde(deserialize_with = "deserialize_nonoptional_map")]
    pub labels: HashMap<String, String>,
    /// The new volume's name. If not specified, Docker generates a name.
    #[serde(rename = "Name")]
    pub name: String,
}

/// VolumeListBody Volume list response
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct VolumeListBody {
    #[serde(rename = "Volumes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes: Option<Vec<VolumeListOkBody>>,
}

/// VolumeListOKBody Volume list response
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct VolumeListOkBody {
    /// List of volumes
    #[serde(rename = "Volumes")]
    #[serde(deserialize_with = "deserialize_nonoptional_vec")]
    pub volumes: Vec<Volume>,
    /// Warnings that occurred when fetching the list of volumes.
    #[serde(rename = "Warnings")]
    #[serde(deserialize_with = "deserialize_nonoptional_vec")]
    pub warnings: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct VolumeOptions {
    #[serde(rename = "DriverConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver_config: Option<Driver>,
    #[serde(rename = "Labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<HashMap<String, String>>,
    #[serde(rename = "NoCopy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_copy: Option<bool>,
}

/// VolumeUsageData Usage details about the volume. This information is used by the `GET /system/df` endpoint, and omitted in other endpoints.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct VolumeUsageData {
    /// The number of containers referencing this volume. This field is set to `-1` if the reference-count is not available.
    #[serde(rename = "RefCount")]
    pub ref_count: i64,
    /// Amount of disk space used by the volume (in bytes). This information is only available for volumes created with the `\"local\"` volume driver. For volumes created with other volume drivers, this field is set to `-1` (\"not available\")
    #[serde(rename = "Size")]
    pub size: i64,
}

/// WeightDevice is a structure that holds device:weight pair
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct WeightDevice {
    #[serde(rename = "Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "Weight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<i64>,
}
