// Coordinator: serializes input actions, consolidates plan for human approval
use super::specialist::{Specialist, SpecialistKind};
use crate::agent::Action;
use std::collections::VecDeque;

pub struct Coordinator {
    specialists: Vec<Specialist>,
    input_queue: VecDeque<Action>,
}

impl Coordinator {
    pub fn new(specialists: Vec<Specialist>) -> Self {
        Self { specialists, input_queue: VecDeque::new() }
    }

    pub fn specialists(&self) -> &[Specialist] { &self.specialists }

    pub fn enqueue_action(&mut self, action: Action) {
        // Single queue to enigo — prevents conflicts between agents
        self.input_queue.push_back(action);
    }

    pub fn next_action(&mut self) -> Option<Action> {
        self.input_queue.pop_front()
    }

    pub fn pending_count(&self) -> usize { self.input_queue.len() }
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
