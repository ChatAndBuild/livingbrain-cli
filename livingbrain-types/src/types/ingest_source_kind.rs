pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Kind of raw input
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IngestSourceKind {
    Note,
    File,
    Url,
    Transcript,
    ChatTurn,
    Integration,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for IngestSourceKind {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Note => serializer.serialize_str("note"),
            Self::File => serializer.serialize_str("file"),
            Self::Url => serializer.serialize_str("url"),
            Self::Transcript => serializer.serialize_str("transcript"),
            Self::ChatTurn => serializer.serialize_str("chat_turn"),
            Self::Integration => serializer.serialize_str("integration"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for IngestSourceKind {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "note" => Ok(Self::Note),
            "file" => Ok(Self::File),
            "url" => Ok(Self::Url),
            "transcript" => Ok(Self::Transcript),
            "chat_turn" => Ok(Self::ChatTurn),
            "integration" => Ok(Self::Integration),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for IngestSourceKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Note => write!(f, "note"),
            Self::File => write!(f, "file"),
            Self::Url => write!(f, "url"),
            Self::Transcript => write!(f, "transcript"),
            Self::ChatTurn => write!(f, "chat_turn"),
            Self::Integration => write!(f, "integration"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
