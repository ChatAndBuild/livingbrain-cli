pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CaptureRequest {
    /// Kind of raw input
    pub kind: CaptureRequestKind,
    /// Raw textual content of the capture
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// URL the service fetches and extracts instead of inlined content. Extraction is driven by `kind`: file -> parse PDF, Word, PPTX, HTML, or UTF-8 text; url -> scrape web page; transcript -> transcribe audio. Legacy .ppt and other unsupported binary formats are rejected. No cloud credentials needed.
    #[serde(rename = "fetchUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fetch_url: Option<String>,
    /// MIME type hint for `fetchUrl` extraction (for example application/vnd.openxmlformats-officedocument.presentationml.presentation)
    #[serde(rename = "mimeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    /// Original filename hint for `fetchUrl` extraction, including its extension
    #[serde(rename = "fileName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    /// Short title / filename / URL for display
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// Origin URL / external id, also used as the idempotency key for dedup
    #[serde(rename = "originRef")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_ref: Option<String>,
    /// Bucket hint for the resulting page(s)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket: Option<CaptureRequestBucket>,
    /// Opaque provenance
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,
}

impl CaptureRequest {
    pub fn builder() -> CaptureRequestBuilder {
        <CaptureRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CaptureRequestBuilder {
    kind: Option<CaptureRequestKind>,
    content: Option<String>,
    fetch_url: Option<String>,
    mime_type: Option<String>,
    file_name: Option<String>,
    label: Option<String>,
    origin_ref: Option<String>,
    bucket: Option<CaptureRequestBucket>,
    source: Option<Source>,
}

impl CaptureRequestBuilder {
    pub fn kind(mut self, value: CaptureRequestKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn content(mut self, value: impl Into<String>) -> Self {
        self.content = Some(value.into());
        self
    }

    pub fn fetch_url(mut self, value: impl Into<String>) -> Self {
        self.fetch_url = Some(value.into());
        self
    }

    pub fn mime_type(mut self, value: impl Into<String>) -> Self {
        self.mime_type = Some(value.into());
        self
    }

    pub fn file_name(mut self, value: impl Into<String>) -> Self {
        self.file_name = Some(value.into());
        self
    }

    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn origin_ref(mut self, value: impl Into<String>) -> Self {
        self.origin_ref = Some(value.into());
        self
    }

    pub fn bucket(mut self, value: CaptureRequestBucket) -> Self {
        self.bucket = Some(value);
        self
    }

    pub fn source(mut self, value: Source) -> Self {
        self.source = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CaptureRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](CaptureRequestBuilder::kind)
    pub fn build(self) -> Result<CaptureRequest, BuildError> {
        Ok(CaptureRequest {
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            content: self.content,
            fetch_url: self.fetch_url,
            mime_type: self.mime_type,
            file_name: self.file_name,
            label: self.label,
            origin_ref: self.origin_ref,
            bucket: self.bucket,
            source: self.source,
        })
    }
}
