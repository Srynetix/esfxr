use super::engine::host_device_setup;
use crate::{buffer::AudioBuffer, engine::stream_setup_for_device};
use color_eyre::eyre;
use cpal::{traits::StreamTrait, Stream};

pub struct AudioOutput {
    pub device: cpal::Device,
    pub config: cpal::SupportedStreamConfig,
    pub buffer: AudioBuffer,
}

impl AudioOutput {
    pub fn new() -> eyre::Result<Self> {
        let (_host, device, config) = host_device_setup()?;
        let sample_rate = config.sample_rate().0;

        Ok(Self {
            device,
            config,
            buffer: AudioBuffer::new(sample_rate),
        })
    }

    pub fn sample_rate(&self) -> u32 {
        self.config.sample_rate().0
    }

    pub fn build_stream(
        &self,
        sample_fn: impl FnMut() -> (f64, f64) + Send + 'static,
    ) -> eyre::Result<Stream> {
        let stream =
            stream_setup_for_device(&self.device, &self.config, self.buffer.clone(), sample_fn)?;
        stream.play()?;

        Ok(stream)
    }
}
