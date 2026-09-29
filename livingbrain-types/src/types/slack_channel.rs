pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SlackChannel {
    /// Slack channel ID.
    #[serde(default)]
    pub id: String,
    /// Slack channel name.
    #[serde(default)]
    pub name: String,
    /// Whether the channel is private.
    #[serde(rename = "isPrivate")]
    #[serde(default)]
    pub is_private: bool,
    /// Whether the Living Brain bot is already a member of the channel.
    #[serde(rename = "isMember")]
    #[serde(default)]
    pub is_member: bool,
}

impl SlackChannel {
    pub fn builder() -> SlackChannelBuilder {
        <SlackChannelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SlackChannelBuilder {
    id: Option<String>,
    name: Option<String>,
    is_private: Option<bool>,
    is_member: Option<bool>,
}

impl SlackChannelBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn is_private(mut self, value: bool) -> Self {
        self.is_private = Some(value);
        self
    }

    pub fn is_member(mut self, value: bool) -> Self {
        self.is_member = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SlackChannel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](SlackChannelBuilder::id)
    /// - [`name`](SlackChannelBuilder::name)
    /// - [`is_private`](SlackChannelBuilder::is_private)
    /// - [`is_member`](SlackChannelBuilder::is_member)
    pub fn build(self) -> Result<SlackChannel, BuildError> {
        Ok(SlackChannel {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            is_private: self.is_private.ok_or_else(|| BuildError::missing_field("is_private"))?,
            is_member: self.is_member.ok_or_else(|| BuildError::missing_field("is_member"))?,
        })
    }
}
