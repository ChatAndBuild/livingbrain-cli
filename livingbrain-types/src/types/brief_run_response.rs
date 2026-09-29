pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BriefRunResponse {
    /// Id of the generated brief page
    #[serde(rename = "pageId")]
    #[serde(default)]
    pub page_id: String,
    /// Slug of the generated brief page
    #[serde(default)]
    pub slug: String,
}

impl BriefRunResponse {
    pub fn builder() -> BriefRunResponseBuilder {
        <BriefRunResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BriefRunResponseBuilder {
    page_id: Option<String>,
    slug: Option<String>,
}

impl BriefRunResponseBuilder {
    pub fn page_id(mut self, value: impl Into<String>) -> Self {
        self.page_id = Some(value.into());
        self
    }

    pub fn slug(mut self, value: impl Into<String>) -> Self {
        self.slug = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BriefRunResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`page_id`](BriefRunResponseBuilder::page_id)
    /// - [`slug`](BriefRunResponseBuilder::slug)
    pub fn build(self) -> Result<BriefRunResponse, BuildError> {
        Ok(BriefRunResponse {
            page_id: self.page_id.ok_or_else(|| BuildError::missing_field("page_id"))?,
            slug: self.slug.ok_or_else(|| BuildError::missing_field("slug"))?,
        })
    }
}
