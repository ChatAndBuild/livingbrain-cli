pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The time range these results cover.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ScoreHistoryRange {
    SevenD,
    ThirtyD,
    NinetyD,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ScoreHistoryRange {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::SevenD => serializer.serialize_str("7d"),
            Self::ThirtyD => serializer.serialize_str("30d"),
            Self::NinetyD => serializer.serialize_str("90d"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ScoreHistoryRange {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "7d" => Ok(Self::SevenD),
            "30d" => Ok(Self::ThirtyD),
            "90d" => Ok(Self::NinetyD),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ScoreHistoryRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SevenD => write!(f, "7d"),
            Self::ThirtyD => write!(f, "30d"),
            Self::NinetyD => write!(f, "90d"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
