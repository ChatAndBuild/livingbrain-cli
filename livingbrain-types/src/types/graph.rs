pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Graph {
    /// Pages to draw, capped by the requested node limit.
    #[serde(default)]
    pub nodes: Vec<GraphNode>,
    /// Connections between the returned nodes.
    #[serde(default)]
    pub edges: Vec<GraphEdge>,
    /// True when eligible pages exceeded the requested node limit
    #[serde(default)]
    pub truncated: bool,
}

impl Graph {
    pub fn builder() -> GraphBuilder {
        <GraphBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GraphBuilder {
    nodes: Option<Vec<GraphNode>>,
    edges: Option<Vec<GraphEdge>>,
    truncated: Option<bool>,
}

impl GraphBuilder {
    pub fn nodes(mut self, value: Vec<GraphNode>) -> Self {
        self.nodes = Some(value);
        self
    }

    pub fn edges(mut self, value: Vec<GraphEdge>) -> Self {
        self.edges = Some(value);
        self
    }

    pub fn truncated(mut self, value: bool) -> Self {
        self.truncated = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Graph`].
    /// This method will fail if any of the following fields are not set:
    /// - [`nodes`](GraphBuilder::nodes)
    /// - [`edges`](GraphBuilder::edges)
    /// - [`truncated`](GraphBuilder::truncated)
    pub fn build(self) -> Result<Graph, BuildError> {
        Ok(Graph {
            nodes: self.nodes.ok_or_else(|| BuildError::missing_field("nodes"))?,
            edges: self.edges.ok_or_else(|| BuildError::missing_field("edges"))?,
            truncated: self.truncated.ok_or_else(|| BuildError::missing_field("truncated"))?,
        })
    }
}
