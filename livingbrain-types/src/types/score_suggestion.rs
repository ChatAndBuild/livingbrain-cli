pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ScoreSuggestion {
    /// Unique identifier.
    #[serde(default)]
    pub id: String,
    /// Metric the suggestion improves
    #[serde(default)]
    pub metric: String,
    /// Human-readable title.
    #[serde(default)]
    pub title: String,
    /// The suggestion text shown to the user.
    #[serde(default)]
    pub body: String,
    /// Host UI action hint
    #[serde(default)]
    pub action: String,
    /// Suggested call-to-action label for a button.
    #[serde(rename = "actionLabel")]
    #[serde(default)]
    pub action_label: String,
    /// Lower = more important
    #[serde(default)]
    pub priority: i64,
}

impl ScoreSuggestion {
    pub fn builder() -> ScoreSuggestionBuilder {
        <ScoreSuggestionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ScoreSuggestionBuilder {
    id: Option<String>,
    metric: Option<String>,
    title: Option<String>,
    body: Option<String>,
    action: Option<String>,
    action_label: Option<String>,
    priority: Option<i64>,
}

impl ScoreSuggestionBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn metric(mut self, value: impl Into<String>) -> Self {
        self.metric = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn body(mut self, value: impl Into<String>) -> Self {
        self.body = Some(value.into());
        self
    }

    pub fn action(mut self, value: impl Into<String>) -> Self {
        self.action = Some(value.into());
        self
    }

    pub fn action_label(mut self, value: impl Into<String>) -> Self {
        self.action_label = Some(value.into());
        self
    }

    pub fn priority(mut self, value: i64) -> Self {
        self.priority = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ScoreSuggestion`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ScoreSuggestionBuilder::id)
    /// - [`metric`](ScoreSuggestionBuilder::metric)
    /// - [`title`](ScoreSuggestionBuilder::title)
    /// - [`body`](ScoreSuggestionBuilder::body)
    /// - [`action`](ScoreSuggestionBuilder::action)
    /// - [`action_label`](ScoreSuggestionBuilder::action_label)
    /// - [`priority`](ScoreSuggestionBuilder::priority)
    pub fn build(self) -> Result<ScoreSuggestion, BuildError> {
        Ok(ScoreSuggestion {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            metric: self.metric.ok_or_else(|| BuildError::missing_field("metric"))?,
            title: self.title.ok_or_else(|| BuildError::missing_field("title"))?,
            body: self.body.ok_or_else(|| BuildError::missing_field("body"))?,
            action: self.action.ok_or_else(|| BuildError::missing_field("action"))?,
            action_label: self.action_label.ok_or_else(|| BuildError::missing_field("action_label"))?,
            priority: self.priority.ok_or_else(|| BuildError::missing_field("priority"))?,
        })
    }
}
