pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for listPages
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListPagesQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    #[serde(rename = "includeDeprecated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_deprecated: Option<bool>,
    /// Include archived pages in the results. Archived pages are hidden from lists and search by default.
    #[serde(rename = "includeArchived")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_archived: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket: Option<ListPagesWorkspaceRequestBucket>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ListPagesWorkspaceRequestStatus>,
}

impl ListPagesQueryRequest {
    pub fn builder() -> ListPagesQueryRequestBuilder {
        <ListPagesQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListPagesQueryRequestBuilder {
    limit: Option<i64>,
    offset: Option<i64>,
    include_deprecated: Option<bool>,
    include_archived: Option<bool>,
    bucket: Option<ListPagesWorkspaceRequestBucket>,
    status: Option<ListPagesWorkspaceRequestStatus>,
}

impl ListPagesQueryRequestBuilder {
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn offset(mut self, value: i64) -> Self {
        self.offset = Some(value);
        self
    }

    pub fn include_deprecated(mut self, value: bool) -> Self {
        self.include_deprecated = Some(value);
        self
    }

    pub fn include_archived(mut self, value: bool) -> Self {
        self.include_archived = Some(value);
        self
    }

    pub fn bucket(mut self, value: ListPagesWorkspaceRequestBucket) -> Self {
        self.bucket = Some(value);
        self
    }

    pub fn status(mut self, value: ListPagesWorkspaceRequestStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListPagesQueryRequest`].
    pub fn build(self) -> Result<ListPagesQueryRequest, BuildError> {
        Ok(ListPagesQueryRequest {
            limit: self.limit,
            offset: self.offset,
            include_deprecated: self.include_deprecated,
            include_archived: self.include_archived,
            bucket: self.bucket,
            status: self.status,
        })
    }
}

