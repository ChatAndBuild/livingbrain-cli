pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CaptureBatchResponse {
    /// Number of sources ingested
    #[serde(default)]
    pub ingested: i64,
    /// One ingest source per accepted item, in the order submitted.
    #[serde(default)]
    pub sources: Vec<IngestSource>,
}

impl CaptureBatchResponse {
    pub fn builder() -> CaptureBatchResponseBuilder {
        <CaptureBatchResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CaptureBatchResponseBuilder {
    ingested: Option<i64>,
    sources: Option<Vec<IngestSource>>,
}

impl CaptureBatchResponseBuilder {
    pub fn ingested(mut self, value: i64) -> Self {
        self.ingested = Some(value);
        self
    }

    pub fn sources(mut self, value: Vec<IngestSource>) -> Self {
        self.sources = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CaptureBatchResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`ingested`](CaptureBatchResponseBuilder::ingested)
    /// - [`sources`](CaptureBatchResponseBuilder::sources)
    pub fn build(self) -> Result<CaptureBatchResponse, BuildError> {
        Ok(CaptureBatchResponse {
            ingested: self.ingested.ok_or_else(|| BuildError::missing_field("ingested"))?,
            sources: self.sources.ok_or_else(|| BuildError::missing_field("sources"))?,
        })
    }
}
