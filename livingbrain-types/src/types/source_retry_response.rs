pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SourceRetryResponse {
    /// Whether the compile job was enqueued
    #[serde(default)]
    pub enqueued: bool,
    /// Ingest source id
    #[serde(default)]
    pub id: String,
}

impl SourceRetryResponse {
    pub fn builder() -> SourceRetryResponseBuilder {
        <SourceRetryResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SourceRetryResponseBuilder {
    enqueued: Option<bool>,
    id: Option<String>,
}

impl SourceRetryResponseBuilder {
    pub fn enqueued(mut self, value: bool) -> Self {
        self.enqueued = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SourceRetryResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`enqueued`](SourceRetryResponseBuilder::enqueued)
    /// - [`id`](SourceRetryResponseBuilder::id)
    pub fn build(self) -> Result<SourceRetryResponse, BuildError> {
        Ok(SourceRetryResponse {
            enqueued: self.enqueued.ok_or_else(|| BuildError::missing_field("enqueued"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
