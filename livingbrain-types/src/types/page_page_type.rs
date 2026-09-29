pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// What kind of knowledge this page holds.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PagePageType {
    Source,
    Topic,
    Entity,
    Project,
    Decision,
    Brief,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PagePageType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Source => serializer.serialize_str("source"),
            Self::Topic => serializer.serialize_str("topic"),
            Self::Entity => serializer.serialize_str("entity"),
            Self::Project => serializer.serialize_str("project"),
            Self::Decision => serializer.serialize_str("decision"),
            Self::Brief => serializer.serialize_str("brief"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PagePageType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "source" => Ok(Self::Source),
            "topic" => Ok(Self::Topic),
            "entity" => Ok(Self::Entity),
            "project" => Ok(Self::Project),
            "decision" => Ok(Self::Decision),
            "brief" => Ok(Self::Brief),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PagePageType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Source => write!(f, "source"),
            Self::Topic => write!(f, "topic"),
            Self::Entity => write!(f, "entity"),
            Self::Project => write!(f, "project"),
            Self::Decision => write!(f, "decision"),
            Self::Brief => write!(f, "brief"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
