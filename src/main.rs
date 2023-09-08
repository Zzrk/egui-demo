#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod serial_windows;

fn main() -> eframe::Result<()> {
    serial_windows::main()
    // env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    // egui_demo::start_puffin_server(); // NOTE: you may only want to call this if the users specifies some flag or clicks a button!
    // let native_options = eframe::NativeOptions {
    //     drag_and_drop_support: true,
    //     initial_window_size: Some(egui::vec2(400.0, 1000.0)),
    //     ..Default::default()
    // };

    // eframe::run_native(
    //     "egui demo",
    //     native_options,
    //     Box::new(|_cc| Box::new(egui_demo::ScreenshotApp::default())),
    // )
}
