//! Options used for configuring the behavior of certain API endpoints

mod containers;
mod exec;
mod images;
mod pods;
mod volumes;

pub use containers::*;
pub use exec::*;
pub use images::*;
pub use pods::*;
pub use volumes::*;

pub type EventsConstraint = (String, Vec<String>);

use std::fmt;

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

impl_opts_builder!(url =>
    /// Adjust how filesystem changes inside a container or image are returned.
    Changes
);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Used with [`ChangesOptsBuilder::diff_type`](ChangesOptsBuilder::diff_type).
pub enum DiffType {
    All,
    Container,
    Image,
}

impl AsRef<str> for DiffType {
    fn as_ref(&self) -> &str {
        match self {
            DiffType::All => "all",
            DiffType::Container => "container",
            DiffType::Image => "image",
        }
    }
}

impl fmt::Display for DiffType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl ChangesOptsBuilder {
    impl_url_enum_field!(
        /// Select what you want to match.
        diff_type: DiffType => "diffType"
    );

    impl_url_str_field!(
        /// Specify a second layer which is used to compare against it instead of the parent layer.
        parent => "parent"
    );
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
