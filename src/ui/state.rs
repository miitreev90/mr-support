// UI state: ID, AI indicator, revoke always visible
use crate::connectivity::DeviceId;
use crate::consent::ConsentSession;
use std::sync::{Arc, Mutex};

pub struct UiState {
    pub device_id: DeviceId,
    pub consent: Arc<Mutex<ConsentSession>>,
    pub remote_id_input: String,
}

impl UiState {
    pub fn new(device_id: DeviceId, consent: Arc<Mutex<ConsentSession>>) -> Self {
        Self {
            device_id,
            consent,
            remote_id_input: String::new(),
        }
    }

    pub fn is_ai_active(&self) -> bool {
        self.consent.lock().map(|c| c.is_active()).unwrap_or(false)
    }

    pub fn ai_banner(&self) -> &'static str {
        if self.is_ai_active() {
            "AI operating — you can revoke anytime"
        } else {
            "AI off — waiting for consent"
        }
    }

    pub fn revoke(&self) {
        if let Ok(mut c) = self.consent.lock() {
            c.revoke();
        }
    }

    pub fn can_connect(&self) -> bool {
        self.remote_id_input.len() == 9 && self.remote_id_input.chars().all(|c| c.is_ascii_digit())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ui_state_ai_banner() {
        let consent = Arc::new(Mutex::new(ConsentSession::new(
            "s1".into(),
            "Client".into(),
            "Fix".into(),
            0,
        )));
        let ui = UiState::new(DeviceId::new(123456789), consent.clone());
        assert_eq!(ui.ai_banner(), "AI off — waiting for consent");
        consent.lock().unwrap().approve();
        assert!(ui.ai_banner().contains("AI operating"));
        ui.revoke();
        assert!(!ui.is_ai_active());
    }

    #[test]
    fn test_can_connect_validates_id() {
        let consent = Arc::new(Mutex::new(ConsentSession::new(
            "s1".into(),
            "Client".into(),
            "Fix".into(),
            0,
        )));
        let mut ui = UiState::new(DeviceId::new(123456789), consent);
        assert!(!ui.can_connect());
        ui.remote_id_input = "123456789".into();
        assert!(ui.can_connect());
        ui.remote_id_input = "abc".into();
        assert!(!ui.can_connect());
    }
}
