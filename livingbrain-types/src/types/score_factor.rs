pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ScoreFactor {
    /// Stable machine-readable identifier, safe to branch on in code.
    #[serde(default)]
    pub key: String,
    /// Human-readable display name.
    #[serde(default)]
    pub label: String,
    /// Human-readable explanation of what this factor measures.
    #[serde(default)]
    pub description: String,
    /// The factor's measured value.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub value: f64,
    /// Unit the value is expressed in, for formatting.
    #[serde(default)]
    pub unit: String,
    /// Whether this value reads as good, neutral, or bad — use it to colour the UI.
    pub tone: ScoreFactorTone,
}

impl ScoreFactor {
    pub fn builder() -> ScoreFactorBuilder {
        <ScoreFactorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ScoreFactorBuilder {
    key: Option<String>,
    label: Option<String>,
    description: Option<String>,
    value: Option<f64>,
    unit: Option<String>,
    tone: Option<ScoreFactorTone>,
}

impl ScoreFactorBuilder {
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());
        self
    }

    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn value(mut self, value: f64) -> Self {
        self.value = Some(value);
        self
    }

    pub fn unit(mut self, value: impl Into<String>) -> Self {
        self.unit = Some(value.into());
        self
    }

    pub fn tone(mut self, value: ScoreFactorTone) -> Self {
        self.tone = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ScoreFactor`].
    /// This method will fail if any of the following fields are not set:
    /// - [`key`](ScoreFactorBuilder::key)
    /// - [`label`](ScoreFactorBuilder::label)
    /// - [`description`](ScoreFactorBuilder::description)
    /// - [`value`](ScoreFactorBuilder::value)
    /// - [`unit`](ScoreFactorBuilder::unit)
    /// - [`tone`](ScoreFactorBuilder::tone)
    pub fn build(self) -> Result<ScoreFactor, BuildError> {
        Ok(ScoreFactor {
            key: self.key.ok_or_else(|| BuildError::missing_field("key"))?,
            label: self.label.ok_or_else(|| BuildError::missing_field("label"))?,
            description: self.description.ok_or_else(|| BuildError::missing_field("description"))?,
            value: self.value.ok_or_else(|| BuildError::missing_field("value"))?,
            unit: self.unit.ok_or_else(|| BuildError::missing_field("unit"))?,
            tone: self.tone.ok_or_else(|| BuildError::missing_field("tone"))?,
        })
    }
}
