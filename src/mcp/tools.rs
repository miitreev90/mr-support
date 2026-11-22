// MCP tools exposed to external assistants
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Tool {
    GetScreenshot,
    Click { x: i32, y: i32 },
    Type { text: String },
    KeyPress { key: String },
    Scroll { dx: i32, dy: i32 },
    RevokeSession,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpRequest {
    pub tool: Tool,
    pub session_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpResponse {
    pub success: bool,
    pub message: String,
}

impl McpResponse {
    pub fn ok(msg: impl Into<String>) -> Self {
        Self {
            success: true,
            message: msg.into(),
        }
    }
    pub fn err(msg: impl Into<String>) -> Self {
        Self {
            success: false,
            message: msg.into(),
        }
    }
}
