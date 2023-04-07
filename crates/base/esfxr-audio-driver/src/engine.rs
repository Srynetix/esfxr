use color_eyre::{eyre, eyre::eyre};
use cpal::{
    traits::{DeviceTrait, HostTrait},
    FromSample, SizedSample,
};
use tracing::{error, info};

pub struct SampleRequestOptions {
    pub sample_rate: f32,
    pub sample_clock: f32,
    pub nchannels: usize,
}

pub(crate) fn stream_setup_for_device(
    device: &cpal::Device,
    config: &cpal::SupportedStreamConfig,
    on_sample: impl FnMut() -> (f64, f64) + Send + 'static,
) -> eyre::Result<cpal::Stream> {
    match config.sample_format() {
        cpal::SampleFormat::F32 => stream_make::<f32>(device, config, on_sample),
        cpal::SampleFormat::I16 => stream_make::<i16>(device, config, on_sample),
        cpal::SampleFormat::U16 => stream_make::<u16>(device, config, on_sample),
        f => panic!("Unsupported sample format: {f}"),
    }
}

pub(crate) fn host_device_setup(
) -> eyre::Result<(cpal::Host, cpal::Device, cpal::SupportedStreamConfig)> {
    let host = cpal::default_host();

    let device = host
        .default_output_device()
        .ok_or_else(|| eyre!("Default output device is not available"))?;

    info!(message = "Output device", name = device.name()?);

    let config = device.default_output_config()?;

    info!(
        message = "Default output config",
        config = ?config
    );

    Ok((host, device, config))
}

fn stream_make<T>(
    device: &cpal::Device,
    config: &cpal::SupportedStreamConfig,
    mut on_sample: impl FnMut() -> (f64, f64) + Send + 'static,
) -> eyre::Result<cpal::Stream>
where
    T: SizedSample + FromSample<f64>,
{
    let sample_rate = config.sample_rate().0 as f32;
    let sample_clock = 0f32;
    let nchannels = config.channels() as usize;
    let mut request = SampleRequestOptions {
        sample_rate,
        sample_clock,
        nchannels,
    };
    let err_fn = |err| {
        error!(
            message = "Error building output sound stream",
            error = %err
        )
    };

    let stream = device.build_output_stream(
        &config.to_owned().into(),
        move |output: &mut [T], _: &cpal::OutputCallbackInfo| {
            on_window(output, &mut request, &mut on_sample)
        },
        err_fn,
        None,
    )?;

    Ok(stream)
}

fn on_window<T>(
    output: &mut [T],
    request: &mut SampleRequestOptions,
    mut on_sample: impl FnMut() -> (f64, f64),
) where
    T: SizedSample + FromSample<f64>,
{
    for frame in output.chunks_mut(request.nchannels) {
        let samples = on_sample();
        for (channel, sample) in frame.iter_mut().enumerate() {
            let sample_choice = if channel % 2 == 0 {
                samples.0
            } else {
                samples.1
            };
            let value = T::from_sample(sample_choice);
            *sample = value;
        }
    }
}
