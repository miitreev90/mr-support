// Consent policy: risk-based approval (M3/M4)
#[derive(Debug, Clone, PartialEq)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone)]
pub struct Policy {
    pub max_actions_per_minute: u32,
    pub allowlist: Vec<String>,
}

impl Default for Policy {
    fn default() -> Self {
        Self {
            max_actions_per_minute: 30,
            allowlist: vec![
                "Click".to_string(),
                "Type".to_string(),
                "KeyPress".to_string(),
                "Scroll".to_string(),
                "Wait".to_string(),
            ],
        }
    }
}

impl Policy {
    pub fn assess_risk(&self, action: &crate::agent::Action) -> RiskLevel {
        if action.is_destructive() {
            RiskLevel::Critical
        } else {
            RiskLevel::Low
        }
    }

    pub fn needs_approval(&self, risk: &RiskLevel) -> bool {
        matches!(risk, RiskLevel::High | RiskLevel::Critical)
    }

    pub fn is_allowed(&self, action_name: &str) -> bool {
        self.allowlist.iter().any(|a| a == action_name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agent::Action;

    #[test]
    fn test_policy_assess() {
        let p = Policy::default();
        let click = Action::Click {
            x: 0,
            y: 0,
            button: crate::agent::MouseButton::Left,
        };
        assert_eq!(p.assess_risk(&click), RiskLevel::Low);
        assert!(!p.needs_approval(&RiskLevel::Low));
        assert!(p.needs_approval(&RiskLevel::Critical));
    }

    #[test]
    fn test_allowlist() {
        let p = Policy::default();
        assert!(p.is_allowed("Click"));
        assert!(!p.is_allowed("DeleteSystem"));
    }
}
