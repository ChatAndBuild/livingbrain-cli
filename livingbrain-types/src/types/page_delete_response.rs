pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PageDeleteResponse {
    /// Whether the page was removed.
    #[serde(default)]
    pub deleted: bool,
    /// Deleted page id
    #[serde(rename = "pageId")]
    #[serde(default)]
    pub page_id: String,
    /// Deleted page slug
    #[serde(default)]
    pub slug: String,
}

impl PageDeleteResponse {
    pub fn builder() -> PageDeleteResponseBuilder {
        <PageDeleteResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PageDeleteResponseBuilder {
    deleted: Option<bool>,
    page_id: Option<String>,
    slug: Option<String>,
}

impl PageDeleteResponseBuilder {
    pub fn deleted(mut self, value: bool) -> Self {
        self.deleted = Some(value);
        self
    }

    pub fn page_id(mut self, value: impl Into<String>) -> Self {
        self.page_id = Some(value.into());
        self
    }

    pub fn slug(mut self, value: impl Into<String>) -> Self {
        self.slug = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PageDeleteResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`deleted`](PageDeleteResponseBuilder::deleted)
    /// - [`page_id`](PageDeleteResponseBuilder::page_id)
    /// - [`slug`](PageDeleteResponseBuilder::slug)
    pub fn build(self) -> Result<PageDeleteResponse, BuildError> {
        Ok(PageDeleteResponse {
            deleted: self.deleted.ok_or_else(|| BuildError::missing_field("deleted"))?,
            page_id: self.page_id.ok_or_else(|| BuildError::missing_field("page_id"))?,
            slug: self.slug.ok_or_else(|| BuildError::missing_field("slug"))?,
        })
    }
}
