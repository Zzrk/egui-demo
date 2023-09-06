use egui::{FontFamily, FontId, RichText, TextStyle};


// 初始化字体文件
fn setup_custom_fonts(ctx: &egui::Context) {
    // Start with the default fonts (we will be adding to them rather than replacing them).
    let mut fonts = egui::FontDefinitions::default();

    // Install my own font (maybe supporting non-latin characters).
    // .ttf and .otf files supported.
    fonts.font_data.insert(
        "my_font".to_owned(),
        egui::FontData::from_static(include_bytes!(
            "./fonts/Hack-Regular.ttf"
        )),
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


// 内容区
fn content(ui: &mut egui::Ui) {
    ui.heading("Top Heading");
    ui.add_space(5.);
    ui.label(LOREM_IPSUM);
    ui.add_space(15.);
    ui.label(RichText::new("Sub Heading").text_style(heading2()).strong());
    ui.monospace(LOREM_IPSUM);
    ui.add_space(15.);
    ui.label(RichText::new("Context").text_style(heading3()).strong());
    ui.add_space(5.);
    ui.label(LOREM_IPSUM);
    ui.add_space(15.);
}


#[derive(Default)]
pub struct MyApp {
    // 默认文本
    text: String,
    allowed_to_close: bool,
    // 当前是否打开确认对话框
    show_confirmation_dialog: bool,
}


impl MyApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setup_custom_fonts(&cc.egui_ctx);
        configure_text_styles(&cc.egui_ctx);
        Self {
            text: "Edit this text field if you want".to_owned(),
            ..Default::default()
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
          content(ui);
          ui.text_edit_multiline(&mut self.text); // 文本编辑器（使用默认文本）
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
