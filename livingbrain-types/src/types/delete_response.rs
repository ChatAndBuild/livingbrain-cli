pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteResponse {
    /// Id of the deleted resource
    #[serde(default)]
    pub id: String,
}

impl DeleteResponse {
    pub fn builder() -> DeleteResponseBuilder {
        <DeleteResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteResponseBuilder {
    id: Option<String>,
}

impl DeleteResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeleteResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](DeleteResponseBuilder::id)
    pub fn build(self) -> Result<DeleteResponse, BuildError> {
        Ok(DeleteResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
