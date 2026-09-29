pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// What happened to the page.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ChangeEventAction {
    Created,
    Updated,
    Renamed,
    Merged,
    Split,
    Archived,
    Restored,
    Recovered,
    Routed,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ChangeEventAction {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Created => serializer.serialize_str("created"),
            Self::Updated => serializer.serialize_str("updated"),
            Self::Renamed => serializer.serialize_str("renamed"),
            Self::Merged => serializer.serialize_str("merged"),
            Self::Split => serializer.serialize_str("split"),
            Self::Archived => serializer.serialize_str("archived"),
            Self::Restored => serializer.serialize_str("restored"),
            Self::Recovered => serializer.serialize_str("recovered"),
            Self::Routed => serializer.serialize_str("routed"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ChangeEventAction {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "created" => Ok(Self::Created),
            "updated" => Ok(Self::Updated),
            "renamed" => Ok(Self::Renamed),
            "merged" => Ok(Self::Merged),
            "split" => Ok(Self::Split),
            "archived" => Ok(Self::Archived),
            "restored" => Ok(Self::Restored),
            "recovered" => Ok(Self::Recovered),
            "routed" => Ok(Self::Routed),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ChangeEventAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Created => write!(f, "created"),
            Self::Updated => write!(f, "updated"),
            Self::Renamed => write!(f, "renamed"),
            Self::Merged => write!(f, "merged"),
            Self::Split => write!(f, "split"),
            Self::Archived => write!(f, "archived"),
            Self::Restored => write!(f, "restored"),
            Self::Recovered => write!(f, "recovered"),
            Self::Routed => write!(f, "routed"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
