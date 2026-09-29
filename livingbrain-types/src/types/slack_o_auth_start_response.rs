pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SlackOAuthStartResponse {
    /// Open this URL to authorize the LivingBrain Slack app
    #[serde(rename = "authorizationUrl")]
    #[serde(default)]
    pub authorization_url: String,
    /// Expiry of the signed OAuth state
    #[serde(rename = "expiresAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub expires_at: DateTime<FixedOffset>,
}

impl SlackOAuthStartResponse {
    pub fn builder() -> SlackOAuthStartResponseBuilder {
        <SlackOAuthStartResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SlackOAuthStartResponseBuilder {
    authorization_url: Option<String>,
    expires_at: Option<DateTime<FixedOffset>>,
}

impl SlackOAuthStartResponseBuilder {
    pub fn authorization_url(mut self, value: impl Into<String>) -> Self {
        self.authorization_url = Some(value.into());
        self
    }

    pub fn expires_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.expires_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SlackOAuthStartResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`authorization_url`](SlackOAuthStartResponseBuilder::authorization_url)
    /// - [`expires_at`](SlackOAuthStartResponseBuilder::expires_at)
    pub fn build(self) -> Result<SlackOAuthStartResponse, BuildError> {
        Ok(SlackOAuthStartResponse {
            authorization_url: self.authorization_url.ok_or_else(|| BuildError::missing_field("authorization_url"))?,
            expires_at: self.expires_at.ok_or_else(|| BuildError::missing_field("expires_at"))?,
        })
    }
}
