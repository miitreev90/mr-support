// AI indicator: visible "AI operating" badge + differentiated cursor
// User can always see when AI is active; revocation hides it instantly.
use crate::consent::ConsentSession;
use std::sync::{Arc, Mutex};

pub struct AiIndicator {
    pub consent: Arc<Mutex<ConsentSession>>,
}

impl AiIndicator {
    pub fn new(consent: Arc<Mutex<ConsentSession>>) -> Self {
        Self { consent }
    }

    pub fn is_visible(&self) -> bool {
        self.consent
            .lock()
            .map(|c| c.indicator_visible)
            .unwrap_or(false)
    }

    pub fn label(&self) -> &'static str {
        if self.is_visible() {
            "AI operating — you can revoke anytime"
        } else {
            ""
        }
    }

    pub fn cursor_style(&self) -> &'static str {
        if self.is_visible() {
            "ai-cursor" // differentiated cursor rendered by UI layer
        } else {
            "normal"
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_indicator_follows_consent() {
        let consent = Arc::new(Mutex::new(ConsentSession::new(
            "s1".into(),
            "Client".into(),
            "Fix".into(),
            0,
        )));
        let indicator = AiIndicator::new(consent.clone());
        assert!(!indicator.is_visible());
        assert_eq!(indicator.cursor_style(), "normal");
        consent.lock().unwrap().approve();
        assert!(indicator.is_visible());
        assert_eq!(indicator.cursor_style(), "ai-cursor");
        consent.lock().unwrap().revoke();
        assert!(!indicator.is_visible());
    }
}
