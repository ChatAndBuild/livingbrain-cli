use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;
use std::collections::HashMap;

pub struct BrainsClient {
    pub http_client: HttpClient,
}

impl BrainsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// List every brain owned by the calling subject, most recently updated first.
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
    ///     client.brains.list(None).await;
    /// }
    /// ```
    pub async fn list(&self, options: Option<RequestOptions>) -> Result<Vec<Brain>, ApiError> {
        self.http_client
            .execute_request(Method::GET, "v1/brains", None, None, options)
            .await
    }

    /// Create a new brain for the calling subject. A brain is an isolated knowledge vault: every capture, page, and connection belongs to exactly one brain. The `x-subject-id` header determines who owns it.
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
    ///         .brains
    ///         .create(
    ///             &CreateBrainRequest {
    ///                 name: "Customer Memory".to_string(),
    ///                 description: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &CreateBrainRequest,
        options: Option<RequestOptions>,
    ) -> Result<Brain, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/brains",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Aggregate counts for many brains in one round trip. Pass a comma-separated `ids` list; brains the caller cannot access are omitted rather than erroring.
    ///
    /// # Arguments
    ///
    /// * `ids` - Comma-separated brain ids, e.g. `ids=a,b,c`. Unowned ids are omitted.
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
    ///         .brains
    ///         .bulk_stats(
    ///             &BulkStatsQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn bulk_stats(
        &self,
        request: &BulkStatsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<HashMap<String, BrainStats>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/brains/stats",
                None,
                QueryBuilder::new()
                    .string("ids", request.ids.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Retrieve a single brain by ID, including its name, description, and brief settings.
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
    ///     client.brains.get(&"brainId".to_string(), None).await;
    /// }
    /// ```
    pub async fn get(
        &self,
        brain_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<Brain, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/brains/{}", brain_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Soft-delete a brain and disable its active data.
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
    ///     client.brains.remove(&"brainId".to_string(), None).await;
    /// }
    /// ```
    pub async fn remove(
        &self,
        brain_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<DeleteResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v1/brains/{}", brain_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Update a brain's mutable fields, such as its name or description. Omitted fields are left unchanged.
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
    ///         .brains
    ///         .update(
    ///             &"brainId".to_string(),
    ///             &UpdateBrainRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update(
        &self,
        brain_id: &str,
        request: &UpdateBrainRequest,
        options: Option<RequestOptions>,
    ) -> Result<Brain, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("v1/brains/{}", brain_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Aggregate counts for a single brain — pages, sources, connections, and unacknowledged changes. Useful for dashboard headline numbers.
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
    ///     client.brains.stats(&"brainId".to_string(), None).await;
    /// }
    /// ```
    pub async fn stats(
        &self,
        brain_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<BrainStats, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/brains/{}/stats", brain_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Update the daily-brief schedule and delivery settings for a brain, including whether briefs are enabled and what time they are generated.
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
    ///         .brains
    ///         .update_brief_settings(
    ///             &"brainId".to_string(),
    ///             &UpdateBriefSettingsRequest {
    ///                 brief_settings: BriefSettings {
    ///                     enabled: true,
    ///                     hour_local: 7,
    ///                     timezone: "Europe/Helsinki".to_string(),
    ///                     ..Default::default()
    ///                 },
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_brief_settings(
        &self,
        brain_id: &str,
        request: &UpdateBriefSettingsRequest,
        options: Option<RequestOptions>,
    ) -> Result<Brain, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("v1/brains/{}/brief-settings", brain_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Generate this brain's daily brief immediately instead of waiting for its scheduled run. Returns the generated brief page's id and slug, or `null` when the last 7 days had no activity to brief.
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
    ///     client.brains.run_brief(&"brainId".to_string(), None).await;
    /// }
    /// ```
    pub async fn run_brief(
        &self,
        brain_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<Option<BriefRunResponse>, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/brains/{}/brief/run", brain_id),
                None,
                None,
                options,
            )
            .await
    }
}
