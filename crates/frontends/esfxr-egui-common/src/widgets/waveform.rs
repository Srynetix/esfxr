use esfxr_dsp::DspParameters;

use crate::app_chain::AppChain;

use super::utils::build_slider;

pub struct WaveformWidget {
    pub parameters: DspParameters,
}

impl WaveformWidget {
    pub fn render(self, ui: &mut egui::Ui, chain: &mut AppChain) {
        ui.group(|ui| {
            ui.vertical(|ui| {
                ui.heading("waveform");
                build_slider(
                    ui,
                    chain,
                    &self.parameters.waveform.sine_amount,
                    "Sine",
                    0.0..=1.0,
                );
                build_slider(
                    ui,
                    chain,
                    &self.parameters.waveform.square_amount,
                    "Square",
                    0.0..=1.0,
                );
                build_slider(
                    ui,
                    chain,
                    &self.parameters.waveform.saw_amount,
                    "Saw",
                    0.0..=1.0,
                );
                build_slider(
                    ui,
                    chain,
                    &self.parameters.waveform.noise_amount,
                    "Noise",
                    0.0..=1.0,
                );
            })
        });
    }
}
