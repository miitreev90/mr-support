// Real egui UI v2 — polished to match approved mockup (same colors/layout)
// Enabled with --features gui. Core stays headless for CI.
#[cfg(feature = "gui")]
pub struct MrSupportApp {
    pub device_id: String,
    pub remote_id: String,
    pub ai_active: bool,
    pub mcp_assistant: String,
    pub audit_lines: Vec<String>,
    pub logo_texture: Option<egui::TextureHandle>,
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
            logo_texture: None,
        }
    }
}

#[cfg(feature = "gui")]
impl MrSupportApp {
    fn load_logo(&mut self, ctx: &egui::Context) {
        if self.logo_texture.is_some() {
            return;
        }
        let bytes = include_bytes!("../../assets/logo.jpg");
        if let Ok(img) = image::load_from_memory(bytes) {
            let rgba = img.to_rgba8();
            let (w, h) = rgba.dimensions();
            let color_image = egui::ColorImage::from_rgba_unmultiplied(
                [w as usize, h as usize],
                rgba.as_flat_samples().as_slice(),
            );
            self.logo_texture =
                Some(ctx.load_texture("logo", color_image, egui::TextureOptions::LINEAR));
        }
    }
}

#[cfg(feature = "gui")]
impl eframe::App for MrSupportApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.load_logo(ctx);
        let mut visuals = egui::Visuals::light();
        visuals.panel_fill = egui::Color32::from_rgb(248, 249, 250);
        visuals.window_fill = egui::Color32::from_rgb(255, 255, 255);
        visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(255, 255, 255);
        visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(240, 253, 244);
        visuals.widgets.active.bg_fill = egui::Color32::from_rgb(220, 252, 231);
        ctx.set_visuals(visuals);

        egui::TopBottomPanel::top("header")
            .frame(
                egui::Frame::none()
                    .fill(egui::Color32::from_rgb(255, 255, 255))
                    .stroke(egui::Stroke::new(
                        1.0f32,
                        egui::Color32::from_rgb(229, 231, 235),
                    )),
            )
            .show(ctx, |ui| {
                ui.add_space(8.0f32);
                ui.horizontal(|ui| {
                    if let Some(tex) = &self.logo_texture {
                        ui.add(
                            egui::Image::from_texture(tex)
                                .fit_to_exact_size(egui::vec2(48.0f32, 48.0f32)),
                        );
                    }
                    ui.vertical(|ui| {
                        ui.heading("Mr. Support");
                        ui.label("AI support, human in control");
                    });
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.add_space(12.0f32);
                        let (dot, label) = if self.ai_active {
                            (egui::Color32::from_rgb(34, 197, 94), "AI ON")
                        } else {
                            (egui::Color32::from_rgb(156, 163, 175), "AI OFF")
                        };
                        ui.colored_label(dot, format!("● {}", label));
                        ui.label(format!("ID: {}", self.device_id));
                    });
                });
                ui.add_space(8.0f32);
            });

        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(egui::Color32::from_rgb(248, 249, 250)))
            .show(ctx, |ui| {
                ui.add_space(12.0f32);
                egui::Frame::group(ui.style())
                    .fill(egui::Color32::from_rgb(255, 255, 255))
                    .stroke(egui::Stroke::new(
                        1.0f32,
                        egui::Color32::from_rgb(229, 231, 235),
                    ))
                    .inner_margin(12.0f32)
                    .show(ui, |ui| {
                        ui.label("Connect to remote device");
                        ui.horizontal(|ui| {
                            ui.label("Remote ID:");
                            ui.add_sized(
                                [200.0f32, 28.0f32],
                                egui::TextEdit::singleline(&mut self.remote_id)
                                    .hint_text("123 456 789"),
                            );
                            if ui
                                .add_sized([100.0f32, 28.0f32], egui::Button::new("Connect"))
                                .clicked()
                            {
                                self.audit_lines.push("Connect requested".into());
                            }
                        });
                        ui.add_space(8.0f32);
                        ui.horizontal(|ui| {
                            if ui
                                .add_sized(
                                    [120.0f32, 36.0f32],
                                    egui::Button::new(
                                        egui::RichText::new("Approve").color(egui::Color32::WHITE),
                                    )
                                    .fill(egui::Color32::from_rgb(34, 197, 94)),
                                )
                                .clicked()
                            {
                                self.ai_active = true;
                            }
                            if ui
                                .add_sized(
                                    [120.0f32, 36.0f32],
                                    egui::Button::new(
                                        egui::RichText::new("Revoke").color(egui::Color32::WHITE),
                                    )
                                    .fill(egui::Color32::from_rgb(239, 68, 68)),
                                )
                                .clicked()
                            {
                                self.ai_active = false;
                                self.audit_lines.push("Session revoked by user".into());
                            }
                        });
                        ui.add_space(8.0f32);
                        if self.ai_active {
                            egui::Frame::none()
                                .fill(egui::Color32::from_rgb(220, 252, 231))
                                .stroke(egui::Stroke::new(
                                    1.0f32,
                                    egui::Color32::from_rgb(34, 197, 94),
                                ))
                                .inner_margin(8.0f32)
                                .show(ui, |ui| {
                                    ui.colored_label(
                                        egui::Color32::from_rgb(22, 163, 74),
                                        "AI operating — you can revoke anytime",
                                    );
                                });
                        } else {
                            ui.colored_label(
                                egui::Color32::from_rgb(107, 114, 128),
                                "AI off — waiting for consent",
                            );
                        }
                    });
                ui.add_space(12.0f32);
                ui.columns(2, |cols| {
                    egui::Frame::group(cols[0].style())
                        .fill(egui::Color32::from_rgb(255, 255, 255))
                        .stroke(egui::Stroke::new(
                            1.0f32,
                            egui::Color32::from_rgb(229, 231, 235),
                        ))
                        .inner_margin(12.0f32)
                        .show(&mut cols[0], |ui| {
                            ui.label("Swarm");
                            ui.label("Coordinator + 5 specialists");
                            ui.label("Diagnostics · Network · Apps · Security · Files");
                        });
                    egui::Frame::group(cols[1].style())
                        .fill(egui::Color32::from_rgb(255, 255, 255))
                        .stroke(egui::Stroke::new(
                            1.0f32,
                            egui::Color32::from_rgb(229, 231, 235),
                        ))
                        .inner_margin(12.0f32)
                        .show(&mut cols[1], |ui| {
                            ui.label("MCP");
                            ui.label(format!("{} connected", self.mcp_assistant));
                            ui.label("Bringing its own LLM — no API key in Mr. Support");
                        });
                });
                ui.add_space(12.0f32);
                egui::Frame::group(ui.style())
                    .fill(egui::Color32::from_rgb(255, 255, 255))
                    .stroke(egui::Stroke::new(
                        1.0f32,
                        egui::Color32::from_rgb(229, 231, 235),
                    ))
                    .inner_margin(12.0f32)
                    .show(ui, |ui| {
                        ui.label("Audit log");
                        egui::Grid::new("audit_grid").striped(true).show(ui, |ui| {
                            ui.label("Time");
                            ui.label("Agent");
                            ui.label("Action");
                            ui.end_row();
                            for line in &self.audit_lines {
                                let parts: Vec<&str> = line.splitn(3, "  ").collect();
                                if parts.len() == 3 {
                                    ui.label(parts[0]);
                                    ui.label(parts[1]);
                                    ui.label(parts[2]);
                                } else {
                                    ui.label(line);
                                    ui.label("");
                                    ui.label("");
                                }
                                ui.end_row();
                            }
                        });
                        ui.add_space(8.0f32);
                        if ui.button("Export JSON").clicked() {
                            self.audit_lines.push("Audit exported".into());
                        }
                    });
            });
    }
}

#[cfg(feature = "gui")]
pub fn run_gui() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([760.0f32, 560.0f32]),
        ..Default::default()
    };
    eframe::run_native(
        "Mr. Support",
        options,
        Box::new(|_cc| Ok(Box::new(MrSupportApp::default()))),
    )
}
