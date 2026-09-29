pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SearchResult {
    /// Page id
    #[serde(rename = "pageId")]
    #[serde(default)]
    pub page_id: String,
    /// URL-safe identifier, unique within the brain.
    #[serde(default)]
    pub slug: String,
    /// Human-readable title.
    #[serde(default)]
    pub title: String,
    /// Summary of the matched page.
    #[serde(default)]
    pub summary: String,
    /// What kind of knowledge this page holds.
    #[serde(rename = "pageType")]
    pub page_type: SearchResultPageType,
    /// Lifecycle state. Deprecated and archived items are hidden from listings by default.
    pub status: SearchResultStatus,
    /// Similarity score (0-1)
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub similarity: f64,
}

impl SearchResult {
    pub fn builder() -> SearchResultBuilder {
        <SearchResultBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SearchResultBuilder {
    page_id: Option<String>,
    slug: Option<String>,
    title: Option<String>,
    summary: Option<String>,
    page_type: Option<SearchResultPageType>,
    status: Option<SearchResultStatus>,
    similarity: Option<f64>,
}

impl SearchResultBuilder {
    pub fn page_id(mut self, value: impl Into<String>) -> Self {
        self.page_id = Some(value.into());
        self
    }

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

    pub fn page_type(mut self, value: SearchResultPageType) -> Self {
        self.page_type = Some(value);
        self
    }

    pub fn status(mut self, value: SearchResultStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn similarity(mut self, value: f64) -> Self {
        self.similarity = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SearchResult`].
    /// This method will fail if any of the following fields are not set:
    /// - [`page_id`](SearchResultBuilder::page_id)
    /// - [`slug`](SearchResultBuilder::slug)
    /// - [`title`](SearchResultBuilder::title)
    /// - [`summary`](SearchResultBuilder::summary)
    /// - [`page_type`](SearchResultBuilder::page_type)
    /// - [`status`](SearchResultBuilder::status)
    /// - [`similarity`](SearchResultBuilder::similarity)
    pub fn build(self) -> Result<SearchResult, BuildError> {
        Ok(SearchResult {
            page_id: self.page_id.ok_or_else(|| BuildError::missing_field("page_id"))?,
            slug: self.slug.ok_or_else(|| BuildError::missing_field("slug"))?,
            title: self.title.ok_or_else(|| BuildError::missing_field("title"))?,
            summary: self.summary.ok_or_else(|| BuildError::missing_field("summary"))?,
            page_type: self.page_type.ok_or_else(|| BuildError::missing_field("page_type"))?,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            similarity: self.similarity.ok_or_else(|| BuildError::missing_field("similarity"))?,
        })
    }
}
