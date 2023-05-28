use crate::{hacker::sfxr_envelope, parameters::DspParameters};
use esfxr_audio_driver::{cpal::Stream, AudioOutput};
use fundsp::hacker::*;

pub struct DspChain {
    output: AudioOutput,
}

pub fn freq_slide<F: Real + Atomic>(
    sample_rate: u32,
    frequency: Shared<F>,
    min_frequency: Shared<F>,
    slide: Shared<F>,
    delta_slide: Shared<F>,
) -> An<Envelope<F, F, impl Fn(F) -> F + Clone, F>> {
    fundsp::prelude::envelope(move |time| {
        let freq = freq_limit(sample_rate, frequency.value(), min_frequency.value());
        let factor = F::from_f64(0.00025);
        let pow = F::from_f64(3.0);
        let slide_value = slide.value().pow(pow) * factor * time;
        let slide_value =
            slide_value + (slide_value * delta_slide.value() * F::from_f64(1000.0) * time);
        let freq = freq + (freq * slide_value);

        freq_limit(sample_rate, freq, min_frequency.value())
    })
}

pub fn freq_limit<F: Real + Atomic>(sample_rate: u32, frequency: F, min_frequency: F) -> F {
    let max_frequency = F::from_f64(sample_rate as f64 / 2.0);
    if frequency < min_frequency || frequency < F::zero() {
        F::zero()
    } else if frequency > max_frequency {
        max_frequency
    } else {
        frequency
    }
}

impl DspChain {
    pub fn new() -> color_eyre::Result<Self> {
        let output = AudioOutput::new()?;

        Ok(Self { output })
    }

    pub fn output(&self) -> &AudioOutput {
        &self.output
    }

    #[allow(clippy::precedence)]
    fn build_dsp_unit(&self, parameters: DspParameters) -> Box<dyn AudioUnit64> {
        let sample_rate = self.output.sample_rate();

        if parameters.frequency.min_frequency.value() > parameters.frequency.start_frequency.value()
        {
            return Box::new(dc(0.0));
        }

        let frequency = freq_slide(
            sample_rate,
            parameters.frequency.start_frequency.into(),
            parameters.frequency.min_frequency.into(),
            parameters.frequency.slide.into(),
            parameters.frequency.delta_slide.into(),
        );
        let waveform_params = parameters.waveform;
        let sine_op = var(&waveform_params.sine_amount) * sine();
        let noise_op =
            var(&waveform_params.noise_amount) * ((noise() | frequency.clone() * 8.0) >> hold(1.0));
        let saw_op = var(&waveform_params.saw_amount) * saw();
        let square_op = var(&waveform_params.square_amount) * square();
        let waveform = (frequency >> (square_op & saw_op & sine_op)) & noise_op;

        let c = waveform
            * sfxr_envelope(
                parameters.envelope.attack_time.into(),
                parameters.envelope.sustain_time.into(),
                parameters.envelope.sustain_punch.into(),
                parameters.envelope.decay_time.into(),
            );

        let c = c >> clip() * var(&parameters.volume);

        Box::new(c)
    }

    pub fn build_stream(&self, parameters: DspParameters) -> color_eyre::Result<Stream> {
        let sample_rate = self.output.sample_rate();

        let mut chain = self.build_dsp_unit(parameters);
        chain.set_sample_rate(sample_rate as f64);

        let sample_fn = move || chain.get_stereo();

        self.output.build_stream(sample_fn)
    }

    pub fn build_empty_stream(&self) -> color_eyre::Result<Stream> {
        let sample_fn = move || (0.0, 0.0);
        self.output.build_stream(sample_fn)
    }
}
