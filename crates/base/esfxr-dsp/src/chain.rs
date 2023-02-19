use crate::hacker::*;
use esfxr_audio_driver::{cpal::Stream, AudioOutput};
use fundsp::hacker::*;
use fundsp::shared::Shared;

#[derive(Clone)]
pub struct Waveform {
    pub sine_amount: Shared<f64>,
    pub square_amount: Shared<f64>,
    pub saw_amount: Shared<f64>,
    pub noise_amount: Shared<f64>,
}

impl Default for Waveform {
    fn default() -> Self {
        Self {
            sine_amount: Shared::new(1.0),
            square_amount: Shared::new(1.0),
            saw_amount: Shared::new(1.0),
            noise_amount: Shared::new(1.0),
        }
    }
}

#[derive(Clone)]
pub struct DspParameters {
    pub pitch: Shared<f64>,
    pub volume: Shared<f64>,
    pub control: Shared<f64>,
    pub waveform: Waveform,
    pub adsr: Adsr,
}

impl Default for DspParameters {
    fn default() -> Self {
        Self {
            pitch: Shared::new(440.0),
            volume: Shared::new(1.0),
            control: Shared::new(-1.0),
            waveform: Waveform::default(),
            adsr: Adsr::default(),
        }
    }
}

#[derive(Clone)]
pub struct Adsr {
    pub attack: Shared<f64>,
    pub decay: Shared<f64>,
    pub sustain: Shared<f64>,
    pub release: Shared<f64>,
}

impl Default for Adsr {
    fn default() -> Self {
        Self {
            attack: Shared::new(0.0),
            decay: Shared::new(0.0),
            sustain: Shared::new(1.0),
            release: Shared::new(0.0),
        }
    }
}

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
