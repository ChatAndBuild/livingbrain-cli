pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PageList {
    /// The pages on this page of results.
    #[serde(default)]
    pub items: Vec<Page>,
    /// Total pages matching the filter (ignores pagination)
    #[serde(default)]
    pub total: i64,
    /// Applied page size
    #[serde(default)]
    pub limit: i64,
    /// Applied offset
    #[serde(default)]
    pub offset: i64,
}

impl PageList {
    pub fn builder() -> PageListBuilder {
        <PageListBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PageListBuilder {
    items: Option<Vec<Page>>,
    total: Option<i64>,
    limit: Option<i64>,
    offset: Option<i64>,
}

impl PageListBuilder {
    pub fn items(mut self, value: Vec<Page>) -> Self {
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

    /// Consumes the builder and constructs a [`PageList`].
    /// This method will fail if any of the following fields are not set:
    /// - [`items`](PageListBuilder::items)
    /// - [`total`](PageListBuilder::total)
    /// - [`limit`](PageListBuilder::limit)
    /// - [`offset`](PageListBuilder::offset)
    pub fn build(self) -> Result<PageList, BuildError> {
        Ok(PageList {
            items: self.items.ok_or_else(|| BuildError::missing_field("items"))?,
            total: self.total.ok_or_else(|| BuildError::missing_field("total"))?,
            limit: self.limit.ok_or_else(|| BuildError::missing_field("limit"))?,
            offset: self.offset.ok_or_else(|| BuildError::missing_field("offset"))?,
        })
    }
}
