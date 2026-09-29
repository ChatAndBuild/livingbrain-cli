pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateWebhookResponse {
    /// Subscription id
    #[serde(default)]
    pub id: String,
    /// Owning tenant id
    #[serde(rename = "tenantId")]
    #[serde(default)]
    pub tenant_id: String,
    /// Delivery endpoint
    #[serde(default)]
    pub url: String,
    /// Subscribed event types; empty = all
    #[serde(default)]
    pub events: Vec<CreateWebhookResponseEventsItem>,
    /// Whether the subscription is currently receiving deliveries.
    #[serde(default)]
    pub active: bool,
    /// ISO 8601 timestamp of when this record was created.
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// ISO 8601 timestamp of when this record was last modified.
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
    /// Raw signing secret for verifying `x-brain-signature` on deliveries. Returned exactly once; store it now.
    #[serde(default)]
    pub secret: String,
}

impl CreateWebhookResponse {
    pub fn builder() -> CreateWebhookResponseBuilder {
        <CreateWebhookResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateWebhookResponseBuilder {
    id: Option<String>,
    tenant_id: Option<String>,
    url: Option<String>,
    events: Option<Vec<CreateWebhookResponseEventsItem>>,
    active: Option<bool>,
    created_at: Option<DateTime<FixedOffset>>,
    updated_at: Option<DateTime<FixedOffset>>,
    secret: Option<String>,
}

impl CreateWebhookResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn tenant_id(mut self, value: impl Into<String>) -> Self {
        self.tenant_id = Some(value.into());
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    pub fn events(mut self, value: Vec<CreateWebhookResponseEventsItem>) -> Self {
        self.events = Some(value);
        self
    }

    pub fn active(mut self, value: bool) -> Self {
        self.active = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    pub fn secret(mut self, value: impl Into<String>) -> Self {
        self.secret = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateWebhookResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](CreateWebhookResponseBuilder::id)
    /// - [`tenant_id`](CreateWebhookResponseBuilder::tenant_id)
    /// - [`url`](CreateWebhookResponseBuilder::url)
    /// - [`events`](CreateWebhookResponseBuilder::events)
    /// - [`active`](CreateWebhookResponseBuilder::active)
    /// - [`created_at`](CreateWebhookResponseBuilder::created_at)
    /// - [`updated_at`](CreateWebhookResponseBuilder::updated_at)
    /// - [`secret`](CreateWebhookResponseBuilder::secret)
    pub fn build(self) -> Result<CreateWebhookResponse, BuildError> {
        Ok(CreateWebhookResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            tenant_id: self.tenant_id.ok_or_else(|| BuildError::missing_field("tenant_id"))?,
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
            events: self.events.ok_or_else(|| BuildError::missing_field("events"))?,
            active: self.active.ok_or_else(|| BuildError::missing_field("active"))?,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
            updated_at: self.updated_at.ok_or_else(|| BuildError::missing_field("updated_at"))?,
            secret: self.secret.ok_or_else(|| BuildError::missing_field("secret"))?,
        })
    }
}
