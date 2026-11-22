// Rendezvous client: register ID, lookup peer (RustDesk hbbs equivalent)
use super::id::DeviceId;

#[derive(Debug, Clone)]
pub struct PeerInfo {
    pub id: DeviceId,
    pub address: String,
    pub relay: Option<String>,
}

pub struct RendezvousClient {
    pub server: String,
    pub registered_id: Option<DeviceId>,
}

impl RendezvousClient {
    pub fn new(server: impl Into<String>) -> Self {
        Self {
            server: server.into(),
            registered_id: None,
        }
    }

    pub fn register(&mut self, id: DeviceId) -> bool {
        self.registered_id = Some(id);
        true
    }

    pub fn lookup(&self, id: &DeviceId) -> Option<PeerInfo> {
        // Placeholder: real impl queries rendezvous server via hbb_common proto
        if self.registered_id.as_ref() == Some(id) {
            Some(PeerInfo {
                id: id.clone(),
                address: format!("{}:21116", self.server),
                relay: Some(format!("{}:21117", self.server)),
            })
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_and_lookup() {
        let mut client = RendezvousClient::new("rendezvous.example.com");
        let id = DeviceId::new(987654321);
        assert!(client.register(id.clone()));
        let peer = client.lookup(&id).unwrap();
        assert_eq!(peer.id, id);
        assert!(peer.address.contains("21116"));
    }

    #[test]
    fn test_lookup_unknown() {
        let client = RendezvousClient::new("rendezvous.example.com");
        assert!(client.lookup(&DeviceId::new(111111111)).is_none());
    }
}
