pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConfirmSlackOAuthRequest {
    /// Slack authorization code
    #[serde(default)]
    pub code: String,
    /// Signed OAuth state
    #[serde(default)]
    pub state: String,
}

impl ConfirmSlackOAuthRequest {
    pub fn builder() -> ConfirmSlackOAuthRequestBuilder {
        <ConfirmSlackOAuthRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConfirmSlackOAuthRequestBuilder {
    code: Option<String>,
    state: Option<String>,
}

impl ConfirmSlackOAuthRequestBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ConfirmSlackOAuthRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](ConfirmSlackOAuthRequestBuilder::code)
    /// - [`state`](ConfirmSlackOAuthRequestBuilder::state)
    pub fn build(self) -> Result<ConfirmSlackOAuthRequest, BuildError> {
        Ok(ConfirmSlackOAuthRequest {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            state: self.state.ok_or_else(|| BuildError::missing_field("state"))?,
        })
    }
}

