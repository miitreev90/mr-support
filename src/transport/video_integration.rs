// Video integration: frame tap wired to RustDesk video_service.rs
// Insertion point in fork: src/server/video_service.rs handle_one_frame(),
// after c.frame() and before encode. Tap sends Screenshot via channel;
// human stream is unaffected (non-blocking, dropped if receiver slow).
use super::frame_tap::FrameTap;
use crate::agent::vision::Screenshot;
use std::sync::mpsc::Receiver;

pub struct VideoTapBridge {
    pub tap: FrameTap,
    pub receiver: Receiver<Screenshot>,
}

impl VideoTapBridge {
    pub fn new(tap: FrameTap, receiver: Receiver<Screenshot>) -> Self {
        Self { tap, receiver }
    }

    /// Called from fork's video_service.rs with raw frame pixels.
    /// Returns true if frame was tapped for AI vision.
    pub fn on_frame(&self, width: u32, height: u32, pixels: Vec<u8>, timestamp_ms: u64) -> bool {
        self.tap.tap_frame(width, height, pixels, timestamp_ms)
    }

    /// Drain tapped screenshots for the agent loop.
    pub fn drain(&self) -> Vec<Screenshot> {
        let mut out = Vec::new();
        while let Ok(shot) = self.receiver.try_recv() {
            out.push(shot);
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transport::frame_tap::FrameTapConfig;

    #[test]
    fn test_video_bridge_tap_and_drain() {
        let (tap, rx) = FrameTap::new(FrameTapConfig::default());
        let bridge = VideoTapBridge::new(tap, rx);
        let pixels = vec![0u8; 100 * 100 * 4];
        assert!(bridge.on_frame(100, 100, pixels, 1234));
        let shots = bridge.drain();
        assert_eq!(shots.len(), 1);
        assert_eq!(shots[0].timestamp_ms, 1234);
    }
}
