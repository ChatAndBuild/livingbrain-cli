pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ScoreDelta {
    /// Change in freshness across the requested range.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub freshness: f64,
    /// Change in readiness across the requested range.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub readiness: f64,
    /// Change in strain across the requested range.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub strain: f64,
}

impl ScoreDelta {
    pub fn builder() -> ScoreDeltaBuilder {
        <ScoreDeltaBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ScoreDeltaBuilder {
    freshness: Option<f64>,
    readiness: Option<f64>,
    strain: Option<f64>,
}

impl ScoreDeltaBuilder {
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

    /// Consumes the builder and constructs a [`ScoreDelta`].
    /// This method will fail if any of the following fields are not set:
    /// - [`freshness`](ScoreDeltaBuilder::freshness)
    /// - [`readiness`](ScoreDeltaBuilder::readiness)
    /// - [`strain`](ScoreDeltaBuilder::strain)
    pub fn build(self) -> Result<ScoreDelta, BuildError> {
        Ok(ScoreDelta {
            freshness: self.freshness.ok_or_else(|| BuildError::missing_field("freshness"))?,
            readiness: self.readiness.ok_or_else(|| BuildError::missing_field("readiness"))?,
            strain: self.strain.ok_or_else(|| BuildError::missing_field("strain"))?,
        })
    }
}
