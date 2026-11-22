// Frame tap: raw frames from scrap before encoding, for AI vision
use crate::agent::Screenshot;

pub struct FrameTap {
    pub fps: u32,
    pub max_width: u32,
}

impl Default for FrameTap {
    fn default() -> Self { Self { fps: 2, max_width: 1280 } }
}

impl FrameTap {
    pub fn capture_to_screenshot(&self, width: u32, height: u32, pixels: Vec<u8>, ts: u64) -> Screenshot {
        Screenshot::new(width, height, pixels, ts)
    }
}
