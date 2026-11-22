// Swarm: coordinator + specialized agents in parallel
pub mod coordinator;
pub mod specialist;

pub use coordinator::Coordinator;
pub use specialist::{Specialist, SpecialistKind};

#[derive(Debug, Clone)]
pub struct SwarmConfig {
    pub max_agents: usize,
    pub share_captures: bool, // share screenshots between agents to save tokens
}

impl Default for SwarmConfig {
    fn default() -> Self { Self { max_agents: 5, share_captures: true } }
}
