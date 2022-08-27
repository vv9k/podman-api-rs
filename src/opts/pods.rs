use crate::models;
use containers_api::opts::{Equality, Filter, FilterItem};
use containers_api::{
    impl_field, impl_filter_func, impl_map_field, impl_opts_builder, impl_str_field,
    impl_url_bool_field, impl_url_field, impl_url_str_field, impl_url_vec_field, impl_vec_field,
};

impl_opts_builder!(url =>
    /// Adjust the list of returned pods with this options.
    PodList
);

#[derive(Debug)]
/// Used to filter listed pods by one of the variants.
pub enum PodListFilter {
    /// A pods's ID
    Id(crate::Id),
    /// Pods with key label.
    LabelKey(String),
    /// Pods with key=value label.
    LabelKeyVal(String, String),
    /// Pods without key label.
    NoLabelKey(String),
    /// Pods without key=value label.
    NoLabelKeyVal(String, String),
    /// A pods's name
    Name(String),
    /// List pods created before this timestamp. The <timestamp> can be Unix timestamps,
    /// date formatted timestamps, or Go duration strings (e.g. 10m, 1h30m) computed
    /// relative to the daemon machine’s time.
    Until(String),
    /// Name or full ID of network.
    Network(String),
    Status(models::PodStatus),
    /// Container name within the pod.
    ContainerName(String),
    /// Container name within the pod.
    ContainerId(crate::Id),
    /// Container status within the pod.
    ContainerStatus(models::ContainerStatus),
    /// Number of containers within the pod.
    ContainerNumber(usize),
}

impl Filter for PodListFilter {
    fn query_item(&self) -> FilterItem {
        use PodListFilter::*;
        match &self {
            Id(id) => FilterItem::new("id", id.to_string(), Equality::Equal),
            LabelKey(key) => FilterItem::new("label", key.clone(), Equality::Equal),
            LabelKeyVal(key, val) => {
                FilterItem::new("label", format!("{}={}", key, val), Equality::Equal)
            }
            NoLabelKey(key) => FilterItem::new("label", key.clone(), Equality::NotEqual),
            NoLabelKeyVal(key, val) => {
                FilterItem::new("label", format!("{}={}", key, val), Equality::NotEqual)
            }
            Name(name) => FilterItem::new("name", name.clone(), Equality::Equal),
            Until(until) => FilterItem::new("until", until.clone(), Equality::Equal),
            Network(net) => FilterItem::new("network", net.clone(), Equality::Equal),
            Status(status) => {
                FilterItem::new("status", status.as_ref().to_string(), Equality::Equal)
            }
            ContainerName(name) => FilterItem::new("ctr-names", name.clone(), Equality::Equal),
            ContainerId(id) => FilterItem::new("ctr-ids", id.to_string(), Equality::Equal),
            ContainerStatus(status) => {
                FilterItem::new("ctr-status", status.as_ref().to_string(), Equality::Equal)
            }
            ContainerNumber(n) => FilterItem::new("ctr-number", n.to_string(), Equality::Equal),
        }
    }
}

impl PodListOptsBuilder {
    impl_filter_func!(PodListFilter);
}

impl_opts_builder!(url =>
    /// Adjust how processes inside a pod are listed.
    PodTop
);

impl PodTopOpts {
    pub(crate) fn stream(&self) -> Self {
        let mut new = self.clone();
        new.params.insert("stream", true.to_string());
        new
    }
}

impl PodTopOptsBuilder {
    impl_url_field!(
        /// If streaming, delay in seconds between updates.
        delay: usize => "delay"
    );

    impl_url_str_field!(
        /// Arguments to pass to ps such as aux. Requires ps(1) to be installed in the container if
        /// no ps(1) compatible AIX descriptors are used.
        ps_args => "ps_args"
    );
}

impl_opts_builder!(url =>
    /// Adjust how stats of a process are listed.
    PodStats
);

impl PodStatsOpts {
    pub(crate) fn stream(&self) -> Self {
        let mut new = self.clone();
        new.params.insert("stream", true.to_string());
        new
    }
}

impl PodStatsOptsBuilder {
    impl_url_bool_field!(
        /// Provide statistics for all running pods.
        all => "all"
    );

    impl_url_vec_field!(
        /// Names or IDs of pods.
        names_or_ids => "namesOrIds"
    );
}

impl_opts_builder!(json =>
    /// Adjust the way a pod is created.
    PodCreate
);

impl PodCreateOptsBuilder {
    impl_str_field!(
        /// The parent for the CGroup that the pod will create. This pod cgroup will, in turn, be
        /// the default cgroup parent for all containers in the pod.
        cgroup_parent => "cgroup_parent"
    );

    impl_vec_field!(
        /// List of CNI networks to join the container to. If this list is empty, the default CNI
        /// network will be joined instead. If at least one entry is present, we will not join the
        /// default network (unless it is part of this list). Only available if NetNS is set to
        /// bridge. Optional. Deprecated: as of podman 4.0 use "Networks" instead.
        cni_networks => "cni_networks"
    );

    impl_field!(
        /// CPU period of the cpuset, determined by --cpus
        cpu_period: u64 => "cpu_period"
    );

    impl_field!(
        /// CPU quota of the cpuset, determined by --cpus
        cpu_quota: i64 => "cpu_quota"
    );

    impl_vec_field!(
        /// Set of DNS options that will be used in the infra container's resolv.conf, which
        /// will, by default, be shared with all containers in the pod. Conflicts with
        /// [`no_infra`](PodCreateOptsBuilder::no_infra) == true.
        dns_option => "dns_option"
    );

    impl_vec_field!(
        /// Set of DNS search domains that will be used in the infra container's resolv.conf,
        /// which will, by default, be shared with all containers in the pod. If not provided, DNS
        /// search domains from the host's resolv.conf will be used. Conflicts with
        /// [`no_infra`](PodCreateOptsBuilder::no_infra) == true.
        dns_search => "dns_search"
    );

    // TODO: dns_server

    impl_vec_field!(
        /// Set of hosts that will be added to the infra container's etc/hosts that will, by
        /// default, be shared with all containers in the pod. Conflicts with
        /// [`no_infra`](PodCreateOptsBuilder::no_infra) == true and
        /// [`no_manage_hosts`](PodCreateOptsBuilder::no_manage_hosts).
        add_hosts => "hostadd"
    );

    impl_str_field!(
        /// The pod's hostname. If not set, the name of the pod will be used (if a name was not
        /// provided here, the name auto-generated for the pod will be used). This will be used by
        /// the infra container and all containers in the pod as long as the UTS namespace is
        /// shared.
        hostname => "hostname"
    );

    // TODO: image_volumes

    impl_vec_field!(
        /// Sets the command that will be used to start the infra container. If not set, the
        /// default set in the Libpod configuration file will be used. Conflicts with
        /// [`no_infra`](PodCreateOptsBuilder::no_infra) == true.
        infra_command => "infra_command"
    );

    impl_str_field!(
        /// Custom path to store the infra container's conmon PID.
        infra_common_pid_file => "infra_common_pid_file"
    );

    impl_str_field!(
        /// The image that will be used for the infra container. If not set, the default set
        /// in the Libpod configuration file will be used. Conflicts with
        /// [`no_infra`](PodCreateOptsBuilder::no_infra) == true.
        infra_image => "infra_image"
    );

    impl_str_field!(
        /// The name that will be used for the infra container. If not set, the default set in the
        /// Libpod configuration file will be used. Conflicts with
        /// [`no_infra`](PodCreateOptsBuilder::no_infra) == true.
        infra_name => "infra_name"
    );

    impl_map_field!(json
        /// Key-value pairs that are used to add metadata to a pod.
        labels => "labels"
    );

    // TODO: mounts

    impl_str_field!(
        /// The name of the pod. If not provided, a name will be generated when the pod is created.
        name => "name"
    );

    impl_field!(
        netns: models::Namespace => "netns"
    );

    impl_map_field!(json
        /// Additional options for each network.
        network_options => "network_options"
    );

    // TODO: network

    impl_field!(
        /// tells the pod not to create an infra container. If this is done, many
        /// networking-related options will become unavailable. Conflicts with any
        /// network or infra related settings.
        no_infra: bool => "no_infra"
    );

    impl_field!(
        /// Indicates that /etc/hosts should not be managed by the pod. Instead, each container
        /// will create a separate /etc/hosts as they would if not in a pod. Conflicts with
        /// [`add_hosts`](PodCreateOptsBuilder::add_hosts).
        no_manage_hosts: bool => "no_manage_hosts"
    );

    impl_field!(
        /// Indicates that /etc/resolv.conf should not be managed by the pod. Instead, each
        /// container will create and manage a separate resolv.conf as if they had not joined a
        /// pod. Conflicts with [`dns_server`](PodCreateOptsBuilder::dns_server),
        /// [`dns_search`](PodCreateOptsBuilder::dns_search),
        /// [`dns_option`](PodCreateOptsBuilder::dns_option),
        /// [`no_infra`](PodCreateOptsBuilder::no_infra).
        no_manage_resolv_conf: bool => "no_manage_resolv_conf"
    );

    // TODO: overlay_volumes

    impl_field!(
        pidns: models::Namespace => "pidns"
    );

    impl_vec_field!(
        /// The command used to create this pod. This will be shown in the output of Inspect() on
        /// the pode and may also be used by some tools that wish to recreate the pod (e.g. podman
        /// generate systemd --new).
        pod_create_command => "pod_create_command"
    );

    impl_vec_field!(
        /// User specified Devices to be added to the Pod.
        pod_devices => "pod_devices"
    );

    // TODO: portmappings

    impl_field!(
        /// Container runtime resource constraints.
        resource_limits: models::LinuxResources => "resource_limits"
    );

    impl_vec_field!(
        /// Instructs the pod to share a set of namespaces. Shared namespaces will be joined (by
        /// default) by every container which joins the pod. If not set and NoInfra is false, the
        /// pod will set a default set of namespaces to share. Conflicts with
        /// [`no_infra`](PodCreateOptsBuilder::no_infra) == true.
        shared_namespaces => "shared_namespaces"
    );

    // TODO: throttleReadBpsDevice

    impl_field!(
        userns: models::Namespace => "userns"
    );

    // TODO: volumes

    impl_vec_field!(
        /// Set of containers whose volumes will be added to this pod. The name or ID of the
        /// container must be provided, and may optionally be followed by a : and then one or more
        /// comma-separated options. Valid options are 'ro', 'rw', and 'z'. Options will be used
        /// for all volumes sourced from the container.
        volumes_from => "volumes_from"
    );
}
