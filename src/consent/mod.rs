// Consent model: explicit per-session approval, visible indicator, instant revocation
pub mod session;
pub mod policy;

pub use session::{ConsentSession, SessionState};
pub use policy::{Policy, RiskLevel};

#[derive(Debug, Clone, PartialEq)]
pub enum ConsentDecision {
    Approved,
    Denied,
    Revoked,
}
