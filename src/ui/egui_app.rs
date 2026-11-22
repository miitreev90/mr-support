// Real egui UI matching approved mockup (same colors/layout)
// Enabled with --features gui. Core stays headless for CI.
#[cfg(feature = "gui")]
pub struct MrSupportApp {
    pub device_id: String,
    pub remote_id: String,
    pub ai_active: bool,
    pub mcp_assistant: String,
    pub audit_lines: Vec<String>,
}

#[cfg(feature = "gui")]
impl Default for MrSupportApp {
    fn default() -> Self {
        Self {
            device_id: "123 456 789".into(),
            remote_id: String::new(),
            ai_active: true,
            mcp_assistant: "Muse".into(),
            audit_lines: vec![
                "10:42:01  Diagnostics  Screenshot captured".into(),
                "10:42:15  Network  Click approved by user".into(),
                "10:42:30  Apps  Type queued (12 chars)".into(),
            ],
        }
    }
}

#[cfg(feature = "gui")]
impl eframe::App for MrSupportApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Light theme matching mockup
        let mut visuals = egui::Visuals::light();
        visuals.panel_fill = egui::Color32::from_rgb(248, 249, 250);
        visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(255, 255, 255);
        ctx.set_visuals(visuals);

        egui::TopBottomPanel::top("header").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Mr. Support");
                ui.label(format!("ID: {}", self.device_id));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let color = if self.ai_active {
                        egui::Color32::from_rgb(34, 197, 94)
                    } else {
                        egui::Color32::from_rgb(156, 163, 175)
                    };
                    ui.colored_label(
                        color,
                        if self.ai_active {
                            "● AI ON"
                        } else {
                            "● AI OFF"
                        },
                    );
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Remote ID:");
                ui.text_edit_singleline(&mut self.remote_id);
                if ui.button("Connect").clicked() {
                    self.audit_lines.push("Connect requested".into());
                }
            });
            ui.horizontal(|ui| {
                if ui
                    .add(egui::Button::new("Approve").fill(egui::Color32::from_rgb(34, 197, 94)))
                    .clicked()
                {
                    self.ai_active = true;
                }
                if ui
                    .add(egui::Button::new("Revoke").fill(egui::Color32::from_rgb(239, 68, 68)))
                    .clicked()
                {
                    self.ai_active = false;
                    self.audit_lines.push("Session revoked by user".into());
                }
            });
            if self.ai_active {
                ui.colored_label(
                    egui::Color32::from_rgb(34, 197, 94),
                    "AI operating — you can revoke anytime",
                );
            } else {
                ui.colored_label(
                    egui::Color32::from_rgb(156, 163, 175),
                    "AI off — waiting for consent",
                );
            }
            ui.separator();
            ui.label("Swarm: Coordinator + 5 specialists");
            ui.label(format!(
                "MCP: {} connected (bringing its own LLM)",
                self.mcp_assistant
            ));
            ui.separator();
            ui.label("Audit log:");
            for line in &self.audit_lines {
                ui.label(line);
            }
            if ui.button("Export JSON").clicked() {
                self.audit_lines.push("Audit exported".into());
            }
        });
    }
}

#[cfg(feature = "gui")]
pub fn run_gui() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([720.0, 520.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Mr. Support",
        options,
        Box::new(|_cc| Ok(Box::new(MrSupportApp::default()))),
    )
}
