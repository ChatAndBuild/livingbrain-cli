pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CaptureBatchRequest {
    /// Captures to ingest
    #[serde(default)]
    pub items: Vec<CaptureRequest>,
}

impl CaptureBatchRequest {
    pub fn builder() -> CaptureBatchRequestBuilder {
        <CaptureBatchRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CaptureBatchRequestBuilder {
    items: Option<Vec<CaptureRequest>>,
}

impl CaptureBatchRequestBuilder {
    pub fn items(mut self, value: Vec<CaptureRequest>) -> Self {
        self.items = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CaptureBatchRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`items`](CaptureBatchRequestBuilder::items)
    pub fn build(self) -> Result<CaptureBatchRequest, BuildError> {
        Ok(CaptureBatchRequest {
            items: self.items.ok_or_else(|| BuildError::missing_field("items"))?,
        })
    }
}

