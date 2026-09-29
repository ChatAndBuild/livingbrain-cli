pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Top-level grouping the page belongs to.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PageBucket {
    Inbox,
    Notes,
    Ideas,
    Projects,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PageBucket {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Inbox => serializer.serialize_str("inbox"),
            Self::Notes => serializer.serialize_str("notes"),
            Self::Ideas => serializer.serialize_str("ideas"),
            Self::Projects => serializer.serialize_str("projects"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PageBucket {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "inbox" => Ok(Self::Inbox),
            "notes" => Ok(Self::Notes),
            "ideas" => Ok(Self::Ideas),
            "projects" => Ok(Self::Projects),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PageBucket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Inbox => write!(f, "inbox"),
            Self::Notes => write!(f, "notes"),
            Self::Ideas => write!(f, "ideas"),
            Self::Projects => write!(f, "projects"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
