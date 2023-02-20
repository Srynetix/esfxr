use crate::hacker::adsr_v;
use crate::parameters::DspParameters;
use esfxr_audio_driver::{cpal::Stream, AudioOutput};
use fundsp::hacker::*;

pub struct DspChain;

impl DspChain {
    #[allow(clippy::precedence)]
    fn build_dsp_unit(parameters: DspParameters) -> Box<dyn AudioUnit64> {
        let waveform_params = parameters.waveform;
        let sine_op = var(&waveform_params.sine_amount) * sine();
        let noise_op = var(&waveform_params.noise_amount) * noise();
        let saw_op = var(&waveform_params.saw_amount) * saw();
        let square_op = var(&waveform_params.square_amount) * square();
        let waveform = square_op & saw_op & (sine_op + noise_op);

        let c = (var(&parameters.pitch)
            >> waveform
                * (var(&parameters.control)
                    >> adsr_v(
                        parameters.adsr.attack,
                        parameters.adsr.decay,
                        parameters.adsr.sustain,
                        parameters.adsr.release,
                    )))
            * var(&parameters.volume);

        let c = c >> declick() >> dcblock();

        Box::new(c)
    }

    pub fn build_stream(parameters: DspParameters) -> color_eyre::Result<Stream> {
        let output = AudioOutput::new()?;
        let sample_rate = output.sample_rate();

        let mut chain = Self::build_dsp_unit(parameters);
        chain.reset(Some(sample_rate as f64));

        let sample_fn = move || {
            let v = chain.get_stereo();
            vec![v.0, v.1]
        };

        output.build_stream(sample_fn)
    }
}
