// Relay client: fallback when direct NAT traversal fails (RustDesk hbbr equivalent)
use super::id::DeviceId;

pub struct RelayClient {
    pub server: String,
    pub connected: bool,
}

impl RelayClient {
    pub fn new(server: impl Into<String>) -> Self {
        Self {
            server: server.into(),
            connected: false,
        }
    }

    pub fn connect(&mut self, _local_id: &DeviceId, _peer_id: &DeviceId) -> bool {
        // Placeholder: real impl opens WebSocket/TCP to relay server
        self.connected = true;
        true
    }

    pub fn disconnect(&mut self) {
        self.connected = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_relay_connect_disconnect() {
        let mut relay = RelayClient::new("relay.example.com");
        assert!(!relay.connected);
        assert!(relay.connect(&DeviceId::new(111111111), &DeviceId::new(222222222)));
        assert!(relay.connected);
        relay.disconnect();
        assert!(!relay.connected);
    }
}
