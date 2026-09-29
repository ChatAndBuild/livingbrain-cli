pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BriefSettings {
    /// Generate a daily brief for this brain
    #[serde(default)]
    pub enabled: bool,
    /// Local-time hour the brief should fire
    #[serde(rename = "hourLocal")]
    #[serde(default)]
    pub hour_local: i64,
    /// IANA timezone
    #[serde(default)]
    pub timezone: String,
}

impl BriefSettings {
    pub fn builder() -> BriefSettingsBuilder {
        <BriefSettingsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BriefSettingsBuilder {
    enabled: Option<bool>,
    hour_local: Option<i64>,
    timezone: Option<String>,
}

impl BriefSettingsBuilder {
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn hour_local(mut self, value: i64) -> Self {
        self.hour_local = Some(value);
        self
    }

    pub fn timezone(mut self, value: impl Into<String>) -> Self {
        self.timezone = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BriefSettings`].
    /// This method will fail if any of the following fields are not set:
    /// - [`enabled`](BriefSettingsBuilder::enabled)
    /// - [`hour_local`](BriefSettingsBuilder::hour_local)
    /// - [`timezone`](BriefSettingsBuilder::timezone)
    pub fn build(self) -> Result<BriefSettings, BuildError> {
        Ok(BriefSettings {
            enabled: self.enabled.ok_or_else(|| BuildError::missing_field("enabled"))?,
            hour_local: self.hour_local.ok_or_else(|| BuildError::missing_field("hour_local"))?,
            timezone: self.timezone.ok_or_else(|| BuildError::missing_field("timezone"))?,
        })
    }
}
