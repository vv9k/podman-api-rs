//! Options used for configuring the behavior of certain API endpoints

pub type EventsConstraint = (String, Vec<String>);

impl_opts_builder!(
    url =>
    /// Used to filter events returned by [Podman::events](crate::Podman::events).
    Events
);

impl EventsOptsBuilder {
    impl_url_str_field!(
        /// Start streaming events from this time
        since: S => "since"
    );

    impl_url_str_field!(
        /// Stop streaming events later than this
        until: U => "until"
    );

    impl_url_bool_field!(
        /// when false, do not follow events
        stream => "stream"
    );

    /// A list of constraints for events
    pub fn filters<F>(mut self, filters: F) -> Self
    where
        F: IntoIterator<Item = EventsConstraint>,
    {
        let filters: std::collections::HashMap<_, _> = filters.into_iter().collect();
        self.params.insert(
            "filters",
            serde_json::to_string(&filters).unwrap_or_default(),
        );
        self
    }
}
