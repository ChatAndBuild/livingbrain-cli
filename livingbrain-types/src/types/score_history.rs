pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ScoreHistory {
    /// Identifier of the brain this belongs to.
    #[serde(rename = "brainId")]
    #[serde(default)]
    pub brain_id: String,
    /// The time range these results cover.
    pub range: ScoreHistoryRange,
    /// Score snapshots over the range, oldest first.
    #[serde(default)]
    pub items: Vec<ScoreHistoryItem>,
    /// Net change between the first and last snapshot.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delta: Option<ScoreDelta>,
}

impl ScoreHistory {
    pub fn builder() -> ScoreHistoryBuilder {
        <ScoreHistoryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ScoreHistoryBuilder {
    brain_id: Option<String>,
    range: Option<ScoreHistoryRange>,
    items: Option<Vec<ScoreHistoryItem>>,
    delta: Option<ScoreDelta>,
}

impl ScoreHistoryBuilder {
    pub fn brain_id(mut self, value: impl Into<String>) -> Self {
        self.brain_id = Some(value.into());
        self
    }

    pub fn range(mut self, value: ScoreHistoryRange) -> Self {
        self.range = Some(value);
        self
    }

    pub fn items(mut self, value: Vec<ScoreHistoryItem>) -> Self {
        self.items = Some(value);
        self
    }

    pub fn delta(mut self, value: ScoreDelta) -> Self {
        self.delta = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ScoreHistory`].
    /// This method will fail if any of the following fields are not set:
    /// - [`brain_id`](ScoreHistoryBuilder::brain_id)
    /// - [`range`](ScoreHistoryBuilder::range)
    /// - [`items`](ScoreHistoryBuilder::items)
    pub fn build(self) -> Result<ScoreHistory, BuildError> {
        Ok(ScoreHistory {
            brain_id: self.brain_id.ok_or_else(|| BuildError::missing_field("brain_id"))?,
            range: self.range.ok_or_else(|| BuildError::missing_field("range"))?,
            items: self.items.ok_or_else(|| BuildError::missing_field("items"))?,
            delta: self.delta,
        })
    }
}
