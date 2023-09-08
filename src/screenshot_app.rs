#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui::{self, ColorImage};

#[derive(Default)]
pub struct ScreenshotApp {
    continuously_take_screenshots: bool,
    texture: Option<egui::TextureHandle>,
    screenshot: Option<ColorImage>,
    save_to_file: bool,
}

impl eframe::App for ScreenshotApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(screenshot) = self.screenshot.take() {
                self.texture = Some(ui.ctx().load_texture(
                    "screenshot",
                    screenshot,
                    Default::default(),
                ));
            }

            ui.horizontal(|ui| {
                ui.checkbox(
                    &mut self.continuously_take_screenshots,
                    "continuously take screenshots",
                );

                if ui.button("save to 'top_left.png'").clicked() {
                    // 保存图片到本地，在 post_rendering 中
                    self.save_to_file = true;
                    frame.request_screenshot();
                }

                ui.with_layout(egui::Layout::top_down(egui::Align::RIGHT), |ui| {
                    if self.continuously_take_screenshots {
                        if ui
                            .add(egui::Label::new("hover me!").sense(egui::Sense::hover()))
                            .hovered()
                        {
                            ctx.set_visuals(egui::Visuals::dark());
                        } else {
                            ctx.set_visuals(egui::Visuals::light());
                        };
                        frame.request_screenshot();
                    } else if ui.button("take screenshot!").clicked() {
                        // 截图
                        frame.request_screenshot();
                    }
                });
            });

            if let Some(texture) = self.texture.as_ref() {
                // 展示 egui 界面的截图
                ui.image(texture, ui.available_size());
            } else {
                ui.spinner();
            }

            ctx.request_repaint();
        });
    }

    fn post_rendering(&mut self, _window_size: [u32; 2], frame: &eframe::Frame) {
        if let Some(screenshot) = frame.screenshot() {
            if self.save_to_file {
                let pixels_per_point = frame.info().native_pixels_per_point;
                let region =
                    egui::Rect::from_two_pos(egui::Pos2::ZERO, egui::Pos2 { x: 100., y: 100. });
                let top_left_corner = screenshot.region(&region, pixels_per_point);
                image::save_buffer(
                    "top_left.png",
                    top_left_corner.as_raw(),
                    top_left_corner.width() as u32,
                    top_left_corner.height() as u32,
                    image::ColorType::Rgba8,
                )
                .unwrap();
                self.save_to_file = false;
            }
            self.screenshot = Some(screenshot);
        }
    }
}
