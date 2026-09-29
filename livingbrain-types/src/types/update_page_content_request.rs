pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdatePageContentRequest {
    /// Human-readable title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Short generated summary of the content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    /// New page content. Replaces the existing body and creates a new version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
}

impl UpdatePageContentRequest {
    pub fn builder() -> UpdatePageContentRequestBuilder {
        <UpdatePageContentRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdatePageContentRequestBuilder {
    title: Option<String>,
    summary: Option<String>,
    body: Option<String>,
}

impl UpdatePageContentRequestBuilder {
    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn summary(mut self, value: impl Into<String>) -> Self {
        self.summary = Some(value.into());
        self
    }

    pub fn body(mut self, value: impl Into<String>) -> Self {
        self.body = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdatePageContentRequest`].
    pub fn build(self) -> Result<UpdatePageContentRequest, BuildError> {
        Ok(UpdatePageContentRequest {
            title: self.title,
            summary: self.summary,
            body: self.body,
        })
    }
}

