pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ScoreHistoryItem {
    /// ISO timestamp
    #[serde(rename = "computedAt")]
    #[serde(default)]
    pub computed_at: String,
    /// Freshness score at this point in time.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub freshness: f64,
    /// Readiness score at this point in time.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub readiness: f64,
    /// Strain score at this point in time.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub strain: f64,
}

impl ScoreHistoryItem {
    pub fn builder() -> ScoreHistoryItemBuilder {
        <ScoreHistoryItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ScoreHistoryItemBuilder {
    computed_at: Option<String>,
    freshness: Option<f64>,
    readiness: Option<f64>,
    strain: Option<f64>,
}

impl ScoreHistoryItemBuilder {
    pub fn computed_at(mut self, value: impl Into<String>) -> Self {
        self.computed_at = Some(value.into());
        self
    }

    pub fn freshness(mut self, value: f64) -> Self {
        self.freshness = Some(value);
        self
    }

    pub fn readiness(mut self, value: f64) -> Self {
        self.readiness = Some(value);
        self
    }

    pub fn strain(mut self, value: f64) -> Self {
        self.strain = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ScoreHistoryItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`computed_at`](ScoreHistoryItemBuilder::computed_at)
    /// - [`freshness`](ScoreHistoryItemBuilder::freshness)
    /// - [`readiness`](ScoreHistoryItemBuilder::readiness)
    /// - [`strain`](ScoreHistoryItemBuilder::strain)
    pub fn build(self) -> Result<ScoreHistoryItem, BuildError> {
        Ok(ScoreHistoryItem {
            computed_at: self.computed_at.ok_or_else(|| BuildError::missing_field("computed_at"))?,
            freshness: self.freshness.ok_or_else(|| BuildError::missing_field("freshness"))?,
            readiness: self.readiness.ok_or_else(|| BuildError::missing_field("readiness"))?,
            strain: self.strain.ok_or_else(|| BuildError::missing_field("strain"))?,
        })
    }
}
