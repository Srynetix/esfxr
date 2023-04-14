use egui::Ui;
use esfxr_dsp::DspChain;

use super::PeakMeterBar;

const PEAK_METER_DECAY: f64 = 0.90;
const PEAK_METER_PEAK_MAX_FRAMES: usize = 60;

#[derive(Default)]
pub struct PeakMeter {
    pub left_channel: f64,
    pub right_channel: f64,
    pub left_channel_target: f64,
    pub right_channel_target: f64,
    pub left_peak_value: f64,
    pub right_peak_value: f64,
    pub left_peak_frames: usize,
    pub right_peak_frames: usize,
}

impl PeakMeter {
    pub fn update_from_chain(&mut self, chain: &DspChain) {
        let (left_value_amp, right_value_amp) = chain.output().buffer.peak();

        if left_value_amp >= self.left_channel {
            self.left_channel = left_value_amp;
        } else {
            self.left_channel *= PEAK_METER_DECAY;
        }

        if right_value_amp >= self.right_channel {
            self.right_channel = right_value_amp;
        } else {
            self.right_channel *= PEAK_METER_DECAY;
        }

        if left_value_amp >= self.left_peak_value {
            self.left_peak_value = left_value_amp;
            self.left_peak_frames = 0;
        } else if self.left_peak_frames < PEAK_METER_PEAK_MAX_FRAMES {
            self.left_peak_frames += 1;
        } else {
            self.left_peak_value = f64::NEG_INFINITY;
        }

        if right_value_amp >= self.right_peak_value {
            self.right_peak_value = right_value_amp;
            self.right_peak_frames = 0;
        } else if self.right_peak_frames < PEAK_METER_PEAK_MAX_FRAMES {
            self.right_peak_frames += 1;
        } else {
            self.right_peak_value = f64::NEG_INFINITY;
        }
    }

    pub fn draw(&self, ui: &mut Ui) {
        let left_bar = PeakMeterBar::new(self.left_channel as f32, self.left_peak_value as f32)
            .desired_height(128.0)
            .desired_width(16.0);
        let right_bar = PeakMeterBar::new(self.right_channel as f32, self.right_peak_value as f32)
            .desired_height(128.0)
            .desired_width(16.0);

        ui.horizontal(|ui| {
            ui.add(left_bar);
            ui.add(right_bar);
        });
    }
}
