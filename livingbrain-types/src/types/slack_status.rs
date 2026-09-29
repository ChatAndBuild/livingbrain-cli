pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SlackStatus {
    /// Slack workspaces installed for this brain.
    #[serde(default)]
    pub installations: Vec<SlackInstallation>,
    /// Slack channels bound to this brain.
    #[serde(default)]
    pub bindings: Vec<SlackBinding>,
}

impl SlackStatus {
    pub fn builder() -> SlackStatusBuilder {
        <SlackStatusBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SlackStatusBuilder {
    installations: Option<Vec<SlackInstallation>>,
    bindings: Option<Vec<SlackBinding>>,
}

impl SlackStatusBuilder {
    pub fn installations(mut self, value: Vec<SlackInstallation>) -> Self {
        self.installations = Some(value);
        self
    }

    pub fn bindings(mut self, value: Vec<SlackBinding>) -> Self {
        self.bindings = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SlackStatus`].
    /// This method will fail if any of the following fields are not set:
    /// - [`installations`](SlackStatusBuilder::installations)
    /// - [`bindings`](SlackStatusBuilder::bindings)
    pub fn build(self) -> Result<SlackStatus, BuildError> {
        Ok(SlackStatus {
            installations: self.installations.ok_or_else(|| BuildError::missing_field("installations"))?,
            bindings: self.bindings.ok_or_else(|| BuildError::missing_field("bindings"))?,
        })
    }
}
