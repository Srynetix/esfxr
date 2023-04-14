use egui::Widget;
use esfxr_dsp::DspParameters;

use super::utils::build_slider;

pub struct WaveformWidget {
    parameters: DspParameters,
}

impl WaveformWidget {
    pub fn new(parameters: DspParameters) -> Self {
        Self { parameters }
    }
}

impl Widget for WaveformWidget {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.vertical(|ui| {
            ui.heading("waveform");
            build_slider(ui, &self.parameters.waveform.sine_amount, "Sine", 0.0..=1.0);
            build_slider(
                ui,
                &self.parameters.waveform.square_amount,
                "Square",
                0.0..=1.0,
            );
            build_slider(ui, &self.parameters.waveform.saw_amount, "Saw", 0.0..=1.0);
            build_slider(
                ui,
                &self.parameters.waveform.noise_amount,
                "Noise",
                0.0..=1.0,
            );
        })
        .response
    }
}
