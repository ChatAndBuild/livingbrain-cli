pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateBrainProfileRequest {
    /// Who the subject is. Omit to leave unchanged.
    #[serde(rename = "whoTheyAre")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub who_they_are: Option<String>,
    /// What the subject works on. Omit to leave unchanged.
    #[serde(rename = "whatTheyDo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub what_they_do: Option<String>,
    /// Current focus. Omit to leave unchanged.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub focus: Option<String>,
    /// Current goals. Omit to leave unchanged.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goals: Option<String>,
    /// Projects in flight. Omit to leave unchanged.
    #[serde(rename = "currentProjects")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_projects: Option<String>,
    /// Known blockers. Omit to leave unchanged.
    #[serde(rename = "stuckOn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stuck_on: Option<String>,
    /// Organization preference. Omit to leave unchanged.
    #[serde(rename = "vaultOrganization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vault_organization: Option<String>,
    /// Standing instructions. Omit to leave unchanged.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directives: Option<ProfileDirectives>,
}

impl UpdateBrainProfileRequest {
    pub fn builder() -> UpdateBrainProfileRequestBuilder {
        <UpdateBrainProfileRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateBrainProfileRequestBuilder {
    who_they_are: Option<String>,
    what_they_do: Option<String>,
    focus: Option<String>,
    goals: Option<String>,
    current_projects: Option<String>,
    stuck_on: Option<String>,
    vault_organization: Option<String>,
    directives: Option<ProfileDirectives>,
}

impl UpdateBrainProfileRequestBuilder {
    pub fn who_they_are(mut self, value: impl Into<String>) -> Self {
        self.who_they_are = Some(value.into());
        self
    }

    pub fn what_they_do(mut self, value: impl Into<String>) -> Self {
        self.what_they_do = Some(value.into());
        self
    }

    pub fn focus(mut self, value: impl Into<String>) -> Self {
        self.focus = Some(value.into());
        self
    }

    pub fn goals(mut self, value: impl Into<String>) -> Self {
        self.goals = Some(value.into());
        self
    }

    pub fn current_projects(mut self, value: impl Into<String>) -> Self {
        self.current_projects = Some(value.into());
        self
    }

    pub fn stuck_on(mut self, value: impl Into<String>) -> Self {
        self.stuck_on = Some(value.into());
        self
    }

    pub fn vault_organization(mut self, value: impl Into<String>) -> Self {
        self.vault_organization = Some(value.into());
        self
    }

    pub fn directives(mut self, value: ProfileDirectives) -> Self {
        self.directives = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateBrainProfileRequest`].
    pub fn build(self) -> Result<UpdateBrainProfileRequest, BuildError> {
        Ok(UpdateBrainProfileRequest {
            who_they_are: self.who_they_are,
            what_they_do: self.what_they_do,
            focus: self.focus,
            goals: self.goals,
            current_projects: self.current_projects,
            stuck_on: self.stuck_on,
            vault_organization: self.vault_organization,
            directives: self.directives,
        })
    }
}

