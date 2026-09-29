pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SlackBinding {
    /// Unique identifier for the channel binding.
    #[serde(default)]
    pub id: String,
    /// ID of the brain this channel is bound to.
    #[serde(rename = "brainId")]
    #[serde(default)]
    pub brain_id: String,
    /// ID of the Slack installation the channel belongs to.
    #[serde(rename = "installationId")]
    #[serde(default)]
    pub installation_id: String,
    /// Slack workspace (team) ID.
    #[serde(rename = "teamId")]
    #[serde(default)]
    pub team_id: String,
    /// Slack channel ID.
    #[serde(rename = "channelId")]
    #[serde(default)]
    pub channel_id: String,
    /// Slack channel name, when available.
    #[serde(rename = "channelName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_name: Option<String>,
    /// Whether the binding is currently delivering notifications.
    pub status: SlackBindingStatus,
    /// Whether the channel receives the daily brief.
    #[serde(rename = "notifyBriefs")]
    #[serde(default)]
    pub notify_briefs: bool,
    /// Whether the channel is notified when strong new connections are discovered.
    #[serde(rename = "notifyConnections")]
    #[serde(default)]
    pub notify_connections: bool,
    /// Whether the channel is notified when pages are updated with new knowledge.
    #[serde(rename = "notifyChanges")]
    #[serde(default)]
    pub notify_changes: bool,
    /// ISO 8601 timestamp of when this record was created.
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
}

impl SlackBinding {
    pub fn builder() -> SlackBindingBuilder {
        <SlackBindingBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SlackBindingBuilder {
    id: Option<String>,
    brain_id: Option<String>,
    installation_id: Option<String>,
    team_id: Option<String>,
    channel_id: Option<String>,
    channel_name: Option<String>,
    status: Option<SlackBindingStatus>,
    notify_briefs: Option<bool>,
    notify_connections: Option<bool>,
    notify_changes: Option<bool>,
    created_at: Option<DateTime<FixedOffset>>,
}

impl SlackBindingBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn brain_id(mut self, value: impl Into<String>) -> Self {
        self.brain_id = Some(value.into());
        self
    }

    pub fn installation_id(mut self, value: impl Into<String>) -> Self {
        self.installation_id = Some(value.into());
        self
    }

    pub fn team_id(mut self, value: impl Into<String>) -> Self {
        self.team_id = Some(value.into());
        self
    }

    pub fn channel_id(mut self, value: impl Into<String>) -> Self {
        self.channel_id = Some(value.into());
        self
    }

    pub fn channel_name(mut self, value: impl Into<String>) -> Self {
        self.channel_name = Some(value.into());
        self
    }

    pub fn status(mut self, value: SlackBindingStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn notify_briefs(mut self, value: bool) -> Self {
        self.notify_briefs = Some(value);
        self
    }

    pub fn notify_connections(mut self, value: bool) -> Self {
        self.notify_connections = Some(value);
        self
    }

    pub fn notify_changes(mut self, value: bool) -> Self {
        self.notify_changes = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SlackBinding`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](SlackBindingBuilder::id)
    /// - [`brain_id`](SlackBindingBuilder::brain_id)
    /// - [`installation_id`](SlackBindingBuilder::installation_id)
    /// - [`team_id`](SlackBindingBuilder::team_id)
    /// - [`channel_id`](SlackBindingBuilder::channel_id)
    /// - [`status`](SlackBindingBuilder::status)
    /// - [`notify_briefs`](SlackBindingBuilder::notify_briefs)
    /// - [`notify_connections`](SlackBindingBuilder::notify_connections)
    /// - [`notify_changes`](SlackBindingBuilder::notify_changes)
    /// - [`created_at`](SlackBindingBuilder::created_at)
    pub fn build(self) -> Result<SlackBinding, BuildError> {
        Ok(SlackBinding {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            brain_id: self.brain_id.ok_or_else(|| BuildError::missing_field("brain_id"))?,
            installation_id: self.installation_id.ok_or_else(|| BuildError::missing_field("installation_id"))?,
            team_id: self.team_id.ok_or_else(|| BuildError::missing_field("team_id"))?,
            channel_id: self.channel_id.ok_or_else(|| BuildError::missing_field("channel_id"))?,
            channel_name: self.channel_name,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            notify_briefs: self.notify_briefs.ok_or_else(|| BuildError::missing_field("notify_briefs"))?,
            notify_connections: self.notify_connections.ok_or_else(|| BuildError::missing_field("notify_connections"))?,
            notify_changes: self.notify_changes.ok_or_else(|| BuildError::missing_field("notify_changes"))?,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
        })
    }
}
