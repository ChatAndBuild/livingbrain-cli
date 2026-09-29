pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MarkdownExport {
    /// Identifier of the brain this belongs to.
    #[serde(rename = "brainId")]
    #[serde(default)]
    pub brain_id: String,
    /// Single markdown bundle with front-matter per page
    #[serde(default)]
    pub markdown: String,
}

impl MarkdownExport {
    pub fn builder() -> MarkdownExportBuilder {
        <MarkdownExportBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MarkdownExportBuilder {
    brain_id: Option<String>,
    markdown: Option<String>,
}

impl MarkdownExportBuilder {
    pub fn brain_id(mut self, value: impl Into<String>) -> Self {
        self.brain_id = Some(value.into());
        self
    }

    pub fn markdown(mut self, value: impl Into<String>) -> Self {
        self.markdown = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`MarkdownExport`].
    /// This method will fail if any of the following fields are not set:
    /// - [`brain_id`](MarkdownExportBuilder::brain_id)
    /// - [`markdown`](MarkdownExportBuilder::markdown)
    pub fn build(self) -> Result<MarkdownExport, BuildError> {
        Ok(MarkdownExport {
            brain_id: self.brain_id.ok_or_else(|| BuildError::missing_field("brain_id"))?,
            markdown: self.markdown.ok_or_else(|| BuildError::missing_field("markdown"))?,
        })
    }
}
