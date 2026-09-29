pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for getPage
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetPageQueryRequest {
    #[serde(rename = "includeDeprecated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_deprecated: Option<bool>,
}

impl GetPageQueryRequest {
    pub fn builder() -> GetPageQueryRequestBuilder {
        <GetPageQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetPageQueryRequestBuilder {
    include_deprecated: Option<bool>,
}

impl GetPageQueryRequestBuilder {
    pub fn include_deprecated(mut self, value: bool) -> Self {
        self.include_deprecated = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetPageQueryRequest`].
    pub fn build(self) -> Result<GetPageQueryRequest, BuildError> {
        Ok(GetPageQueryRequest {
            include_deprecated: self.include_deprecated,
        })
    }
}

