pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateWebhookRequest {
    /// New delivery URL. Omit to leave unchanged.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// New event list. Replaces the existing subscription set. Omit to leave unchanged.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<UpdateWebhookRequestEventsItem>>,
    /// Set false to pause deliveries without deleting the subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
}

impl UpdateWebhookRequest {
    pub fn builder() -> UpdateWebhookRequestBuilder {
        <UpdateWebhookRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateWebhookRequestBuilder {
    url: Option<String>,
    events: Option<Vec<UpdateWebhookRequestEventsItem>>,
    active: Option<bool>,
}

impl UpdateWebhookRequestBuilder {
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    pub fn events(mut self, value: Vec<UpdateWebhookRequestEventsItem>) -> Self {
        self.events = Some(value);
        self
    }

    pub fn active(mut self, value: bool) -> Self {
        self.active = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateWebhookRequest`].
    pub fn build(self) -> Result<UpdateWebhookRequest, BuildError> {
        Ok(UpdateWebhookRequest {
            url: self.url,
            events: self.events,
            active: self.active,
        })
    }
}

