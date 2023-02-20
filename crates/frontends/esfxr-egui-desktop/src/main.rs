// hide console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;
use esfxr_egui_common::App;

fn main() -> Result<(), color_eyre::Report> {
    tracing_subscriber::fmt::init();

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 640.0)),
        resizable: false,
        ..Default::default()
    };

    eframe::run_native(
        "esfxr",
        options,
        Box::new(|_cc| Box::new(App::new_with_stream())),
    )
    .unwrap();

    Ok(())
}
