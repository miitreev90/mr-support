// Mr. Support library root.
// Modules: agent, swarm, consent, audit, transport, platform
pub mod agent;
pub mod audit;
pub mod connectivity;
pub mod consent;
pub mod mcp;
pub mod platform;
pub mod swarm;
pub mod transport;
pub mod ui;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swarm_coordinator_queue() {
        let mut coord = swarm::Coordinator::new(swarm::coordinator::default_specialists());
        assert_eq!(coord.specialists().len(), 5);
        coord.enqueue_action(agent::Action::Click {
            x: 100,
            y: 200,
            button: agent::action::MouseButton::Left,
        });
        assert_eq!(coord.pending_count(), 1);
        assert!(coord.next_action().is_some());
        assert_eq!(coord.pending_count(), 0);
    }

    #[test]
    fn test_consent_session() {
        let mut s =
            consent::ConsentSession::new("s1".into(), "Dr. Ilse".into(), "Install app".into(), 0);
        assert!(!s.is_active());
        s.approve();
        assert!(s.is_active());
        s.revoke();
        assert!(!s.is_active());
    }

    #[test]
    fn test_audit_log() {
        let mut log = audit::AuditLog::new();
        assert!(log.is_empty());
        log.record(audit::AuditEntry {
            timestamp_ms: 0,
            session_id: "s1".into(),
            agent_name: "Network".into(),
            action_description: "click".into(),
            approved_by: "user".into(),
            result: "ok".into(),
        });
        assert_eq!(log.len(), 1);
        assert!(log.export_json().contains("Network"));
    }

    #[test]
    fn test_platform_detection() {
        let p = platform::Platform::current();
        assert!(matches!(
            p,
            platform::Platform::Linux | platform::Platform::MacOS | platform::Platform::Windows
        ));
    }
}
