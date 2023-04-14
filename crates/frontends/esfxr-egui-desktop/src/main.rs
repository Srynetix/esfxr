// hide console window on Windows in release
// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;
use esfxr_egui_common::App;
use tracing_subscriber::{filter::LevelFilter, EnvFilter};

fn main() -> Result<(), color_eyre::Report> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .init();

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 640.0)),
        resizable: true,
        ..Default::default()
    };

    eframe::run_native(
        "esfxr",
        options,
        Box::new(|_cc| Box::new(App::new_with_empty_stream())),
    )
    .unwrap();

    Ok(())
}
