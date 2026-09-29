pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for graph
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GraphQueryRequest {
    #[serde(rename = "nodeLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_limit: Option<i64>,
}

impl GraphQueryRequest {
    pub fn builder() -> GraphQueryRequestBuilder {
        <GraphQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GraphQueryRequestBuilder {
    node_limit: Option<i64>,
}

impl GraphQueryRequestBuilder {
    pub fn node_limit(mut self, value: i64) -> Self {
        self.node_limit = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GraphQueryRequest`].
    pub fn build(self) -> Result<GraphQueryRequest, BuildError> {
        Ok(GraphQueryRequest {
            node_limit: self.node_limit,
        })
    }
}

