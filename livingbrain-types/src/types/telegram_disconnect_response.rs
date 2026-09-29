pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TelegramDisconnectResponse {
    /// False when no bot was connected
    #[serde(default)]
    pub disconnected: bool,
}

impl TelegramDisconnectResponse {
    pub fn builder() -> TelegramDisconnectResponseBuilder {
        <TelegramDisconnectResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TelegramDisconnectResponseBuilder {
    disconnected: Option<bool>,
}

impl TelegramDisconnectResponseBuilder {
    pub fn disconnected(mut self, value: bool) -> Self {
        self.disconnected = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TelegramDisconnectResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`disconnected`](TelegramDisconnectResponseBuilder::disconnected)
    pub fn build(self) -> Result<TelegramDisconnectResponse, BuildError> {
        Ok(TelegramDisconnectResponse {
            disconnected: self.disconnected.ok_or_else(|| BuildError::missing_field("disconnected"))?,
        })
    }
}
