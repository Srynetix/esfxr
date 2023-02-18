use std::{
    path::PathBuf,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread::{self, JoinHandle},
};

use super::engine::host_device_setup;
use super::export;
use crate::engine::stream_setup_for_device;
use color_eyre::eyre;
use cpal::traits::StreamTrait;

pub enum AudioOutput {
    Wav(WavOutput),
    Direct(DirectOutput),
}

impl AudioOutput {
    pub fn sample_rate(&self) -> u32 {
        match &self {
            Self::Wav(params) => params.spec.sample_rate,
            Self::Direct(params) => params.config.sample_rate().0,
        }
    }

    pub fn new_direct() -> eyre::Result<Self> {
        let (_host, device, config) = host_device_setup()?;
        Ok(Self::Direct(DirectOutput { device, config }))
    }
}

pub struct WavOutput {
    pub path: PathBuf,
    pub spec: hound::WavSpec,
    pub duration: usize,
}

pub struct DirectOutput {
    pub device: cpal::Device,
    pub config: cpal::SupportedStreamConfig,
}

fn stream_loop_spinlock(
    device: cpal::Device,
    config: cpal::SupportedStreamConfig,
    sample_fn: impl FnMut() -> Vec<f64> + Send + 'static,
    running: Arc<AtomicBool>,
) -> eyre::Result<()> {
    let stream = stream_setup_for_device(device, config, sample_fn)?;
    stream.play()?;

    while running.load(Ordering::Relaxed) {
        thread::yield_now();
    }

    Ok(())
}

pub fn start_stream_thread(
    output: AudioOutput,
    sample_fn: impl FnMut() -> Vec<f64> + Send + 'static,
    running: Arc<AtomicBool>,
) -> eyre::Result<JoinHandle<()>> {
    Ok(std::thread::spawn(|| {
        start_stream_blocking(output, sample_fn, running).unwrap()
    }))
}

pub fn start_stream_blocking(
    output: AudioOutput,
    sample_fn: impl FnMut() -> Vec<f64> + Send + 'static,
    running: Arc<AtomicBool>,
) -> eyre::Result<()> {
    match output {
        AudioOutput::Wav(params) => {
            export::export_to_wav(&params.path, params.spec, params.duration, sample_fn).unwrap();
        }
        AudioOutput::Direct(params) => {
            stream_loop_spinlock(params.device, params.config, sample_fn, running).unwrap();
        }
    }

    Ok(())
}
