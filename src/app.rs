use egui::{FontFamily, FontId, Key, RichText, ScrollArea, TextStyle};

// 初始化字体文件
fn setup_custom_fonts(ctx: &egui::Context) {
    // Start with the default fonts (we will be adding to them rather than replacing them).
    let mut fonts = egui::FontDefinitions::default();

    // Install my own font (maybe supporting non-latin characters).
    // .ttf and .otf files supported.
    fonts.font_data.insert(
        "my_font".to_owned(),
        egui::FontData::from_static(include_bytes!("./fonts/Hack-Regular.ttf")),
    );

    // 设置为比例字体的最高优先级
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "my_font".to_owned());

    // 设置为等宽字体的最低优先级
    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push("my_font".to_owned());

    // Tell egui to use these fonts:
    ctx.set_fonts(fonts);
}

// 初始化字体
#[inline]
fn heading2() -> TextStyle {
    TextStyle::Name("Heading2".into())
}

#[inline]
fn heading3() -> TextStyle {
    TextStyle::Name("ContextHeading".into())
}

fn configure_text_styles(ctx: &egui::Context) {
    use FontFamily::{Monospace, Proportional};

    let mut style = (*ctx.style()).clone();
    style.text_styles = [
        (TextStyle::Heading, FontId::new(25.0, Proportional)), // heading
        (heading2(), FontId::new(22.0, Proportional)),
        (heading3(), FontId::new(19.0, Proportional)),
        (TextStyle::Body, FontId::new(16.0, Proportional)), // label
        (TextStyle::Monospace, FontId::new(12.0, Monospace)), // monospace
        (TextStyle::Button, FontId::new(12.0, Proportional)),
        (TextStyle::Small, FontId::new(8.0, Proportional)),
    ]
    .into();
    ctx.set_style(style);
}

// 预览拖拽的文件
fn preview_files_being_dropped(ctx: &egui::Context) {
    use egui::*;
    use std::fmt::Write as _;

    if !ctx.input(|i| i.raw.hovered_files.is_empty()) {
        // 遮罩层文字
        let text = ctx.input(|i| {
            let mut text = "Dropping files:\n".to_owned();
            for file in &i.raw.hovered_files {
                if let Some(path) = &file.path {
                    write!(text, "\n{}", path.display()).ok();
                } else if !file.mime.is_empty() {
                    write!(text, "\n{}", file.mime).ok();
                } else {
                    text += "\n???";
                }
            }
            text
        });

        // 遮罩层
        let painter =
            ctx.layer_painter(LayerId::new(Order::Foreground, Id::new("file_drop_target")));
        // 遮罩层尺寸
        let screen_rect = ctx.screen_rect();
        painter.rect_filled(screen_rect, 0.0, Color32::from_black_alpha(192));
        painter.text(
            screen_rect.center(),
            Align2::CENTER_CENTER,
            text,
            TextStyle::Heading.resolve(&ctx.style()),
            Color32::WHITE,
        );
    }
}

// puffin
pub fn start_puffin_server() {
    puffin::set_scopes_on(true); // tell puffin to collect data

    match puffin_http::Server::new("0.0.0.0:8585") {
        Ok(puffin_server) => {
            eprintln!("Run:  cargo install puffin_viewer && puffin_viewer --url 127.0.0.1:8585");

            // We can store the server if we want, but in this case we just want
            // it to keep running. Dropping it closes the server, so let's not drop it!
            #[allow(clippy::mem_forget)]
            std::mem::forget(puffin_server);
        }
        Err(err) => {
            eprintln!("Failed to start puffin server: {err}");
        }
    };
}

// 内容区
fn content(app: &mut MyApp, ui: &mut egui::Ui) {
    ui.heading("egui demo");
    ui.add_space(15.);

    // 同一水平区域
    ui.horizontal(|ui| {
        let name_label = ui.label("Your name: ");
        ui.text_edit_singleline(&mut app.name)
            .labelled_by(name_label.id);
    });
    // Slider
    ui.add(egui::Slider::new(&mut app.age, 0..=120).text("age"));
    if ui.button("Click each year").clicked() {
        app.age += 1;
    }
    ui.label(format!("Hello '{}', age {}", app.name, app.age));

    ui.add_space(15.);
    ui.monospace(LOREM_IPSUM);
    ui.add_space(15.);
    ui.label(RichText::new("Sub Heading").text_style(heading2()).strong());
    ui.add_space(15.);

    // 文本编辑器（使用默认文本）
    ui.text_edit_multiline(&mut app.text);
}

pub struct MyApp {
    name: String,
    age: u32,
    // 默认文本
    text: String,
    allowed_to_close: bool,
    // 当前是否打开确认对话框
    show_confirmation_dialog: bool,
    // 拖拽的文件
    dropped_files: Vec<egui::DroppedFile>,
    // 选择的文件路径
    picked_path: Option<String>,
}

impl MyApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setup_custom_fonts(&cc.egui_ctx);
        configure_text_styles(&cc.egui_ctx);
        Self {
            ..Default::default()
        }
    }
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "Zzrk".to_owned(),
            age: 18,
            text: "Edit this text field if you want".to_owned(),
            allowed_to_close: false,
            show_confirmation_dialog: false,
            dropped_files: Vec::new(),
            picked_path: None,
        }
    }
}

impl eframe::App for MyApp {
    fn on_close_event(&mut self) -> bool {
        self.show_confirmation_dialog = true;
        self.allowed_to_close
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // egui::CentralPanel 用于覆盖屏幕的剩余部分
        egui::CentralPanel::default().show(ctx, |ui| {
            content(self, ui);

            // 展示选择的文件
            if ui.button("Open file…").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_file() {
                    self.picked_path = Some(path.display().to_string());
                }
            }
            if let Some(picked_path) = &self.picked_path {
                ui.horizontal(|ui| {
                    ui.label("Picked file:");
                    ui.monospace(picked_path);
                });
            }

            // 展示拖拽的文件
            if !self.dropped_files.is_empty() {
                ui.group(|ui| {
                    ui.label("Dropped files:");

                    for file in &self.dropped_files {
                        let mut info = if let Some(path) = &file.path {
                            path.display().to_string()
                        } else if !file.name.is_empty() {
                            file.name.clone()
                        } else {
                            "???".to_owned()
                        };

                        let mut additional_info = vec![];
                        if !file.name.is_empty() {
                            additional_info.push(format!("type: {}", file.name));
                        }
                        if let Some(bytes) = &file.bytes {
                            additional_info.push(format!("{} bytes", bytes.len()));
                        }
                        if !additional_info.is_empty() {
                            info += &format!(" ({})", additional_info.join(", "));
                        }

                        ui.label(info);
                    }
                });
            }

            // 键盘事件
            // ScrollArea::vertical()
            //     .auto_shrink([false; 2])
            //     .stick_to_bottom(true)
            //     .show(ui, |ui| {
            //         ui.label(&self.text);
            //     });

            // if ctx.input(|i| i.key_pressed(Key::A)) {
            //     self.text.push_str("\nPressed");
            // }
            // if ctx.input(|i| i.key_down(Key::A)) {
            //     self.text.push_str("\nHeld");
            //     ui.ctx().request_repaint(); // make sure we note the holding.
            // }
            // if ctx.input(|i| i.key_released(Key::A)) {
            //     self.text.push_str("\nReleased");
            // }

            // puffin
            // ui.separator();
            // let cmd = "cargo install puffin_viewer && puffin_viewer --url 127.0.0.1:8585";
            // ui.label("To connect, run this:");
            // ui.horizontal(|ui| {
            //     ui.monospace(cmd);
            //     if ui.small_button("📋").clicked() {
            //         ui.output_mut(|o| o.copied_text = cmd.into());
            //     }
            // });
            // ui.separator();
            // ui.label("Note that this app runs in 'reactive' mode, so you must interact with the app for new profile events to be sent. Waving the mouse over this window is enough.");
            // if ui.button("Click to sleep a bit. That should be visible as a spike in the profiler view!").clicked() {
            //     puffin::profile_scope!("sleep");
            //     std::thread::sleep(std::time::Duration::from_millis(50));
            // }
        });

        preview_files_being_dropped(ctx);
        // 保存拖拽的文件
        ctx.input(|i| {
            if !i.raw.dropped_files.is_empty() {
                self.dropped_files = i.raw.dropped_files.clone();
            }
        });

        // 确认对话框
        if self.show_confirmation_dialog {
            egui::Window::new("Do you want to quit?")
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        if ui.button("Cancel").clicked() {
                            self.show_confirmation_dialog = false;
                        }

                        if ui.button("Yes!").clicked() {
                            self.allowed_to_close = true;
                            frame.close();
                        }
                    });
                });
        }
    }
}

pub const LOREM_IPSUM: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";
