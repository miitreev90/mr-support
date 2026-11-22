// Real egui UI v3 — polished as much as egui allows (rounded cards, shadows, custom theme)
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

    fn card_frame() -> egui::Frame {
        egui::Frame::new()
            .fill(egui::Color32::from_rgb(255, 255, 255))
            .stroke(egui::Stroke::new(
                1.0f32,
                egui::Color32::from_rgb(229, 231, 235),
            ))
            .corner_radius(egui::CornerRadius::same(12))
            .shadow(egui::Shadow {
                offset: [0, 2],
                blur: 8,
                spread: 0,
                color: egui::Color32::from_black_alpha(20),
            })
            .inner_margin(egui::Margin::same(14))
    }

    fn section_title(ui: &mut egui::Ui, text: &str) {
        ui.label(
            egui::RichText::new(text)
                .size(15.0f32)
                .strong()
                .color(egui::Color32::from_rgb(17, 24, 39)),
        );
        ui.add_space(4.0f32);
    }
}

#[cfg(feature = "gui")]
impl eframe::App for MrSupportApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.load_logo(ctx);

        let mut visuals = egui::Visuals::light();
        visuals.panel_fill = egui::Color32::from_rgb(248, 249, 250);
        visuals.window_fill = egui::Color32::from_rgb(255, 255, 255);
        visuals.window_corner_radius = egui::CornerRadius::same(12);
        visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(255, 255, 255);
        visuals.widgets.inactive.corner_radius = egui::CornerRadius::same(8);
        visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(240, 253, 244);
        visuals.widgets.hovered.corner_radius = egui::CornerRadius::same(8);
        visuals.widgets.active.bg_fill = egui::Color32::from_rgb(220, 252, 231);
        visuals.widgets.active.corner_radius = egui::CornerRadius::same(8);
        visuals.selection.bg_fill = egui::Color32::from_rgb(34, 197, 94);
        ctx.set_visuals(visuals);

        ctx.style_mut(|s| {
            s.spacing.item_spacing = egui::vec2(8.0f32, 8.0f32);
            s.spacing.button_padding = egui::vec2(14.0f32, 8.0f32);
            s.text_styles.insert(
                egui::TextStyle::Heading,
                egui::FontId::new(22.0f32, egui::FontFamily::Proportional),
            );
            s.text_styles.insert(
                egui::TextStyle::Body,
                egui::FontId::new(14.0f32, egui::FontFamily::Proportional),
            );
            s.text_styles.insert(
                egui::TextStyle::Button,
                egui::FontId::new(14.0f32, egui::FontFamily::Proportional),
            );
        });

        egui::TopBottomPanel::top("header")
            .frame(
                egui::Frame::new()
                    .fill(egui::Color32::from_rgb(255, 255, 255))
                    .stroke(egui::Stroke::new(
                        1.0f32,
                        egui::Color32::from_rgb(229, 231, 235),
                    ))
                    .inner_margin(egui::Margin::symmetric(16, 10)),
            )
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    if let Some(tex) = &self.logo_texture {
                        ui.add(
                            egui::Image::from_texture(tex)
                                .fit_to_exact_size(egui::vec2(52.0f32, 52.0f32))
                                .corner_radius(egui::CornerRadius::same(10)),
                        );
                    }
                    ui.add_space(6.0f32);
                    ui.vertical(|ui| {
                        ui.label(
                            egui::RichText::new("Mr. Support")
                                .size(20.0f32)
                                .strong()
                                .color(egui::Color32::from_rgb(17, 24, 39)),
                        );
                        ui.label(
                            egui::RichText::new("AI support, human in control")
                                .size(12.0f32)
                                .color(egui::Color32::from_rgb(107, 114, 128)),
                        );
                    });
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let (dot, label) = if self.ai_active {
                            (egui::Color32::from_rgb(34, 197, 94), "AI ON")
                        } else {
                            (egui::Color32::from_rgb(156, 163, 175), "AI OFF")
                        };
                        egui::Frame::new()
                            .fill(if self.ai_active {
                                egui::Color32::from_rgb(220, 252, 231)
                            } else {
                                egui::Color32::from_rgb(243, 244, 246)
                            })
                            .corner_radius(egui::CornerRadius::same(20))
                            .inner_margin(egui::Margin::symmetric(12, 6))
                            .show(ui, |ui| {
                                ui.label(
                                    egui::RichText::new(format!("● {}", label))
                                        .size(13.0f32)
                                        .strong()
                                        .color(dot),
                                );
                            });
                        ui.add_space(8.0f32);
                        ui.label(
                            egui::RichText::new(format!("ID: {}", self.device_id))
                                .size(13.0f32)
                                .color(egui::Color32::from_rgb(55, 65, 81)),
                        );
                    });
                });
            });

        egui::CentralPanel::default()
            .frame(egui::Frame::NONE.fill(egui::Color32::from_rgb(248, 249, 250)))
            .show(ctx, |ui| {
                ui.add_space(14.0f32);
                Self::card_frame().show(ui, |ui| {
                    Self::section_title(ui, "Connect to remote device");
                    ui.horizontal(|ui| {
                        ui.label(
                            egui::RichText::new("Remote ID:")
                                .size(13.0f32)
                                .color(egui::Color32::from_rgb(55, 65, 81)),
                        );
                        ui.add_sized(
                            [210.0f32, 32.0f32],
                            egui::TextEdit::singleline(&mut self.remote_id)
                                .hint_text("123 456 789"),
                        );
                        if ui
                            .add_sized(
                                [110.0f32, 32.0f32],
                                egui::Button::new(
                                    egui::RichText::new("Connect")
                                        .size(14.0f32)
                                        .color(egui::Color32::WHITE),
                                )
                                .fill(egui::Color32::from_rgb(37, 99, 235))
                                .corner_radius(egui::CornerRadius::same(8)),
                            )
                            .clicked()
                        {
                            self.audit_lines.push("Connect requested".into());
                        }
                    });
                    ui.add_space(10.0f32);
                    ui.horizontal(|ui| {
                        if ui
                            .add_sized(
                                [140.0f32, 40.0f32],
                                egui::Button::new(
                                    egui::RichText::new("Approve")
                                        .size(15.0f32)
                                        .strong()
                                        .color(egui::Color32::WHITE),
                                )
                                .fill(egui::Color32::from_rgb(34, 197, 94))
                                .corner_radius(egui::CornerRadius::same(10)),
                            )
                            .clicked()
                        {
                            self.ai_active = true;
                        }
                        ui.add_space(8.0f32);
                        if ui
                            .add_sized(
                                [140.0f32, 40.0f32],
                                egui::Button::new(
                                    egui::RichText::new("Revoke")
                                        .size(15.0f32)
                                        .strong()
                                        .color(egui::Color32::WHITE),
                                )
                                .fill(egui::Color32::from_rgb(239, 68, 68))
                                .corner_radius(egui::CornerRadius::same(10)),
                            )
                            .clicked()
                        {
                            self.ai_active = false;
                            self.audit_lines.push("Session revoked by user".into());
                        }
                    });
                    ui.add_space(10.0f32);
                    if self.ai_active {
                        egui::Frame::new()
                            .fill(egui::Color32::from_rgb(220, 252, 231))
                            .stroke(egui::Stroke::new(
                                1.0f32,
                                egui::Color32::from_rgb(34, 197, 94),
                            ))
                            .corner_radius(egui::CornerRadius::same(10))
                            .inner_margin(egui::Margin::symmetric(12, 8))
                            .show(ui, |ui| {
                                ui.label(
                                    egui::RichText::new("● AI operating — you can revoke anytime")
                                        .size(13.0f32)
                                        .strong()
                                        .color(egui::Color32::from_rgb(22, 163, 74)),
                                );
                            });
                    } else {
                        ui.label(
                            egui::RichText::new("AI off — waiting for consent")
                                .size(13.0f32)
                                .color(egui::Color32::from_rgb(107, 114, 128)),
                        );
                    }
                });
                ui.add_space(14.0f32);
                ui.columns(2, |cols| {
                    Self::card_frame().show(&mut cols[0], |ui| {
                        Self::section_title(ui, "Swarm");
                        ui.label(
                            egui::RichText::new("Coordinator + 5 specialists")
                                .size(13.0f32)
                                .color(egui::Color32::from_rgb(55, 65, 81)),
                        );
                        ui.label(
                            egui::RichText::new("Diagnostics · Network · Apps · Security · Files")
                                .size(12.0f32)
                                .color(egui::Color32::from_rgb(107, 114, 128)),
                        );
                    });
                    Self::card_frame().show(&mut cols[1], |ui| {
                        Self::section_title(ui, "MCP");
                        ui.label(
                            egui::RichText::new(format!("{} connected", self.mcp_assistant))
                                .size(13.0f32)
                                .color(egui::Color32::from_rgb(55, 65, 81)),
                        );
                        ui.label(
                            egui::RichText::new("Bringing its own LLM — no API key in Mr. Support")
                                .size(12.0f32)
                                .color(egui::Color32::from_rgb(107, 114, 128)),
                        );
                    });
                });
                ui.add_space(14.0f32);
                Self::card_frame().show(ui, |ui| {
                    Self::section_title(ui, "Audit log");
                    egui::Grid::new("audit_grid").striped(true).show(ui, |ui| {
                        ui.label(
                            egui::RichText::new("Time")
                                .size(12.0f32)
                                .strong()
                                .color(egui::Color32::from_rgb(107, 114, 128)),
                        );
                        ui.label(
                            egui::RichText::new("Agent")
                                .size(12.0f32)
                                .strong()
                                .color(egui::Color32::from_rgb(107, 114, 128)),
                        );
                        ui.label(
                            egui::RichText::new("Action")
                                .size(12.0f32)
                                .strong()
                                .color(egui::Color32::from_rgb(107, 114, 128)),
                        );
                        ui.end_row();
                        for line in &self.audit_lines {
                            let parts: Vec<&str> = line.splitn(3, "  ").collect();
                            if parts.len() == 3 {
                                ui.label(
                                    egui::RichText::new(parts[0])
                                        .size(13.0f32)
                                        .color(egui::Color32::from_rgb(55, 65, 81)),
                                );
                                ui.label(
                                    egui::RichText::new(parts[1])
                                        .size(13.0f32)
                                        .color(egui::Color32::from_rgb(55, 65, 81)),
                                );
                                ui.label(
                                    egui::RichText::new(parts[2])
                                        .size(13.0f32)
                                        .color(egui::Color32::from_rgb(55, 65, 81)),
                                );
                            } else {
                                ui.label(
                                    egui::RichText::new(line.clone())
                                        .size(13.0f32)
                                        .color(egui::Color32::from_rgb(55, 65, 81)),
                                );
                                ui.label("");
                                ui.label("");
                            }
                            ui.end_row();
                        }
                    });
                    ui.add_space(10.0f32);
                    if ui
                        .add(
                            egui::Button::new(
                                egui::RichText::new("Export JSON")
                                    .size(13.0f32)
                                    .color(egui::Color32::from_rgb(37, 99, 235)),
                            )
                            .fill(egui::Color32::from_rgb(239, 246, 255))
                            .corner_radius(egui::CornerRadius::same(8)),
                        )
                        .clicked()
                    {
                        self.audit_lines.push("Audit exported".into());
                    }
                });
            });
    }
}

#[cfg(feature = "gui")]
pub fn run_gui() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([780.0f32, 600.0f32]),
        ..Default::default()
    };
    eframe::run_native(
        "Mr. Support",
        options,
        Box::new(|_cc| Ok(Box::new(MrSupportApp::default()))),
    )
}
