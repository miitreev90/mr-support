// Agent loop: frame -> decision (placeholder for LLM) -> consent check -> action -> audit (M2)
use super::action::Action;
use crate::audit::AuditLog;
use crate::consent::ConsentSession;
use crate::swarm::Coordinator;
use std::sync::mpsc::Receiver;

pub struct AgentLoop {
    pub coordinator: Coordinator,
    pub consent: ConsentSession,
    pub audit: AuditLog,
    pub frame_rx: Receiver<crate::agent::Screenshot>,
    pub step_count: u64,
}

impl AgentLoop {
    pub fn new(
        coordinator: Coordinator,
        consent: ConsentSession,
        frame_rx: Receiver<crate::agent::Screenshot>,
    ) -> Self {
        Self {
            coordinator,
            consent,
            audit: AuditLog::new(),
            frame_rx,
            step_count: 0,
        }
    }

    /// One loop iteration: receive screenshot, decide action, check consent, execute, log.
    /// Decision is a placeholder heuristic until LLM integration (M4).
    pub fn step(&mut self) -> LoopResult {
        if !self.consent.is_active() {
            return LoopResult::ConsentInactive;
        }
        let shot = match self.frame_rx.try_recv() {
            Ok(s) => s,
            Err(_) => return LoopResult::NoFrame,
        };
        self.step_count += 1;
        // Placeholder decision: click center of screen
        let action = Action::Click {
            x: (shot.width / 2) as i32,
            y: (shot.height / 2) as i32,
            button: super::action::MouseButton::Left,
        };
        // Consent policy check
        if action.is_destructive() && !self.consent.is_active() {
            return LoopResult::BlockedByPolicy;
        }
        // Enqueue through coordinator (single input queue)
        self.coordinator.enqueue_action(action.clone());
        // Audit log
        self.audit.record(crate::audit::AuditEntry {
            timestamp_ms: shot.timestamp_ms,
            session_id: self.consent.session_id.clone(),
            agent_name: "Coordinator".to_string(),
            action_description: format!("{:?}", action),
            approved_by: self.consent.client_name.clone(),
            result: "queued".to_string(),
        });
        LoopResult::ActionQueued
    }

    pub fn drain_actions(&mut self) -> Vec<Action> {
        let mut actions = Vec::new();
        while let Some(a) = self.coordinator.next_action() {
            actions.push(a);
        }
        actions
    }
}

#[derive(Debug, PartialEq)]
pub enum LoopResult {
    ActionQueued,
    NoFrame,
    ConsentInactive,
    BlockedByPolicy,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agent::Screenshot;
    use crate::consent::ConsentSession;
    use crate::swarm::coordinator::default_specialists;
    use crate::swarm::Coordinator;
    use std::sync::mpsc;

    #[test]
    fn test_agent_loop_step() {
        let (tx, rx) = mpsc::channel();
        let mut consent = ConsentSession::new("s1".into(), "Test".into(), "Fix".into(), 0);
        consent.approve();
        let coord = Coordinator::new(default_specialists());
        let mut agent_loop = AgentLoop::new(coord, consent, rx);
        tx.send(Screenshot::new(1920, 1080, vec![0u8; 100], 1000))
            .unwrap();
        assert_eq!(agent_loop.step(), LoopResult::ActionQueued);
        assert_eq!(agent_loop.step_count, 1);
        assert_eq!(agent_loop.audit.len(), 1);
        let actions = agent_loop.drain_actions();
        assert_eq!(actions.len(), 1);
    }

    #[test]
    fn test_agent_loop_no_consent() {
        let (_tx, rx) = mpsc::channel::<Screenshot>();
        let consent = ConsentSession::new("s1".into(), "Test".into(), "Fix".into(), 0);
        let coord = Coordinator::new(default_specialists());
        let mut agent_loop = AgentLoop::new(coord, consent, rx);
        assert_eq!(agent_loop.step(), LoopResult::ConsentInactive);
    }

    #[test]
    fn test_agent_loop_no_frame() {
        let (_tx, rx) = mpsc::channel::<Screenshot>();
        let mut consent = ConsentSession::new("s1".into(), "Test".into(), "Fix".into(), 0);
        consent.approve();
        let coord = Coordinator::new(default_specialists());
        let mut agent_loop = AgentLoop::new(coord, consent, rx);
        assert_eq!(agent_loop.step(), LoopResult::NoFrame);
    }
}
