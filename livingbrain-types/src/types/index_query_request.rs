pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for index
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct IndexQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

impl IndexQueryRequest {
    pub fn builder() -> IndexQueryRequestBuilder {
        <IndexQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct IndexQueryRequestBuilder {
    limit: Option<i64>,
}

impl IndexQueryRequestBuilder {
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`IndexQueryRequest`].
    pub fn build(self) -> Result<IndexQueryRequest, BuildError> {
        Ok(IndexQueryRequest {
            limit: self.limit,
        })
    }
}

