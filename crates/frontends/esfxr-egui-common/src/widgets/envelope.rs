use egui::Ui;
use esfxr_dsp::DspParameters;

use crate::app_chain::AppChain;

use super::utils::build_slider;

pub struct EnvelopeWidget {
    pub parameters: DspParameters,
}

impl EnvelopeWidget {
    pub fn render(self, ui: &mut Ui, chain: &mut AppChain) {
        ui.group(|ui| {
            ui.vertical(|ui| {
                ui.heading("time");
                build_slider(
                    ui,
                    chain,
                    &self.parameters.envelope.attack_time,
                    "Attack time (s)",
                    0.0..=2.268,
                );
                build_slider(
                    ui,
                    chain,
                    &self.parameters.envelope.sustain_time,
                    "Sustain time (s)",
                    0.0..=2.268,
                );
                build_slider(
                    ui,
                    chain,
                    &self.parameters.envelope.sustain_punch,
                    "Sustain punch (%)",
                    0.0..=100.0,
                );
                build_slider(
                    ui,
                    chain,
                    &self.parameters.envelope.decay_time,
                    "Decay time (s)",
                    0.0..=2.268,
                );
            })
        });
    }
}
