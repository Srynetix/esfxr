// hide console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{
    ops::RangeInclusive,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

use eframe::egui;
use egui::Ui;
use esfxr_chain::{run_chain_in_thread, DspParameters};
use esfxr_dsp::fundsp::shared::Shared;

struct App {
    pub parameters: DspParameters,
}

impl App {
    fn build_slider(
        &self,
        ui: &mut Ui,
        param: &Shared<f64>,
        name: &str,
        range: RangeInclusive<f64>,
    ) {
        let mut value = param.value();
        let slider = egui::Slider::new(&mut value, range).text(name);
        if ui.add(slider).changed() {
            param.set_value(value);
        }
    }

    fn build_logarithmic_slider(
        &self,
        ui: &mut Ui,
        param: &Shared<f64>,
        name: &str,
        range: RangeInclusive<f64>,
    ) {
        let mut value = param.value();
        let slider = egui::Slider::new(&mut value, range)
            .text(name)
            .logarithmic(true);
        if ui.add(slider).changed() {
            param.set_value(value);
        }
    }

    fn draw_volume_controls(&self, ui: &mut Ui) {
        self.build_slider(ui, &self.parameters.volume, "Volume", 0.0..=1.0);
        self.build_logarithmic_slider(ui, &self.parameters.pitch, "Pitch", 110.0..=440.0 * 8.0);
    }

    fn draw_adsr_envelope(&self, ui: &mut Ui) {
        ui.heading("envelope");
        self.build_slider(ui, &self.parameters.adsr.attack, "Attack", 0.0..=1.0);
        self.build_slider(ui, &self.parameters.adsr.decay, "Decay", 0.0..=1.0);
        self.build_slider(ui, &self.parameters.adsr.sustain, "Sustain", 0.0..=1.0);
        self.build_slider(ui, &self.parameters.adsr.release, "Release", 0.0..=2.0);
    }

    fn draw_waveform_controls(&self, ui: &mut Ui) {
        ui.heading("waveform");
        self.build_slider(ui, &self.parameters.waveform.sine_amount, "Sine", 0.0..=1.0);
        self.build_slider(
            ui,
            &self.parameters.waveform.square_amount,
            "Square",
            0.0..=1.0,
        );
        self.build_slider(ui, &self.parameters.waveform.saw_amount, "Saw", 0.0..=1.0);
        self.build_slider(
            ui,
            &self.parameters.waveform.noise_amount,
            "Noise",
            0.0..=1.0,
        );
    }

    fn draw_play_button(&self, ui: &mut Ui) {
        let button = egui::Button::new("Play");
        if ui.add(button).is_pointer_button_down_on() {
            self.parameters.control.set_value(1.0);
        } else {
            self.parameters.control.set_value(-1.0);
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("esfxr");

            self.draw_volume_controls(ui);
            self.draw_waveform_controls(ui);
            self.draw_adsr_envelope(ui);

            self.draw_play_button(ui);
        });
    }
}

fn main() -> Result<(), color_eyre::Report> {
    tracing_subscriber::fmt::init();

    let audio_running = Arc::new(AtomicBool::new(true));
    let parameters = DspParameters::default();

    let app = Box::new(App {
        parameters: parameters.clone(),
    });

    let handle = run_chain_in_thread(parameters, audio_running.clone())?;

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 320.0)),
        resizable: false,
        ..Default::default()
    };

    eframe::run_native("esfxr", options, Box::new(|_cc| app)).unwrap();

    audio_running.store(false, Ordering::Relaxed);
    handle.join().unwrap();

    Ok(())
}
