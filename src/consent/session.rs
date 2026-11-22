// Consent session: explicit approval, visible indicator, instant revocation (M3)
#[derive(Debug, Clone, PartialEq)]
pub enum SessionState {
    Pending,
    Active,
    Revoked,
    Expired,
    Closed,
}

#[derive(Debug, Clone)]
pub struct ConsentSession {
    pub session_id: String,
    pub client_name: String,
    pub task_description: String,
    pub state: SessionState,
    pub started_at_ms: u64,
    pub indicator_visible: bool,
}

impl ConsentSession {
    pub fn new(
        session_id: String,
        client_name: String,
        task_description: String,
        started_at_ms: u64,
    ) -> Self {
        Self {
            session_id,
            client_name,
            task_description,
            state: SessionState::Pending,
            started_at_ms,
            indicator_visible: false,
        }
    }

    pub fn approve(&mut self) {
        self.state = SessionState::Active;
        self.indicator_visible = true;
    }

    pub fn revoke(&mut self) {
        self.state = SessionState::Revoked;
        self.indicator_visible = false;
    }

    pub fn is_active(&self) -> bool {
        self.state == SessionState::Active && self.indicator_visible
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_revocation_hides_indicator() {
        let mut s = ConsentSession::new("s1".into(), "Client".into(), "Fix".into(), 0);
        assert!(!s.is_active());
        s.approve();
        assert!(s.is_active());
        assert!(s.indicator_visible);
        s.revoke();
        assert!(!s.is_active());
        assert!(!s.indicator_visible);
    }
}
