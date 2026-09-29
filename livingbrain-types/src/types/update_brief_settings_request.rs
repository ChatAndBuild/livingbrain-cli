pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateBriefSettingsRequest {
    /// New daily-brief settings. Replaces the existing configuration.
    #[serde(rename = "briefSettings")]
    #[serde(default)]
    pub brief_settings: BriefSettings,
}

impl UpdateBriefSettingsRequest {
    pub fn builder() -> UpdateBriefSettingsRequestBuilder {
        <UpdateBriefSettingsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateBriefSettingsRequestBuilder {
    brief_settings: Option<BriefSettings>,
}

impl UpdateBriefSettingsRequestBuilder {
    pub fn brief_settings(mut self, value: BriefSettings) -> Self {
        self.brief_settings = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateBriefSettingsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`brief_settings`](UpdateBriefSettingsRequestBuilder::brief_settings)
    pub fn build(self) -> Result<UpdateBriefSettingsRequest, BuildError> {
        Ok(UpdateBriefSettingsRequest {
            brief_settings: self.brief_settings.ok_or_else(|| BuildError::missing_field("brief_settings"))?,
        })
    }
}

