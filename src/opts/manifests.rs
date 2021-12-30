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

impl_opts_builder!(json =>
    /// Adjust how an image is added to a manifest list.
    ManifestImageAdd
);

impl ManifestImageAddOptsBuilder {
    impl_field!(
        all: bool => "all"
    );

    impl_vec_field!(
        annotation => "annotation"
    );

    impl_str_field!(
        arch => "arch"
    );

    impl_vec_field!(
        features => "features"
    );

    impl_vec_field!(
        images => "images"
    );

    impl_str_field!(
        os => "os"
    );

    impl_str_field!(
        os_version => "os_version"
    );

    impl_str_field!(
        variant => "variant"
    );
}
