pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RenamePageRequest {
    /// Human-readable title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// New URL-safe slug. External references to the old slug will stop resolving.
    #[serde(rename = "newSlug")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_slug: Option<String>,
}

impl RenamePageRequest {
    pub fn builder() -> RenamePageRequestBuilder {
        <RenamePageRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RenamePageRequestBuilder {
    title: Option<String>,
    new_slug: Option<String>,
}

impl RenamePageRequestBuilder {
    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn new_slug(mut self, value: impl Into<String>) -> Self {
        self.new_slug = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RenamePageRequest`].
    pub fn build(self) -> Result<RenamePageRequest, BuildError> {
        Ok(RenamePageRequest {
            title: self.title,
            new_slug: self.new_slug,
        })
    }
}

