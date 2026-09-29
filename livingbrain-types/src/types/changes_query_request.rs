pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for changes
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ChangesQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    #[serde(rename = "unackedOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unacked_only: Option<bool>,
    #[serde(rename = "pageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_id: Option<String>,
    /// ISO timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since: Option<String>,
}

impl ChangesQueryRequest {
    pub fn builder() -> ChangesQueryRequestBuilder {
        <ChangesQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ChangesQueryRequestBuilder {
    limit: Option<i64>,
    offset: Option<i64>,
    unacked_only: Option<bool>,
    page_id: Option<String>,
    since: Option<String>,
}

impl ChangesQueryRequestBuilder {
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

    pub fn page_id(mut self, value: impl Into<String>) -> Self {
        self.page_id = Some(value.into());
        self
    }

    pub fn since(mut self, value: impl Into<String>) -> Self {
        self.since = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ChangesQueryRequest`].
    pub fn build(self) -> Result<ChangesQueryRequest, BuildError> {
        Ok(ChangesQueryRequest {
            limit: self.limit,
            offset: self.offset,
            unacked_only: self.unacked_only,
            page_id: self.page_id,
            since: self.since,
        })
    }
}

