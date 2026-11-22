// Agent layer: vision + grounding loop (screenshot -> model -> action -> verify)
pub mod vision;
pub mod action;

pub use vision::Screenshot;
pub use action::Action;

#[derive(Debug, Clone)]
pub struct AgentContext {
    pub session_id: String,
    pub screenshot: Screenshot,
    pub history: Vec<Action>,
}
