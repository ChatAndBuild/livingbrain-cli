pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateWebhookRequest {
    /// HTTPS endpoint to deliver events to
    #[serde(default)]
    pub url: String,
    /// Event types to subscribe to; empty = all
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<CreateWebhookRequestEventsItem>>,
    /// Signing secret to use for this subscription. When omitted a random secret is generated and returned once. Hosts that verify deliveries against a pre-configured secret (e.g. an env var) should pass it here so signatures match. Write-only: never returned on reads.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
}

impl CreateWebhookRequest {
    pub fn builder() -> CreateWebhookRequestBuilder {
        <CreateWebhookRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateWebhookRequestBuilder {
    url: Option<String>,
    events: Option<Vec<CreateWebhookRequestEventsItem>>,
    secret: Option<String>,
}

impl CreateWebhookRequestBuilder {
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    pub fn events(mut self, value: Vec<CreateWebhookRequestEventsItem>) -> Self {
        self.events = Some(value);
        self
    }

    pub fn secret(mut self, value: impl Into<String>) -> Self {
        self.secret = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateWebhookRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`url`](CreateWebhookRequestBuilder::url)
    pub fn build(self) -> Result<CreateWebhookRequest, BuildError> {
        Ok(CreateWebhookRequest {
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
            events: self.events,
            secret: self.secret,
        })
    }
}

