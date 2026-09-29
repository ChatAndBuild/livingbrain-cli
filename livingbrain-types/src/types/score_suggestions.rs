pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ScoreSuggestions {
    /// Identifier of the brain this belongs to.
    #[serde(rename = "brainId")]
    #[serde(default)]
    pub brain_id: String,
    /// What produced these suggestions — a model, or a deterministic rule set.
    #[serde(rename = "generatedBy")]
    #[serde(default)]
    pub generated_by: String,
    /// Suggestions ranked by expected impact on the score.
    #[serde(default)]
    pub suggestions: Vec<ScoreSuggestion>,
    /// ISO timestamp
    #[serde(rename = "computedAt")]
    #[serde(default)]
    pub computed_at: String,
}

impl ScoreSuggestions {
    pub fn builder() -> ScoreSuggestionsBuilder {
        <ScoreSuggestionsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ScoreSuggestionsBuilder {
    brain_id: Option<String>,
    generated_by: Option<String>,
    suggestions: Option<Vec<ScoreSuggestion>>,
    computed_at: Option<String>,
}

impl ScoreSuggestionsBuilder {
    pub fn brain_id(mut self, value: impl Into<String>) -> Self {
        self.brain_id = Some(value.into());
        self
    }

    pub fn generated_by(mut self, value: impl Into<String>) -> Self {
        self.generated_by = Some(value.into());
        self
    }

    pub fn suggestions(mut self, value: Vec<ScoreSuggestion>) -> Self {
        self.suggestions = Some(value);
        self
    }

    pub fn computed_at(mut self, value: impl Into<String>) -> Self {
        self.computed_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ScoreSuggestions`].
    /// This method will fail if any of the following fields are not set:
    /// - [`brain_id`](ScoreSuggestionsBuilder::brain_id)
    /// - [`generated_by`](ScoreSuggestionsBuilder::generated_by)
    /// - [`suggestions`](ScoreSuggestionsBuilder::suggestions)
    /// - [`computed_at`](ScoreSuggestionsBuilder::computed_at)
    pub fn build(self) -> Result<ScoreSuggestions, BuildError> {
        Ok(ScoreSuggestions {
            brain_id: self.brain_id.ok_or_else(|| BuildError::missing_field("brain_id"))?,
            generated_by: self.generated_by.ok_or_else(|| BuildError::missing_field("generated_by"))?,
            suggestions: self.suggestions.ok_or_else(|| BuildError::missing_field("suggestions"))?,
            computed_at: self.computed_at.ok_or_else(|| BuildError::missing_field("computed_at"))?,
        })
    }
}
