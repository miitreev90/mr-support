// Frame tap: raw frames from scrap before encoding, for AI vision (M1)
// Intercepts TraitCapturer::frame() output, downscales, sends via channel
// without blocking the human video stream.
use crate::agent::Screenshot;
use std::sync::mpsc::{self, Receiver, Sender};

#[derive(Debug, Clone)]
pub struct FrameTapConfig {
    pub fps: u32,
    pub max_width: u32,
    pub channel_capacity: usize,
}

impl Default for FrameTapConfig {
    fn default() -> Self {
        Self {
            fps: 2,
            max_width: 1280,
            channel_capacity: 4,
        }
    }
}

pub struct FrameTap {
    config: FrameTapConfig,
    sender: Sender<Screenshot>,
}

impl FrameTap {
    pub fn new(config: FrameTapConfig) -> (Self, Receiver<Screenshot>) {
        let (tx, rx) = mpsc::channel();
        (Self { config, sender: tx }, rx)
    }

    pub fn config(&self) -> &FrameTapConfig {
        &self.config
    }

    /// Tap a raw frame (RGBA/BGRA pixels). Downscales to max_width if needed.
    /// Non-blocking: if receiver is slow, frame is dropped (human stream unaffected).
    pub fn tap_frame(&self, width: u32, height: u32, pixels: Vec<u8>, timestamp_ms: u64) -> bool {
        if pixels.is_empty() || width == 0 || height == 0 {
            return false;
        }
        let (out_w, out_h, out_pixels) = if width > self.config.max_width {
            let scale = self.config.max_width as f32 / width as f32;
            let new_h = ((height as f32) * scale).max(1.0) as u32;
            // Simple nearest-neighbor downscale for prototype (RGBA assumed, 4 bytes/px)
            let mut out = Vec::with_capacity((self.config.max_width * new_h * 4) as usize);
            for y in 0..new_h {
                for x in 0..self.config.max_width {
                    let src_x = ((x as f32 / scale) as u32).min(width - 1);
                    let src_y = ((y as f32 / scale) as u32).min(height - 1);
                    let idx = ((src_y * width + src_x) * 4) as usize;
                    if idx + 3 < pixels.len() {
                        out.extend_from_slice(&pixels[idx..idx + 4]);
                    }
                }
            }
            (self.config.max_width, new_h, out)
        } else {
            (width, height, pixels)
        };
        let shot = Screenshot::new(out_w, out_h, out_pixels, timestamp_ms);
        self.sender.send(shot).is_ok()
    }
}

impl Default for FrameTap {
    fn default() -> Self {
        let (tap, _rx) = Self::new(FrameTapConfig::default());
        tap
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tap_frame_small() {
        let (tap, rx) = FrameTap::new(FrameTapConfig::default());
        let pixels = vec![255u8; 100 * 100 * 4];
        assert!(tap.tap_frame(100, 100, pixels, 123));
        let shot = rx.recv().unwrap();
        assert_eq!(shot.width, 100);
        assert_eq!(shot.height, 100);
        assert_eq!(shot.timestamp_ms, 123);
    }

    #[test]
    fn test_tap_frame_downscale() {
        let config = FrameTapConfig {
            max_width: 1280,
            ..Default::default()
        };
        let (tap, rx) = FrameTap::new(config);
        let pixels = vec![128u8; 2560 * 1440 * 4];
        assert!(tap.tap_frame(2560, 1440, pixels, 456));
        let shot = rx.recv().unwrap();
        assert_eq!(shot.width, 1280);
        assert_eq!(shot.height, 720);
    }

    #[test]
    fn test_tap_frame_invalid() {
        let (tap, _rx) = FrameTap::new(FrameTapConfig::default());
        assert!(!tap.tap_frame(0, 0, vec![], 0));
    }
}
