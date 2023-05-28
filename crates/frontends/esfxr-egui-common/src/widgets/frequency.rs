use egui::Ui;
use esfxr_dsp::DspParameters;

use crate::app_chain::AppChain;

use super::utils::{build_logarithmic_slider, build_slider};

pub struct FrequencyControls {
    pub parameters: DspParameters,
}

impl FrequencyControls {
    pub fn render(self, ui: &mut Ui, chain: &mut AppChain) {
        ui.group(|ui| {
            ui.vertical(|ui| {
                ui.heading("frequency");
                build_logarithmic_slider(
                    ui,
                    chain,
                    &self.parameters.frequency.start_frequency,
                    "Start frequency (Hz)",
                    20.0..=20_000.0,
                );
                build_logarithmic_slider(
                    ui,
                    chain,
                    &self.parameters.frequency.min_frequency,
                    "Min. frequency (Hz)",
                    0.0..=20_000.0,
                );
                build_slider(
                    ui,
                    chain,
                    &self.parameters.frequency.slide,
                    "Slide (8va/sec)",
                    -600.0..=600.0,
                );
                build_slider(
                    ui,
                    chain,
                    &self.parameters.frequency.delta_slide,
                    "Delta slide (8va/sec^2)",
                    8.88201e-2..=-8.88201e-2,
                );
            })
        });
    }
}
