// Action policies: allow-list, risk levels
use crate::agent::Action;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum RiskLevel { Low, Medium, High, Critical }

#[derive(Debug, Clone)]
pub struct Policy {
    pub require_approval_for_destructive: bool,
    pub allow_unattended: bool,
}

impl Default for Policy {
    fn default() -> Self { Self { require_approval_for_destructive: true, allow_unattended: false } }
}

impl Policy {
    pub fn assess(&self, action: &Action) -> RiskLevel {
        if action.is_destructive() { RiskLevel::Critical } else { RiskLevel::Low }
    }
    pub fn needs_approval(&self, action: &Action) -> bool {
        self.require_approval_for_destructive && action.is_destructive()
    }
}
