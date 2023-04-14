use egui::{Ui, Widget};
use esfxr_dsp::DspParameters;

use super::utils::{build_logarithmic_slider, build_slider};

pub struct VolumeWidget {
    parameters: DspParameters,
}

impl VolumeWidget {
    pub fn new(parameters: DspParameters) -> Self {
        Self { parameters }
    }
}

impl Widget for VolumeWidget {
    fn ui(self, ui: &mut Ui) -> egui::Response {
        ui.group(|ui| {
            ui.vertical(|ui| {
                ui.heading("volume");
                build_slider(ui, &self.parameters.volume, "Volume", 0.0..=1.0);
                build_logarithmic_slider(ui, &self.parameters.pitch, "Pitch (Hz)", 20.0..=20000.0);
            })
        })
        .response
    }
}
