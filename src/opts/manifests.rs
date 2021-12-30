impl_opts_required_builder!(url =>
    /// Adjust how a manifest list is created.
    ManifestCreate,
    ///
    /// Parameters:
    /// * name - Manifest list name.
    name => "name"
);

impl ManifestCreateOptsBuilder {
    impl_url_bool_field!(
        /// Add all contents if given list.
        all => "all"
    );

    impl_url_str_field!(
        /// Name of the image.
        image => "image"
    );
}
