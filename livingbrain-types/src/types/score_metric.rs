pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ScoreMetric {
    /// Stable machine-readable identifier, safe to branch on in code.
    #[serde(default)]
    pub key: String,
    /// Human-readable display name.
    #[serde(default)]
    pub label: String,
    /// Score 0-100
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub score: f64,
    /// Human-readable status label
    #[serde(default)]
    pub status: String,
    /// One-line explanation of what this metric captures.
    #[serde(default)]
    pub summary: String,
    /// Whether a larger value is an improvement. Drives arrow direction in trend UI.
    #[serde(rename = "higherIsBetter")]
    #[serde(default)]
    pub higher_is_better: bool,
    /// The individual factors that roll up into this metric.
    #[serde(default)]
    pub factors: Vec<ScoreFactor>,
}

impl ScoreMetric {
    pub fn builder() -> ScoreMetricBuilder {
        <ScoreMetricBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ScoreMetricBuilder {
    key: Option<String>,
    label: Option<String>,
    score: Option<f64>,
    status: Option<String>,
    summary: Option<String>,
    higher_is_better: Option<bool>,
    factors: Option<Vec<ScoreFactor>>,
}

impl ScoreMetricBuilder {
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());
        self
    }

    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn score(mut self, value: f64) -> Self {
        self.score = Some(value);
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    pub fn summary(mut self, value: impl Into<String>) -> Self {
        self.summary = Some(value.into());
        self
    }

    pub fn higher_is_better(mut self, value: bool) -> Self {
        self.higher_is_better = Some(value);
        self
    }

    pub fn factors(mut self, value: Vec<ScoreFactor>) -> Self {
        self.factors = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ScoreMetric`].
    /// This method will fail if any of the following fields are not set:
    /// - [`key`](ScoreMetricBuilder::key)
    /// - [`label`](ScoreMetricBuilder::label)
    /// - [`score`](ScoreMetricBuilder::score)
    /// - [`status`](ScoreMetricBuilder::status)
    /// - [`summary`](ScoreMetricBuilder::summary)
    /// - [`higher_is_better`](ScoreMetricBuilder::higher_is_better)
    /// - [`factors`](ScoreMetricBuilder::factors)
    pub fn build(self) -> Result<ScoreMetric, BuildError> {
        Ok(ScoreMetric {
            key: self.key.ok_or_else(|| BuildError::missing_field("key"))?,
            label: self.label.ok_or_else(|| BuildError::missing_field("label"))?,
            score: self.score.ok_or_else(|| BuildError::missing_field("score"))?,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            summary: self.summary.ok_or_else(|| BuildError::missing_field("summary"))?,
            higher_is_better: self.higher_is_better.ok_or_else(|| BuildError::missing_field("higher_is_better"))?,
            factors: self.factors.ok_or_else(|| BuildError::missing_field("factors"))?,
        })
    }
}
