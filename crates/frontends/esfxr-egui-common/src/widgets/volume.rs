use egui::Ui;
use esfxr_dsp::DspParameters;

use crate::app_chain::AppChain;

use super::utils::build_slider;

pub struct VolumeWidget {
    pub parameters: DspParameters,
}

impl VolumeWidget {
    pub fn render(self, ui: &mut Ui, chain: &mut AppChain) {
        ui.group(|ui| {
            ui.vertical(|ui| {
                ui.heading("volume");
                build_slider(ui, chain, &self.parameters.volume, "Volume", 0.0..=1.0);
            })
        });
    }
}
