use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct TelegramClient {
    pub http_client: HttpClient,
}

impl TelegramClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Return the current Telegram bot connection status for a brain, including the connected bot's username when one is linked.
    ///
    /// # Arguments
    ///
    /// * `brain_id` - Brain id
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use livingbrain_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         api_key: Some("<value>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = LivingbrainClient::new(config).expect("Failed to build client");
    ///     client.telegram.status(&"brainId".to_string(), None).await;
    /// }
    /// ```
    pub async fn status(
        &self,
        brain_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<Option<TelegramConnection>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/brains/{}/telegram", brain_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Connect a Telegram bot to a brain so messages sent to the bot are captured automatically.
    ///
    /// # Arguments
    ///
    /// * `brain_id` - Brain id
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use livingbrain_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         api_key: Some("<value>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = LivingbrainClient::new(config).expect("Failed to build client");
    ///     client
    ///         .telegram
    ///         .connect(
    ///             &"brainId".to_string(),
    ///             &ConnectTelegramRequest {
    ///                 bot_token: "123456:ABC-DEF...".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn connect(
        &self,
        brain_id: &str,
        request: &ConnectTelegramRequest,
        options: Option<RequestOptions>,
    ) -> Result<TelegramConnection, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/brains/{}/telegram", brain_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Disconnect the Telegram bot from a brain. Already-captured messages are retained.
    ///
    /// # Arguments
    ///
    /// * `brain_id` - Brain id
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use livingbrain_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         api_key: Some("<value>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = LivingbrainClient::new(config).expect("Failed to build client");
    ///     client
    ///         .telegram
    ///         .disconnect(&"brainId".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn disconnect(
        &self,
        brain_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<TelegramDisconnectResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v1/brains/{}/telegram", brain_id),
                None,
                None,
                options,
            )
            .await
    }
}
