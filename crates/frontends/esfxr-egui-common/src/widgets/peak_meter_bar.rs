use egui::{
    vec2, Color32, NumExt, Rect, Response, Sense, Stroke, TextStyle, Ui, Vec2, Widget, WidgetText,
};
use esfxr_dsp::fundsp::prelude::amp_db;

use crate::color::Color32Ext;

/// A peak meter bar.
#[must_use = "You should put this widget in an ui with `ui.add(widget);`"]
pub struct PeakMeterBar {
    progress: f32,
    peak: f32,
    desired_width: Option<f32>,
    desired_height: Option<f32>,
}

impl PeakMeterBar {
    /// Progress in the `[0, 1]` range, where `1` means "completed".
    pub fn new(progress: f32, peak: f32) -> Self {
        Self {
            progress: progress.clamp(0.0, 1.0),
            peak,
            desired_width: None,
            desired_height: None,
        }
    }

    /// The desired width of the bar. Will use all horizontal space if not set.
    pub fn desired_width(mut self, desired_width: f32) -> Self {
        self.desired_width = Some(desired_width);
        self
    }

    /// The desired height of the bar. Will use all vertical space if not set.
    pub fn desired_height(mut self, desired_height: f32) -> Self {
        self.desired_height = Some(desired_height);
        self
    }

    fn amp_to_db(&self, value: f32) -> f32 {
        let value = amp_db(value);
        if value < -80.0 {
            -f32::INFINITY
        } else {
            value
        }
    }
}

impl Widget for PeakMeterBar {
    fn ui(self, ui: &mut Ui) -> Response {
        let PeakMeterBar {
            progress,
            peak,
            desired_width,
            desired_height,
        } = self;

        let desired_width =
            desired_width.unwrap_or_else(|| ui.available_size_before_wrap().x.at_least(96.0));
        let desired_height =
            desired_height.unwrap_or_else(|| ui.available_size_before_wrap().y.at_least(96.0));

        let (outer_rect, response) =
            ui.allocate_exact_size(vec2(desired_width, desired_height), Sense::hover());

        if ui.is_rect_visible(response.rect) {
            let visuals = ui.style().visuals.clone();
            ui.painter()
                .rect(outer_rect, 0.0, visuals.extreme_bg_color, Stroke::NONE);

            let bar_height = outer_rect.height() * progress;
            let inner_rect =
                Rect::from_min_size(outer_rect.min, vec2(outer_rect.width(), bar_height))
                    .translate(vec2(0.0, desired_height - bar_height));

            let color = Color32::GREEN.blend_mix(Color32::RED, progress);
            ui.painter().rect(inner_rect, 0.0, color, Stroke::NONE);

            // Show current dB
            {
                let text: WidgetText = format!("{:0.1}", self.amp_to_db(progress)).into();
                let galley = text.into_galley(ui, Some(false), f32::INFINITY, TextStyle::Small);
                let text_pos = outer_rect.center_bottom() - Vec2::new(galley.size().x / 2.0, 0.0);
                let text_color = visuals
                    .override_text_color
                    .unwrap_or(visuals.selection.stroke.color);
                galley.paint_with_fallback_color(ui.painter(), text_pos, text_color);
            }

            // Show peak dB
            {
                let db = self.amp_to_db(peak);
                if db.is_finite() {
                    let text: WidgetText = format!("{:0.1}", db).into();
                    let galley = text.into_galley(ui, Some(false), f32::INFINITY, TextStyle::Small);
                    let text_pos =
                        outer_rect.center_top() - Vec2::new(galley.size().x / 2.0, galley.size().y);
                    let text_color = visuals
                        .override_text_color
                        .unwrap_or(visuals.selection.stroke.color);
                    galley.paint_with_fallback_color(ui.painter(), text_pos, text_color);

                    let peak_pos =
                        outer_rect.left_bottom() - Vec2::new(0.0, outer_rect.height() * peak);
                    let rect =
                        Rect::from_two_pos(peak_pos, peak_pos + Vec2::new(outer_rect.width(), 1.0));
                    let color = Color32::GREEN.blend_mix(Color32::RED, peak);
                    ui.painter().rect(rect, 0.0, color, Stroke::NONE);
                }
            }
        }

        response
    }
}
