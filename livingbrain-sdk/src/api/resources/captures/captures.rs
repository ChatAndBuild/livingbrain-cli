use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use crate::{AsyncPaginator, PaginationResult};
use reqwest::Method;

pub struct CapturesClient {
    pub http_client: HttpClient,
}

impl CapturesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Push a single item into a brain for ingestion. Accepts notes, URLs, raw text, chat transcripts, and PDF/DOCX/PPTX documents. Capture is asynchronous: this returns an ingest source whose `status` you can poll or receive via webhook.
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
    ///         .captures
    ///         .capture(
    ///             &"brainId".to_string(),
    ///             &CaptureRequest {
    ///                 kind: CaptureRequestKind::Note,
    ///                 content: None,
    ///                 fetch_url: None,
    ///                 mime_type: None,
    ///                 file_name: None,
    ///                 label: None,
    ///                 origin_ref: None,
    ///                 bucket: None,
    ///                 source: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn capture(
        &self,
        brain_id: &str,
        request: &CaptureRequest,
        options: Option<RequestOptions>,
    ) -> Result<IngestSource, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/brains/{}/captures", brain_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Push many captures in a single request. Each item is processed independently, so a malformed entry fails on its own without rejecting the whole batch.
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
    ///         .captures
    ///         .capture_batch(
    ///             &"brainId".to_string(),
    ///             &CaptureBatchRequest {
    ///                 items: vec![
    ///                     CaptureRequest {
    ///                         kind: CaptureRequestKind::Note,
    ///                         content: Some("First note".to_string()),
    ///                         fetch_url: None,
    ///                         mime_type: None,
    ///                         file_name: None,
    ///                         label: None,
    ///                         origin_ref: None,
    ///                         bucket: None,
    ///                         source: None,
    ///                     },
    ///                     CaptureRequest {
    ///                         kind: CaptureRequestKind::Note,
    ///                         content: Some("Second note".to_string()),
    ///                         fetch_url: None,
    ///                         mime_type: None,
    ///                         file_name: None,
    ///                         label: None,
    ///                         origin_ref: None,
    ///                         bucket: None,
    ///                         source: None,
    ///                     },
    ///                 ],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn capture_batch(
        &self,
        brain_id: &str,
        request: &CaptureBatchRequest,
        options: Option<RequestOptions>,
    ) -> Result<CaptureBatchResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/brains/{}/captures/batch", brain_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Submit a single chat turn for worthiness classification. The service decides whether the turn contains durable knowledge and only captures it if so, which keeps conversational filler out of the brain.
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
    ///         .captures
    ///         .capture_chat_turn(
    ///             &"brainId".to_string(),
    ///             &CaptureChatTurnRequest {
    ///                 text: "I just closed the Series A, focus shifts to hiring engineers.".to_string(),
    ///                 sender: CaptureChatTurnRequestSender::User,
    ///                 agent_name: None,
    ///                 origin_ref: None,
    ///                 source: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn capture_chat_turn(
        &self,
        brain_id: &str,
        request: &CaptureChatTurnRequest,
        options: Option<RequestOptions>,
    ) -> Result<CaptureChatTurnResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/brains/{}/captures/chat-turn", brain_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// List ingest sources for a brain, optionally filtered by processing status. Use this to surface an ingestion queue or to find sources that failed to compile.
    ///
    /// # Arguments
    ///
    /// * `brain_id` - Brain id
    /// * `ids` - Comma-separated source ids for a targeted lookup (e.g. resolving a page’s source refs regardless of age)
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
    ///         .captures
    ///         .list_sources(
    ///             &"brainId".to_string(),
    ///             &ListSourcesQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_sources(
        &self,
        brain_id: &str,
        request: &ListSourcesQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<SourceList, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/brains/{}/sources", brain_id),
                None,
                QueryBuilder::new()
                    .int("limit", request.limit.clone())
                    .int("offset", request.offset.clone())
                    .serialize("status", request.status.clone())
                    .string("ids", request.ids.clone())
                    .build(),
                options,
            )
            .await
    }

    pub async fn list_sources_paginated(
        &self,
        brain_id: &str,
        request: &ListSourcesQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AsyncPaginator<serde_json::Value>, ApiError> {
        let http_client = std::sync::Arc::new(self.http_client.clone());
        let base_query_params = QueryBuilder::new()
            .int("limit", request.limit.clone())
            .serialize("status", request.status.clone())
            .string("ids", request.ids.clone())
            .build();
        let options_clone = options.clone();
        let brain_id_clone = brain_id.to_string();

        AsyncPaginator::new(
            http_client,
            move |client, page_token| {
                let mut query_params: Vec<(String, String)> =
                    base_query_params.clone().unwrap_or_default();

                // Use page_token as offset/page number (start from 0 if None)
                let current_page = page_token.unwrap_or_else(|| "0".to_string());
                query_params.push(("offset".to_string(), current_page.clone()));
                let page_size: Option<i64> = query_params
                    .iter()
                    .find(|(name, _)| name == "limit")
                    .and_then(|(_, value)| value.parse::<i64>().ok());

                let options_for_request = options_clone.clone();

                // Clone captured variables to move into the async block
                let brain_id_for_async = brain_id_clone.clone();

                Box::pin(async move {
                    let raw_response = client
                        .execute_request_raw::<serde_json::Value>(
                            Method::GET,
                            &format!("v1/brains/{}/sources", brain_id_for_async),
                            None,
                            Some(query_params),
                            options_for_request,
                        )
                        .await?;
                    let response = raw_response.body;

                    // Extract pagination info from response
                    // Generic field extraction for offset pagination
                    let items: Vec<serde_json::Value> = response
                        .get("items")
                        .and_then(|v| v.as_array())
                        .map(|arr| arr.clone())
                        .unwrap_or_default();

                    let has_next_page = match page_size {
                        Some(size) => items.len() as i64 >= size,
                        None => !items.is_empty(),
                    };
                    let next_cursor: Option<String> = if has_next_page {
                        let current_offset: i64 = current_page.parse().unwrap_or(0);
                        Some((current_offset + items.len() as i64).to_string())
                    } else {
                        None
                    };

                    Ok(PaginationResult {
                        items,
                        next_cursor,
                        has_next_page,
                        response: Some(response),
                        status_code: raw_response.status_code,
                        headers: raw_response.headers,
                    })
                })
            },
            None, // Start with page 0
        )
    }

    /// Retry compiling an ingest source that previously failed. Only sources in a failed state can be retried.
    ///
    /// # Arguments
    ///
    /// * `brain_id` - Brain id
    /// * `source_id` - Ingest source id
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
    ///         .captures
    ///         .retry_source(&"brainId".to_string(), &"sourceId".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn retry_source(
        &self,
        brain_id: &str,
        source_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<SourceRetryResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/brains/{}/sources/{}/retry", brain_id, source_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Delete an ingest source. Pages already derived from the source are retained unless you delete them separately.
    ///
    /// # Arguments
    ///
    /// * `brain_id` - Brain id
    /// * `source_id` - Ingest source id
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
    ///         .captures
    ///         .remove_source(&"brainId".to_string(), &"sourceId".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn remove_source(
        &self,
        brain_id: &str,
        source_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<DeleteResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v1/brains/{}/sources/{}", brain_id, source_id),
                None,
                None,
                options,
            )
            .await
    }
}
