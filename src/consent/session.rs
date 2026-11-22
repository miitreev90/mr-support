// Consent session lifecycle
use super::ConsentDecision;

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
    pub purpose: String,
    pub state: SessionState,
    pub created_at_ms: u64,
}

impl ConsentSession {
    pub fn new(session_id: String, client_name: String, purpose: String, created_at_ms: u64) -> Self {
        Self { session_id, client_name, purpose, state: SessionState::Pending, created_at_ms }
    }

    pub fn approve(&mut self) { self.state = SessionState::Active; }
    pub fn revoke(&mut self) { self.state = SessionState::Revoked; }
    pub fn is_active(&self) -> bool { self.state == SessionState::Active }
    pub fn decision(&self) -> ConsentDecision {
        match self.state {
            SessionState::Active => ConsentDecision::Approved,
            SessionState::Revoked => ConsentDecision::Revoked,
            _ => ConsentDecision::Denied,
        }
    }
}
