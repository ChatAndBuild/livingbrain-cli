pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UpdateWebhookRequestEventsItem {
    BrainPageUpdated,
    BrainChangeNew,
    BrainConnectionNew,
    BrainBriefReady,
    BrainImportProgress,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for UpdateWebhookRequestEventsItem {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::BrainPageUpdated => serializer.serialize_str("brain.page.updated"),
            Self::BrainChangeNew => serializer.serialize_str("brain.change.new"),
            Self::BrainConnectionNew => serializer.serialize_str("brain.connection.new"),
            Self::BrainBriefReady => serializer.serialize_str("brain.brief.ready"),
            Self::BrainImportProgress => serializer.serialize_str("brain.import.progress"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for UpdateWebhookRequestEventsItem {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "brain.page.updated" => Ok(Self::BrainPageUpdated),
            "brain.change.new" => Ok(Self::BrainChangeNew),
            "brain.connection.new" => Ok(Self::BrainConnectionNew),
            "brain.brief.ready" => Ok(Self::BrainBriefReady),
            "brain.import.progress" => Ok(Self::BrainImportProgress),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for UpdateWebhookRequestEventsItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BrainPageUpdated => write!(f, "brain.page.updated"),
            Self::BrainChangeNew => write!(f, "brain.change.new"),
            Self::BrainConnectionNew => write!(f, "brain.connection.new"),
            Self::BrainBriefReady => write!(f, "brain.brief.ready"),
            Self::BrainImportProgress => write!(f, "brain.import.progress"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
