// Consent dialog: explicit approval before AI session starts
use crate::consent::ConsentSession;
use std::sync::{Arc, Mutex};

pub struct ConsentDialog {
    pub consent: Arc<Mutex<ConsentSession>>,
    pub approved: bool,
}

impl ConsentDialog {
    pub fn new(consent: Arc<Mutex<ConsentSession>>) -> Self {
        Self {
            consent,
            approved: false,
        }
    }

    pub fn approve(&mut self) {
        if let Ok(mut c) = self.consent.lock() {
            c.approve();
        }
        self.approved = true;
    }

    pub fn deny(&mut self) {
        if let Ok(mut c) = self.consent.lock() {
            c.revoke();
        }
        self.approved = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consent_dialog_approve_deny() {
        let consent = Arc::new(Mutex::new(ConsentSession::new(
            "s1".into(),
            "Client".into(),
            "Fix".into(),
            0,
        )));
        let mut dialog = ConsentDialog::new(consent.clone());
        dialog.approve();
        assert!(dialog.approved);
        assert!(consent.lock().unwrap().is_active());
        dialog.deny();
        assert!(!dialog.approved);
        assert!(!consent.lock().unwrap().is_active());
    }
}
