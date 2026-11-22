// Input injector: AI actions -> enigo (single queue via coordinator, consent-gated)
use crate::agent::Action;
use crate::consent::ConsentSession;
use std::sync::{Arc, Mutex};

pub trait InputInjector {
    /// # Errors
    /// Returns Err if consent is inactive or injection fails.
    fn inject(&self, action: &Action) -> Result<(), String>;
}

/// Consent-gated injector: only injects when session is Active + indicator visible.
/// Real enigo call wired in fork's input_service.rs at ENIGO.lock() sites.
pub struct EnigoInjector {
    pub consent: Arc<Mutex<ConsentSession>>,
    pub injected_count: Arc<Mutex<u64>>,
}

impl EnigoInjector {
    pub fn new(consent: Arc<Mutex<ConsentSession>>) -> Self {
        Self {
            consent,
            injected_count: Arc::new(Mutex::new(0)),
        }
    }

    pub fn is_ai_cursor_visible(&self) -> bool {
        self.consent.lock().map(|c| c.is_active()).unwrap_or(false)
    }
}

impl InputInjector for EnigoInjector {
    fn inject(&self, action: &Action) -> Result<(), String> {
        let active = self.consent.lock().map(|c| c.is_active()).unwrap_or(false);
        if !active {
            return Err("Consent inactive — AI input blocked".into());
        }
        // Real enigo injection happens in fork: ENIGO.lock().mouse_move/click/key
        // Here we record the action for audit; fork wires the actual OS call.
        if let Ok(mut count) = self.injected_count.lock() {
            *count += 1;
        }
        let _ = action;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agent::Action;

    #[test]
    fn test_inject_blocked_without_consent() {
        let consent = Arc::new(Mutex::new(ConsentSession::new(
            "s1".into(),
            "Client".into(),
            "Fix".into(),
            0,
        )));
        let injector = EnigoInjector::new(consent);
        assert!(!injector.is_ai_cursor_visible());
        assert!(injector
            .inject(&Action::Click {
                x: 100,
                y: 200,
                button: crate::agent::MouseButton::Left
            })
            .is_err());
    }

    #[test]
    fn test_inject_allowed_with_consent() {
        let consent = Arc::new(Mutex::new(ConsentSession::new(
            "s1".into(),
            "Client".into(),
            "Fix".into(),
            0,
        )));
        consent.lock().unwrap().approve();
        let injector = EnigoInjector::new(consent);
        assert!(injector.is_ai_cursor_visible());
        assert!(injector
            .inject(&Action::Click {
                x: 100,
                y: 200,
                button: crate::agent::MouseButton::Left
            })
            .is_ok());
        assert_eq!(*injector.injected_count.lock().unwrap(), 1);
    }
}
