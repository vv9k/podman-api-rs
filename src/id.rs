use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
/// Represents unique identifier given by libpod to an object upon creation.
pub struct Id(String);

impl From<String> for Id {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&str> for Id {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl From<std::borrow::Cow<'_, str>> for Id {
    fn from(s: std::borrow::Cow<'_, str>) -> Self {
        Self(s.to_string())
    }
}

impl AsRef<str> for Id {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
