pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for listSources
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListSourcesQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ListSourcesCapturesRequestStatus>,
    /// Comma-separated source ids for a targeted lookup (e.g. resolving a page’s source refs regardless of age)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<String>,
}

impl ListSourcesQueryRequest {
    pub fn builder() -> ListSourcesQueryRequestBuilder {
        <ListSourcesQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListSourcesQueryRequestBuilder {
    limit: Option<i64>,
    offset: Option<i64>,
    status: Option<ListSourcesCapturesRequestStatus>,
    ids: Option<String>,
}

impl ListSourcesQueryRequestBuilder {
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn offset(mut self, value: i64) -> Self {
        self.offset = Some(value);
        self
    }

    pub fn status(mut self, value: ListSourcesCapturesRequestStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn ids(mut self, value: impl Into<String>) -> Self {
        self.ids = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListSourcesQueryRequest`].
    pub fn build(self) -> Result<ListSourcesQueryRequest, BuildError> {
        Ok(ListSourcesQueryRequest {
            limit: self.limit,
            offset: self.offset,
            status: self.status,
            ids: self.ids,
        })
    }
}

