// hide console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use eframe::egui;
use esfxr_dsp::{run_chain_in_thread, DspParameters};
use esfxr_egui_common::App;

fn main() -> Result<(), color_eyre::Report> {
    tracing_subscriber::fmt::init();

    let audio_running = Arc::new(AtomicBool::new(true));
    let parameters = DspParameters::default();

    let handle = run_chain_in_thread(parameters.clone(), audio_running.clone())?;

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 320.0)),
        resizable: false,
        ..Default::default()
    };

    eframe::run_native(
        "esfxr",
        options,
        Box::new(|_cc| Box::new(App { parameters })),
    )
    .unwrap();

    audio_running.store(false, Ordering::Relaxed);
    handle.join().unwrap();

    Ok(())
}
