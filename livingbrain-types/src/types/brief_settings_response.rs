pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BriefSettingsResponse {
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
    /// ISO date (YYYY-MM-DD) of the last generated brief
    #[serde(rename = "lastBriefDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_brief_date: Option<String>,
}

impl BriefSettingsResponse {
    pub fn builder() -> BriefSettingsResponseBuilder {
        <BriefSettingsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BriefSettingsResponseBuilder {
    enabled: Option<bool>,
    hour_local: Option<i64>,
    timezone: Option<String>,
    last_brief_date: Option<String>,
}

impl BriefSettingsResponseBuilder {
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

    pub fn last_brief_date(mut self, value: impl Into<String>) -> Self {
        self.last_brief_date = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BriefSettingsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`enabled`](BriefSettingsResponseBuilder::enabled)
    /// - [`hour_local`](BriefSettingsResponseBuilder::hour_local)
    /// - [`timezone`](BriefSettingsResponseBuilder::timezone)
    pub fn build(self) -> Result<BriefSettingsResponse, BuildError> {
        Ok(BriefSettingsResponse {
            enabled: self.enabled.ok_or_else(|| BuildError::missing_field("enabled"))?,
            hour_local: self.hour_local.ok_or_else(|| BuildError::missing_field("hour_local"))?,
            timezone: self.timezone.ok_or_else(|| BuildError::missing_field("timezone"))?,
            last_brief_date: self.last_brief_date,
        })
    }
}
