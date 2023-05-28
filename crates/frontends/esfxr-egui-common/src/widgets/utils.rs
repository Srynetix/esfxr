use std::ops::RangeInclusive;

use egui::Ui;
use esfxr_dsp::fundsp::shared::Shared;

use crate::app_chain::AppChain;

pub fn build_slider(
    ui: &mut Ui,
    chain: &mut AppChain,
    param: &Shared<f64>,
    name: &str,
    range: RangeInclusive<f64>,
) {
    let mut value = param.value();
    let slider = egui::Slider::new(&mut value, range).text(name);
    let response = ui.add(slider);

    if response.changed() {
        chain.update_parameters_string();
        param.set_value(value);
    }

    if response.drag_released() {
        chain.play_stream();
    }
}

#[allow(dead_code)]
pub fn build_mapped_slider(
    ui: &mut Ui,
    chain: &mut AppChain,
    param: &Shared<f64>,
    name: &str,
    range: RangeInclusive<f64>,
    cb: impl Fn(f64) -> f64,
    inv_cb: impl Fn(f64) -> f64,
) {
    let mut value = cb(param.value());
    let range_min = cb(*range.start());
    let range_max = cb(*range.end());

    let slider = egui::Slider::new(&mut value, range_min..=range_max).text(name);
    let response = ui.add(slider);

    if response.changed() {
        chain.update_parameters_string();
        param.set_value(inv_cb(value));
    }

    if response.drag_released() {
        chain.play_stream();
    }
}

pub fn build_logarithmic_slider(
    ui: &mut Ui,
    chain: &mut AppChain,
    param: &Shared<f64>,
    name: &str,
    range: RangeInclusive<f64>,
) {
    let mut value = param.value();
    let slider = egui::Slider::new(&mut value, range)
        .text(name)
        .logarithmic(true);
    let response = ui.add(slider);

    if response.changed() {
        chain.update_parameters_string();
        param.set_value(value);
    }

    if response.drag_released() {
        chain.play_stream();
    }
}
