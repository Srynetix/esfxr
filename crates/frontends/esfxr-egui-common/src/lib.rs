mod color;
mod widgets;

use eframe::egui;
use egui::Ui;
use esfxr_dsp::{cpal, DspChain, DspParameters};
use widgets::{EnvelopeWidget, PeakMeter, VolumeWidget, WaveformWidget};

#[derive(Default)]
pub struct App {
    pub parameters: DspParameters,
    pub chain: Option<DspChain>,
    pub stream: Option<cpal::Stream>,
    peak_meter: PeakMeter,
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_with_empty_stream() -> Self {
        let parameters = DspParameters::default();
        let chain = DspChain::new().expect("could not build dsp chain");
        let stream = chain
            .build_empty_stream()
            .expect("could not build audio stream");

        Self {
            parameters,
            chain: Some(chain),
            stream: Some(stream),
            peak_meter: Default::default(),
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

    fn draw_peak_meter(&mut self, ui: &mut Ui) {
        if let Some(chain) = self.chain.as_ref() {
            self.peak_meter.update_from_chain(chain);
            self.peak_meter.draw(ui);
        }
    }

    fn draw_play_button(&mut self, ui: &mut Ui) {
        let button = egui::Button::new("Play");
        if ui.add(button).clicked() {
            self.ensure_stream_ready();
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal_top(|ui| {
                ui.vertical(|ui| {
                    ui.add(VolumeWidget::new(self.parameters.clone()));
                    ui.add(WaveformWidget::new(self.parameters.clone()));
                    ui.add(EnvelopeWidget::new(self.parameters.clone()));
                });

                self.draw_peak_meter(ui);
            });

            self.draw_play_button(ui);
        });

        ctx.request_repaint();
    }
}
