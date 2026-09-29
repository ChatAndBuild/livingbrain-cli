pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ChangeEvent {
    /// Change event id
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
    /// Affected page id
    #[serde(rename = "pageId")]
    #[serde(default)]
    pub page_id: String,
    /// Slug of the page this refers to.
    #[serde(rename = "pageSlug")]
    #[serde(default)]
    pub page_slug: String,
    /// Title of the page at the time of the event.
    #[serde(rename = "pageTitle")]
    #[serde(default)]
    pub page_title: String,
    /// What happened to the page.
    pub action: ChangeEventAction,
    /// One-line human-readable description of the change.
    #[serde(rename = "changeSummary")]
    #[serde(default)]
    pub change_summary: String,
    /// Page version after the change.
    #[serde(rename = "toVersion")]
    #[serde(default)]
    pub to_version: i64,
    /// Page version before the change.
    #[serde(rename = "fromVersion")]
    #[serde(default)]
    pub from_version: i64,
    /// Ingest source that triggered the change, if any
    #[serde(rename = "ingestSourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingest_source_id: Option<String>,
    /// Contradiction notes raised by the compile
    #[serde(default)]
    pub contradictions: Vec<String>,
    /// ISO 8601 timestamp of when a user acknowledged this, or null if still unacknowledged.
    #[serde(rename = "ackedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub acked_at: Option<DateTime<FixedOffset>>,
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

impl ChangeEvent {
    pub fn builder() -> ChangeEventBuilder {
        <ChangeEventBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ChangeEventBuilder {
    id: Option<String>,
    tenant_id: Option<String>,
    subject_id: Option<String>,
    brain_id: Option<String>,
    page_id: Option<String>,
    page_slug: Option<String>,
    page_title: Option<String>,
    action: Option<ChangeEventAction>,
    change_summary: Option<String>,
    to_version: Option<i64>,
    from_version: Option<i64>,
    ingest_source_id: Option<String>,
    contradictions: Option<Vec<String>>,
    acked_at: Option<DateTime<FixedOffset>>,
    created_at: Option<DateTime<FixedOffset>>,
    updated_at: Option<DateTime<FixedOffset>>,
}

impl ChangeEventBuilder {
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

    pub fn page_id(mut self, value: impl Into<String>) -> Self {
        self.page_id = Some(value.into());
        self
    }

    pub fn page_slug(mut self, value: impl Into<String>) -> Self {
        self.page_slug = Some(value.into());
        self
    }

    pub fn page_title(mut self, value: impl Into<String>) -> Self {
        self.page_title = Some(value.into());
        self
    }

    pub fn action(mut self, value: ChangeEventAction) -> Self {
        self.action = Some(value);
        self
    }

    pub fn change_summary(mut self, value: impl Into<String>) -> Self {
        self.change_summary = Some(value.into());
        self
    }

    pub fn to_version(mut self, value: i64) -> Self {
        self.to_version = Some(value);
        self
    }

    pub fn from_version(mut self, value: i64) -> Self {
        self.from_version = Some(value);
        self
    }

    pub fn ingest_source_id(mut self, value: impl Into<String>) -> Self {
        self.ingest_source_id = Some(value.into());
        self
    }

    pub fn contradictions(mut self, value: Vec<String>) -> Self {
        self.contradictions = Some(value);
        self
    }

    pub fn acked_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.acked_at = Some(value);
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

    /// Consumes the builder and constructs a [`ChangeEvent`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ChangeEventBuilder::id)
    /// - [`tenant_id`](ChangeEventBuilder::tenant_id)
    /// - [`subject_id`](ChangeEventBuilder::subject_id)
    /// - [`brain_id`](ChangeEventBuilder::brain_id)
    /// - [`page_id`](ChangeEventBuilder::page_id)
    /// - [`page_slug`](ChangeEventBuilder::page_slug)
    /// - [`page_title`](ChangeEventBuilder::page_title)
    /// - [`action`](ChangeEventBuilder::action)
    /// - [`change_summary`](ChangeEventBuilder::change_summary)
    /// - [`to_version`](ChangeEventBuilder::to_version)
    /// - [`from_version`](ChangeEventBuilder::from_version)
    /// - [`contradictions`](ChangeEventBuilder::contradictions)
    /// - [`created_at`](ChangeEventBuilder::created_at)
    /// - [`updated_at`](ChangeEventBuilder::updated_at)
    pub fn build(self) -> Result<ChangeEvent, BuildError> {
        Ok(ChangeEvent {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            tenant_id: self.tenant_id.ok_or_else(|| BuildError::missing_field("tenant_id"))?,
            subject_id: self.subject_id.ok_or_else(|| BuildError::missing_field("subject_id"))?,
            brain_id: self.brain_id.ok_or_else(|| BuildError::missing_field("brain_id"))?,
            page_id: self.page_id.ok_or_else(|| BuildError::missing_field("page_id"))?,
            page_slug: self.page_slug.ok_or_else(|| BuildError::missing_field("page_slug"))?,
            page_title: self.page_title.ok_or_else(|| BuildError::missing_field("page_title"))?,
            action: self.action.ok_or_else(|| BuildError::missing_field("action"))?,
            change_summary: self.change_summary.ok_or_else(|| BuildError::missing_field("change_summary"))?,
            to_version: self.to_version.ok_or_else(|| BuildError::missing_field("to_version"))?,
            from_version: self.from_version.ok_or_else(|| BuildError::missing_field("from_version"))?,
            ingest_source_id: self.ingest_source_id,
            contradictions: self.contradictions.ok_or_else(|| BuildError::missing_field("contradictions"))?,
            acked_at: self.acked_at,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
            updated_at: self.updated_at.ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
