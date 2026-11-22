// MCP server: JSON-RPC over stdio/HTTP, consent-gated
use super::tools::{McpRequest, McpResponse, Tool};
use crate::consent::ConsentSession;

pub struct McpServer {
    pub consent: ConsentSession,
    pub connected_assistant: Option<String>,
}

impl McpServer {
    pub fn new(consent: ConsentSession) -> Self {
        Self {
            consent,
            connected_assistant: None,
        }
    }

    pub fn connect_assistant(&mut self, name: &str) {
        self.connected_assistant = Some(name.to_string());
    }

    pub fn handle(&mut self, req: McpRequest) -> McpResponse {
        if !self.consent.is_active() {
            return McpResponse::err("Consent inactive — user must approve session first");
        }
        if req.session_id != self.consent.session_id {
            return McpResponse::err("Invalid session_id");
        }
        match req.tool {
            Tool::GetScreenshot => McpResponse::ok("Screenshot queued for assistant vision"),
            Tool::Click { x, y } => McpResponse::ok(format!("Click queued at ({}, {})", x, y)),
            Tool::Type { text } => McpResponse::ok(format!("Type queued: {} chars", text.len())),
            Tool::KeyPress { key } => McpResponse::ok(format!("KeyPress queued: {}", key)),
            Tool::Scroll { dx, dy } => McpResponse::ok(format!("Scroll queued: ({}, {})", dx, dy)),
            Tool::RevokeSession => {
                self.consent.revoke();
                McpResponse::ok("Session revoked by assistant request")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::consent::ConsentSession;

    #[test]
    fn test_mcp_requires_consent() {
        let consent = ConsentSession::new("s1".into(), "Client".into(), "Fix".into(), 0);
        let mut server = McpServer::new(consent);
        let resp = server.handle(McpRequest {
            tool: Tool::GetScreenshot,
            session_id: "s1".into(),
        });
        assert!(!resp.success);
    }

    #[test]
    fn test_mcp_with_consent() {
        let mut consent = ConsentSession::new("s1".into(), "Client".into(), "Fix".into(), 0);
        consent.approve();
        let mut server = McpServer::new(consent);
        server.connect_assistant("Muse");
        assert_eq!(server.connected_assistant.as_deref(), Some("Muse"));
        let resp = server.handle(McpRequest {
            tool: Tool::Click { x: 100, y: 200 },
            session_id: "s1".into(),
        });
        assert!(resp.success);
    }

    #[test]
    fn test_mcp_invalid_session() {
        let mut consent = ConsentSession::new("s1".into(), "Client".into(), "Fix".into(), 0);
        consent.approve();
        let mut server = McpServer::new(consent);
        let resp = server.handle(McpRequest {
            tool: Tool::GetScreenshot,
            session_id: "wrong".into(),
        });
        assert!(!resp.success);
    }
}
