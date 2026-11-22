// Specialized agents: each limited to its domain
#[derive(Debug, Clone, PartialEq)]
pub enum SpecialistKind {
    Diagnostics,
    Network,
    Apps,
    Security,
    Files,
}

#[derive(Debug, Clone)]
pub struct Specialist {
    pub kind: SpecialistKind,
    pub name: String,
}

impl Specialist {
    pub fn new(kind: SpecialistKind) -> Self {
        let name = match kind {
            SpecialistKind::Diagnostics => "Diagnostics",
            SpecialistKind::Network => "Network",
            SpecialistKind::Apps => "Apps",
            SpecialistKind::Security => "Security",
            SpecialistKind::Files => "Files",
        }.to_string();
        Self { kind, name }
    }
}
