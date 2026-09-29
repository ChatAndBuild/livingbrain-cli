pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for exportMarkdown
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ExportMarkdownQueryRequest {
    #[serde(rename = "includeDeprecated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_deprecated: Option<bool>,
}

impl ExportMarkdownQueryRequest {
    pub fn builder() -> ExportMarkdownQueryRequestBuilder {
        <ExportMarkdownQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExportMarkdownQueryRequestBuilder {
    include_deprecated: Option<bool>,
}

impl ExportMarkdownQueryRequestBuilder {
    pub fn include_deprecated(mut self, value: bool) -> Self {
        self.include_deprecated = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ExportMarkdownQueryRequest`].
    pub fn build(self) -> Result<ExportMarkdownQueryRequest, BuildError> {
        Ok(ExportMarkdownQueryRequest {
            include_deprecated: self.include_deprecated,
        })
    }
}

