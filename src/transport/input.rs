// Input injector: AI actions -> enigo (single queue via coordinator)
use crate::agent::Action;

pub trait InputInjector {
    fn inject(&self, action: &Action) -> Result<(), String>;
}

// Placeholder for RustDesk enigo integration
pub struct EnigoInjector;

impl InputInjector for EnigoInjector {
    fn inject(&self, _action: &Action) -> Result<(), String> {
        // TODO M1: call ENIGO.lock() in input_service.rs
        Ok(())
    }
}
