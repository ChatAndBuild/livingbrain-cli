pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SlackOAuthConfirmResponse {
    /// ID of the Slack installation that was created or updated.
    #[serde(rename = "installationId")]
    #[serde(default)]
    pub installation_id: String,
    /// ID of the brain the installation is attached to.
    #[serde(rename = "brainId")]
    #[serde(default)]
    pub brain_id: String,
    /// Slack workspace (team) ID.
    #[serde(rename = "teamId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
    /// Slack workspace name.
    #[serde(rename = "teamName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_name: Option<String>,
}

impl SlackOAuthConfirmResponse {
    pub fn builder() -> SlackOAuthConfirmResponseBuilder {
        <SlackOAuthConfirmResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SlackOAuthConfirmResponseBuilder {
    installation_id: Option<String>,
    brain_id: Option<String>,
    team_id: Option<String>,
    team_name: Option<String>,
}

impl SlackOAuthConfirmResponseBuilder {
    pub fn installation_id(mut self, value: impl Into<String>) -> Self {
        self.installation_id = Some(value.into());
        self
    }

    pub fn brain_id(mut self, value: impl Into<String>) -> Self {
        self.brain_id = Some(value.into());
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

    /// Consumes the builder and constructs a [`SlackOAuthConfirmResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`installation_id`](SlackOAuthConfirmResponseBuilder::installation_id)
    /// - [`brain_id`](SlackOAuthConfirmResponseBuilder::brain_id)
    pub fn build(self) -> Result<SlackOAuthConfirmResponse, BuildError> {
        Ok(SlackOAuthConfirmResponse {
            installation_id: self.installation_id.ok_or_else(|| BuildError::missing_field("installation_id"))?,
            brain_id: self.brain_id.ok_or_else(|| BuildError::missing_field("brain_id"))?,
            team_id: self.team_id,
            team_name: self.team_name,
        })
    }
}
