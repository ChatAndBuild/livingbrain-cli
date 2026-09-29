pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Brain {
    /// Brain id
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
    /// User-facing name
    #[serde(default)]
    pub name: String,
    /// URL-safe identifier, unique per (tenant, subject)
    #[serde(default)]
    pub slug: String,
    /// Description
    #[serde(default)]
    pub description: String,
    /// Daily-brief schedule and delivery settings for this brain.
    #[serde(rename = "briefSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brief_settings: Option<BriefSettingsResponse>,
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

impl Brain {
    pub fn builder() -> BrainBuilder {
        <BrainBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BrainBuilder {
    id: Option<String>,
    tenant_id: Option<String>,
    subject_id: Option<String>,
    name: Option<String>,
    slug: Option<String>,
    description: Option<String>,
    brief_settings: Option<BriefSettingsResponse>,
    created_at: Option<DateTime<FixedOffset>>,
    updated_at: Option<DateTime<FixedOffset>>,
}

impl BrainBuilder {
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

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn slug(mut self, value: impl Into<String>) -> Self {
        self.slug = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn brief_settings(mut self, value: BriefSettingsResponse) -> Self {
        self.brief_settings = Some(value);
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

    /// Consumes the builder and constructs a [`Brain`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](BrainBuilder::id)
    /// - [`tenant_id`](BrainBuilder::tenant_id)
    /// - [`subject_id`](BrainBuilder::subject_id)
    /// - [`name`](BrainBuilder::name)
    /// - [`slug`](BrainBuilder::slug)
    /// - [`description`](BrainBuilder::description)
    /// - [`created_at`](BrainBuilder::created_at)
    /// - [`updated_at`](BrainBuilder::updated_at)
    pub fn build(self) -> Result<Brain, BuildError> {
        Ok(Brain {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            tenant_id: self.tenant_id.ok_or_else(|| BuildError::missing_field("tenant_id"))?,
            subject_id: self.subject_id.ok_or_else(|| BuildError::missing_field("subject_id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            slug: self.slug.ok_or_else(|| BuildError::missing_field("slug"))?,
            description: self.description.ok_or_else(|| BuildError::missing_field("description"))?,
            brief_settings: self.brief_settings,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
            updated_at: self.updated_at.ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
