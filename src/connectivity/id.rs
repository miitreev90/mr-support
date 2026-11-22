// Device ID: 9-digit numeric ID like AnyDesk/TeamViewer/RustDesk
use std::fmt;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DeviceId(pub u64);

impl DeviceId {
    pub fn new(value: u64) -> Self {
        Self(value % 1_000_000_000)
    }
    pub fn generate_from_seed(seed: u64) -> Self {
        // Deterministic placeholder; real impl uses machine fingerprint + random
        Self((seed.wrapping_mul(2654435761) % 900_000_000) + 100_000_000)
    }
    pub fn parse(s: &str) -> Option<Self> {
        let t = s.trim().replace(' ', "");
        if t.len() != 9 || !t.chars().all(|c| c.is_ascii_digit()) {
            return None;
        }
        t.parse::<u64>().ok().map(Self::new)
    }
    pub fn default_path() -> PathBuf {
        let base = std::env::home_dir().unwrap_or_else(|| PathBuf::from("."));
        base.join(".config")
            .join("mr-support")
            .join("device_id.json")
    }
    /// Load persisted ID from `path`, or generate + save a new one.
    pub fn load_or_create(path: &Path) -> Self {
        if let Ok(content) = std::fs::read_to_string(path) {
            if let Ok(v) = serde_json::from_str::<serde_json::Value>(&content) {
                if let Some(s) = v.get("device_id").and_then(|x| x.as_str()) {
                    if let Some(id) = Self::parse(s) {
                        return id;
                    }
                }
            }
        }
        let seed = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(42);
        let id = Self::generate_from_seed(seed);
        let _ = id.save(path);
        id
    }
    pub fn save(&self, path: &Path) -> std::io::Result<()> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let json = serde_json::json!({ "device_id": self.to_string() });
        std::fs::write(path, json.to_string())
    }
}

impl fmt::Display for DeviceId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:09}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_device_id_format() {
        let id = DeviceId::new(123456789);
        assert_eq!(format!("{}", id), "123456789");
    }

    #[test]
    fn test_device_id_deterministic() {
        let a = DeviceId::generate_from_seed(42);
        let b = DeviceId::generate_from_seed(42);
        assert_eq!(a, b);
        assert!(a.0 >= 100_000_000 && a.0 < 1_000_000_000);
    }

    #[test]
    fn test_device_id_parse() {
        assert_eq!(DeviceId::parse("124051724").map(|i| i.0), Some(124051724));
        assert_eq!(DeviceId::parse("124 051 724").map(|i| i.0), Some(124051724));
        assert!(DeviceId::parse("123").is_none());
        assert!(DeviceId::parse("abcdefghi").is_none());
    }

    #[test]
    fn test_device_id_persist_roundtrip() {
        let dir = std::env::temp_dir().join("mr-support-test-id");
        let _ = std::fs::create_dir_all(&dir);
        let path = dir.join("device_id.json");
        let _ = std::fs::remove_file(&path);
        let id = DeviceId::load_or_create(&path);
        assert!(path.exists());
        let loaded = DeviceId::load_or_create(&path);
        assert_eq!(id, loaded);
        let _ = std::fs::remove_file(&path);
    }
}
