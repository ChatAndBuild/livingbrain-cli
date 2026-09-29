//! Service clients and API endpoints
//!
//! This module contains client implementations for:
//!
//! - **Brains**
//! - **Captures**
//! - **Workspace**
//! - **Webhooks**
//! - **Telegram**
//! - **Slack**

use crate::{ApiError, ClientConfig};

pub mod brains;
pub mod captures;
pub mod slack;
pub mod telegram;
pub mod webhooks;
pub mod workspace;
pub struct ApiClient {
    pub config: ClientConfig,
    pub brains: BrainsClient,
    pub captures: CapturesClient,
    pub workspace: WorkspaceClient,
    pub webhooks: WebhooksClient,
    pub telegram: TelegramClient,
    pub slack: SlackClient,
}

impl ApiClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            config: config.clone(),
            brains: BrainsClient::new(config.clone())?,
            captures: CapturesClient::new(config.clone())?,
            workspace: WorkspaceClient::new(config.clone())?,
            webhooks: WebhooksClient::new(config.clone())?,
            telegram: TelegramClient::new(config.clone())?,
            slack: SlackClient::new(config.clone())?,
        })
    }
}

pub use brains::BrainsClient;
pub use captures::CapturesClient;
pub use slack::SlackClient;
pub use telegram::TelegramClient;
pub use webhooks::WebhooksClient;
pub use workspace::WorkspaceClient;
