// Mr. Support — AI support, human in control.
// Open-source Rust remote support with multi-agent swarm.
// Platforms: Windows, macOS, Linux, Android, iOS

use mr_support::{agent, audit, consent, platform, swarm, transport};

fn main() {
    println!(
        "Mr. Support v{} — AI support, human in control.",
        mr_support::VERSION
    );
    println!("Platform: {:?}", platform::Platform::current());
    println!(
        "Swarm specialists: {}",
        swarm::coordinator::default_specialists().len()
    );
    println!(
        "Consent default policy: {:?}",
        consent::Policy::default().assess_risk(&agent::Action::Wait { ms: 100 })
    );
    let _ = audit::AuditLog::new();
    let _ = transport::TransportConfig::default();
    println!("Multi-agent swarm + consent model ready for M1.");
}
