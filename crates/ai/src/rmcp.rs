use std::sync::Arc;
use std::time::Duration;

use serde::{Deserialize, Serialize};

pub mod model {
    use super::*;

    pub type JsonObject = serde_json::Map<String, serde_json::Value>;

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct RawTextContent {
        pub text: String,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct RawImageContent {
        pub data: String,
        pub mime_type: String,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct RawAudioContent {
        pub data: String,
        pub mime_type: String,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct EmbeddedResource {
        pub resource: ResourceContents,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum RawContent {
        Text(RawTextContent),
        Image(RawImageContent),
        Resource(EmbeddedResource),
        Audio(RawAudioContent),
        ResourceLink(RawResource),
    }

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct Content {
        pub raw: RawContent,
    }

    impl Content {
        pub fn text(text: impl Into<String>) -> Self {
            Self {
                raw: RawContent::Text(RawTextContent { text: text.into() }),
            }
        }

        pub fn image(data: impl Into<String>, mime_type: impl Into<String>) -> Self {
            Self {
                raw: RawContent::Image(RawImageContent {
                    data: data.into(),
                    mime_type: mime_type.into(),
                }),
            }
        }

        pub fn resource(resource: ResourceContents) -> Self {
            Self {
                raw: RawContent::Resource(EmbeddedResource { resource }),
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum ResourceContents {
        TextResourceContents {
            uri: String,
            mime_type: Option<String>,
            text: String,
            meta: Option<serde_json::Value>,
        },
        BlobResourceContents {
            uri: String,
            mime_type: Option<String>,
            blob: String,
            meta: Option<serde_json::Value>,
        },
    }

    impl ResourceContents {
        pub fn text(text: impl Into<String>, uri: impl Into<String>) -> Self {
            Self::TextResourceContents {
                uri: uri.into(),
                mime_type: None,
                text: text.into(),
                meta: None,
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct RawResource {
        pub uri: String,
        pub name: String,
        pub description: Option<String>,
        pub mime_type: Option<String>,
        pub meta: Option<serde_json::Value>,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct Resource {
        pub raw: RawResource,
        pub uri: String,
        pub name: String,
        pub description: Option<String>,
        pub mime_type: Option<String>,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct Tool {
        pub name: String,
        pub description: Option<String>,
        pub input_schema: Arc<JsonObject>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct CallToolResult {
        pub content: Vec<Content>,
        pub is_error: Option<bool>,
        pub structured_content: Option<serde_json::Value>,
    }

    impl CallToolResult {
        pub fn success(content: Vec<Content>) -> Self {
            Self {
                content,
                is_error: Some(false),
                structured_content: None,
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct CallToolRequestParam {
        pub name: String,
        pub arguments: Option<JsonObject>,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct ReadResourceRequestParam {
        pub uri: String,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct ReadResourceResult {
        pub contents: Vec<ResourceContents>,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct ErrorData {
        pub code: ErrorCode,
        pub message: String,
        pub data: Option<serde_json::Value>,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub struct ErrorCode(pub i32);

    impl ErrorCode {
        pub const INTERNAL_ERROR: Self = Self(-32603);
        pub const METHOD_NOT_FOUND: Self = Self(-32601);
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ServiceError {
    #[error("MCP error: {0:?}")]
    McpError(model::ErrorData),
    #[error("transport send error: {0}")]
    TransportSend(String),
    #[error("transport closed")]
    TransportClosed,
    #[error("unexpected response")]
    UnexpectedResponse,
    #[error("cancelled: {reason}")]
    Cancelled { reason: String },
    #[error("timeout after {timeout:?}")]
    Timeout { timeout: Duration },
}

#[derive(Debug, thiserror::Error)]
pub enum RmcpError {
    #[error("client initialize failed: {0}")]
    ClientInitialize(String),
    #[error("server initialize failed: {0}")]
    ServerInitialize(String),
    #[error("transport creation failed: {error}")]
    TransportCreation { error: String },
    #[error("runtime error: {0}")]
    Runtime(String),
    #[error(transparent)]
    Service(ServiceError),
}

impl RmcpError {
    pub fn transport_creation<T>(error: impl ToString) -> Self {
        let _ = std::any::type_name::<T>();
        Self::TransportCreation {
            error: error.to_string(),
        }
    }
}
