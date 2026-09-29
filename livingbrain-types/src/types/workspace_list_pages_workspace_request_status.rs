pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListPagesWorkspaceRequestStatus {
    Active,
    Deprecated,
    Stale,
    Contested,
    Archived,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ListPagesWorkspaceRequestStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Active => serializer.serialize_str("active"),
            Self::Deprecated => serializer.serialize_str("deprecated"),
            Self::Stale => serializer.serialize_str("stale"),
            Self::Contested => serializer.serialize_str("contested"),
            Self::Archived => serializer.serialize_str("archived"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ListPagesWorkspaceRequestStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "active" => Ok(Self::Active),
            "deprecated" => Ok(Self::Deprecated),
            "stale" => Ok(Self::Stale),
            "contested" => Ok(Self::Contested),
            "archived" => Ok(Self::Archived),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ListPagesWorkspaceRequestStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Active => write!(f, "active"),
            Self::Deprecated => write!(f, "deprecated"),
            Self::Stale => write!(f, "stale"),
            Self::Contested => write!(f, "contested"),
            Self::Archived => write!(f, "archived"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
