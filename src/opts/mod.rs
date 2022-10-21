//! Options used for configuring the behavior of certain API endpoints

mod containers;
mod exec;
mod images;
mod manifests;
mod networks;
mod pods;
mod volumes;

pub use containers::*;
pub use exec::*;
pub use images::*;
pub use manifests::*;
pub use networks::*;
pub use pods::*;
pub use volumes::*;

pub type EventsConstraint = (String, Vec<String>);

use containers_api::{
    impl_opts_builder, impl_opts_required_builder, impl_url_bool_field, impl_url_enum_field,
    impl_url_field, impl_url_str_field, impl_url_vec_field,
};
use std::fmt;

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
    pub fn filters(mut self, filters: impl IntoIterator<Item = EventsConstraint>) -> Self {
        let filters: std::collections::HashMap<_, _> = filters.into_iter().collect();
        self.params.insert(
            "filters",
            serde_json::to_string(&filters).unwrap_or_default(),
        );
        self
    }
}

impl_opts_builder!(url =>
    /// Adjust how filesystem changes inside a container or image are returned.
    Changes
);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Used with [`ChangesOptsBuilder::diff_type`](ChangesOptsBuilder::diff_type).
pub enum DiffType {
    All,
    Container,
    Image,
}

impl AsRef<str> for DiffType {
    fn as_ref(&self) -> &str {
        match self {
            DiffType::All => "all",
            DiffType::Container => "container",
            DiffType::Image => "image",
        }
    }
}

impl fmt::Display for DiffType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl ChangesOptsBuilder {
    impl_url_enum_field!(
        /// Select what you want to match.
        diff_type: DiffType => "diffType"
    );

    impl_url_str_field!(
        /// Specify a second layer which is used to compare against it instead of the parent layer.
        parent => "parent"
    );
}

impl_opts_builder!(url =>
    /// Adjust how systemd units are generated from a container or pod.
    SystemdUnits
);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Used with [`SystemdUnitsOptsBuilder::restart_policy`](SystemdUnitsOptsBuilder::restart_policy).
pub enum RestartPolicy {
    No,
    OnSuccess,
    OnFailure,
    OnAbnormal,
    OnWatchdog,
    OnAbort,
    Always,
}

impl AsRef<str> for RestartPolicy {
    fn as_ref(&self) -> &str {
        use RestartPolicy::*;
        match self {
            No => "no",
            OnSuccess => "on-success",
            OnFailure => "on-failure",
            OnAbnormal => "on-abnormal",
            OnWatchdog => "on-watchdog",
            OnAbort => "on-abort",
            Always => "always",
        }
    }
}

impl fmt::Display for RestartPolicy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl SystemdUnitsOptsBuilder {
    impl_url_str_field!(
        /// Systemd unit name prefix for containers.
        container_prefix => "containerPrefix"
    );

    impl_url_bool_field!(
        /// Create a new container instead of starting an existing one.
        new => "new"
    );

    impl_url_bool_field!(
        /// Do not generate the header including the Podman version and the timestamp.
        no_header => "noHeader"
    );

    impl_url_str_field!(
        /// Systemd unit name prefix for pods.
        pod_prefix => "podPrefix"
    );

    impl_url_enum_field!(
        /// Systemd restart-policy.
        restart_policy: RestartPolicy => "restartPolicy"
    );

    impl_url_field!(
        /// Configures the time to sleep before restarting a service.
        restart_sec: usize => "restartSec"
    );

    impl_url_str_field!(
        /// Systemd unit name separator between name/id and prefix.
        separator => "separator"
    );

    impl_url_field!(
        /// Start timeout in seconds.
        start_timeout: usize => "startTimeout"
    );

    impl_url_field!(
        /// Stop timeout in seconds.
        stop_timeout: usize => "stopTimeout"
    );

    impl_url_bool_field!(
        /// Use container/pod names instead of IDs.
        use_name => "useName"
    );
}

impl_opts_builder!(url =>
    /// Adjust how a kubernetes YAML will create pods and containers.
    PlayKubernetesYaml
);

impl PlayKubernetesYamlOptsBuilder {
    impl_url_str_field!(
        /// Logging driver for the containers in the pod.
        log_driver => "logDriver"
    );

    impl_url_vec_field!(
        /// Use the network mode or specify an array of networks.
        network => "network"
    );

    impl_url_bool_field!(
        /// Start the pod after creating it.
        start => "start"
    );

    impl_url_vec_field!(
        /// Static IPs used for the pods.
        static_ips => "staticIPs"
    );

    impl_url_vec_field!(
        /// Static MACs used for the pods.
        static_macs => "static_macs"
    );

    impl_url_bool_field!(
        /// Require HTTPS and verify signatures when contacting registries.
        tls_verify => "tlsVerify"
    );
}

//####################################################################################################
//
// Secrets
//
//####################################################################################################

impl_opts_required_builder!(url =>
    /// Used to create a [Secret](crate::api::Secret).
    SecretCreate,
    /// The name of the secret
    name => "name"
);

impl SecretCreateOptsBuilder {
    impl_url_str_field!(
        /// Secret driver. Default is `file`.
        driver => "driver"
    );

    impl_url_str_field!(
        /// Secret driver options.
        driver_opts => "driveropts"
    );
}
