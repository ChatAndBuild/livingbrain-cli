pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TelegramConnection {
    /// Connection row id
    #[serde(default)]
    pub id: String,
    /// Owning brain id
    #[serde(rename = "brainId")]
    #[serde(default)]
    pub brain_id: String,
    /// Bot username (without @)
    #[serde(rename = "botUsername")]
    #[serde(default)]
    pub bot_username: String,
    /// Display name of the connected Telegram bot.
    #[serde(rename = "botFirstName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_first_name: Option<String>,
    /// Lifecycle state. Deprecated and archived items are hidden from listings by default.
    pub status: TelegramConnectionStatus,
    /// ISO 8601 timestamp of when the bot was linked.
    #[serde(rename = "connectedAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub connected_at: DateTime<FixedOffset>,
    /// ISO 8601 timestamp of the most recent message captured via Telegram.
    #[serde(rename = "lastCaptureAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub last_capture_at: Option<DateTime<FixedOffset>>,
}

impl TelegramConnection {
    pub fn builder() -> TelegramConnectionBuilder {
        <TelegramConnectionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TelegramConnectionBuilder {
    id: Option<String>,
    brain_id: Option<String>,
    bot_username: Option<String>,
    bot_first_name: Option<String>,
    status: Option<TelegramConnectionStatus>,
    connected_at: Option<DateTime<FixedOffset>>,
    last_capture_at: Option<DateTime<FixedOffset>>,
}

impl TelegramConnectionBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn brain_id(mut self, value: impl Into<String>) -> Self {
        self.brain_id = Some(value.into());
        self
    }

    pub fn bot_username(mut self, value: impl Into<String>) -> Self {
        self.bot_username = Some(value.into());
        self
    }

    pub fn bot_first_name(mut self, value: impl Into<String>) -> Self {
        self.bot_first_name = Some(value.into());
        self
    }

    pub fn status(mut self, value: TelegramConnectionStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn connected_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.connected_at = Some(value);
        self
    }

    pub fn last_capture_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.last_capture_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TelegramConnection`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](TelegramConnectionBuilder::id)
    /// - [`brain_id`](TelegramConnectionBuilder::brain_id)
    /// - [`bot_username`](TelegramConnectionBuilder::bot_username)
    /// - [`status`](TelegramConnectionBuilder::status)
    /// - [`connected_at`](TelegramConnectionBuilder::connected_at)
    pub fn build(self) -> Result<TelegramConnection, BuildError> {
        Ok(TelegramConnection {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            brain_id: self.brain_id.ok_or_else(|| BuildError::missing_field("brain_id"))?,
            bot_username: self.bot_username.ok_or_else(|| BuildError::missing_field("bot_username"))?,
            bot_first_name: self.bot_first_name,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            connected_at: self.connected_at.ok_or_else(|| BuildError::missing_field("connected_at"))?,
            last_capture_at: self.last_capture_at,
        })
    }
}
