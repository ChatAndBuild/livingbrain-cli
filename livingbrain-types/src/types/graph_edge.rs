pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GraphEdge {
    /// Connection id
    #[serde(default)]
    pub id: String,
    /// Identifier of the first page in the pair.
    #[serde(rename = "pageAId")]
    #[serde(default)]
    pub page_a_id: String,
    /// Identifier of the second page in the pair.
    #[serde(rename = "pageBId")]
    #[serde(default)]
    pub page_b_id: String,
    /// Slug of the first page in the pair.
    #[serde(rename = "pageASlug")]
    #[serde(default)]
    pub page_a_slug: String,
    /// Slug of the second page in the pair.
    #[serde(rename = "pageBSlug")]
    #[serde(default)]
    pub page_b_slug: String,
    /// Title of the first page in the pair.
    #[serde(rename = "pageATitle")]
    #[serde(default)]
    pub page_a_title: String,
    /// Title of the second page in the pair.
    #[serde(rename = "pageBTitle")]
    #[serde(default)]
    pub page_b_title: String,
    /// How strongly the two nodes are related; use it to weight edge thickness when rendering.
    pub strength: GraphEdgeStrength,
    /// Cosine similarity (0-1)
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub similarity: f64,
    /// Model-generated explanation of why these two pages are connected.
    #[serde(default)]
    pub reasoning: String,
    /// ISO timestamp
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
}

impl GraphEdge {
    pub fn builder() -> GraphEdgeBuilder {
        <GraphEdgeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GraphEdgeBuilder {
    id: Option<String>,
    page_a_id: Option<String>,
    page_b_id: Option<String>,
    page_a_slug: Option<String>,
    page_b_slug: Option<String>,
    page_a_title: Option<String>,
    page_b_title: Option<String>,
    strength: Option<GraphEdgeStrength>,
    similarity: Option<f64>,
    reasoning: Option<String>,
    created_at: Option<String>,
}

impl GraphEdgeBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn page_a_id(mut self, value: impl Into<String>) -> Self {
        self.page_a_id = Some(value.into());
        self
    }

    pub fn page_b_id(mut self, value: impl Into<String>) -> Self {
        self.page_b_id = Some(value.into());
        self
    }

    pub fn page_a_slug(mut self, value: impl Into<String>) -> Self {
        self.page_a_slug = Some(value.into());
        self
    }

    pub fn page_b_slug(mut self, value: impl Into<String>) -> Self {
        self.page_b_slug = Some(value.into());
        self
    }

    pub fn page_a_title(mut self, value: impl Into<String>) -> Self {
        self.page_a_title = Some(value.into());
        self
    }

    pub fn page_b_title(mut self, value: impl Into<String>) -> Self {
        self.page_b_title = Some(value.into());
        self
    }

    pub fn strength(mut self, value: GraphEdgeStrength) -> Self {
        self.strength = Some(value);
        self
    }

    pub fn similarity(mut self, value: f64) -> Self {
        self.similarity = Some(value);
        self
    }

    pub fn reasoning(mut self, value: impl Into<String>) -> Self {
        self.reasoning = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GraphEdge`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](GraphEdgeBuilder::id)
    /// - [`page_a_id`](GraphEdgeBuilder::page_a_id)
    /// - [`page_b_id`](GraphEdgeBuilder::page_b_id)
    /// - [`page_a_slug`](GraphEdgeBuilder::page_a_slug)
    /// - [`page_b_slug`](GraphEdgeBuilder::page_b_slug)
    /// - [`page_a_title`](GraphEdgeBuilder::page_a_title)
    /// - [`page_b_title`](GraphEdgeBuilder::page_b_title)
    /// - [`strength`](GraphEdgeBuilder::strength)
    /// - [`similarity`](GraphEdgeBuilder::similarity)
    /// - [`reasoning`](GraphEdgeBuilder::reasoning)
    /// - [`created_at`](GraphEdgeBuilder::created_at)
    pub fn build(self) -> Result<GraphEdge, BuildError> {
        Ok(GraphEdge {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            page_a_id: self.page_a_id.ok_or_else(|| BuildError::missing_field("page_a_id"))?,
            page_b_id: self.page_b_id.ok_or_else(|| BuildError::missing_field("page_b_id"))?,
            page_a_slug: self.page_a_slug.ok_or_else(|| BuildError::missing_field("page_a_slug"))?,
            page_b_slug: self.page_b_slug.ok_or_else(|| BuildError::missing_field("page_b_slug"))?,
            page_a_title: self.page_a_title.ok_or_else(|| BuildError::missing_field("page_a_title"))?,
            page_b_title: self.page_b_title.ok_or_else(|| BuildError::missing_field("page_b_title"))?,
            strength: self.strength.ok_or_else(|| BuildError::missing_field("strength"))?,
            similarity: self.similarity.ok_or_else(|| BuildError::missing_field("similarity"))?,
            reasoning: self.reasoning.ok_or_else(|| BuildError::missing_field("reasoning"))?,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
        })
    }
}
