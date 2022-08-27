use containers_api::opts::{Equality, Filter, FilterItem};
use containers_api::{
    impl_field, impl_filter_func, impl_map_field, impl_opts_builder, impl_str_field, impl_vec_field,
};

impl_opts_builder!(json =>
    /// Adjust how a network is created.
    NetworkCreate
);

impl NetworkCreateOptsBuilder {
    // #TODO: creted

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

    // #TODO: subnets
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
    /// Matches networks based on the presence of a key label.
    LabelKey(String),
    /// Matches networks based on the presence of a key-value label.
    LabelKeyVal(String, String),
    /// Matches all networks that were create before the given timestamp.
    // TODO: use DateTime
    Until(String),
}

impl Filter for NetworkListFilter {
    fn query_item(&self) -> FilterItem {
        use NetworkListFilter::*;
        match &self {
            Name(name) => FilterItem::new("name", name.clone(), Equality::Equal),
            Id(id) => FilterItem::new("id", id.to_string(), Equality::Equal),
            Driver(driver) => FilterItem::new("driver", driver.clone(), Equality::Equal),
            LabelKey(key) => FilterItem::new("label", key.clone(), Equality::Equal),
            LabelKeyVal(key, val) => {
                FilterItem::new("label", format!("{}={}", key, val), Equality::Equal)
            }
            Until(until) => FilterItem::new("until", until.clone(), Equality::Equal),
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
    // TODO: label!=key=val
    /// Matches networks based on the presence of a key label.
    LabelKey(String),
    /// Matches networks based on the presence of a key-value label.
    LabelKeyVal(String, String),
    /// Matches all networks that were create before the given timestamp.
    // TODO: use DateTime
    Until(String),
}

impl_opts_builder!(url =>
    /// Adjust how unused networks are removed.
    NetworkPrune
);

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
        new.params.insert(
            "container",
            serde_json::Value::String(container.to_string()),
        );
        new
    }
}
