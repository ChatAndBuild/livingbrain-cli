pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SourceList {
    /// The ingest sources on this page of results.
    #[serde(default)]
    pub items: Vec<IngestSource>,
    /// Total sources matching the filter (ignores pagination)
    #[serde(default)]
    pub total: i64,
    /// Applied page size
    #[serde(default)]
    pub limit: i64,
    /// Applied offset
    #[serde(default)]
    pub offset: i64,
}

impl SourceList {
    pub fn builder() -> SourceListBuilder {
        <SourceListBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SourceListBuilder {
    items: Option<Vec<IngestSource>>,
    total: Option<i64>,
    limit: Option<i64>,
    offset: Option<i64>,
}

impl SourceListBuilder {
    pub fn items(mut self, value: Vec<IngestSource>) -> Self {
        self.items = Some(value);
        self
    }

    pub fn total(mut self, value: i64) -> Self {
        self.total = Some(value);
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn offset(mut self, value: i64) -> Self {
        self.offset = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SourceList`].
    /// This method will fail if any of the following fields are not set:
    /// - [`items`](SourceListBuilder::items)
    /// - [`total`](SourceListBuilder::total)
    /// - [`limit`](SourceListBuilder::limit)
    /// - [`offset`](SourceListBuilder::offset)
    pub fn build(self) -> Result<SourceList, BuildError> {
        Ok(SourceList {
            items: self.items.ok_or_else(|| BuildError::missing_field("items"))?,
            total: self.total.ok_or_else(|| BuildError::missing_field("total"))?,
            limit: self.limit.ok_or_else(|| BuildError::missing_field("limit"))?,
            offset: self.offset.ok_or_else(|| BuildError::missing_field("offset"))?,
        })
    }
}
