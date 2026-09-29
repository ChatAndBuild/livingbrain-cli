pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SlackInstallation {
    /// Unique identifier for the Slack installation.
    #[serde(default)]
    pub id: String,
    /// Slack workspace (team) ID.
    #[serde(rename = "teamId")]
    #[serde(default)]
    pub team_id: String,
    /// Slack workspace name, when Slack provides it.
    #[serde(rename = "teamName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_name: Option<String>,
    /// Slack Enterprise Grid organization ID, for Enterprise Grid installs.
    #[serde(rename = "enterpriseId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enterprise_id: Option<String>,
    /// Slack Enterprise Grid organization name, for Enterprise Grid installs.
    #[serde(rename = "enterpriseName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enterprise_name: Option<String>,
    /// Installation state. `revoked` means the app was removed from the Slack workspace; `failed` means the install did not complete.
    pub status: SlackInstallationStatus,
    /// ISO 8601 timestamp of when the Slack workspace was connected.
    #[serde(rename = "connectedAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub connected_at: DateTime<FixedOffset>,
}

impl SlackInstallation {
    pub fn builder() -> SlackInstallationBuilder {
        <SlackInstallationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SlackInstallationBuilder {
    id: Option<String>,
    team_id: Option<String>,
    team_name: Option<String>,
    enterprise_id: Option<String>,
    enterprise_name: Option<String>,
    status: Option<SlackInstallationStatus>,
    connected_at: Option<DateTime<FixedOffset>>,
}

impl SlackInstallationBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn team_id(mut self, value: impl Into<String>) -> Self {
        self.team_id = Some(value.into());
        self
    }

    pub fn team_name(mut self, value: impl Into<String>) -> Self {
        self.team_name = Some(value.into());
        self
    }

    pub fn enterprise_id(mut self, value: impl Into<String>) -> Self {
        self.enterprise_id = Some(value.into());
        self
    }

    pub fn enterprise_name(mut self, value: impl Into<String>) -> Self {
        self.enterprise_name = Some(value.into());
        self
    }

    pub fn status(mut self, value: SlackInstallationStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn connected_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.connected_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SlackInstallation`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](SlackInstallationBuilder::id)
    /// - [`team_id`](SlackInstallationBuilder::team_id)
    /// - [`status`](SlackInstallationBuilder::status)
    /// - [`connected_at`](SlackInstallationBuilder::connected_at)
    pub fn build(self) -> Result<SlackInstallation, BuildError> {
        Ok(SlackInstallation {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            team_id: self.team_id.ok_or_else(|| BuildError::missing_field("team_id"))?,
            team_name: self.team_name,
            enterprise_id: self.enterprise_id,
            enterprise_name: self.enterprise_name,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            connected_at: self.connected_at.ok_or_else(|| BuildError::missing_field("connected_at"))?,
        })
    }
}
