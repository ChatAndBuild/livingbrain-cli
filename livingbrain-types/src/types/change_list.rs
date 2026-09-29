pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ChangeList {
    /// The change events on this page of results.
    #[serde(default)]
    pub items: Vec<ChangeEvent>,
    /// Total events matching the filter (ignores pagination)
    #[serde(default)]
    pub total: i64,
    /// Applied page size
    #[serde(default)]
    pub limit: i64,
    /// Applied offset
    #[serde(default)]
    pub offset: i64,
}

impl ChangeList {
    pub fn builder() -> ChangeListBuilder {
        <ChangeListBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ChangeListBuilder {
    items: Option<Vec<ChangeEvent>>,
    total: Option<i64>,
    limit: Option<i64>,
    offset: Option<i64>,
}

impl ChangeListBuilder {
    pub fn items(mut self, value: Vec<ChangeEvent>) -> Self {
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

    /// Consumes the builder and constructs a [`ChangeList`].
    /// This method will fail if any of the following fields are not set:
    /// - [`items`](ChangeListBuilder::items)
    /// - [`total`](ChangeListBuilder::total)
    /// - [`limit`](ChangeListBuilder::limit)
    /// - [`offset`](ChangeListBuilder::offset)
    pub fn build(self) -> Result<ChangeList, BuildError> {
        Ok(ChangeList {
            items: self.items.ok_or_else(|| BuildError::missing_field("items"))?,
            total: self.total.ok_or_else(|| BuildError::missing_field("total"))?,
            limit: self.limit.ok_or_else(|| BuildError::missing_field("limit"))?,
            offset: self.offset.ok_or_else(|| BuildError::missing_field("offset"))?,
        })
    }
}
