pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for bulkStats
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BulkStatsQueryRequest {
    /// Comma-separated brain ids, e.g. `ids=a,b,c`. Unowned ids are omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<String>,
}

impl BulkStatsQueryRequest {
    pub fn builder() -> BulkStatsQueryRequestBuilder {
        <BulkStatsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkStatsQueryRequestBuilder {
    ids: Option<String>,
}

impl BulkStatsQueryRequestBuilder {
    pub fn ids(mut self, value: impl Into<String>) -> Self {
        self.ids = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BulkStatsQueryRequest`].
    pub fn build(self) -> Result<BulkStatsQueryRequest, BuildError> {
        Ok(BulkStatsQueryRequest {
            ids: self.ids,
        })
    }
}

