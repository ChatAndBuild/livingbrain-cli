pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BrainScore {
    /// Identifier of the brain this belongs to.
    #[serde(rename = "brainId")]
    #[serde(default)]
    pub brain_id: String,
    /// How recently the brain's pages have been updated or referenced.
    #[serde(default)]
    pub freshness: ScoreMetric,
    /// How complete and well-connected the brain is.
    #[serde(default)]
    pub readiness: ScoreMetric,
    /// Backlog pressure — unprocessed captures, contested pages, and unacknowledged changes.
    #[serde(default)]
    pub strain: ScoreMetric,
    /// ISO timestamp
    #[serde(rename = "computedAt")]
    #[serde(default)]
    pub computed_at: String,
}

impl BrainScore {
    pub fn builder() -> BrainScoreBuilder {
        <BrainScoreBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BrainScoreBuilder {
    brain_id: Option<String>,
    freshness: Option<ScoreMetric>,
    readiness: Option<ScoreMetric>,
    strain: Option<ScoreMetric>,
    computed_at: Option<String>,
}

impl BrainScoreBuilder {
    pub fn brain_id(mut self, value: impl Into<String>) -> Self {
        self.brain_id = Some(value.into());
        self
    }

    pub fn freshness(mut self, value: ScoreMetric) -> Self {
        self.freshness = Some(value);
        self
    }

    pub fn readiness(mut self, value: ScoreMetric) -> Self {
        self.readiness = Some(value);
        self
    }

    pub fn strain(mut self, value: ScoreMetric) -> Self {
        self.strain = Some(value);
        self
    }

    pub fn computed_at(mut self, value: impl Into<String>) -> Self {
        self.computed_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BrainScore`].
    /// This method will fail if any of the following fields are not set:
    /// - [`brain_id`](BrainScoreBuilder::brain_id)
    /// - [`freshness`](BrainScoreBuilder::freshness)
    /// - [`readiness`](BrainScoreBuilder::readiness)
    /// - [`strain`](BrainScoreBuilder::strain)
    /// - [`computed_at`](BrainScoreBuilder::computed_at)
    pub fn build(self) -> Result<BrainScore, BuildError> {
        Ok(BrainScore {
            brain_id: self.brain_id.ok_or_else(|| BuildError::missing_field("brain_id"))?,
            freshness: self.freshness.ok_or_else(|| BuildError::missing_field("freshness"))?,
            readiness: self.readiness.ok_or_else(|| BuildError::missing_field("readiness"))?,
            strain: self.strain.ok_or_else(|| BuildError::missing_field("strain"))?,
            computed_at: self.computed_at.ok_or_else(|| BuildError::missing_field("computed_at"))?,
        })
    }
}
