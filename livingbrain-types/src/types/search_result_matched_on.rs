pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Why the page matched: semantic similarity, keyword (title/summary/body words), or both. Keyword matches are returned even when similarity is below minSimilarity.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SearchResultMatchedOn {
    Semantic,
    Keyword,
    Both,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SearchResultMatchedOn {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Semantic => serializer.serialize_str("semantic"),
            Self::Keyword => serializer.serialize_str("keyword"),
            Self::Both => serializer.serialize_str("both"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SearchResultMatchedOn {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "semantic" => Ok(Self::Semantic),
            "keyword" => Ok(Self::Keyword),
            "both" => Ok(Self::Both),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SearchResultMatchedOn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Semantic => write!(f, "semantic"),
            Self::Keyword => write!(f, "keyword"),
            Self::Both => write!(f, "both"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
