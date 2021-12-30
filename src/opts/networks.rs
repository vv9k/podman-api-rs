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
