// Mr. Support — AI support, human in control.
// Open-source Rust remote support with multi-agent swarm.
// Platforms: Windows, macOS, Linux, Android, iOS

use mr_support::connectivity::id::DeviceId;
use mr_support::{agent, audit, consent, platform, swarm, transport};

fn arg_value(name: &str) -> Option<String> {
    let mut args = std::env::args();
    while let Some(a) = args.next() {
        if a == name {
            return args.next();
        }
        if let Some(v) = a.strip_prefix(&format!("{name}=")) {
            return Some(v.to_string());
        }
    }
    None
}

fn main() {
    let device_id = match arg_value("--id") {
        Some(s) => DeviceId::parse(&s).unwrap_or_else(|| {
            eprintln!("Invalid --id '{s}', expected 9 digits");
            std::process::exit(2);
        }),
        None => DeviceId::load_or_create(&DeviceId::default_path()),
    };
    let device_id_str = device_id.to_string();

    let remote_id = arg_value("--remote-id");
    let server = arg_value("--server");
    let relay = arg_value("--relay");
    #[cfg(feature = "gui")]
    let gui = std::env::args().any(|a| a == "--gui");

    #[cfg(feature = "gui")]
    {
        if gui {
            if let Err(e) = mr_support::ui::egui_app::run_gui(device_id_str) {
                eprintln!("GUI error: {e}");
                std::process::exit(1);
            }
            return;
        }
    }

    println!(
        "Mr. Support v{} — AI support, human in control.",
        mr_support::VERSION
    );
    println!("Device ID: {device_id_str}");
    if let Some(r) = &remote_id {
        println!("Remote ID: {r}");
    }
    if let Some(s) = &server {
        println!("Server: {s}");
    }
    if let Some(r) = &relay {
        println!("Relay: {r}");
    }
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
