pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for scoreHistory
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ScoreHistoryQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<ScoreHistoryWorkspaceRequestRange>,
}

impl ScoreHistoryQueryRequest {
    pub fn builder() -> ScoreHistoryQueryRequestBuilder {
        <ScoreHistoryQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ScoreHistoryQueryRequestBuilder {
    range: Option<ScoreHistoryWorkspaceRequestRange>,
}

impl ScoreHistoryQueryRequestBuilder {
    pub fn range(mut self, value: ScoreHistoryWorkspaceRequestRange) -> Self {
        self.range = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ScoreHistoryQueryRequest`].
    pub fn build(self) -> Result<ScoreHistoryQueryRequest, BuildError> {
        Ok(ScoreHistoryQueryRequest {
            range: self.range,
        })
    }
}

