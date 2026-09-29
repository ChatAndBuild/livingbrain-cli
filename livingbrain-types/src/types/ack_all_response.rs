pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AckAllResponse {
    /// Number of events acknowledged
    #[serde(default)]
    pub acked: i64,
}

impl AckAllResponse {
    pub fn builder() -> AckAllResponseBuilder {
        <AckAllResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AckAllResponseBuilder {
    acked: Option<i64>,
}

impl AckAllResponseBuilder {
    pub fn acked(mut self, value: i64) -> Self {
        self.acked = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AckAllResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`acked`](AckAllResponseBuilder::acked)
    pub fn build(self) -> Result<AckAllResponse, BuildError> {
        Ok(AckAllResponse {
            acked: self.acked.ok_or_else(|| BuildError::missing_field("acked"))?,
        })
    }
}
