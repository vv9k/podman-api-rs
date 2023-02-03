use containers_api::opts::{Filter, FilterItem};
use containers_api::{impl_filter_func, impl_map_field, impl_opts_builder, impl_str_field};

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
    /// Volumes with a key=value label.
    LabelKeyVal(String, String),
    /// Volumes without a label.
    NoLabelKey(String),
    /// Volumes without a key=value label.
    NoLabelKeyVal(String, String),
    /// Volume with name
    Name(String),
    /// Volumes with storage driver opts
    Opt(String),
    /// The <timestamp> can be Unix timestamps, date formatted timestamps, or Go
    /// duration strings (e.g. 10m, 1h30m) computed relative to the daemon machine’s time.
    Until(String),
}

impl Filter for VolumeListFilter {
    fn query_item(&self) -> FilterItem {
        use VolumeListFilter::*;
        match &self {
            Driver(driver) => FilterItem::new("driver", driver.clone()),
            LabelKey(key) => FilterItem::new("label", key.clone()),
            LabelKeyVal(key, val) => FilterItem::new("label", format!("{key}={val}")),
            NoLabelKey(key) => FilterItem::new("label!", key.clone()),
            NoLabelKeyVal(key, val) => FilterItem::new("label!", format!("{key}={val}")),
            Name(name) => FilterItem::new("name", name.clone()),
            Opt(opt) => FilterItem::new("opt", opt.clone()),
            Until(t) => FilterItem::new("until", t.clone()),
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
    /// Volumes with a key=value label.
    LabelKeyVal(String, String),
    /// Volumes without a label.
    NoLabelKey(String),
    /// Volumes without a key=value label.
    NoLabelKeyVal(String, String),
    /// The <timestamp> can be Unix timestamps, date formatted timestamps, or Go
    /// duration strings (e.g. 10m, 1h30m) computed relative to the daemon machine’s time.
    Until(String),
}

impl Filter for VolumePruneFilter {
    fn query_item(&self) -> FilterItem {
        use VolumePruneFilter::*;
        match &self {
            LabelKey(key) => FilterItem::new("label", key.clone()),
            LabelKeyVal(key, val) => FilterItem::new("label", format!("{key}={val}")),
            NoLabelKey(key) => FilterItem::new("label", key.clone()),
            NoLabelKeyVal(key, val) => FilterItem::new("label", format!("{key}={val}")),
            Until(t) => FilterItem::new("until", t.clone()),
        }
    }
}

impl VolumePruneOptsBuilder {
    impl_filter_func!(VolumePruneFilter);
}
