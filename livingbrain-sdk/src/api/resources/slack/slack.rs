use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct SlackClient {
    pub http_client: HttpClient,
}

impl SlackClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Return every Slack workspace installed for a brain along with its channel bindings. Use the installation IDs returned here to list channels or create bindings.
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
    ///     client.slack.status(&"brainId".to_string(), None).await;
    /// }
    /// ```
    pub async fn status(
        &self,
        brain_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<SlackStatus, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/brains/{}/slack", brain_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Create a signed Slack OAuth installation URL for a brain. Redirect the user to `authorizationUrl`; Slack sends them back to your redirect URL with a `code` and `state` to pass to `confirmOauth`. The signed state expires at `expiresAt`.
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
    ///     client.slack.start_oauth(&"brainId".to_string(), None).await;
    /// }
    /// ```
    pub async fn start_oauth(
        &self,
        brain_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<SlackOAuthStartResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/brains/{}/slack/oauth/start", brain_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Complete a Slack installation by exchanging the `code` and `state` Slack returned to your redirect URL. Returns the new installation ID.
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
    ///         .slack
    ///         .confirm_oauth(
    ///             &"brainId".to_string(),
    ///             &ConfirmSlackOAuthRequest {
    ///                 code: "code".to_string(),
    ///                 state: "state".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn confirm_oauth(
        &self,
        brain_id: &str,
        request: &ConfirmSlackOAuthRequest,
        options: Option<RequestOptions>,
    ) -> Result<SlackOAuthConfirmResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/brains/{}/slack/oauth/confirm", brain_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// List the channels visible to the Living Brain bot in a Slack installation. `isMember` tells you whether the bot has already been added to the channel.
    ///
    /// # Arguments
    ///
    /// * `brain_id` - Brain id
    /// * `installation_id` - Slack installation id returned by the status endpoint
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
    ///         .slack
    ///         .channels(
    ///             &"brainId".to_string(),
    ///             &ChannelsQueryRequest {
    ///                 installation_id: "installationId".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn channels(
        &self,
        brain_id: &str,
        request: &ChannelsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<Vec<SlackChannel>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/brains/{}/slack/channels", brain_id),
                None,
                QueryBuilder::new()
                    .string("installationId", request.installation_id.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Connect a Slack channel to a brain so it receives brain notifications. Configure which notifications it receives with `updateBinding`.
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
    ///         .slack
    ///         .bind(
    ///             &"brainId".to_string(),
    ///             &BindSlackChannelRequest {
    ///                 installation_id: "installationId".to_string(),
    ///                 channel_id: "C0123456789".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn bind(
        &self,
        brain_id: &str,
        request: &BindSlackChannelRequest,
        options: Option<RequestOptions>,
    ) -> Result<SlackBinding, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/brains/{}/slack/bindings", brain_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Disconnect a single Slack channel from a brain. The Slack installation and any other bindings are unaffected.
    ///
    /// # Arguments
    ///
    /// * `brain_id` - Brain id
    /// * `binding_id` - Slack channel binding id
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
    ///         .slack
    ///         .unbind(&"brainId".to_string(), &"bindingId".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn unbind(
        &self,
        brain_id: &str,
        binding_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<SlackDisconnectResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v1/brains/{}/slack/bindings/{}", brain_id, binding_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Update which notifications a bound Slack channel receives — daily briefs, new connections, and page changes. Omitted fields are left unchanged.
    ///
    /// # Arguments
    ///
    /// * `brain_id` - Brain id
    /// * `binding_id` - Slack channel binding id
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
    ///         .slack
    ///         .update_binding(
    ///             &"brainId".to_string(),
    ///             &"bindingId".to_string(),
    ///             &UpdateSlackBindingRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_binding(
        &self,
        brain_id: &str,
        binding_id: &str,
        request: &UpdateSlackBindingRequest,
        options: Option<RequestOptions>,
    ) -> Result<SlackBinding, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("v1/brains/{}/slack/bindings/{}", brain_id, binding_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Disconnect a Slack workspace from a brain and remove all of its channel bindings.
    ///
    /// # Arguments
    ///
    /// * `brain_id` - Brain id
    /// * `installation_id` - Slack installation id
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
    ///         .slack
    ///         .disconnect_installation(&"brainId".to_string(), &"installationId".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn disconnect_installation(
        &self,
        brain_id: &str,
        installation_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<SlackDisconnectResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!(
                    "v1/brains/{}/slack/installations/{}",
                    brain_id, installation_id
                ),
                None,
                None,
                options,
            )
            .await
    }
}
