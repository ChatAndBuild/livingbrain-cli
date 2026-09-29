pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SearchRequest {
    /// Search query
    #[serde(default)]
    pub query: String,
    /// Max results
    #[serde(rename = "topK")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_k: Option<i64>,
    /// Minimum similarity (0-1)
    #[serde(rename = "minSimilarity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub min_similarity: Option<f64>,
}

impl SearchRequest {
    pub fn builder() -> SearchRequestBuilder {
        <SearchRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SearchRequestBuilder {
    query: Option<String>,
    top_k: Option<i64>,
    min_similarity: Option<f64>,
}

impl SearchRequestBuilder {
    pub fn query(mut self, value: impl Into<String>) -> Self {
        self.query = Some(value.into());
        self
    }

    pub fn top_k(mut self, value: i64) -> Self {
        self.top_k = Some(value);
        self
    }

    pub fn min_similarity(mut self, value: f64) -> Self {
        self.min_similarity = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SearchRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`query`](SearchRequestBuilder::query)
    pub fn build(self) -> Result<SearchRequest, BuildError> {
        Ok(SearchRequest {
            query: self.query.ok_or_else(|| BuildError::missing_field("query"))?,
            top_k: self.top_k,
            min_similarity: self.min_similarity,
        })
    }
}

