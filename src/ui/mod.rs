// UI: lightweight Rust interface for AI supervision (not RustDesk Flutter clone)
// Skeleton is GUI-agnostic and testable; egui rendering added behind feature flag.
pub mod audit_viewer;
pub mod consent_dialog;
pub mod mcp_panel;
pub mod state;

pub use audit_viewer::AuditViewer;
pub use consent_dialog::ConsentDialog;
pub use mcp_panel::McpPanel;
pub use state::UiState;
