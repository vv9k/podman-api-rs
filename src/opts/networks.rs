use crate::{models, Value};

use containers_api::opts::{Filter, FilterItem};
use containers_api::{
    impl_field, impl_filter_func, impl_map_field, impl_opts_builder, impl_str_field, impl_vec_field,
};

impl_opts_builder!(json =>
    /// Adjust how a network is created.
    NetworkCreate
);

impl NetworkCreateOptsBuilder {
    impl_str_field!(
        /// The timestamp when this network was created. Example: `"2019-08-24T14:15:22Z"`
        created => "created"
    );

    impl_field!(
        /// Whether name resolution is active for container on this Network.
        dns_enabled: bool => "dns_enabled"
    );

    impl_str_field!(
        /// Driver for this Network, e.g. bridge, macvlan...
        driver => "driver"
    );

    impl_str_field!(
        /// ID of the Network.
        id => "id"
    );

    impl_field!(
        /// Whether the Network should not have external routes to public or other Networks.
        internal: bool => "internal"
    );

    impl_map_field!(json
        /// Contains options used for the ip assignment.
        ipam_options => "ipam_options"
    );

    impl_field!(
        /// If set to true an ipv6 subnet should be created for this net.
        ipv6_enabled: bool => "ipv6_enabled"
    );

    impl_map_field!(json
        /// A set of key-value labels that have been applied to the Network.
        labels => "labels"
    );

    impl_str_field!(
        /// Name of the Network.
        name => "name"
    );

    impl_str_field!(
        /// The network interface name on the host.
        network_interface => "network_interface"
    );

    impl_map_field!(json
        /// A set of key-value options that have been applied to the Network.
        options => "options"
    );

    impl_vec_field!(
        /// Subnets to use for this network.
        subnets: models::Subnet => "subnets"
    );
}

#[derive(Debug)]
/// Used to filter listed network configurations by one of the variants.
pub enum NetworkListFilter {
    /// Matches network name (accepts regex).
    Name(String),
    /// Matches for full or partial ID.
    Id(crate::Id),
    /// Only bridge is supported.
    Driver(String),
    /// Matches networks with key label.
    LabelKey(String),
    /// Matches networks with key=value label.
    LabelKeyVal(String, String),
    /// Matches networks without key label.
    NoLabelKey(String),
    /// Matches networks without key=value label.
    NoLabelKeyVal(String, String),
    /// Matches all networks that were create before the given timestamp.
    // TODO: use DateTime
    Until(String),
}

impl Filter for NetworkListFilter {
    fn query_item(&self) -> FilterItem {
        use NetworkListFilter::*;
        match &self {
            Name(name) => FilterItem::new("name", name.clone()),
            Id(id) => FilterItem::new("id", id.to_string()),
            Driver(driver) => FilterItem::new("driver", driver.clone()),
            LabelKey(key) => FilterItem::new("label", key.clone()),
            LabelKeyVal(key, val) => FilterItem::new("label", format!("{key}={val}")),
            NoLabelKey(key) => FilterItem::new("label!", key.clone()),
            NoLabelKeyVal(key, val) => FilterItem::new("label!", format!("{key}={val}")),
            Until(until) => FilterItem::new("until", until.clone()),
        }
    }
}

impl_opts_builder!(url =>
    /// Adjust how networks are listed.
    NetworkList
);

impl NetworkListOptsBuilder {
    impl_filter_func!(NetworkListFilter);
}

#[derive(Debug)]
/// Used to filter unused networks to remove.
pub enum NetworkPruneFilter {
    /// Matches networks with key label.
    LabelKey(String),
    /// Matches networks with key=value label.
    LabelKeyVal(String, String),
    /// Matches networks without key label.
    NoLabelKey(String),
    /// Matches networks without key=value label.
    NoLabelKeyVal(String, String),
    /// Matches all networks that were create before the given timestamp.
    Until(String),
}

impl Filter for NetworkPruneFilter {
    fn query_item(&self) -> FilterItem {
        use NetworkPruneFilter::*;
        match &self {
            LabelKey(key) => FilterItem::new("label", key.clone()),
            LabelKeyVal(key, val) => FilterItem::new("label", format!("{key}={val}")),
            NoLabelKey(key) => FilterItem::new("label!", key.clone()),
            NoLabelKeyVal(key, val) => FilterItem::new("label!", format!("{key}={val}")),
            Until(until) => FilterItem::new("until", until.clone()),
        }
    }
}

impl_opts_builder!(url =>
    /// Adjust how unused networks are removed.
    NetworkPrune
);

impl NetworkPruneOptsBuilder {
    impl_filter_func!(NetworkPruneFilter);
}

impl_opts_builder!(json =>
    /// Adjust how a container is disconnected from a network.
    NetworkDisconnect
);

impl NetworkDisconnectOptsBuilder {
    impl_str_field!(
        /// Name or ID of the container to disconnect.
        container => "Container"
    );

    impl_field!(
        /// Force disconnect the container.
        force: bool => "Force"
    );
}

impl_opts_builder!(json =>
    /// Adjust how a container is connected to a network.
    NetworkConnect
);

impl NetworkConnectOptsBuilder {
    impl_vec_field!(
        /// Aliases contains a list of names which the dns server should resolve to this container.
        /// Should only be set when DNSEnabled is true on the Network. If aliases are set but there
        /// is no dns support for this network the network interface implementation should ignore
        /// this and NOT error.
        aliases => "aliases"
    );

    impl_str_field!(
        container => "container"
    );

    impl_str_field!(
        /// Interface name for this container. Required in the backend. Optional in the frontend.
        /// Will be filled with ethX (where X is a integer) when empty.
        interface_name => "interface_name"
    );

    /// Static IPs for the container.
    pub fn static_ips<I>(mut self, ips: impl IntoIterator<Item = I>) -> Self
    where
        I: IntoIterator<Item = u8>,
    {
        let ips: Vec<Vec<_>> = ips.into_iter().map(|a| a.into_iter().collect()).collect();
        self.params.insert("static_ips", serde_json::json!(ips));
        self
    }

    /// Static mac for the container.
    pub fn static_mac(mut self, mac: impl IntoIterator<Item = u8>) -> Self {
        let mac: Vec<_> = mac.into_iter().collect();
        self.params.insert("static_mac", serde_json::json!(mac));
        self
    }
}

impl NetworkConnectOpts {
    pub(crate) fn for_container(&self, container: &crate::Id) -> Self {
        let mut new = self.clone();
        new.params
            .insert("container", Value::String(container.to_string()));
        new
    }
}
