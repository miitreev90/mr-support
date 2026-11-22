// In-memory audit log (persist to JSON later)
use super::AuditEntry;

#[derive(Debug, Default)]
pub struct AuditLog {
    entries: Vec<AuditEntry>,
}

impl AuditLog {
    pub fn new() -> Self { Self::default() }
    pub fn record(&mut self, entry: AuditEntry) { self.entries.push(entry); }
    pub fn entries(&self) -> &[AuditEntry] { &self.entries }
    pub fn len(&self) -> usize { self.entries.len() }
    pub fn is_empty(&self) -> bool { self.entries.is_empty() }
    pub fn export_json(&self) -> String {
        serde_json::to_string_pretty(&self.entries).unwrap_or_else(|_| "[]".to_string())
    }
}
