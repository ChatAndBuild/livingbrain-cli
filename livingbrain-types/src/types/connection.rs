pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Connection {
    /// Connection id
    #[serde(default)]
    pub id: String,
    /// Owning tenant id
    #[serde(rename = "tenantId")]
    #[serde(default)]
    pub tenant_id: String,
    /// Owning subject id (host-defined)
    #[serde(rename = "subjectId")]
    #[serde(default)]
    pub subject_id: String,
    /// Owning brain id
    #[serde(rename = "brainId")]
    #[serde(default)]
    pub brain_id: String,
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
    /// Title of the first page in the pair.
    #[serde(rename = "pageATitle")]
    #[serde(default)]
    pub page_a_title: String,
    /// Slug of the second page in the pair.
    #[serde(rename = "pageBSlug")]
    #[serde(default)]
    pub page_b_slug: String,
    /// Title of the second page in the pair.
    #[serde(rename = "pageBTitle")]
    #[serde(default)]
    pub page_b_title: String,
    /// Cosine similarity (0-1)
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub similarity: f64,
    /// How strongly the two pages are related.
    pub strength: ConnectionStrength,
    /// Model-generated explanation of why these two pages are connected.
    #[serde(default)]
    pub reasoning: String,
    /// Whether the connection was discovered automatically or created by a user.
    pub source: ConnectionSource,
    /// ISO 8601 timestamp of when a user acknowledged this, or null if still unacknowledged.
    #[serde(rename = "ackedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub acked_at: Option<DateTime<FixedOffset>>,
    /// ISO 8601 timestamp of when this record was created.
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// ISO 8601 timestamp of when this record was last modified.
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
}

impl Connection {
    pub fn builder() -> ConnectionBuilder {
        <ConnectionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConnectionBuilder {
    id: Option<String>,
    tenant_id: Option<String>,
    subject_id: Option<String>,
    brain_id: Option<String>,
    page_a_id: Option<String>,
    page_b_id: Option<String>,
    page_a_slug: Option<String>,
    page_a_title: Option<String>,
    page_b_slug: Option<String>,
    page_b_title: Option<String>,
    similarity: Option<f64>,
    strength: Option<ConnectionStrength>,
    reasoning: Option<String>,
    source: Option<ConnectionSource>,
    acked_at: Option<DateTime<FixedOffset>>,
    created_at: Option<DateTime<FixedOffset>>,
    updated_at: Option<DateTime<FixedOffset>>,
}

impl ConnectionBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn tenant_id(mut self, value: impl Into<String>) -> Self {
        self.tenant_id = Some(value.into());
        self
    }

    pub fn subject_id(mut self, value: impl Into<String>) -> Self {
        self.subject_id = Some(value.into());
        self
    }

    pub fn brain_id(mut self, value: impl Into<String>) -> Self {
        self.brain_id = Some(value.into());
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

    pub fn page_a_title(mut self, value: impl Into<String>) -> Self {
        self.page_a_title = Some(value.into());
        self
    }

    pub fn page_b_slug(mut self, value: impl Into<String>) -> Self {
        self.page_b_slug = Some(value.into());
        self
    }

    pub fn page_b_title(mut self, value: impl Into<String>) -> Self {
        self.page_b_title = Some(value.into());
        self
    }

    pub fn similarity(mut self, value: f64) -> Self {
        self.similarity = Some(value);
        self
    }

    pub fn strength(mut self, value: ConnectionStrength) -> Self {
        self.strength = Some(value);
        self
    }

    pub fn reasoning(mut self, value: impl Into<String>) -> Self {
        self.reasoning = Some(value.into());
        self
    }

    pub fn source(mut self, value: ConnectionSource) -> Self {
        self.source = Some(value);
        self
    }

    pub fn acked_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.acked_at = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Connection`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ConnectionBuilder::id)
    /// - [`tenant_id`](ConnectionBuilder::tenant_id)
    /// - [`subject_id`](ConnectionBuilder::subject_id)
    /// - [`brain_id`](ConnectionBuilder::brain_id)
    /// - [`page_a_id`](ConnectionBuilder::page_a_id)
    /// - [`page_b_id`](ConnectionBuilder::page_b_id)
    /// - [`page_a_slug`](ConnectionBuilder::page_a_slug)
    /// - [`page_a_title`](ConnectionBuilder::page_a_title)
    /// - [`page_b_slug`](ConnectionBuilder::page_b_slug)
    /// - [`page_b_title`](ConnectionBuilder::page_b_title)
    /// - [`similarity`](ConnectionBuilder::similarity)
    /// - [`strength`](ConnectionBuilder::strength)
    /// - [`reasoning`](ConnectionBuilder::reasoning)
    /// - [`source`](ConnectionBuilder::source)
    /// - [`created_at`](ConnectionBuilder::created_at)
    /// - [`updated_at`](ConnectionBuilder::updated_at)
    pub fn build(self) -> Result<Connection, BuildError> {
        Ok(Connection {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            tenant_id: self.tenant_id.ok_or_else(|| BuildError::missing_field("tenant_id"))?,
            subject_id: self.subject_id.ok_or_else(|| BuildError::missing_field("subject_id"))?,
            brain_id: self.brain_id.ok_or_else(|| BuildError::missing_field("brain_id"))?,
            page_a_id: self.page_a_id.ok_or_else(|| BuildError::missing_field("page_a_id"))?,
            page_b_id: self.page_b_id.ok_or_else(|| BuildError::missing_field("page_b_id"))?,
            page_a_slug: self.page_a_slug.ok_or_else(|| BuildError::missing_field("page_a_slug"))?,
            page_a_title: self.page_a_title.ok_or_else(|| BuildError::missing_field("page_a_title"))?,
            page_b_slug: self.page_b_slug.ok_or_else(|| BuildError::missing_field("page_b_slug"))?,
            page_b_title: self.page_b_title.ok_or_else(|| BuildError::missing_field("page_b_title"))?,
            similarity: self.similarity.ok_or_else(|| BuildError::missing_field("similarity"))?,
            strength: self.strength.ok_or_else(|| BuildError::missing_field("strength"))?,
            reasoning: self.reasoning.ok_or_else(|| BuildError::missing_field("reasoning"))?,
            source: self.source.ok_or_else(|| BuildError::missing_field("source"))?,
            acked_at: self.acked_at,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
            updated_at: self.updated_at.ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
