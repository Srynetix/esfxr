use std::ops::RangeInclusive;

use egui::Ui;
use esfxr_dsp::fundsp::shared::Shared;

pub fn build_slider(ui: &mut Ui, param: &Shared<f64>, name: &str, range: RangeInclusive<f64>) {
    let mut value = param.value();
    let slider = egui::Slider::new(&mut value, range).text(name);
    if ui.add(slider).changed() {
        param.set_value(value);
    }
}

pub fn build_logarithmic_slider(
    ui: &mut Ui,
    param: &Shared<f64>,
    name: &str,
    range: RangeInclusive<f64>,
) {
    let mut value = param.value();
    let slider = egui::Slider::new(&mut value, range)
        .text(name)
        .logarithmic(true);
    if ui.add(slider).changed() {
        param.set_value(value);
    }
}
