pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BindSlackChannelRequest {
    /// Slack installation id
    #[serde(rename = "installationId")]
    #[serde(default)]
    pub installation_id: String,
    /// Slack channel id
    #[serde(rename = "channelId")]
    #[serde(default)]
    pub channel_id: String,
}

impl BindSlackChannelRequest {
    pub fn builder() -> BindSlackChannelRequestBuilder {
        <BindSlackChannelRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BindSlackChannelRequestBuilder {
    installation_id: Option<String>,
    channel_id: Option<String>,
}

impl BindSlackChannelRequestBuilder {
    pub fn installation_id(mut self, value: impl Into<String>) -> Self {
        self.installation_id = Some(value.into());
        self
    }

    pub fn channel_id(mut self, value: impl Into<String>) -> Self {
        self.channel_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BindSlackChannelRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`installation_id`](BindSlackChannelRequestBuilder::installation_id)
    /// - [`channel_id`](BindSlackChannelRequestBuilder::channel_id)
    pub fn build(self) -> Result<BindSlackChannelRequest, BuildError> {
        Ok(BindSlackChannelRequest {
            installation_id: self.installation_id.ok_or_else(|| BuildError::missing_field("installation_id"))?,
            channel_id: self.channel_id.ok_or_else(|| BuildError::missing_field("channel_id"))?,
        })
    }
}

