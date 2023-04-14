use egui::{Ui, Widget};
use esfxr_dsp::DspParameters;

use super::utils::build_slider;

pub struct EnvelopeWidget {
    parameters: DspParameters,
}

impl EnvelopeWidget {
    pub fn new(parameters: DspParameters) -> Self {
        Self { parameters }
    }
}

impl Widget for EnvelopeWidget {
    fn ui(self, ui: &mut Ui) -> egui::Response {
        ui.group(|ui| {
            ui.vertical(|ui| {
                ui.heading("time");
                build_slider(
                    ui,
                    &self.parameters.envelope.attack_time,
                    "Attack time (s)",
                    0.0..=2.268,
                );
                build_slider(
                    ui,
                    &self.parameters.envelope.sustain_time,
                    "Sustain time (s)",
                    0.0..=2.268,
                );
                build_slider(
                    ui,
                    &self.parameters.envelope.sustain_punch,
                    "Sustain punch (%)",
                    0.0..=100.0,
                );
                build_slider(
                    ui,
                    &self.parameters.envelope.decay_time,
                    "Decay time (s)",
                    0.0..=2.268,
                );
            })
        })
        .response
    }
}
