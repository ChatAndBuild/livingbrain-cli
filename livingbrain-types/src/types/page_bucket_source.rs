pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Whether the bucket was set automatically or by the user
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PageBucketSource {
    Auto,
    User,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PageBucketSource {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Auto => serializer.serialize_str("auto"),
            Self::User => serializer.serialize_str("user"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PageBucketSource {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "auto" => Ok(Self::Auto),
            "user" => Ok(Self::User),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PageBucketSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Auto => write!(f, "auto"),
            Self::User => write!(f, "user"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
