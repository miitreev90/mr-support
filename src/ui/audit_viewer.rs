// Audit viewer: shows AI actions with before/after captures, exportable JSON
use crate::audit::AuditLog;

pub struct AuditViewer {
    pub log: AuditLog,
}

impl AuditViewer {
    pub fn new(log: AuditLog) -> Self {
        Self { log }
    }

    pub fn entry_count(&self) -> usize {
        self.log.len()
    }

    pub fn export_json(&self) -> String {
        self.log.export_json()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::audit::AuditEntry;

    #[test]
    fn test_audit_viewer_count_and_export() {
        let mut log = AuditLog::new();
        log.record(AuditEntry {
            timestamp_ms: 0,
            session_id: "s1".into(),
            agent_name: "agent".into(),
            action_description: "click".into(),
            approved_by: "user".into(),
            result: "ok".into(),
        });
        let viewer = AuditViewer::new(log);
        assert_eq!(viewer.entry_count(), 1);
        assert!(viewer.export_json().contains("click"));
    }
}
