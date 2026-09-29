pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PageSourceRef {
    /// Contributing ingest source id
    #[serde(rename = "ingestSourceId")]
    #[serde(default)]
    pub ingest_source_id: String,
    /// Knowledge chunk uuids backing the source
    #[serde(rename = "chunkUuids")]
    #[serde(default)]
    pub chunk_uuids: Vec<String>,
    /// ISO 8601 timestamp of when this source was attached to the page.
    #[serde(rename = "attachedAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub attached_at: DateTime<FixedOffset>,
}

impl PageSourceRef {
    pub fn builder() -> PageSourceRefBuilder {
        <PageSourceRefBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PageSourceRefBuilder {
    ingest_source_id: Option<String>,
    chunk_uuids: Option<Vec<String>>,
    attached_at: Option<DateTime<FixedOffset>>,
}

impl PageSourceRefBuilder {
    pub fn ingest_source_id(mut self, value: impl Into<String>) -> Self {
        self.ingest_source_id = Some(value.into());
        self
    }

    pub fn chunk_uuids(mut self, value: Vec<String>) -> Self {
        self.chunk_uuids = Some(value);
        self
    }

    pub fn attached_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.attached_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PageSourceRef`].
    /// This method will fail if any of the following fields are not set:
    /// - [`ingest_source_id`](PageSourceRefBuilder::ingest_source_id)
    /// - [`chunk_uuids`](PageSourceRefBuilder::chunk_uuids)
    /// - [`attached_at`](PageSourceRefBuilder::attached_at)
    pub fn build(self) -> Result<PageSourceRef, BuildError> {
        Ok(PageSourceRef {
            ingest_source_id: self.ingest_source_id.ok_or_else(|| BuildError::missing_field("ingest_source_id"))?,
            chunk_uuids: self.chunk_uuids.ok_or_else(|| BuildError::missing_field("chunk_uuids"))?,
            attached_at: self.attached_at.ok_or_else(|| BuildError::missing_field("attached_at"))?,
        })
    }
}
