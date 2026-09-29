pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PageVersion {
    /// Version snapshot id
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
    /// Page id
    #[serde(rename = "pageId")]
    #[serde(default)]
    pub page_id: String,
    /// Monotonically increasing version number; every edit or recompile increments it.
    #[serde(default)]
    pub version: i64,
    /// Page title as of this version.
    #[serde(default)]
    pub title: String,
    /// Page summary as of this version.
    #[serde(default)]
    pub summary: String,
    /// Markdown body at this version
    #[serde(default)]
    pub body: String,
    /// Ingest source that produced this version, if any
    #[serde(rename = "ingestSourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingest_source_id: Option<String>,
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

impl PageVersion {
    pub fn builder() -> PageVersionBuilder {
        <PageVersionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PageVersionBuilder {
    id: Option<String>,
    tenant_id: Option<String>,
    subject_id: Option<String>,
    brain_id: Option<String>,
    page_id: Option<String>,
    version: Option<i64>,
    title: Option<String>,
    summary: Option<String>,
    body: Option<String>,
    ingest_source_id: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    updated_at: Option<DateTime<FixedOffset>>,
}

impl PageVersionBuilder {
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

    pub fn version(mut self, value: i64) -> Self {
        self.version = Some(value);
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn summary(mut self, value: impl Into<String>) -> Self {
        self.summary = Some(value.into());
        self
    }

    pub fn body(mut self, value: impl Into<String>) -> Self {
        self.body = Some(value.into());
        self
    }

    pub fn ingest_source_id(mut self, value: impl Into<String>) -> Self {
        self.ingest_source_id = Some(value.into());
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

    /// Consumes the builder and constructs a [`PageVersion`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PageVersionBuilder::id)
    /// - [`tenant_id`](PageVersionBuilder::tenant_id)
    /// - [`subject_id`](PageVersionBuilder::subject_id)
    /// - [`brain_id`](PageVersionBuilder::brain_id)
    /// - [`page_id`](PageVersionBuilder::page_id)
    /// - [`version`](PageVersionBuilder::version)
    /// - [`title`](PageVersionBuilder::title)
    /// - [`summary`](PageVersionBuilder::summary)
    /// - [`body`](PageVersionBuilder::body)
    /// - [`created_at`](PageVersionBuilder::created_at)
    /// - [`updated_at`](PageVersionBuilder::updated_at)
    pub fn build(self) -> Result<PageVersion, BuildError> {
        Ok(PageVersion {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            tenant_id: self.tenant_id.ok_or_else(|| BuildError::missing_field("tenant_id"))?,
            subject_id: self.subject_id.ok_or_else(|| BuildError::missing_field("subject_id"))?,
            brain_id: self.brain_id.ok_or_else(|| BuildError::missing_field("brain_id"))?,
            page_id: self.page_id.ok_or_else(|| BuildError::missing_field("page_id"))?,
            version: self.version.ok_or_else(|| BuildError::missing_field("version"))?,
            title: self.title.ok_or_else(|| BuildError::missing_field("title"))?,
            summary: self.summary.ok_or_else(|| BuildError::missing_field("summary"))?,
            body: self.body.ok_or_else(|| BuildError::missing_field("body"))?,
            ingest_source_id: self.ingest_source_id,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
            updated_at: self.updated_at.ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
