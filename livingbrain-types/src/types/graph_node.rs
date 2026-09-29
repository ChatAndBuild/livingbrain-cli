pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct GraphNode {
    /// Page id
    #[serde(default)]
    pub id: String,
    /// URL-safe identifier, unique within the brain.
    #[serde(default)]
    pub slug: String,
    /// Human-readable title.
    #[serde(default)]
    pub title: String,
    /// Top-level grouping the page belongs to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket: Option<GraphNodeBucket>,
    /// Lifecycle state. Deprecated and archived items are hidden from listings by default.
    pub status: GraphNodeStatus,
    /// Pinned pages rank first in the prompt index and are exempt from deprecation.
    #[serde(default)]
    pub pinned: bool,
    /// Short summary, suitable for a tooltip on the node.
    #[serde(default)]
    pub summary: String,
    /// ISO timestamp
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    pub updated_at: String,
}

impl GraphNode {
    pub fn builder() -> GraphNodeBuilder {
        <GraphNodeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GraphNodeBuilder {
    id: Option<String>,
    slug: Option<String>,
    title: Option<String>,
    bucket: Option<GraphNodeBucket>,
    status: Option<GraphNodeStatus>,
    pinned: Option<bool>,
    summary: Option<String>,
    updated_at: Option<String>,
}

impl GraphNodeBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn slug(mut self, value: impl Into<String>) -> Self {
        self.slug = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn bucket(mut self, value: GraphNodeBucket) -> Self {
        self.bucket = Some(value);
        self
    }

    pub fn status(mut self, value: GraphNodeStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn pinned(mut self, value: bool) -> Self {
        self.pinned = Some(value);
        self
    }

    pub fn summary(mut self, value: impl Into<String>) -> Self {
        self.summary = Some(value.into());
        self
    }

    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GraphNode`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](GraphNodeBuilder::id)
    /// - [`slug`](GraphNodeBuilder::slug)
    /// - [`title`](GraphNodeBuilder::title)
    /// - [`status`](GraphNodeBuilder::status)
    /// - [`pinned`](GraphNodeBuilder::pinned)
    /// - [`summary`](GraphNodeBuilder::summary)
    /// - [`updated_at`](GraphNodeBuilder::updated_at)
    pub fn build(self) -> Result<GraphNode, BuildError> {
        Ok(GraphNode {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            slug: self.slug.ok_or_else(|| BuildError::missing_field("slug"))?,
            title: self.title.ok_or_else(|| BuildError::missing_field("title"))?,
            bucket: self.bucket,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            pinned: self.pinned.ok_or_else(|| BuildError::missing_field("pinned"))?,
            summary: self.summary.ok_or_else(|| BuildError::missing_field("summary"))?,
            updated_at: self.updated_at.ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
