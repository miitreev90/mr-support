// Connectivity: ID + rendezvous + relay (RustDesk-style, no public IP needed)
pub mod id;
pub mod relay;
pub mod rendezvous;

pub use id::DeviceId;
pub use relay::RelayClient;
pub use rendezvous::RendezvousClient;
