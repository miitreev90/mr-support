// Consent model: explicit per-session approval, visible indicator, instant revocation
pub mod policy;
pub mod session;

pub use policy::{Policy, RiskLevel};
pub use session::{ConsentSession, SessionState};

#[derive(Debug, Clone, PartialEq)]
pub enum ConsentDecision {
    Approved,
    Denied,
    Revoked,
}
