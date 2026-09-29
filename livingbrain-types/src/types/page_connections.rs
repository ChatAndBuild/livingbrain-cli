pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PageConnections {
    /// Connections attached to the page, each with the related page inlined.
    #[serde(default)]
    pub items: Vec<Connection>,
    /// Id of the page the connections were fetched for
    #[serde(rename = "pageId")]
    #[serde(default)]
    pub page_id: String,
}

impl PageConnections {
    pub fn builder() -> PageConnectionsBuilder {
        <PageConnectionsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PageConnectionsBuilder {
    items: Option<Vec<Connection>>,
    page_id: Option<String>,
}

impl PageConnectionsBuilder {
    pub fn items(mut self, value: Vec<Connection>) -> Self {
        self.items = Some(value);
        self
    }

    pub fn page_id(mut self, value: impl Into<String>) -> Self {
        self.page_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PageConnections`].
    /// This method will fail if any of the following fields are not set:
    /// - [`items`](PageConnectionsBuilder::items)
    /// - [`page_id`](PageConnectionsBuilder::page_id)
    pub fn build(self) -> Result<PageConnections, BuildError> {
        Ok(PageConnections {
            items: self.items.ok_or_else(|| BuildError::missing_field("items"))?,
            page_id: self.page_id.ok_or_else(|| BuildError::missing_field("page_id"))?,
        })
    }
}
