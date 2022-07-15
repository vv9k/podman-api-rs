use containers_api::opts::Filter;

impl_opts_builder!(url =>
    /// Adjust the list of returned volumes with this options.
    VolumeList
);

#[derive(Debug)]
/// Used to filter listed volumes by one of the variants.
pub enum VolumeListFilter {
    /// Match a volume based on a driver.
    Driver(String),
    /// Volumes with a label.
    LabelKey(String),
    /// Volumes with a key-value label.
    LabelKeyVal(String, String),
    /// Volume with name
    Name(String),
    /// Volumes with storage driver opts
    Opt(String),
    /// The <timestamp> can be Unix timestamps, date formatted timestamps, or Go
    /// duration strings (e.g. 10m, 1h30m) computed relative to the daemon machine’s time.
    Until(String),
}

impl Filter for VolumeListFilter {
    fn query_key_val(&self) -> (&'static str, String) {
        use VolumeListFilter::*;
        match &self {
            Driver(driver) => ("driver", driver.clone()),
            LabelKey(key) => ("label", key.clone()),
            LabelKeyVal(key, val) => ("label", format!("{}={}", key, val)),
            Name(name) => ("name", name.clone()),
            Opt(opt) => ("opt", opt.clone()),
            Until(t) => ("until", t.clone()),
        }
    }
}

impl VolumeListOptsBuilder {
    impl_filter_func!(VolumeListFilter);
}

impl_opts_builder!(json =>
    /// Adjust how a volume is created
    VolumeCreate
);

impl VolumeCreateOptsBuilder {
    impl_str_field!(
        /// Storage driver to use.
        driver => "Driver"
    );

    impl_map_field!(json
        /// Key/value user-defined metadatada.
        labels => "Labels"
    );

    impl_str_field!(
        /// Name for the volume.
        name => "Name"
    );

    impl_map_field!(json
        /// Mapping of storage driver options and values
        options => "Options"
    );
}

impl_opts_builder!(url =>
    /// Adjust how volumes will be pruned.
    VolumePrune
);

#[derive(Debug)]
/// Used to filter pruned volumes.
pub enum VolumePruneFilter {
    /// Volumes with a label.
    LabelKey(String),
    /// Volumes with a key-value label.
    LabelKeyVal(String, String),
    /// The <timestamp> can be Unix timestamps, date formatted timestamps, or Go
    /// duration strings (e.g. 10m, 1h30m) computed relative to the daemon machine’s time.
    Until(String),
}

impl Filter for VolumePruneFilter {
    fn query_key_val(&self) -> (&'static str, String) {
        use VolumePruneFilter::*;
        match &self {
            LabelKey(key) => ("label", key.clone()),
            LabelKeyVal(key, val) => ("label", format!("{}={}", key, val)),
            Until(t) => ("until", t.clone()),
        }
    }
}

impl VolumePruneOptsBuilder {
    impl_filter_func!(VolumePruneFilter);
}
