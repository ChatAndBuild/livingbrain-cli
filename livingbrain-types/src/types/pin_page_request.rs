pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PinPageRequest {
    /// Pin or unpin
    #[serde(default)]
    pub pinned: bool,
}

impl PinPageRequest {
    pub fn builder() -> PinPageRequestBuilder {
        <PinPageRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PinPageRequestBuilder {
    pinned: Option<bool>,
}

impl PinPageRequestBuilder {
    pub fn pinned(mut self, value: bool) -> Self {
        self.pinned = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PinPageRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`pinned`](PinPageRequestBuilder::pinned)
    pub fn build(self) -> Result<PinPageRequest, BuildError> {
        Ok(PinPageRequest {
            pinned: self.pinned.ok_or_else(|| BuildError::missing_field("pinned"))?,
        })
    }
}

