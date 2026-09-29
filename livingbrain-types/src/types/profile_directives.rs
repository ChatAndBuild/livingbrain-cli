pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ProfileDirectives {
    /// Surface non-obvious connections between captures
    #[serde(rename = "surfaceConnections")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surface_connections: Option<bool>,
    /// Challenge the user's assumptions before agreeing
    #[serde(rename = "challengeAssumptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub challenge_assumptions: Option<bool>,
    /// Answer from vault context, not generically
    #[serde(rename = "preferVaultContext")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefer_vault_context: Option<bool>,
    /// Flag contradictions with earlier saved beliefs
    #[serde(rename = "flagContradictions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flag_contradictions: Option<bool>,
    /// Custom free-form directives
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom: Option<String>,
}

impl ProfileDirectives {
    pub fn builder() -> ProfileDirectivesBuilder {
        <ProfileDirectivesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProfileDirectivesBuilder {
    surface_connections: Option<bool>,
    challenge_assumptions: Option<bool>,
    prefer_vault_context: Option<bool>,
    flag_contradictions: Option<bool>,
    custom: Option<String>,
}

impl ProfileDirectivesBuilder {
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

    /// Consumes the builder and constructs a [`ProfileDirectives`].
    pub fn build(self) -> Result<ProfileDirectives, BuildError> {
        Ok(ProfileDirectives {
            surface_connections: self.surface_connections,
            challenge_assumptions: self.challenge_assumptions,
            prefer_vault_context: self.prefer_vault_context,
            flag_contradictions: self.flag_contradictions,
            custom: self.custom,
        })
    }
}
