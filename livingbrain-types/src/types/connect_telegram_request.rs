pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConnectTelegramRequest {
    /// BotFather bot token to connect to this brain
    #[serde(rename = "botToken")]
    #[serde(default)]
    pub bot_token: String,
}

impl ConnectTelegramRequest {
    pub fn builder() -> ConnectTelegramRequestBuilder {
        <ConnectTelegramRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConnectTelegramRequestBuilder {
    bot_token: Option<String>,
}

impl ConnectTelegramRequestBuilder {
    pub fn bot_token(mut self, value: impl Into<String>) -> Self {
        self.bot_token = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ConnectTelegramRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`bot_token`](ConnectTelegramRequestBuilder::bot_token)
    pub fn build(self) -> Result<ConnectTelegramRequest, BuildError> {
        Ok(ConnectTelegramRequest {
            bot_token: self.bot_token.ok_or_else(|| BuildError::missing_field("bot_token"))?,
        })
    }
}

