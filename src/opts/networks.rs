use crate::api::Filter;

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
    fn query_key_val(&self) -> (&'static str, String) {
        use NetworkListFilter::*;
        match &self {
            Name(name) => ("name", name.clone()),
            Id(id) => ("id", id.to_string()),
            Driver(driver) => ("driver", driver.clone()),
            LabelKey(key) => ("label", key.clone()),
            LabelKeyVal(key, val) => ("label", format!("{}={}", key, val)),
            Until(until) => ("until", until.clone()),
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
