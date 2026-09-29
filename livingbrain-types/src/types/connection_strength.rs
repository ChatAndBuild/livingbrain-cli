pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// How strongly the two pages are related.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConnectionStrength {
    Weak,
    Medium,
    Strong,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ConnectionStrength {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Weak => serializer.serialize_str("weak"),
            Self::Medium => serializer.serialize_str("medium"),
            Self::Strong => serializer.serialize_str("strong"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ConnectionStrength {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "weak" => Ok(Self::Weak),
            "medium" => Ok(Self::Medium),
            "strong" => Ok(Self::Strong),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ConnectionStrength {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Weak => write!(f, "weak"),
            Self::Medium => write!(f, "medium"),
            Self::Strong => write!(f, "strong"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
