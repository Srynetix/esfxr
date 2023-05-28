use cursive::{
    view::Resizable,
    views::{DummyView, LinearLayout, SliderView, TextView},
    View,
};
use cursive::{Cursive, CursiveExt};
use esfxr_dsp::{fundsp::shared::Shared, math::map, DspChain, DspParameters};

const SLIDER_PRECISION: usize = 2 * 20;
const MIN_PITCH: f64 = 20.0;
const MAX_PITCH: f64 = 440.0 * 8.0;

struct SliderBuilder {
    min_value: f64,
    max_value: f64,
    scale: SliderScale,
}

pub enum SliderScale {
    Linear,
    Logarithmic,
}

impl Default for SliderBuilder {
    fn default() -> Self {
        Self {
            min_value: 0.0,
            max_value: 1.0,
            scale: SliderScale::Linear,
        }
    }
}

impl SliderBuilder {
    pub fn min_value(mut self, value: f64) -> Self {
        self.min_value = value;
        self
    }

    pub fn max_value(mut self, value: f64) -> Self {
        self.max_value = value;
        self
    }

    pub fn scale(mut self, scale: SliderScale) -> Self {
        self.scale = scale;
        self
    }

    pub fn build(self, name: &str, value: &Shared<f64>) -> impl View {
        let min_value = self.min_value;
        let max_value = self.max_value;
        let scale = self.scale;

        let initial_value = match scale {
            SliderScale::Linear => map_parameter_to_slider(value.value(), min_value, max_value),
            SliderScale::Logarithmic => {
                map_parameter_to_slider_logarithmic(value.value(), min_value, max_value)
            }
        };

        let value = value.clone();
        LinearLayout::horizontal().child(TextView::new(name)).child(
            SliderView::horizontal(SLIDER_PRECISION + 1)
                .value(initial_value)
                .on_change(move |_s, v| {
                    value.set_value(match scale {
                        SliderScale::Linear => map_slider_to_parameter(v, min_value, max_value),
                        SliderScale::Logarithmic => {
                            map_slider_to_parameter_logarithmic(v, min_value, max_value)
                        }
                    })
                }),
        )
    }
}

fn map_parameter_to_slider(value: f64, min_limit: f64, max_limit: f64) -> usize {
    map(value, min_limit, max_limit, 0.0, SLIDER_PRECISION as f64) as usize
}

fn map_parameter_to_slider_logarithmic(value: f64, min_limit: f64, max_limit: f64) -> usize {
    let min_limit = min_limit.ln();
    let max_limit = max_limit.ln();
    let value = value.ln();

    let scale = (max_limit - min_limit) / (SLIDER_PRECISION as f64);
    ((value - min_limit) / scale) as usize
}

fn map_slider_to_parameter(value: usize, min_limit: f64, max_limit: f64) -> f64 {
    map(
        value as f64,
        0.0,
        SLIDER_PRECISION as f64,
        min_limit,
        max_limit,
    )
}

fn map_slider_to_parameter_logarithmic(value: usize, min_limit: f64, max_limit: f64) -> f64 {
    let min_limit = min_limit.ln();
    let max_limit = max_limit.ln();

    let scale = (max_limit - min_limit) / (SLIDER_PRECISION as f64);
    (value as f64 * scale + min_limit).exp()
}

fn main() -> color_eyre::Result<()> {
    let mut siv = Cursive::new();

    let parameters = DspParameters::default();
    let chain = DspChain::new()?;
    let mut _stream = chain.build_stream(parameters.clone())?;

    {
        let parameters = parameters.clone();

        siv.add_layer(
            LinearLayout::vertical()
                .child(TextView::new("esfxr"))
                .child(TextView::new("press 'p' to play, 'q' to quit"))
                .child(DummyView.fixed_height(1))
                .child(SliderBuilder::default().build("Volume", &parameters.volume))
                .child(DummyView.fixed_height(1))
                .child(TextView::new("waveform"))
                .child(SliderBuilder::default().build("Sine", &parameters.waveform.sine_amount))
                .child(SliderBuilder::default().build("Square", &parameters.waveform.square_amount))
                .child(SliderBuilder::default().build("Saw", &parameters.waveform.saw_amount))
                .child(SliderBuilder::default().build("Noise", &parameters.waveform.noise_amount))
                .child(DummyView.fixed_height(1))
                .child(TextView::new("envelope"))
                .child(
                    SliderBuilder::default()
                        .max_value(2.268)
                        .build("Attack time", &parameters.envelope.attack_time),
                )
                .child(
                    SliderBuilder::default()
                        .max_value(2.268)
                        .build("Sustain time", &parameters.envelope.sustain_time),
                )
                .child(
                    SliderBuilder::default()
                        .max_value(100.0)
                        .build("Sustain punch", &parameters.envelope.sustain_punch),
                )
                .child(
                    SliderBuilder::default()
                        .max_value(2.268)
                        .build("Decay time", &parameters.envelope.decay_time),
                )
                .child(DummyView.fixed_height(1))
                .child(TextView::new("frequency"))
                .child(
                    SliderBuilder::default()
                        .min_value(MIN_PITCH)
                        .max_value(MAX_PITCH)
                        .scale(SliderScale::Logarithmic)
                        .build(
                            "Start frequency (Hz)",
                            &parameters.frequency.start_frequency,
                        ),
                )
                .child(
                    SliderBuilder::default()
                        .min_value(0.0)
                        .max_value(MAX_PITCH)
                        .scale(SliderScale::Logarithmic)
                        .build("Min. frequency (Hz)", &parameters.frequency.min_frequency),
                )
                .child(
                    SliderBuilder::default()
                        .min_value(-600.0)
                        .max_value(600.0)
                        .build("Slide (8va/sec)", &parameters.frequency.slide),
                )
                .child(
                    SliderBuilder::default()
                        .min_value(8.88201e-2)
                        .max_value(-8.88201e-2)
                        .build("Delta slide (8va/sec^2)", &parameters.frequency.delta_slide),
                ),
        );
    }

    siv.add_global_callback('q', |s| s.quit());

    {
        let parameters = parameters;
        siv.add_global_callback('p', move |_s| {
            _stream = chain
                .build_stream(parameters.clone())
                .expect("could not build stream");
        });
    }

    siv.run();

    Ok(())
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use crate::{
        map_parameter_to_slider, map_parameter_to_slider_logarithmic, map_slider_to_parameter,
        map_slider_to_parameter_logarithmic, SLIDER_PRECISION,
    };

    #[test]
    fn test_map_parameter_to_slider() {
        let slider_max = SLIDER_PRECISION;

        assert_eq!(map_parameter_to_slider(0.0, 0.0, 1.0), 0);
        assert_eq!(map_parameter_to_slider(0.0, 1.0, 2.0), 0);
        assert_eq!(map_parameter_to_slider(1.0, 0.0, 1.0), slider_max);
        assert_eq!(map_parameter_to_slider(0.5, 0.0, 1.0), slider_max / 2);
        assert_eq!(map_parameter_to_slider(5.0, 0.0, 10.0), slider_max / 2);
    }

    #[test]
    fn test_map_slider_to_parameter() {
        let slider_max = SLIDER_PRECISION;

        assert_relative_eq!(map_slider_to_parameter(0, 0.0, 1.0), 0.0);
        assert_relative_eq!(map_slider_to_parameter(slider_max, 0.0, 1.0), 1.0);
        assert_relative_eq!(map_slider_to_parameter(slider_max / 2, 0.0, 1.0), 0.5);
        assert_relative_eq!(map_slider_to_parameter(slider_max / 2, 0.0, 10.0), 5.0);
    }

    #[test]
    fn test_map_parameter_to_slider_logarithmic() {
        let slider_max = SLIDER_PRECISION;

        assert_eq!(map_parameter_to_slider_logarithmic(20.0, 20.0, 20_000.0), 0);
        assert_eq!(map_parameter_to_slider_logarithmic(20.0, 20.0, 20_000.0), 0);
        assert_eq!(
            map_parameter_to_slider_logarithmic(20_000.0, 20.0, 20_000.0),
            slider_max
        );
    }

    #[test]
    fn test_map_slider_to_parameter_logarithmic() {
        let slider_max = SLIDER_PRECISION;

        assert_relative_eq!(
            map_slider_to_parameter_logarithmic(0, 20.0, 20000.0),
            20.0,
            epsilon = 1e-5f64
        );
        assert_relative_eq!(
            map_slider_to_parameter_logarithmic(slider_max, 20.0, 20000.0),
            20000.0,
            epsilon = 1e-5f64
        );
    }
}
