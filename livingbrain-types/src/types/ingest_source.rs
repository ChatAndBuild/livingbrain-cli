pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IngestSource {
    /// Ingest source id
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
    /// Kind of raw input
    pub kind: IngestSourceKind,
    /// Bucket hint for the resulting page(s)
    pub bucket: IngestSourceBucket,
    /// Short display label
    #[serde(default)]
    pub label: String,
    /// Extracted text; null for `fetchUrl` sources not yet extracted
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// Origin URL / external id, also the idempotency key for dedup
    #[serde(rename = "originRef")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_ref: Option<String>,
    /// Extraction hints (e.g. mimeType, fileName)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    /// Knowledge chunk uuids backing this source
    #[serde(rename = "chunkUuids")]
    #[serde(default)]
    pub chunk_uuids: Vec<String>,
    /// Compile pipeline status
    pub status: IngestSourceStatus,
    /// Original file/note bytes counted toward account storage
    #[serde(rename = "originalBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_bytes: Option<i64>,
    /// Last compile error, if any
    #[serde(rename = "lastError")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error: Option<String>,
    /// Ids of pages this source contributed to
    #[serde(rename = "affectedPageIds")]
    #[serde(default)]
    pub affected_page_ids: Vec<String>,
    /// Compile attempts so far
    #[serde(default)]
    pub attempts: i64,
    /// When the compile completed
    #[serde(rename = "compiledAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub compiled_at: Option<DateTime<FixedOffset>>,
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

impl IngestSource {
    pub fn builder() -> IngestSourceBuilder {
        <IngestSourceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct IngestSourceBuilder {
    id: Option<String>,
    tenant_id: Option<String>,
    subject_id: Option<String>,
    brain_id: Option<String>,
    source: Option<Source>,
    kind: Option<IngestSourceKind>,
    bucket: Option<IngestSourceBucket>,
    label: Option<String>,
    content: Option<String>,
    origin_ref: Option<String>,
    metadata: Option<HashMap<String, serde_json::Value>>,
    chunk_uuids: Option<Vec<String>>,
    status: Option<IngestSourceStatus>,
    original_bytes: Option<i64>,
    last_error: Option<String>,
    affected_page_ids: Option<Vec<String>>,
    attempts: Option<i64>,
    compiled_at: Option<DateTime<FixedOffset>>,
    created_at: Option<DateTime<FixedOffset>>,
    updated_at: Option<DateTime<FixedOffset>>,
}

impl IngestSourceBuilder {
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

    pub fn kind(mut self, value: IngestSourceKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn bucket(mut self, value: IngestSourceBucket) -> Self {
        self.bucket = Some(value);
        self
    }

    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn content(mut self, value: impl Into<String>) -> Self {
        self.content = Some(value.into());
        self
    }

    pub fn origin_ref(mut self, value: impl Into<String>) -> Self {
        self.origin_ref = Some(value.into());
        self
    }

    pub fn metadata(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn chunk_uuids(mut self, value: Vec<String>) -> Self {
        self.chunk_uuids = Some(value);
        self
    }

    pub fn status(mut self, value: IngestSourceStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn original_bytes(mut self, value: i64) -> Self {
        self.original_bytes = Some(value);
        self
    }

    pub fn last_error(mut self, value: impl Into<String>) -> Self {
        self.last_error = Some(value.into());
        self
    }

    pub fn affected_page_ids(mut self, value: Vec<String>) -> Self {
        self.affected_page_ids = Some(value);
        self
    }

    pub fn attempts(mut self, value: i64) -> Self {
        self.attempts = Some(value);
        self
    }

    pub fn compiled_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.compiled_at = Some(value);
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

    /// Consumes the builder and constructs a [`IngestSource`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](IngestSourceBuilder::id)
    /// - [`tenant_id`](IngestSourceBuilder::tenant_id)
    /// - [`subject_id`](IngestSourceBuilder::subject_id)
    /// - [`brain_id`](IngestSourceBuilder::brain_id)
    /// - [`kind`](IngestSourceBuilder::kind)
    /// - [`bucket`](IngestSourceBuilder::bucket)
    /// - [`label`](IngestSourceBuilder::label)
    /// - [`chunk_uuids`](IngestSourceBuilder::chunk_uuids)
    /// - [`status`](IngestSourceBuilder::status)
    /// - [`affected_page_ids`](IngestSourceBuilder::affected_page_ids)
    /// - [`attempts`](IngestSourceBuilder::attempts)
    /// - [`created_at`](IngestSourceBuilder::created_at)
    /// - [`updated_at`](IngestSourceBuilder::updated_at)
    pub fn build(self) -> Result<IngestSource, BuildError> {
        Ok(IngestSource {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            tenant_id: self.tenant_id.ok_or_else(|| BuildError::missing_field("tenant_id"))?,
            subject_id: self.subject_id.ok_or_else(|| BuildError::missing_field("subject_id"))?,
            brain_id: self.brain_id.ok_or_else(|| BuildError::missing_field("brain_id"))?,
            source: self.source,
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            bucket: self.bucket.ok_or_else(|| BuildError::missing_field("bucket"))?,
            label: self.label.ok_or_else(|| BuildError::missing_field("label"))?,
            content: self.content,
            origin_ref: self.origin_ref,
            metadata: self.metadata,
            chunk_uuids: self.chunk_uuids.ok_or_else(|| BuildError::missing_field("chunk_uuids"))?,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            original_bytes: self.original_bytes,
            last_error: self.last_error,
            affected_page_ids: self.affected_page_ids.ok_or_else(|| BuildError::missing_field("affected_page_ids"))?,
            attempts: self.attempts.ok_or_else(|| BuildError::missing_field("attempts"))?,
            compiled_at: self.compiled_at,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
            updated_at: self.updated_at.ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
