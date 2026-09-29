pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ResolveContestedRequest {
    /// Resolution note
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<String>,
}

impl ResolveContestedRequest {
    pub fn builder() -> ResolveContestedRequestBuilder {
        <ResolveContestedRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResolveContestedRequestBuilder {
    resolution: Option<String>,
}

impl ResolveContestedRequestBuilder {
    pub fn resolution(mut self, value: impl Into<String>) -> Self {
        self.resolution = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ResolveContestedRequest`].
    pub fn build(self) -> Result<ResolveContestedRequest, BuildError> {
        Ok(ResolveContestedRequest {
            resolution: self.resolution,
        })
    }
}

