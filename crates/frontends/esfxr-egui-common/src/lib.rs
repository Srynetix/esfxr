use std::ops::RangeInclusive;

use eframe::egui;
use egui::Ui;
use esfxr_dsp::{cpal, fundsp::shared::Shared, DspChain, DspParameters};

#[derive(Default)]
pub struct App {
    pub parameters: DspParameters,
    pub chain: Option<DspChain>,
    pub stream: Option<cpal::Stream>,
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_with_stream() -> Self {
        let parameters = DspParameters::default();
        let chain = DspChain::new().expect("could not build dsp chain");
        let stream = chain
            .build_stream(parameters.clone())
            .expect("could not build audio stream");

        Self {
            parameters,
            chain: Some(chain),
            stream: Some(stream),
        }
    }

    pub fn ensure_stream_ready(&mut self) {
        if self.chain.is_none() {
            self.chain = Some(DspChain::new().expect("could not build dsp chain"));
        }

        let stream = self
            .chain
            .as_ref()
            .expect("chain should be initialized")
            .build_stream(self.parameters.clone())
            .expect("could not build audio stream");
        self.stream = Some(stream);
    }

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
        self.build_logarithmic_slider(ui, &self.parameters.pitch, "Pitch", 20.0..=20000.0);
    }

    #[allow(dead_code)]
    fn draw_time_controls(&self, ui: &mut Ui) {
        ui.heading("time");
        self.build_slider(
            ui,
            &self.parameters.envelope.attack_time,
            "Attack time (s)",
            0.0..=2.268,
        );
        self.build_slider(
            ui,
            &self.parameters.envelope.sustain_time,
            "Sustain time (s)",
            0.0..=2.268,
        );
        self.build_slider(
            ui,
            &self.parameters.envelope.sustain_punch,
            "Sustain punch (%)",
            0.0..=100.0,
        );
        self.build_slider(
            ui,
            &self.parameters.envelope.decay_time,
            "Decay time (s)",
            0.0..=2.268,
        );
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

    fn draw_play_button(&mut self, ui: &mut Ui) {
        let button = egui::Button::new("Play");
        if ui.add(button).clicked() {
            self.ensure_stream_ready();
            self.parameters.control.set_value(-1.0);
            self.parameters.control.set_value(1.0);
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("esfxr");

            self.draw_volume_controls(ui);
            self.draw_time_controls(ui);
            self.draw_waveform_controls(ui);

            self.draw_play_button(ui);
        });
    }
}
