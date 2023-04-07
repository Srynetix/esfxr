use crate::{hacker::sfxr_envelope, parameters::DspParameters};
use esfxr_audio_driver::{cpal::Stream, AudioOutput};
use fundsp::hacker::*;

pub struct DspChain {
    output: AudioOutput,
}

impl DspChain {
    pub fn new() -> color_eyre::Result<Self> {
        let output = AudioOutput::new()?;

        Ok(Self { output })
    }

    #[allow(clippy::precedence)]
    fn build_dsp_unit(&self, parameters: DspParameters) -> Box<dyn AudioUnit64> {
        let waveform_params = parameters.waveform;
        let sine_op = var(&waveform_params.sine_amount) * sine();
        let noise_op = var(&waveform_params.noise_amount) * noise();
        let saw_op = var(&waveform_params.saw_amount) * saw();
        let square_op = var(&waveform_params.square_amount) * square();
        let waveform = (var(&parameters.pitch) >> (square_op & saw_op & sine_op)) & noise_op;

        let c = waveform
            * sfxr_envelope(
                parameters.envelope.attack_time,
                parameters.envelope.sustain_time,
                parameters.envelope.sustain_punch,
                parameters.envelope.decay_time,
            )
            * var(&parameters.volume)
            >> clip();

        // let c = c >> declick();

        Box::new(c)
    }

    pub fn build_stream(&self, parameters: DspParameters) -> color_eyre::Result<Stream> {
        let sample_rate = self.output.sample_rate();

        let mut chain = self.build_dsp_unit(parameters);
        chain.reset(Some(sample_rate as f64));

        let sample_fn = move || chain.get_stereo();

        self.output.build_stream(sample_fn)
    }
}
