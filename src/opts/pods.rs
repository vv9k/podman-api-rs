use crate::{api::Filter, models};

impl_opts_builder!(url =>
    /// Adjust the list of returned pods with this options.
    PodList
);

#[derive(Debug)]
/// Used to filter listed pods by one of the variants.
pub enum PodListFilter {
    /// A pods's ID
    Id(crate::Id),
    /// Pod key label.
    LabelKey(String),
    /// Pod key-value label.
    LabelKeyVal(String, String),
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
    fn query_key_val(&self) -> (&'static str, String) {
        use PodListFilter::*;
        match &self {
            Id(id) => ("id", id.to_string()),
            LabelKey(key) => ("label", key.clone()),
            LabelKeyVal(key, val) => ("label", format!("{}={}", key, val)),
            Name(name) => ("name", name.clone()),
            Until(until) => ("until", until.clone()),
            Network(net) => ("network", net.clone()),
            Status(status) => ("status", status.as_ref().to_string()),
            ContainerName(name) => ("ctr-names", name.clone()),
            ContainerId(id) => ("ctr-ids", id.to_string()),
            ContainerStatus(status) => ("ctr-status", status.as_ref().to_string()),
            ContainerNumber(n) => ("ctr-number", n.to_string()),
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
