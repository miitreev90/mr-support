// Vision input: raw frame tap from RustDesk scrap
#[derive(Debug, Clone)]
pub struct Screenshot {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>, // RGBA or BGRA pixels
    pub timestamp_ms: u64,
}

impl Screenshot {
    pub fn new(width: u32, height: u32, data: Vec<u8>, timestamp_ms: u64) -> Self {
        Self {
            width,
            height,
            data,
            timestamp_ms,
        }
    }
}
