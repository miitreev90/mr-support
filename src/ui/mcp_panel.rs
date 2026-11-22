// MCP panel: shows connected external assistant (Muse, OpenCode, Codex, Claude Code)
pub struct McpPanel {
    pub connected_assistant: Option<String>,
}

impl McpPanel {
    pub fn new() -> Self {
        Self {
            connected_assistant: None,
        }
    }

    pub fn connect(&mut self, name: &str) {
        self.connected_assistant = Some(name.to_string());
    }

    pub fn status(&self) -> String {
        match &self.connected_assistant {
            Some(name) => format!(
                "MCP connected: {} (bringing its own LLM, no API key in Mr. Support)",
                name
            ),
            None => "MCP disconnected — no external assistant".into(),
        }
    }
}

impl Default for McpPanel {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mcp_panel_status() {
        let mut panel = McpPanel::new();
        assert!(panel.status().contains("disconnected"));
        panel.connect("Muse");
        assert!(panel.status().contains("Muse"));
    }
}
