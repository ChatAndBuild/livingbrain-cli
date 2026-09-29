pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for deletePermanently
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DeletePermanentlyQueryRequest {
    /// Explicit irreversible-deletion confirmation
    pub permanent: DeletePermanentlyWorkspaceRequestPermanent,
}

impl DeletePermanentlyQueryRequest {
    pub fn builder() -> DeletePermanentlyQueryRequestBuilder {
        <DeletePermanentlyQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeletePermanentlyQueryRequestBuilder {
    permanent: Option<DeletePermanentlyWorkspaceRequestPermanent>,
}

impl DeletePermanentlyQueryRequestBuilder {
    pub fn permanent(mut self, value: DeletePermanentlyWorkspaceRequestPermanent) -> Self {
        self.permanent = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DeletePermanentlyQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`permanent`](DeletePermanentlyQueryRequestBuilder::permanent)
    pub fn build(self) -> Result<DeletePermanentlyQueryRequest, BuildError> {
        Ok(DeletePermanentlyQueryRequest {
            permanent: self.permanent.ok_or_else(|| BuildError::missing_field("permanent"))?,
        })
    }
}

