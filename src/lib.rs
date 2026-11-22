// Mr. Support library root.
// Modules: agent, swarm, consent, audit, transport, platform
pub mod agent;
pub mod swarm;
pub mod consent;
pub mod audit;
pub mod transport;
pub mod platform;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
