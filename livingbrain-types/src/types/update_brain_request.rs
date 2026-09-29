pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateBrainRequest {
    /// New name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// New description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl UpdateBrainRequest {
    pub fn builder() -> UpdateBrainRequestBuilder {
        <UpdateBrainRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateBrainRequestBuilder {
    name: Option<String>,
    description: Option<String>,
}

impl UpdateBrainRequestBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateBrainRequest`].
    pub fn build(self) -> Result<UpdateBrainRequest, BuildError> {
        Ok(UpdateBrainRequest {
            name: self.name,
            description: self.description,
        })
    }
}

