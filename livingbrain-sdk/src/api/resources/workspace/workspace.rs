use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use crate::{AsyncPaginator, PaginationResult};
use reqwest::Method;
use std::collections::HashMap;

pub struct WorkspaceClient {
    pub http_client: HttpClient,
}

impl WorkspaceClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Search pages using combined semantic and keyword matching. Results are ranked by relevance and include a snippet showing why each page matched.
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
    ///         .workspace
    ///         .search_pages(
    ///             &"brainId".to_string(),
    ///             &SearchRequest {
    ///                 query: "what does the customer care about?".to_string(),
    ///                 top_k: None,
    ///                 min_similarity: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn search_pages(
        &self,
        brain_id: &str,
        request: &SearchRequest,
        options: Option<RequestOptions>,
    ) -> Result<Vec<SearchResult>, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/brains/{}/search", brain_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Return a compact index of the brain's top pages, formatted for injection into an LLM prompt. This is the intended way to give an agent working context without sending the whole brain.
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
    ///         .workspace
    ///         .index(
    ///             &"brainId".to_string(),
    ///             &IndexQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn index(
        &self,
        brain_id: &str,
        request: &IndexQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<Vec<IndexEntry>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/brains/{}/index", brain_id),
                None,
                QueryBuilder::new()
                    .int("limit", request.limit.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Return a renderable subgraph of nodes and edges for visualizing how the brain's pages relate. Use `nodeLimit` to keep the payload small enough to draw.
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
    ///         .workspace
    ///         .graph(
    ///             &"brainId".to_string(),
    ///             &GraphQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn graph(
        &self,
        brain_id: &str,
        request: &GraphQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<Graph, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/brains/{}/graph", brain_id),
                None,
                QueryBuilder::new()
                    .int("nodeLimit", request.node_limit.clone())
                    .build(),
                options,
            )
            .await
    }

    /// List pages in a brain with offset pagination. Filter by bucket or status, and optionally include deprecated pages that are normally hidden.
    ///
    /// # Arguments
    ///
    /// * `brain_id` - Brain id
    /// * `include_archived` - Include archived pages in the results. Archived pages are hidden from lists and search by default.
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
    ///         .workspace
    ///         .list_pages(
    ///             &"brainId".to_string(),
    ///             &ListPagesQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_pages(
        &self,
        brain_id: &str,
        request: &ListPagesQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<PageList, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/brains/{}/pages", brain_id),
                None,
                QueryBuilder::new()
                    .int("limit", request.limit.clone())
                    .int("offset", request.offset.clone())
                    .bool("includeDeprecated", request.include_deprecated.clone())
                    .bool("includeArchived", request.include_archived.clone())
                    .serialize("bucket", request.bucket.clone())
                    .serialize("status", request.status.clone())
                    .build(),
                options,
            )
            .await
    }

    pub async fn list_pages_paginated(
        &self,
        brain_id: &str,
        request: &ListPagesQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AsyncPaginator<serde_json::Value>, ApiError> {
        let http_client = std::sync::Arc::new(self.http_client.clone());
        let base_query_params = QueryBuilder::new()
            .int("limit", request.limit.clone())
            .bool("includeDeprecated", request.include_deprecated.clone())
            .bool("includeArchived", request.include_archived.clone())
            .serialize("bucket", request.bucket.clone())
            .serialize("status", request.status.clone())
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
                            &format!("v1/brains/{}/pages", brain_id_for_async),
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

    /// Return per-bucket page counts for the brain — the fastest way to render bucket navigation with badges.
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
    ///     client.workspace.buckets(&"brainId".to_string(), None).await;
    /// }
    /// ```
    pub async fn buckets(
        &self,
        brain_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<HashMap<String, i64>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/brains/{}/buckets", brain_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Read a single page by slug. This bumps the page's `lastReferencedAt`, which feeds the brain health score — use it when a human or agent actually consumes the content.
    ///
    /// # Arguments
    ///
    /// * `brain_id` - Brain id
    /// * `slug` - Page slug
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
    ///         .workspace
    ///         .get_page(
    ///             &"brainId".to_string(),
    ///             &"slug".to_string(),
    ///             &GetPageQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_page(
        &self,
        brain_id: &str,
        slug: &str,
        request: &GetPageQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<Page, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/brains/{}/pages/{}", brain_id, slug),
                None,
                QueryBuilder::new()
                    .bool("includeDeprecated", request.include_deprecated.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Permanently delete a page and every record that depends on it, including its versions and connections. This cannot be undone.
    ///
    /// # Arguments
    ///
    /// * `brain_id` - Brain id
    /// * `slug` - Page slug
    /// * `permanent` - Explicit irreversible-deletion confirmation
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
    ///         .workspace
    ///         .delete_permanently(
    ///             &"brainId".to_string(),
    ///             &"slug".to_string(),
    ///             &DeletePermanentlyQueryRequest {
    ///                 permanent: DeletePermanentlyWorkspaceRequestPermanent::True,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn delete_permanently(
        &self,
        brain_id: &str,
        slug: &str,
        request: &DeletePermanentlyQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<PageDeleteResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v1/brains/{}/pages/{}", brain_id, slug),
                None,
                QueryBuilder::new()
                    .serialize("permanent", Some(request.permanent.clone()))
                    .build(),
                options,
            )
            .await
    }

    /// List a page's version history, newest first. Every content edit and automated recompile produces a version.
    ///
    /// # Arguments
    ///
    /// * `brain_id` - Brain id
    /// * `slug` - Page slug
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
    ///         .workspace
    ///         .page_versions(&"brainId".to_string(), &"slug".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn page_versions(
        &self,
        brain_id: &str,
        slug: &str,
        options: Option<RequestOptions>,
    ) -> Result<Vec<PageVersion>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/brains/{}/pages/{}/versions", brain_id, slug),
                None,
                None,
                options,
            )
            .await
    }

    /// List the connections attached to a single page, with the related page inlined on each connection.
    ///
    /// # Arguments
    ///
    /// * `brain_id` - Brain id
    /// * `slug` - Page slug
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
    ///         .workspace
    ///         .page_connections(&"brainId".to_string(), &"slug".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn page_connections(
        &self,
        brain_id: &str,
        slug: &str,
        options: Option<RequestOptions>,
    ) -> Result<PageConnections, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/brains/{}/pages/{}/connections", brain_id, slug),
                None,
                None,
                options,
            )
            .await
    }

    /// Manually overwrite a page's content. This creates a new version, so the previous content stays retrievable through the version history.
    ///
    /// # Arguments
    ///
    /// * `brain_id` - Brain id
    /// * `slug` - Page slug
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
    ///         .workspace
    ///         .edit_content(
    ///             &"brainId".to_string(),
    ///             &"slug".to_string(),
    ///             &UpdatePageContentRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn edit_content(
        &self,
        brain_id: &str,
        slug: &str,
        request: &UpdatePageContentRequest,
        options: Option<RequestOptions>,
    ) -> Result<Page, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("v1/brains/{}/pages/{}/content", brain_id, slug),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Move a page into a different bucket. Buckets are the brain's top-level organizational grouping.
    ///
    /// # Arguments
    ///
    /// * `brain_id` - Brain id
    /// * `slug` - Page slug
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
    ///         .workspace
    ///         .move_bucket(
    ///             &"brainId".to_string(),
    ///             &"slug".to_string(),
    ///             &MovePageBucketRequest {
    ///                 bucket: MovePageBucketRequestBucket::Inbox,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn move_bucket(
        &self,
        brain_id: &str,
        slug: &str,
        request: &MovePageBucketRequest,
        options: Option<RequestOptions>,
    ) -> Result<Page, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("v1/brains/{}/pages/{}/bucket", brain_id, slug),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Pin or unpin a page. Pinned pages are prioritized in the prompt index and are exempt from deprecation.
    ///
    /// # Arguments
    ///
    /// * `brain_id` - Brain id
    /// * `slug` - Page slug
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
    ///         .workspace
    ///         .pin(
    ///             &"brainId".to_string(),
    ///             &"slug".to_string(),
    ///             &PinPageRequest { pinned: true },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn pin(
        &self,
        brain_id: &str,
        slug: &str,
        request: &PinPageRequest,
        options: Option<RequestOptions>,
    ) -> Result<Page, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/brains/{}/pages/{}/pin", brain_id, slug),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Rename a page's title and slug. Existing connections follow the page, but any external references to the old slug will break.
    ///
    /// # Arguments
    ///
    /// * `brain_id` - Brain id
    /// * `slug` - Page slug
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
    ///         .workspace
    ///         .rename(
    ///             &"brainId".to_string(),
    ///             &"slug".to_string(),
    ///             &RenamePageRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn rename(
        &self,
        brain_id: &str,
        slug: &str,
        request: &RenamePageRequest,
        options: Option<RequestOptions>,
    ) -> Result<Page, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/brains/{}/pages/{}/rename", brain_id, slug),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Archive a page. Archived pages are hidden from lists and search until restored, but their content, versions, and connections are kept. Use `restore` to bring the page back.
    ///
    /// # Arguments
    ///
    /// * `brain_id` - Brain id
    /// * `slug` - Page slug
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
    ///         .workspace
    ///         .archive(&"brainId".to_string(), &"slug".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn archive(
        &self,
        brain_id: &str,
        slug: &str,
        options: Option<RequestOptions>,
    ) -> Result<Page, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/brains/{}/pages/{}/archive", brain_id, slug),
                None,
                None,
                options,
            )
            .await
    }

    /// Restore an archived page and return it to active status, so it reappears in lists and search.
    ///
    /// # Arguments
    ///
    /// * `brain_id` - Brain id
    /// * `slug` - Page slug
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
    ///         .workspace
    ///         .restore(&"brainId".to_string(), &"slug".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn restore(
        &self,
        brain_id: &str,
        slug: &str,
        options: Option<RequestOptions>,
    ) -> Result<Page, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/brains/{}/pages/{}/restore", brain_id, slug),
                None,
                None,
                options,
            )
            .await
    }

    /// Recover a deprecated page and return it to active status.
    ///
    /// # Arguments
    ///
    /// * `brain_id` - Brain id
    /// * `slug` - Page slug
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
    ///         .workspace
    ///         .recover(&"brainId".to_string(), &"slug".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn recover(
        &self,
        brain_id: &str,
        slug: &str,
        options: Option<RequestOptions>,
    ) -> Result<Page, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/brains/{}/pages/{}/recover", brain_id, slug),
                None,
                None,
                options,
            )
            .await
    }

    /// Resolve a contested page. A page becomes contested when newly captured material contradicts what the page already asserts; resolving records which version wins.
    ///
    /// # Arguments
    ///
    /// * `brain_id` - Brain id
    /// * `slug` - Page slug
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
    ///         .workspace
    ///         .resolve(
    ///             &"brainId".to_string(),
    ///             &"slug".to_string(),
    ///             &ResolveContestedRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn resolve(
        &self,
        brain_id: &str,
        slug: &str,
        request: &ResolveContestedRequest,
        options: Option<RequestOptions>,
    ) -> Result<Page, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/brains/{}/pages/{}/resolve", brain_id, slug),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Legacy alias for recovering a deprecated page. Prefer `recover` in new code; this endpoint is retained for backwards compatibility.
    ///
    /// # Arguments
    ///
    /// * `brain_id` - Brain id
    /// * `slug` - Page slug
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
    ///         .workspace
    ///         .reviewed(&"brainId".to_string(), &"slug".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn reviewed(
        &self,
        brain_id: &str,
        slug: &str,
        options: Option<RequestOptions>,
    ) -> Result<Page, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/brains/{}/pages/{}/reviewed", brain_id, slug),
                None,
                None,
                options,
            )
            .await
    }

    /// List change-feed events with offset pagination. This is the primary way to sync an external system: poll with `since`, or filter to `unackedOnly` to build an inbox.
    ///
    /// # Arguments
    ///
    /// * `brain_id` - Brain id
    /// * `since` - ISO timestamp
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
    ///         .workspace
    ///         .changes(
    ///             &"brainId".to_string(),
    ///             &ChangesQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn changes(
        &self,
        brain_id: &str,
        request: &ChangesQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ChangeList, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/brains/{}/changes", brain_id),
                None,
                QueryBuilder::new()
                    .int("limit", request.limit.clone())
                    .int("offset", request.offset.clone())
                    .bool("unackedOnly", request.unacked_only.clone())
                    .string("pageId", request.page_id.clone())
                    .string("since", request.since.clone())
                    .build(),
                options,
            )
            .await
    }

    pub async fn changes_paginated(
        &self,
        brain_id: &str,
        request: &ChangesQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AsyncPaginator<serde_json::Value>, ApiError> {
        let http_client = std::sync::Arc::new(self.http_client.clone());
        let base_query_params = QueryBuilder::new()
            .int("limit", request.limit.clone())
            .bool("unackedOnly", request.unacked_only.clone())
            .string("pageId", request.page_id.clone())
            .string("since", request.since.clone())
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
                            &format!("v1/brains/{}/changes", brain_id_for_async),
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

    /// Acknowledge a single change event, removing it from unacknowledged views.
    ///
    /// # Arguments
    ///
    /// * `brain_id` - Brain id
    /// * `change_id` - Change event id
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
    ///         .workspace
    ///         .ack_change(&"brainId".to_string(), &"changeId".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn ack_change(
        &self,
        brain_id: &str,
        change_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ChangeEvent, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/brains/{}/changes/{}/ack", brain_id, change_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Acknowledge every outstanding change event in one call, and return how many were cleared.
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
    ///     client.workspace.ack_all(&"brainId".to_string(), None).await;
    /// }
    /// ```
    pub async fn ack_all(
        &self,
        brain_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<AckAllResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/brains/{}/changes/ack-all", brain_id),
                None,
                None,
                options,
            )
            .await
    }

    /// List connections across the whole brain with offset pagination. Filter by strength, or restrict to connections the user has not yet dismissed.
    ///
    /// # Arguments
    ///
    /// * `brain_id` - Brain id
    /// * `strength` - Minimum strength (inclusive)
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
    ///         .workspace
    ///         .list_connections(
    ///             &"brainId".to_string(),
    ///             &ListConnectionsQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_connections(
        &self,
        brain_id: &str,
        request: &ListConnectionsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ConnectionList, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/brains/{}/connections", brain_id),
                None,
                QueryBuilder::new()
                    .int("limit", request.limit.clone())
                    .int("offset", request.offset.clone())
                    .bool("unackedOnly", request.unacked_only.clone())
                    .serialize("strength", request.strength.clone())
                    .build(),
                options,
            )
            .await
    }

    pub async fn list_connections_paginated(
        &self,
        brain_id: &str,
        request: &ListConnectionsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AsyncPaginator<serde_json::Value>, ApiError> {
        let http_client = std::sync::Arc::new(self.http_client.clone());
        let base_query_params = QueryBuilder::new()
            .int("limit", request.limit.clone())
            .bool("unackedOnly", request.unacked_only.clone())
            .serialize("strength", request.strength.clone())
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
                            &format!("v1/brains/{}/connections", brain_id_for_async),
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

    /// Dismiss a connection so it stops appearing in unacknowledged views. The connection itself is preserved.
    ///
    /// # Arguments
    ///
    /// * `brain_id` - Brain id
    /// * `connection_id` - Connection id
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
    ///         .workspace
    ///         .ack_connection(&"brainId".to_string(), &"connectionId".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn ack_connection(
        &self,
        brain_id: &str,
        connection_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<Connection, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/brains/{}/connections/{}/ack", brain_id, connection_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Return the brain's health score with its contributing factors and metrics. The score reflects coverage, freshness, and connectedness.
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
    ///         .workspace
    ///         .get_score(&"brainId".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_score(
        &self,
        brain_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<BrainScore, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/brains/{}/score", brain_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Return the score trend over a time range, including the delta versus the start of the period.
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
    ///         .workspace
    ///         .score_history(
    ///             &"brainId".to_string(),
    ///             &ScoreHistoryQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn score_history(
        &self,
        brain_id: &str,
        request: &ScoreHistoryQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ScoreHistory, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/brains/{}/score/history", brain_id),
                None,
                QueryBuilder::new()
                    .serialize("range", request.range.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Return concrete, ranked suggestions for raising the brain's health score, derived from whichever factors are currently weakest.
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
    ///         .workspace
    ///         .score_suggestions(&"brainId".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn score_suggestions(
        &self,
        brain_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ScoreSuggestions, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/brains/{}/score/suggestions", brain_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Return the brain profile — the standing directives and context that shape how the brain interprets and organizes new captures.
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
    ///         .workspace
    ///         .get_profile(&"brainId".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_profile(
        &self,
        brain_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<Option<BrainProfile>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/brains/{}/profile", brain_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Update the brain profile's directives. These take effect on subsequent captures, not retroactively.
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
    ///         .workspace
    ///         .update_profile(
    ///             &"brainId".to_string(),
    ///             &UpdateBrainProfileRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_profile(
        &self,
        brain_id: &str,
        request: &UpdateBrainProfileRequest,
        options: Option<RequestOptions>,
    ) -> Result<BrainProfile, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("v1/brains/{}/profile", brain_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Suggest profile fields inferred from what the brain already contains. Best-effort: returns an empty set when there is not yet enough signal.
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
    ///         .workspace
    ///         .profile_suggestions(&"brainId".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn profile_suggestions(
        &self,
        brain_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ProfileSuggestions, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/brains/{}/profile/suggestions", brain_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Export the entire brain as a single markdown bundle, suitable for backup or for handing to another tool.
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
    ///         .workspace
    ///         .export_markdown(
    ///             &"brainId".to_string(),
    ///             &ExportMarkdownQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn export_markdown(
        &self,
        brain_id: &str,
        request: &ExportMarkdownQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<MarkdownExport, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/brains/{}/export/markdown", brain_id),
                None,
                QueryBuilder::new()
                    .bool("includeDeprecated", request.include_deprecated.clone())
                    .build(),
                options,
            )
            .await
    }
}
