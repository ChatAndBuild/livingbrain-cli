pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ErrorResponse {
    /// HTTP status code
    #[serde(rename = "statusCode")]
    #[serde(default)]
    pub status_code: i64,
    /// ISO timestamp of the failure
    #[serde(default)]
    pub timestamp: String,
    /// Request path (query string stripped)
    #[serde(default)]
    pub path: String,
    /// Human-readable error message
    #[serde(default)]
    pub message: String,
    /// Original framework exception response when available, e.g. `{ statusCode, message, error }` with per-field validation messages
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<HashMap<String, serde_json::Value>>,
}

impl ErrorResponse {
    pub fn builder() -> ErrorResponseBuilder {
        <ErrorResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ErrorResponseBuilder {
    status_code: Option<i64>,
    timestamp: Option<String>,
    path: Option<String>,
    message: Option<String>,
    body: Option<HashMap<String, serde_json::Value>>,
}

impl ErrorResponseBuilder {
    pub fn status_code(mut self, value: i64) -> Self {
        self.status_code = Some(value);
        self
    }

    pub fn timestamp(mut self, value: impl Into<String>) -> Self {
        self.timestamp = Some(value.into());
        self
    }

    pub fn path(mut self, value: impl Into<String>) -> Self {
        self.path = Some(value.into());
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn body(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.body = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ErrorResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status_code`](ErrorResponseBuilder::status_code)
    /// - [`timestamp`](ErrorResponseBuilder::timestamp)
    /// - [`path`](ErrorResponseBuilder::path)
    /// - [`message`](ErrorResponseBuilder::message)
    pub fn build(self) -> Result<ErrorResponse, BuildError> {
        Ok(ErrorResponse {
            status_code: self.status_code.ok_or_else(|| BuildError::missing_field("status_code"))?,
            timestamp: self.timestamp.ok_or_else(|| BuildError::missing_field("timestamp"))?,
            path: self.path.ok_or_else(|| BuildError::missing_field("path"))?,
            message: self.message.ok_or_else(|| BuildError::missing_field("message"))?,
            body: self.body,
        })
    }
}
