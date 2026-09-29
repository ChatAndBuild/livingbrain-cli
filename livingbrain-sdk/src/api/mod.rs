//! API client and types for the Living Brain Service
//!
//! This module contains all the API definitions including request/response types
//! and client implementations for interacting with the API.
//!
//! ## Modules
//!
//! - [`resources`] - Service clients and endpoints

pub mod resources;

pub use resources::{
    ApiClient, BrainsClient, CapturesClient, SlackClient, TelegramClient, WebhooksClient,
    WorkspaceClient,
};

pub use livingbrain_types::*;
