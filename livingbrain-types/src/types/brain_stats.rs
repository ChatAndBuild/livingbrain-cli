pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BrainStats {
    /// Active (non-deprecated, non-archived) pages
    #[serde(rename = "pageCount")]
    #[serde(default)]
    pub page_count: i64,
    /// Deprecated/stale pages awaiting review or recovery
    #[serde(rename = "deprecatedPageCount")]
    #[serde(default)]
    pub deprecated_page_count: i64,
    /// Archived pages
    #[serde(rename = "archivedPageCount")]
    #[serde(default)]
    pub archived_page_count: i64,
    /// Ingest sources ever pushed to the brain
    #[serde(rename = "sourceCount")]
    #[serde(default)]
    pub source_count: i64,
    /// Connections between active pages
    #[serde(rename = "connectionCount")]
    #[serde(default)]
    pub connection_count: i64,
    /// Host concept; always 0 here (the host may merge its own count)
    #[serde(rename = "agentCount")]
    #[serde(default)]
    pub agent_count: i64,
    /// Approximate size of all page bodies in bytes
    #[serde(rename = "estimatedBytes")]
    #[serde(default)]
    pub estimated_bytes: i64,
    /// Active page count per bucket
    #[serde(rename = "bucketCounts")]
    #[serde(default)]
    pub bucket_counts: HashMap<String, f64>,
    /// ISO timestamp of the last compile/read activity
    #[serde(rename = "lastActivityAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_activity_at: Option<String>,
}

impl BrainStats {
    pub fn builder() -> BrainStatsBuilder {
        <BrainStatsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BrainStatsBuilder {
    page_count: Option<i64>,
    deprecated_page_count: Option<i64>,
    archived_page_count: Option<i64>,
    source_count: Option<i64>,
    connection_count: Option<i64>,
    agent_count: Option<i64>,
    estimated_bytes: Option<i64>,
    bucket_counts: Option<HashMap<String, f64>>,
    last_activity_at: Option<String>,
}

impl BrainStatsBuilder {
    pub fn page_count(mut self, value: i64) -> Self {
        self.page_count = Some(value);
        self
    }

    pub fn deprecated_page_count(mut self, value: i64) -> Self {
        self.deprecated_page_count = Some(value);
        self
    }

    pub fn archived_page_count(mut self, value: i64) -> Self {
        self.archived_page_count = Some(value);
        self
    }

    pub fn source_count(mut self, value: i64) -> Self {
        self.source_count = Some(value);
        self
    }

    pub fn connection_count(mut self, value: i64) -> Self {
        self.connection_count = Some(value);
        self
    }

    pub fn agent_count(mut self, value: i64) -> Self {
        self.agent_count = Some(value);
        self
    }

    pub fn estimated_bytes(mut self, value: i64) -> Self {
        self.estimated_bytes = Some(value);
        self
    }

    pub fn bucket_counts(mut self, value: HashMap<String, f64>) -> Self {
        self.bucket_counts = Some(value);
        self
    }

    pub fn last_activity_at(mut self, value: impl Into<String>) -> Self {
        self.last_activity_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BrainStats`].
    /// This method will fail if any of the following fields are not set:
    /// - [`page_count`](BrainStatsBuilder::page_count)
    /// - [`deprecated_page_count`](BrainStatsBuilder::deprecated_page_count)
    /// - [`archived_page_count`](BrainStatsBuilder::archived_page_count)
    /// - [`source_count`](BrainStatsBuilder::source_count)
    /// - [`connection_count`](BrainStatsBuilder::connection_count)
    /// - [`agent_count`](BrainStatsBuilder::agent_count)
    /// - [`estimated_bytes`](BrainStatsBuilder::estimated_bytes)
    /// - [`bucket_counts`](BrainStatsBuilder::bucket_counts)
    pub fn build(self) -> Result<BrainStats, BuildError> {
        Ok(BrainStats {
            page_count: self.page_count.ok_or_else(|| BuildError::missing_field("page_count"))?,
            deprecated_page_count: self.deprecated_page_count.ok_or_else(|| BuildError::missing_field("deprecated_page_count"))?,
            archived_page_count: self.archived_page_count.ok_or_else(|| BuildError::missing_field("archived_page_count"))?,
            source_count: self.source_count.ok_or_else(|| BuildError::missing_field("source_count"))?,
            connection_count: self.connection_count.ok_or_else(|| BuildError::missing_field("connection_count"))?,
            agent_count: self.agent_count.ok_or_else(|| BuildError::missing_field("agent_count"))?,
            estimated_bytes: self.estimated_bytes.ok_or_else(|| BuildError::missing_field("estimated_bytes"))?,
            bucket_counts: self.bucket_counts.ok_or_else(|| BuildError::missing_field("bucket_counts"))?,
            last_activity_at: self.last_activity_at,
        })
    }
}
