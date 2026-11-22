// Agent layer: vision + grounding loop (screenshot -> model -> action -> verify)
pub mod action;
pub mod vision;

pub use action::Action;
pub use vision::Screenshot;

#[derive(Debug, Clone)]
pub struct AgentContext {
    pub session_id: String,
    pub screenshot: Screenshot,
    pub history: Vec<Action>,
}
