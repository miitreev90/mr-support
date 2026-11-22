// Audit log: append-only record with before/after captures
pub mod log;

pub use log::{AuditLog, AuditEntry};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntry {
    pub timestamp_ms: u64,
    pub session_id: String,
    pub agent_name: String,
    pub action_description: String,
    pub approved_by: String,
    pub result: String,
}
