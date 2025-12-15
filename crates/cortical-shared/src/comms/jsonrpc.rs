//! JSON-RPC 2.0 types for MCP-style stdio transport.
//!
//! Implements the JSON-RPC 2.0 specification for brain-to-brain communication
//! over stdin/stdout, following MCP patterns.

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// JSON-RPC version constant
pub const JSONRPC_VERSION: &str = "2.0";

/// JSON-RPC request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcRequest {
    pub jsonrpc: String,
    pub method: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<Value>,
    /// If None, this is a notification (no response expected)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<RequestId>,
}

impl JsonRpcRequest {
    /// Create a new request
    pub fn new(method: impl Into<String>, params: Option<Value>, id: RequestId) -> Self {
        Self {
            jsonrpc: JSONRPC_VERSION.to_string(),
            method: method.into(),
            params,
            id: Some(id),
        }
    }

    /// Create a notification (no response expected)
    pub fn notification(method: impl Into<String>, params: Option<Value>) -> Self {
        Self {
            jsonrpc: JSONRPC_VERSION.to_string(),
            method: method.into(),
            params,
            id: None,
        }
    }

    /// Check if this is a notification
    pub fn is_notification(&self) -> bool {
        self.id.is_none()
    }
}

/// JSON-RPC response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcResponse {
    pub jsonrpc: String,
    pub id: RequestId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<JsonRpcError>,
}

impl JsonRpcResponse {
    /// Create a success response
    pub fn success(id: RequestId, result: Value) -> Self {
        Self {
            jsonrpc: JSONRPC_VERSION.to_string(),
            id,
            result: Some(result),
            error: None,
        }
    }

    /// Create an error response
    pub fn error(id: RequestId, error: JsonRpcError) -> Self {
        Self {
            jsonrpc: JSONRPC_VERSION.to_string(),
            id,
            result: None,
            error: Some(error),
        }
    }

    /// Check if response is an error
    pub fn is_error(&self) -> bool {
        self.error.is_some()
    }
}

/// Request ID - can be string or number
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RequestId {
    Number(i64),
    String(String),
}

impl From<i64> for RequestId {
    fn from(n: i64) -> Self {
        Self::Number(n)
    }
}

impl From<String> for RequestId {
    fn from(s: String) -> Self {
        Self::String(s)
    }
}

impl From<&str> for RequestId {
    fn from(s: &str) -> Self {
        Self::String(s.to_string())
    }
}

/// JSON-RPC error object
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcError {
    pub code: i32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

impl JsonRpcError {
    pub fn new(code: i32, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
            data: None,
        }
    }

    pub fn with_data(mut self, data: Value) -> Self {
        self.data = Some(data);
        self
    }

    // Standard JSON-RPC errors
    pub fn parse_error() -> Self {
        Self::new(-32700, "Parse error")
    }

    pub fn invalid_request() -> Self {
        Self::new(-32600, "Invalid request")
    }

    pub fn method_not_found() -> Self {
        Self::new(-32601, "Method not found")
    }

    pub fn invalid_params() -> Self {
        Self::new(-32602, "Invalid params")
    }

    pub fn internal_error() -> Self {
        Self::new(-32603, "Internal error")
    }

    // Custom neurostack errors
    pub fn brain_not_ready() -> Self {
        Self::new(-32000, "Brain not ready")
    }

    pub fn capability_not_supported() -> Self {
        Self::new(-32001, "Capability not supported")
    }

    pub fn trust_level_insufficient() -> Self {
        Self::new(-32002, "Trust level insufficient")
    }

    pub fn rate_limit_exceeded() -> Self {
        Self::new(-32003, "Rate limit exceeded")
    }

    pub fn encryption_required() -> Self {
        Self::new(-32004, "Encryption required")
    }
}

/// Standard method names for brain operations
pub mod methods {
    // Lifecycle
    pub const INITIALIZE: &str = "initialize";
    pub const SHUTDOWN: &str = "shutdown";

    // Core operations
    pub const QUERY: &str = "query";
    pub const STORE: &str = "store";
    pub const DECIDE: &str = "decide";

    // Inter-brain
    pub const INTEROP_MESSAGE: &str = "interop/message";
    pub const INTEROP_HANDSHAKE: &str = "interop/handshake";

    // Notifications
    pub const LOG: &str = "log";
    pub const PROGRESS: &str = "progress";
    pub const CAPABILITY_CHANGED: &str = "capability_changed";
}

/// Initialize request params
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitializeParams {
    pub protocol_version: String,
    pub client_info: ClientInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientInfo {
    pub name: String,
    pub version: String,
}

/// Initialize response result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitializeResult {
    pub protocol_version: String,
    pub brain_info: BrainInfo,
    pub server_capabilities: ServerCapabilities,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrainInfo {
    pub brain_id: String,
    pub name: String,
    pub capabilities: std::collections::HashMap<String, u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerCapabilities {
    pub tools: bool,
    pub resources: bool,
    pub prompts: bool,
}

impl Default for ServerCapabilities {
    fn default() -> Self {
        Self {
            tools: true,
            resources: true,
            prompts: false,
        }
    }
}

/// Query request params
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryParams {
    pub embedding: Vec<f32>,
    #[serde(default = "default_limit")]
    pub limit: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Value>,
}

fn default_limit() -> usize {
    10
}

/// Query response result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryResult {
    pub matches: Vec<QueryMatch>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryMatch {
    pub id: String,
    pub score: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<Value>,
}

/// Store request params
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoreParams {
    pub experience_id: String,
    pub embedding: Vec<f32>,
    pub payload: Value,
}

/// Store response result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoreResult {
    pub success: bool,
    pub id: String,
}

/// Log notification params
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogParams {
    pub level: LogLevel,
    pub message: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

/// Progress notification params
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressParams {
    pub operation_id: String,
    pub progress: f32,
    pub message: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request_serialization() {
        let req = JsonRpcRequest::new(
            "query",
            Some(serde_json::json!({"embedding": [0.1, 0.2]})),
            1.into(),
        );
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"jsonrpc\":\"2.0\""));
        assert!(json.contains("\"method\":\"query\""));
    }

    #[test]
    fn test_notification() {
        let notif = JsonRpcRequest::notification(
            "log",
            Some(serde_json::json!({"level": "info", "message": "test"})),
        );
        assert!(notif.is_notification());
        let json = serde_json::to_string(&notif).unwrap();
        assert!(!json.contains("\"id\""));
    }

    #[test]
    fn test_response_success() {
        let resp = JsonRpcResponse::success(1.into(), serde_json::json!({"ok": true}));
        assert!(!resp.is_error());
    }

    #[test]
    fn test_response_error() {
        let resp = JsonRpcResponse::error(1.into(), JsonRpcError::method_not_found());
        assert!(resp.is_error());
        assert_eq!(resp.error.unwrap().code, -32601);
    }
}
