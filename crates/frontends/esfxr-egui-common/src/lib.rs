mod app_chain;
mod color;
mod widgets;

// eNp1y7ERABAQRNHfE_npRxPqoQyNiEVXgtkxEsML_-yCuMUg3Xgau5ecTmlV5rX3zx8W0RYMBA==

use app_chain::AppChain;
use eframe::egui;
use egui::Ui;
use widgets::{EnvelopeWidget, FrequencyControls, PeakMeter, VolumeWidget, WaveformWidget};

#[derive(Default)]
pub struct App {
    chain: AppChain,
    peak_meter: PeakMeter,
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_with_empty_stream() -> Self {
        Self {
            chain: AppChain::new_with_empty_stream(),
            peak_meter: PeakMeter::default(),
        }
    }

    fn draw_peak_meter(&mut self, ui: &mut Ui) {
        if let Some(chain) = self.chain.chain.as_ref() {
            self.peak_meter.update_from_chain(chain);
            self.peak_meter.draw(ui);
        }
    }

    fn draw_play_button(&mut self, ui: &mut Ui) {
        let button = egui::Button::new("Play");
        if ui.add(button).clicked() {
            self.chain.play_stream();
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Share");
                ui.text_edit_singleline(&mut self.chain.parameters_string);
                if ui.button("Load").clicked() {
                    self.chain.load_from_parameters_string();
                }
            });

            ui.add_space(32.0);

            ui.horizontal_top(|ui| {
                ui.vertical(|ui| {
                    VolumeWidget {
                        parameters: self.chain.parameters.clone(),
                    }
                    .render(ui, &mut self.chain);

                    WaveformWidget {
                        parameters: self.chain.parameters.clone(),
                    }
                    .render(ui, &mut self.chain);

                    EnvelopeWidget {
                        parameters: self.chain.parameters.clone(),
                    }
                    .render(ui, &mut self.chain);

                    FrequencyControls {
                        parameters: self.chain.parameters.clone(),
                    }
                    .render(ui, &mut self.chain);
                });

                self.draw_peak_meter(ui);
            });

            self.draw_play_button(ui);
        });

        ctx.request_repaint();
    }
}
