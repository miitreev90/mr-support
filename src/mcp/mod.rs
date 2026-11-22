// MCP server: exposes Mr. Support to external AI assistants (Muse, OpenCode, Codex, Claude Code)
// No API key required in Mr. Support itself — the connected assistant brings its own LLM.
pub mod server;
pub mod tools;

pub use server::McpServer;
pub use tools::{McpRequest, McpResponse, Tool};
