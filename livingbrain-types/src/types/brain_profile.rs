pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BrainProfile {
    /// Profile id
    #[serde(default)]
    pub id: String,
    /// Owning tenant id
    #[serde(rename = "tenantId")]
    #[serde(default)]
    pub tenant_id: String,
    /// Owning subject id (host-defined)
    #[serde(rename = "subjectId")]
    #[serde(default)]
    pub subject_id: String,
    /// Owning brain id
    #[serde(rename = "brainId")]
    #[serde(default)]
    pub brain_id: String,
    /// Who the subject is — role, background, and context that frames how captures are interpreted.
    #[serde(rename = "whoTheyAre")]
    #[serde(default)]
    pub who_they_are: String,
    /// What the subject works on day to day.
    #[serde(rename = "whatTheyDo")]
    #[serde(default)]
    pub what_they_do: String,
    /// What the subject is concentrating on right now.
    #[serde(default)]
    pub focus: String,
    /// What the subject is trying to achieve.
    #[serde(default)]
    pub goals: String,
    /// Projects currently in flight.
    #[serde(rename = "currentProjects")]
    #[serde(default)]
    pub current_projects: String,
    /// Known blockers, so the brain can surface relevant material.
    #[serde(rename = "stuckOn")]
    #[serde(default)]
    pub stuck_on: String,
    /// How the subject prefers their knowledge organized.
    #[serde(rename = "vaultOrganization")]
    #[serde(default)]
    pub vault_organization: String,
    /// Standing instructions that shape how new captures are compiled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directives: Option<ProfileDirectivesResponse>,
    /// ISO 8601 timestamp of when this record was created.
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// ISO 8601 timestamp of when this record was last modified.
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
}

impl BrainProfile {
    pub fn builder() -> BrainProfileBuilder {
        <BrainProfileBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BrainProfileBuilder {
    id: Option<String>,
    tenant_id: Option<String>,
    subject_id: Option<String>,
    brain_id: Option<String>,
    who_they_are: Option<String>,
    what_they_do: Option<String>,
    focus: Option<String>,
    goals: Option<String>,
    current_projects: Option<String>,
    stuck_on: Option<String>,
    vault_organization: Option<String>,
    directives: Option<ProfileDirectivesResponse>,
    created_at: Option<DateTime<FixedOffset>>,
    updated_at: Option<DateTime<FixedOffset>>,
}

impl BrainProfileBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn tenant_id(mut self, value: impl Into<String>) -> Self {
        self.tenant_id = Some(value.into());
        self
    }

    pub fn subject_id(mut self, value: impl Into<String>) -> Self {
        self.subject_id = Some(value.into());
        self
    }

    pub fn brain_id(mut self, value: impl Into<String>) -> Self {
        self.brain_id = Some(value.into());
        self
    }

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

    pub fn directives(mut self, value: ProfileDirectivesResponse) -> Self {
        self.directives = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BrainProfile`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](BrainProfileBuilder::id)
    /// - [`tenant_id`](BrainProfileBuilder::tenant_id)
    /// - [`subject_id`](BrainProfileBuilder::subject_id)
    /// - [`brain_id`](BrainProfileBuilder::brain_id)
    /// - [`who_they_are`](BrainProfileBuilder::who_they_are)
    /// - [`what_they_do`](BrainProfileBuilder::what_they_do)
    /// - [`focus`](BrainProfileBuilder::focus)
    /// - [`goals`](BrainProfileBuilder::goals)
    /// - [`current_projects`](BrainProfileBuilder::current_projects)
    /// - [`stuck_on`](BrainProfileBuilder::stuck_on)
    /// - [`vault_organization`](BrainProfileBuilder::vault_organization)
    /// - [`created_at`](BrainProfileBuilder::created_at)
    /// - [`updated_at`](BrainProfileBuilder::updated_at)
    pub fn build(self) -> Result<BrainProfile, BuildError> {
        Ok(BrainProfile {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            tenant_id: self.tenant_id.ok_or_else(|| BuildError::missing_field("tenant_id"))?,
            subject_id: self.subject_id.ok_or_else(|| BuildError::missing_field("subject_id"))?,
            brain_id: self.brain_id.ok_or_else(|| BuildError::missing_field("brain_id"))?,
            who_they_are: self.who_they_are.ok_or_else(|| BuildError::missing_field("who_they_are"))?,
            what_they_do: self.what_they_do.ok_or_else(|| BuildError::missing_field("what_they_do"))?,
            focus: self.focus.ok_or_else(|| BuildError::missing_field("focus"))?,
            goals: self.goals.ok_or_else(|| BuildError::missing_field("goals"))?,
            current_projects: self.current_projects.ok_or_else(|| BuildError::missing_field("current_projects"))?,
            stuck_on: self.stuck_on.ok_or_else(|| BuildError::missing_field("stuck_on"))?,
            vault_organization: self.vault_organization.ok_or_else(|| BuildError::missing_field("vault_organization"))?,
            directives: self.directives,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
            updated_at: self.updated_at.ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
