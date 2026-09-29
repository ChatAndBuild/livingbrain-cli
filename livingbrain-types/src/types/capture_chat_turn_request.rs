pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CaptureChatTurnRequest {
    /// Raw chat turn text
    #[serde(default)]
    pub text: String,
    /// Who produced the turn
    pub sender: CaptureChatTurnRequestSender,
    /// Agent display name for classifier context
    #[serde(rename = "agentName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_name: Option<String>,
    /// Idempotency key for the turn (e.g. chat:<id>:<ts>)
    #[serde(rename = "originRef")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_ref: Option<String>,
    /// Opaque provenance
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,
}

impl CaptureChatTurnRequest {
    pub fn builder() -> CaptureChatTurnRequestBuilder {
        <CaptureChatTurnRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CaptureChatTurnRequestBuilder {
    text: Option<String>,
    sender: Option<CaptureChatTurnRequestSender>,
    agent_name: Option<String>,
    origin_ref: Option<String>,
    source: Option<Source>,
}

impl CaptureChatTurnRequestBuilder {
    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn sender(mut self, value: CaptureChatTurnRequestSender) -> Self {
        self.sender = Some(value);
        self
    }

    pub fn agent_name(mut self, value: impl Into<String>) -> Self {
        self.agent_name = Some(value.into());
        self
    }

    pub fn origin_ref(mut self, value: impl Into<String>) -> Self {
        self.origin_ref = Some(value.into());
        self
    }

    pub fn source(mut self, value: Source) -> Self {
        self.source = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CaptureChatTurnRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text`](CaptureChatTurnRequestBuilder::text)
    /// - [`sender`](CaptureChatTurnRequestBuilder::sender)
    pub fn build(self) -> Result<CaptureChatTurnRequest, BuildError> {
        Ok(CaptureChatTurnRequest {
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
            sender: self.sender.ok_or_else(|| BuildError::missing_field("sender"))?,
            agent_name: self.agent_name,
            origin_ref: self.origin_ref,
            source: self.source,
        })
    }
}

