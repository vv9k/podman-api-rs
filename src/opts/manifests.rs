use containers_api::{
    impl_field, impl_opts_builder, impl_opts_required_builder, impl_str_field, impl_url_bool_field,
    impl_url_vec_field, impl_vec_field,
};

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

    impl_url_bool_field!(
        /// Modify an existing list if one with the desired name already exists.
        amend => "amend"
    );

    impl_url_vec_field!(
        /// One or more names of an image or a manifest list. Repeat parameter as needed.
        images => "images"
    );
}

impl_opts_builder!(json =>
    /// Adjust how an image is added to a manifest list.
    ManifestImageAdd
);

impl ManifestImageAddOptsBuilder {
    impl_field!(
        /// True when operating on a list to include all images.
        all: bool => "all"
    );

    impl_vec_field!(
        /// Annotation to add to manifest list.
        annotation => "annotation"
    );

    impl_str_field!(
        /// Overrides the architecture for the image.
        arch => "arch"
    );

    impl_vec_field!(
        /// Feature list for the image.
        features => "features"
    );

    impl_vec_field!(
        /// Optional list of images to add to manifest list.
        images => "images"
    );

    impl_str_field!(
        /// Overrides the operating system for the image.
        os => "os"
    );

    impl_str_field!(
        /// OS features for the image.
        os_features => "os_features"
    );

    impl_str_field!(
        /// Overrides the operating system for the image.
        os_version => "os_version"
    );

    impl_str_field!(
        /// Variant for the image.
        variant => "variant"
    );
}

impl_opts_required_builder!(url =>
    /// Adjust how a manifest list is pushed to a registry.
    ManifestPush,
    /// The destination for the manifest
    destination => "destination"
);

impl ManifestPushOptsBuilder {
    impl_url_bool_field!(
        /// Push all images.
        all => "all"
    );
}
