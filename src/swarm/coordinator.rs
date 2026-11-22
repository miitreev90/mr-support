// Swarm coordinator: single input queue, dispatch to specialists (M2b)
use super::specialist::{Specialist, SpecialistKind};
use crate::agent::Action;
use std::collections::VecDeque;

pub struct Coordinator {
    specialists: Vec<Specialist>,
    input_queue: VecDeque<Action>,
    shared_memory: Vec<String>,
}

impl Coordinator {
    pub fn new(specialists: Vec<Specialist>) -> Self {
        Self {
            specialists,
            input_queue: VecDeque::new(),
            shared_memory: Vec::new(),
        }
    }

    pub fn specialists(&self) -> &[Specialist] {
        &self.specialists
    }

    pub fn enqueue_action(&mut self, action: Action) {
        self.input_queue.push_back(action);
    }

    pub fn next_action(&mut self) -> Option<Action> {
        self.input_queue.pop_front()
    }

    pub fn pending_count(&self) -> usize {
        self.input_queue.len()
    }

    /// Dispatch a task to the specialist matching the kind (M2b)
    pub fn dispatch(&mut self, kind: SpecialistKind, task: &str) -> bool {
        if let Some(s) = self.specialists.iter_mut().find(|s| s.kind == kind) {
            s.assign(task);
            self.shared_memory.push(format!("{:?}: {}", kind, task));
            true
        } else {
            false
        }
    }

    pub fn shared_memory(&self) -> &[String] {
        &self.shared_memory
    }
}

pub fn default_specialists() -> Vec<Specialist> {
    vec![
        Specialist::new(SpecialistKind::Diagnostics),
        Specialist::new(SpecialistKind::Network),
        Specialist::new(SpecialistKind::Apps),
        Specialist::new(SpecialistKind::Security),
        Specialist::new(SpecialistKind::Files),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dispatch_specialist() {
        let mut c = Coordinator::new(default_specialists());
        assert!(c.dispatch(SpecialistKind::Network, "check DNS"));
        assert_eq!(c.shared_memory().len(), 1);
        assert!(c.shared_memory()[0].contains("check DNS"));
    }
}
