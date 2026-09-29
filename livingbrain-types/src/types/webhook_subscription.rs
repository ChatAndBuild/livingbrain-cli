pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WebhookSubscription {
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
    pub events: Vec<WebhookSubscriptionEventsItem>,
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
}

impl WebhookSubscription {
    pub fn builder() -> WebhookSubscriptionBuilder {
        <WebhookSubscriptionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebhookSubscriptionBuilder {
    id: Option<String>,
    tenant_id: Option<String>,
    url: Option<String>,
    events: Option<Vec<WebhookSubscriptionEventsItem>>,
    active: Option<bool>,
    created_at: Option<DateTime<FixedOffset>>,
    updated_at: Option<DateTime<FixedOffset>>,
}

impl WebhookSubscriptionBuilder {
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

    pub fn events(mut self, value: Vec<WebhookSubscriptionEventsItem>) -> Self {
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

    /// Consumes the builder and constructs a [`WebhookSubscription`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](WebhookSubscriptionBuilder::id)
    /// - [`tenant_id`](WebhookSubscriptionBuilder::tenant_id)
    /// - [`url`](WebhookSubscriptionBuilder::url)
    /// - [`events`](WebhookSubscriptionBuilder::events)
    /// - [`active`](WebhookSubscriptionBuilder::active)
    /// - [`created_at`](WebhookSubscriptionBuilder::created_at)
    /// - [`updated_at`](WebhookSubscriptionBuilder::updated_at)
    pub fn build(self) -> Result<WebhookSubscription, BuildError> {
        Ok(WebhookSubscription {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            tenant_id: self.tenant_id.ok_or_else(|| BuildError::missing_field("tenant_id"))?,
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
            events: self.events.ok_or_else(|| BuildError::missing_field("events"))?,
            active: self.active.ok_or_else(|| BuildError::missing_field("active"))?,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
            updated_at: self.updated_at.ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
