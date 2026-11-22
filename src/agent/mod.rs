// Agent layer: vision + action + loop
pub mod action;
pub mod loop_;
pub mod vision;

pub use action::{Action, MouseButton};
pub use loop_::{AgentLoop, LoopResult};
pub use vision::Screenshot;

#[derive(Debug, Clone)]
pub struct AgentContext {
    pub session_id: String,
    pub client_name: String,
    pub task_description: String,
}
