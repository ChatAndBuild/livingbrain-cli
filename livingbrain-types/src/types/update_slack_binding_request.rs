pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateSlackBindingRequest {
    /// Receive daily summary of brain updates
    #[serde(rename = "notifyBriefs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify_briefs: Option<bool>,
    /// Get notified when strong new connections are discovered
    #[serde(rename = "notifyConnections")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify_connections: Option<bool>,
    /// Get notified when pages are updated with new knowledge
    #[serde(rename = "notifyChanges")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify_changes: Option<bool>,
}

impl UpdateSlackBindingRequest {
    pub fn builder() -> UpdateSlackBindingRequestBuilder {
        <UpdateSlackBindingRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateSlackBindingRequestBuilder {
    notify_briefs: Option<bool>,
    notify_connections: Option<bool>,
    notify_changes: Option<bool>,
}

impl UpdateSlackBindingRequestBuilder {
    pub fn notify_briefs(mut self, value: bool) -> Self {
        self.notify_briefs = Some(value);
        self
    }

    pub fn notify_connections(mut self, value: bool) -> Self {
        self.notify_connections = Some(value);
        self
    }

    pub fn notify_changes(mut self, value: bool) -> Self {
        self.notify_changes = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateSlackBindingRequest`].
    pub fn build(self) -> Result<UpdateSlackBindingRequest, BuildError> {
        Ok(UpdateSlackBindingRequest {
            notify_briefs: self.notify_briefs,
            notify_connections: self.notify_connections,
            notify_changes: self.notify_changes,
        })
    }
}

