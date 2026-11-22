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
    pub current_task: Option<String>,
}

impl Specialist {
    pub fn new(kind: SpecialistKind) -> Self {
        let name = match kind {
            SpecialistKind::Diagnostics => "Diagnostics",
            SpecialistKind::Network => "Network",
            SpecialistKind::Apps => "Apps",
            SpecialistKind::Security => "Security",
            SpecialistKind::Files => "Files",
        }
        .to_string();
        Self {
            kind,
            name,
            current_task: None,
        }
    }

    pub fn assign(&mut self, task: &str) {
        self.current_task = Some(task.to_string());
    }

    pub fn is_busy(&self) -> bool {
        self.current_task.is_some()
    }
}
