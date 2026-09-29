pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SlackDisconnectResponse {
    /// True when the binding or installation was removed.
    #[serde(default)]
    pub disconnected: bool,
}

impl SlackDisconnectResponse {
    pub fn builder() -> SlackDisconnectResponseBuilder {
        <SlackDisconnectResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SlackDisconnectResponseBuilder {
    disconnected: Option<bool>,
}

impl SlackDisconnectResponseBuilder {
    pub fn disconnected(mut self, value: bool) -> Self {
        self.disconnected = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SlackDisconnectResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`disconnected`](SlackDisconnectResponseBuilder::disconnected)
    pub fn build(self) -> Result<SlackDisconnectResponse, BuildError> {
        Ok(SlackDisconnectResponse {
            disconnected: self.disconnected.ok_or_else(|| BuildError::missing_field("disconnected"))?,
        })
    }
}
