pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Source {
    /// Provenance channel
    #[serde(default)]
    pub channel: String,
    /// Opaque host reference (e.g. agent id)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,
    /// Human-readable provenance label
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

impl Source {
    pub fn builder() -> SourceBuilder {
        <SourceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SourceBuilder {
    channel: Option<String>,
    r#ref: Option<String>,
    label: Option<String>,
}

impl SourceBuilder {
    pub fn channel(mut self, value: impl Into<String>) -> Self {
        self.channel = Some(value.into());
        self
    }

    pub fn r#ref(mut self, value: impl Into<String>) -> Self {
        self.r#ref = Some(value.into());
        self
    }

    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`Source`].
    /// This method will fail if any of the following fields are not set:
    /// - [`channel`](SourceBuilder::channel)
    pub fn build(self) -> Result<Source, BuildError> {
        Ok(Source {
            channel: self.channel.ok_or_else(|| BuildError::missing_field("channel"))?,
            r#ref: self.r#ref,
            label: self.label,
        })
    }
}
