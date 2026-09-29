pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for listConnections
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListConnectionsQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    #[serde(rename = "unackedOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unacked_only: Option<bool>,
    /// Minimum strength (inclusive)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strength: Option<ListConnectionsWorkspaceRequestStrength>,
}

impl ListConnectionsQueryRequest {
    pub fn builder() -> ListConnectionsQueryRequestBuilder {
        <ListConnectionsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListConnectionsQueryRequestBuilder {
    limit: Option<i64>,
    offset: Option<i64>,
    unacked_only: Option<bool>,
    strength: Option<ListConnectionsWorkspaceRequestStrength>,
}

impl ListConnectionsQueryRequestBuilder {
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn offset(mut self, value: i64) -> Self {
        self.offset = Some(value);
        self
    }

    pub fn unacked_only(mut self, value: bool) -> Self {
        self.unacked_only = Some(value);
        self
    }

    pub fn strength(mut self, value: ListConnectionsWorkspaceRequestStrength) -> Self {
        self.strength = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListConnectionsQueryRequest`].
    pub fn build(self) -> Result<ListConnectionsQueryRequest, BuildError> {
        Ok(ListConnectionsQueryRequest {
            limit: self.limit,
            offset: self.offset,
            unacked_only: self.unacked_only,
            strength: self.strength,
        })
    }
}

