// Device ID: 9-digit numeric ID like AnyDesk/TeamViewer/RustDesk
use std::fmt;

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
}
