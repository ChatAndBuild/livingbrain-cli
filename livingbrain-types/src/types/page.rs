pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Page {
    /// Page id
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
    /// Opaque provenance
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,
    /// What kind of knowledge this page holds.
    #[serde(rename = "pageType")]
    pub page_type: PagePageType,
    /// Top-level grouping the page belongs to.
    pub bucket: PageBucket,
    /// Whether the bucket was set automatically or by the user
    #[serde(rename = "bucketSource")]
    pub bucket_source: PageBucketSource,
    /// URL-safe identifier, unique per brain
    #[serde(default)]
    pub slug: String,
    /// Human-readable title.
    #[serde(default)]
    pub title: String,
    /// Short generated summary of the content.
    #[serde(default)]
    pub summary: String,
    /// Markdown body
    #[serde(default)]
    pub body: String,
    /// Named entities extracted from the body
    #[serde(default)]
    pub entities: Vec<String>,
    /// Contributing ingest sources
    #[serde(default)]
    pub sources: Vec<PageSourceRef>,
    /// Wikilink slugs referenced by the body
    #[serde(default)]
    pub backlinks: Vec<String>,
    /// Vector embedding used for semantic search (internal; can be large)
    #[serde(default)]
    pub embedding: Vec<f64>,
    /// Monotonic version, bumped on every compile/edit
    #[serde(default)]
    pub version: i64,
    /// Pinned pages rank first in the prompt index and are exempt from deprecation.
    #[serde(default)]
    pub pinned: bool,
    /// Whether the page has been retired by the user.
    #[serde(default)]
    pub archived: bool,
    /// Lifecycle state. Deprecated and archived items are hidden from listings by default.
    pub status: PageStatus,
    /// ISO 8601 timestamp of the last time this page was recompiled from its sources.
    #[serde(rename = "lastCompiledAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub last_compiled_at: DateTime<FixedOffset>,
    /// ISO 8601 timestamp of the last read. Reading a page bumps this, and it feeds the brain health score.
    #[serde(rename = "lastReferencedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub last_referenced_at: Option<DateTime<FixedOffset>>,
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

impl Page {
    pub fn builder() -> PageBuilder {
        <PageBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PageBuilder {
    id: Option<String>,
    tenant_id: Option<String>,
    subject_id: Option<String>,
    brain_id: Option<String>,
    source: Option<Source>,
    page_type: Option<PagePageType>,
    bucket: Option<PageBucket>,
    bucket_source: Option<PageBucketSource>,
    slug: Option<String>,
    title: Option<String>,
    summary: Option<String>,
    body: Option<String>,
    entities: Option<Vec<String>>,
    sources: Option<Vec<PageSourceRef>>,
    backlinks: Option<Vec<String>>,
    embedding: Option<Vec<f64>>,
    version: Option<i64>,
    pinned: Option<bool>,
    archived: Option<bool>,
    status: Option<PageStatus>,
    last_compiled_at: Option<DateTime<FixedOffset>>,
    last_referenced_at: Option<DateTime<FixedOffset>>,
    created_at: Option<DateTime<FixedOffset>>,
    updated_at: Option<DateTime<FixedOffset>>,
}

impl PageBuilder {
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

    pub fn source(mut self, value: Source) -> Self {
        self.source = Some(value);
        self
    }

    pub fn page_type(mut self, value: PagePageType) -> Self {
        self.page_type = Some(value);
        self
    }

    pub fn bucket(mut self, value: PageBucket) -> Self {
        self.bucket = Some(value);
        self
    }

    pub fn bucket_source(mut self, value: PageBucketSource) -> Self {
        self.bucket_source = Some(value);
        self
    }

    pub fn slug(mut self, value: impl Into<String>) -> Self {
        self.slug = Some(value.into());
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

    pub fn entities(mut self, value: Vec<String>) -> Self {
        self.entities = Some(value);
        self
    }

    pub fn sources(mut self, value: Vec<PageSourceRef>) -> Self {
        self.sources = Some(value);
        self
    }

    pub fn backlinks(mut self, value: Vec<String>) -> Self {
        self.backlinks = Some(value);
        self
    }

    pub fn embedding(mut self, value: Vec<f64>) -> Self {
        self.embedding = Some(value);
        self
    }

    pub fn version(mut self, value: i64) -> Self {
        self.version = Some(value);
        self
    }

    pub fn pinned(mut self, value: bool) -> Self {
        self.pinned = Some(value);
        self
    }

    pub fn archived(mut self, value: bool) -> Self {
        self.archived = Some(value);
        self
    }

    pub fn status(mut self, value: PageStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn last_compiled_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.last_compiled_at = Some(value);
        self
    }

    pub fn last_referenced_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.last_referenced_at = Some(value);
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

    /// Consumes the builder and constructs a [`Page`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PageBuilder::id)
    /// - [`tenant_id`](PageBuilder::tenant_id)
    /// - [`subject_id`](PageBuilder::subject_id)
    /// - [`brain_id`](PageBuilder::brain_id)
    /// - [`page_type`](PageBuilder::page_type)
    /// - [`bucket`](PageBuilder::bucket)
    /// - [`bucket_source`](PageBuilder::bucket_source)
    /// - [`slug`](PageBuilder::slug)
    /// - [`title`](PageBuilder::title)
    /// - [`summary`](PageBuilder::summary)
    /// - [`body`](PageBuilder::body)
    /// - [`entities`](PageBuilder::entities)
    /// - [`sources`](PageBuilder::sources)
    /// - [`backlinks`](PageBuilder::backlinks)
    /// - [`embedding`](PageBuilder::embedding)
    /// - [`version`](PageBuilder::version)
    /// - [`pinned`](PageBuilder::pinned)
    /// - [`archived`](PageBuilder::archived)
    /// - [`status`](PageBuilder::status)
    /// - [`last_compiled_at`](PageBuilder::last_compiled_at)
    /// - [`created_at`](PageBuilder::created_at)
    /// - [`updated_at`](PageBuilder::updated_at)
    pub fn build(self) -> Result<Page, BuildError> {
        Ok(Page {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            tenant_id: self.tenant_id.ok_or_else(|| BuildError::missing_field("tenant_id"))?,
            subject_id: self.subject_id.ok_or_else(|| BuildError::missing_field("subject_id"))?,
            brain_id: self.brain_id.ok_or_else(|| BuildError::missing_field("brain_id"))?,
            source: self.source,
            page_type: self.page_type.ok_or_else(|| BuildError::missing_field("page_type"))?,
            bucket: self.bucket.ok_or_else(|| BuildError::missing_field("bucket"))?,
            bucket_source: self.bucket_source.ok_or_else(|| BuildError::missing_field("bucket_source"))?,
            slug: self.slug.ok_or_else(|| BuildError::missing_field("slug"))?,
            title: self.title.ok_or_else(|| BuildError::missing_field("title"))?,
            summary: self.summary.ok_or_else(|| BuildError::missing_field("summary"))?,
            body: self.body.ok_or_else(|| BuildError::missing_field("body"))?,
            entities: self.entities.ok_or_else(|| BuildError::missing_field("entities"))?,
            sources: self.sources.ok_or_else(|| BuildError::missing_field("sources"))?,
            backlinks: self.backlinks.ok_or_else(|| BuildError::missing_field("backlinks"))?,
            embedding: self.embedding.ok_or_else(|| BuildError::missing_field("embedding"))?,
            version: self.version.ok_or_else(|| BuildError::missing_field("version"))?,
            pinned: self.pinned.ok_or_else(|| BuildError::missing_field("pinned"))?,
            archived: self.archived.ok_or_else(|| BuildError::missing_field("archived"))?,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            last_compiled_at: self.last_compiled_at.ok_or_else(|| BuildError::missing_field("last_compiled_at"))?,
            last_referenced_at: self.last_referenced_at,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
            updated_at: self.updated_at.ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
