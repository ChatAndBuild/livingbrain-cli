use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct WebhooksClient {
    pub http_client: HttpClient,
}

impl WebhooksClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// List all webhook subscriptions for the calling subject. Signing secrets are not included.
    ///
    /// # Arguments
    ///
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
    ///     client.webhooks.list(None).await;
    /// }
    /// ```
    pub async fn list(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<Vec<WebhookSubscription>, ApiError> {
        self.http_client
            .execute_request(Method::GET, "v1/webhooks", None, None, options)
            .await
    }

    /// Register a webhook endpoint and receive its signing secret. The secret is returned only on creation — store it immediately, as it cannot be retrieved again.
    ///
    /// # Arguments
    ///
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
    ///         .webhooks
    ///         .create(
    ///             &CreateWebhookRequest {
    ///                 url: "https://your-app.com/webhooks/living-brain".to_string(),
    ///                 events: None,
    ///                 secret: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &CreateWebhookRequest,
        options: Option<RequestOptions>,
    ) -> Result<CreateWebhookResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/webhooks",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Delete a webhook subscription. Deliveries stop immediately.
    ///
    /// # Arguments
    ///
    /// * `id` - Webhook subscription id
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
    ///     client.webhooks.remove(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn remove(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<DeleteResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v1/webhooks/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Update a webhook subscription's URL or subscribed event types. The signing secret is unchanged.
    ///
    /// # Arguments
    ///
    /// * `id` - Webhook subscription id
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
    ///         .webhooks
    ///         .update(
    ///             &"id".to_string(),
    ///             &UpdateWebhookRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update(
        &self,
        id: &str,
        request: &UpdateWebhookRequest,
        options: Option<RequestOptions>,
    ) -> Result<WebhookSubscription, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("v1/webhooks/{}", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
