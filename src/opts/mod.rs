//! Options used for configuring the behavior of certain API endpoints

mod containers;
mod exec;
mod images;
mod pods;

pub use containers::*;
pub use exec::*;
pub use images::*;
pub use pods::*;

pub type EventsConstraint = (String, Vec<String>);

impl_opts_builder!(
    url =>
    /// Used to filter events returned by [Podman::events](crate::Podman::events).
    Events
);

impl EventsOptsBuilder {
    impl_url_str_field!(
        /// Start streaming events from this time
        since => "since"
    );

    impl_url_str_field!(
        /// Stop streaming events later than this
        until => "until"
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

//####################################################################################################
//
// Secrets
//
//####################################################################################################

#[derive(serde::Serialize, Debug, Clone)]
pub struct SecretCreateOpts {
    name: String,
    driver: Option<String>,
}

impl SecretCreateOpts {
    /// Returns a new instance of a buildetr for `SecretCreateOpts`.
    ///
    /// Parameters:
    /// * name - User-defined name of the secret.
    ///
    pub fn builder(name: impl Into<String>) -> SecretCreateOptsBuilder {
        SecretCreateOptsBuilder {
            name: name.into(),
            driver: None,
        }
    }

    pub fn serialize(&self) -> Option<String> {
        if self.name.is_empty() {
            return None;
        }

        let mut params = vec![("name", self.name.clone())];

        if let Some(driver) = &self.driver {
            params.push(("driver", driver.clone()));
        }

        Some(crate::util::url::encoded_pairs(params))
    }
}

#[derive(Debug, Clone)]
pub struct SecretCreateOptsBuilder {
    name: String,
    driver: Option<String>,
}

impl SecretCreateOptsBuilder {
    /// Secret driver. Default is `file`.
    pub fn driver(mut self, driver: impl Into<String>) -> Self {
        self.driver = Some(driver.into());
        self
    }

    /// Finish building `SecretCreateOpts`.
    pub fn build(self) -> SecretCreateOpts {
        SecretCreateOpts {
            name: self.name,
            driver: self.driver,
        }
    }
}
