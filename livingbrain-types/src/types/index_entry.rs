pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct IndexEntry {
    /// URL-safe identifier, unique within the brain.
    #[serde(default)]
    pub slug: String,
    /// Human-readable title.
    #[serde(default)]
    pub title: String,
    /// Summary truncated to 160 chars
    #[serde(default)]
    pub summary: String,
    /// What kind of knowledge this page holds.
    #[serde(rename = "pageType")]
    pub page_type: IndexEntryPageType,
    /// Pinned entries are placed first in the prompt index.
    #[serde(default)]
    pub pinned: bool,
    /// Lifecycle state. Deprecated and archived items are hidden from listings by default.
    pub status: IndexEntryStatus,
}

impl IndexEntry {
    pub fn builder() -> IndexEntryBuilder {
        <IndexEntryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct IndexEntryBuilder {
    slug: Option<String>,
    title: Option<String>,
    summary: Option<String>,
    page_type: Option<IndexEntryPageType>,
    pinned: Option<bool>,
    status: Option<IndexEntryStatus>,
}

impl IndexEntryBuilder {
    pub fn slug(mut self, value: impl Into<String>) -> Self {
        self.slug = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn summary(mut self, value: impl Into<String>) -> Self {
        self.summary = Some(value.into());
        self
    }

    pub fn page_type(mut self, value: IndexEntryPageType) -> Self {
        self.page_type = Some(value);
        self
    }

    pub fn pinned(mut self, value: bool) -> Self {
        self.pinned = Some(value);
        self
    }

    pub fn status(mut self, value: IndexEntryStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`IndexEntry`].
    /// This method will fail if any of the following fields are not set:
    /// - [`slug`](IndexEntryBuilder::slug)
    /// - [`title`](IndexEntryBuilder::title)
    /// - [`summary`](IndexEntryBuilder::summary)
    /// - [`page_type`](IndexEntryBuilder::page_type)
    /// - [`pinned`](IndexEntryBuilder::pinned)
    /// - [`status`](IndexEntryBuilder::status)
    pub fn build(self) -> Result<IndexEntry, BuildError> {
        Ok(IndexEntry {
            slug: self.slug.ok_or_else(|| BuildError::missing_field("slug"))?,
            title: self.title.ok_or_else(|| BuildError::missing_field("title"))?,
            summary: self.summary.ok_or_else(|| BuildError::missing_field("summary"))?,
            page_type: self.page_type.ok_or_else(|| BuildError::missing_field("page_type"))?,
            pinned: self.pinned.ok_or_else(|| BuildError::missing_field("pinned"))?,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
