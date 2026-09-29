pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ProfileSuggestions {
    /// Identifier of the brain this belongs to.
    #[serde(rename = "brainId")]
    #[serde(default)]
    pub brain_id: String,
    /// Suggested profile field values keyed by field name
    #[serde(default)]
    pub suggestions: HashMap<String, String>,
}

impl ProfileSuggestions {
    pub fn builder() -> ProfileSuggestionsBuilder {
        <ProfileSuggestionsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProfileSuggestionsBuilder {
    brain_id: Option<String>,
    suggestions: Option<HashMap<String, String>>,
}

impl ProfileSuggestionsBuilder {
    pub fn brain_id(mut self, value: impl Into<String>) -> Self {
        self.brain_id = Some(value.into());
        self
    }

    pub fn suggestions(mut self, value: HashMap<String, String>) -> Self {
        self.suggestions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ProfileSuggestions`].
    /// This method will fail if any of the following fields are not set:
    /// - [`brain_id`](ProfileSuggestionsBuilder::brain_id)
    /// - [`suggestions`](ProfileSuggestionsBuilder::suggestions)
    pub fn build(self) -> Result<ProfileSuggestions, BuildError> {
        Ok(ProfileSuggestions {
            brain_id: self.brain_id.ok_or_else(|| BuildError::missing_field("brain_id"))?,
            suggestions: self.suggestions.ok_or_else(|| BuildError::missing_field("suggestions"))?,
        })
    }
}
