use std::{
    sync::{atomic::AtomicBool, Arc},
    thread::JoinHandle,
};

use crate::hacker::*;
use esfxr_core::{start_stream_blocking, start_stream_thread, AudioOutput};
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

#[allow(clippy::precedence)]
pub fn build_dsp_chain(parameters: DspParameters) -> Box<dyn AudioUnit64> {
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

pub fn run_chain_in_thread(
    parameters: DspParameters,
    audio_running: Arc<AtomicBool>,
) -> color_eyre::Result<JoinHandle<()>> {
    let output = AudioOutput::new_direct()?;
    let sample_rate = output.sample_rate();

    let mut chain = build_dsp_chain(parameters);
    chain.reset(Some(sample_rate as f64));

    let sample_fn = move || {
        let v = chain.get_stereo();
        vec![v.0, v.1]
    };

    start_stream_thread(output, sample_fn, audio_running)
}

pub fn run_chain_blocking(
    parameters: DspParameters,
    audio_running: Arc<AtomicBool>,
) -> color_eyre::Result<()> {
    let output = AudioOutput::new_direct()?;
    let sample_rate = output.sample_rate();

    let mut chain = build_dsp_chain(parameters);
    chain.reset(Some(sample_rate as f64));

    let sample_fn = move || {
        let v = chain.get_stereo();
        vec![v.0, v.1]
    };

    start_stream_blocking(output, sample_fn, audio_running)
}
