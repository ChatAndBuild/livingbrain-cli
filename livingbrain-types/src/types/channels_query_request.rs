pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for channels
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ChannelsQueryRequest {
    /// Slack installation id returned by the status endpoint
    #[serde(rename = "installationId")]
    #[serde(default)]
    pub installation_id: String,
}

impl ChannelsQueryRequest {
    pub fn builder() -> ChannelsQueryRequestBuilder {
        <ChannelsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ChannelsQueryRequestBuilder {
    installation_id: Option<String>,
}

impl ChannelsQueryRequestBuilder {
    pub fn installation_id(mut self, value: impl Into<String>) -> Self {
        self.installation_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ChannelsQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`installation_id`](ChannelsQueryRequestBuilder::installation_id)
    pub fn build(self) -> Result<ChannelsQueryRequest, BuildError> {
        Ok(ChannelsQueryRequest {
            installation_id: self.installation_id.ok_or_else(|| BuildError::missing_field("installation_id"))?,
        })
    }
}

