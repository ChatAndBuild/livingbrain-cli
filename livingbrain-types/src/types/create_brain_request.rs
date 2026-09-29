pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateBrainRequest {
    /// User-facing name of the brain
    #[serde(default)]
    pub name: String,
    /// Optional description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl CreateBrainRequest {
    pub fn builder() -> CreateBrainRequestBuilder {
        <CreateBrainRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateBrainRequestBuilder {
    name: Option<String>,
    description: Option<String>,
}

impl CreateBrainRequestBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateBrainRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](CreateBrainRequestBuilder::name)
    pub fn build(self) -> Result<CreateBrainRequest, BuildError> {
        Ok(CreateBrainRequest {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            description: self.description,
        })
    }
}

