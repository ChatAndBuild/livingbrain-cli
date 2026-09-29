pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ProfileDirectivesResponse {
    /// Whether the brain should proactively surface non-obvious connections.
    #[serde(rename = "surfaceConnections")]
    #[serde(default)]
    pub surface_connections: bool,
    /// Whether the brain should push back on assumptions rather than agreeing.
    #[serde(rename = "challengeAssumptions")]
    #[serde(default)]
    pub challenge_assumptions: bool,
    /// Whether stored knowledge should outweigh the model's own priors.
    #[serde(rename = "preferVaultContext")]
    #[serde(default)]
    pub prefer_vault_context: bool,
    /// Whether to mark pages as contested when new material disagrees with them.
    #[serde(rename = "flagContradictions")]
    #[serde(default)]
    pub flag_contradictions: bool,
    /// Free-form directives appended to the compiled instructions.
    #[serde(default)]
    pub custom: String,
}

impl ProfileDirectivesResponse {
    pub fn builder() -> ProfileDirectivesResponseBuilder {
        <ProfileDirectivesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProfileDirectivesResponseBuilder {
    surface_connections: Option<bool>,
    challenge_assumptions: Option<bool>,
    prefer_vault_context: Option<bool>,
    flag_contradictions: Option<bool>,
    custom: Option<String>,
}

impl ProfileDirectivesResponseBuilder {
    pub fn surface_connections(mut self, value: bool) -> Self {
        self.surface_connections = Some(value);
        self
    }

    pub fn challenge_assumptions(mut self, value: bool) -> Self {
        self.challenge_assumptions = Some(value);
        self
    }

    pub fn prefer_vault_context(mut self, value: bool) -> Self {
        self.prefer_vault_context = Some(value);
        self
    }

    pub fn flag_contradictions(mut self, value: bool) -> Self {
        self.flag_contradictions = Some(value);
        self
    }

    pub fn custom(mut self, value: impl Into<String>) -> Self {
        self.custom = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ProfileDirectivesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`surface_connections`](ProfileDirectivesResponseBuilder::surface_connections)
    /// - [`challenge_assumptions`](ProfileDirectivesResponseBuilder::challenge_assumptions)
    /// - [`prefer_vault_context`](ProfileDirectivesResponseBuilder::prefer_vault_context)
    /// - [`flag_contradictions`](ProfileDirectivesResponseBuilder::flag_contradictions)
    /// - [`custom`](ProfileDirectivesResponseBuilder::custom)
    pub fn build(self) -> Result<ProfileDirectivesResponse, BuildError> {
        Ok(ProfileDirectivesResponse {
            surface_connections: self.surface_connections.ok_or_else(|| BuildError::missing_field("surface_connections"))?,
            challenge_assumptions: self.challenge_assumptions.ok_or_else(|| BuildError::missing_field("challenge_assumptions"))?,
            prefer_vault_context: self.prefer_vault_context.ok_or_else(|| BuildError::missing_field("prefer_vault_context"))?,
            flag_contradictions: self.flag_contradictions.ok_or_else(|| BuildError::missing_field("flag_contradictions"))?,
            custom: self.custom.ok_or_else(|| BuildError::missing_field("custom"))?,
        })
    }
}
