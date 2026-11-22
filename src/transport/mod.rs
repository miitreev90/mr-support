// Transport: RustDesk integration (scrap capture, enigo input, hbb_common)
pub mod frame_tap;
pub mod input;

pub use frame_tap::FrameTap;
pub use input::InputInjector;

#[derive(Debug, Clone)]
pub struct TransportConfig {
    pub relay_host: String,
    pub use_e2ee: bool,
}

impl Default for TransportConfig {
    fn default() -> Self { Self { relay_host: "localhost".to_string(), use_e2ee: true } }
}
