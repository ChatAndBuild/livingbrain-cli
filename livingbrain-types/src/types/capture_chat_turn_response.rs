pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CaptureChatTurnResponse {
    /// Whether the turn was deemed worth capturing
    #[serde(default)]
    pub worthy: bool,
    /// Classifier reasoning (or drop reason)
    #[serde(default)]
    pub reason: String,
    /// Created ingest source id when worthy
    #[serde(rename = "sourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_id: Option<String>,
}

impl CaptureChatTurnResponse {
    pub fn builder() -> CaptureChatTurnResponseBuilder {
        <CaptureChatTurnResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CaptureChatTurnResponseBuilder {
    worthy: Option<bool>,
    reason: Option<String>,
    source_id: Option<String>,
}

impl CaptureChatTurnResponseBuilder {
    pub fn worthy(mut self, value: bool) -> Self {
        self.worthy = Some(value);
        self
    }

    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
        self
    }

    pub fn source_id(mut self, value: impl Into<String>) -> Self {
        self.source_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CaptureChatTurnResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`worthy`](CaptureChatTurnResponseBuilder::worthy)
    /// - [`reason`](CaptureChatTurnResponseBuilder::reason)
    pub fn build(self) -> Result<CaptureChatTurnResponse, BuildError> {
        Ok(CaptureChatTurnResponse {
            worthy: self.worthy.ok_or_else(|| BuildError::missing_field("worthy"))?,
            reason: self.reason.ok_or_else(|| BuildError::missing_field("reason"))?,
            source_id: self.source_id,
        })
    }
}
