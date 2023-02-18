use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use cursive::{
    view::Resizable,
    views::{Button, DummyView, LinearLayout, SliderView, TextView},
    View,
};
use cursive::{Cursive, CursiveExt};
use esfxr_chain::{run_chain_in_thread, DspParameters};
use esfxr_dsp::fundsp::shared::Shared;

const SLIDER_PRECISION: usize = 2 * 10;
const MAX_PITCH: f64 = 440.0 * 8.0;
const MAX_DURATION: usize = 1;

fn draw_unit_slider(name: &str, value: &Shared<f64>) -> impl View {
    let initial_value = (value.value() * SLIDER_PRECISION as f64) as usize;

    let value = value.clone();
    LinearLayout::horizontal().child(TextView::new(name)).child(
        SliderView::horizontal(SLIDER_PRECISION + 1)
            .value(initial_value)
            .on_change(move |_s, v| {
                value.set_value((v as f64) / SLIDER_PRECISION as f64);
            }),
    )
}

fn draw_pitch_slider(name: &str, value: &Shared<f64>) -> impl View {
    let initial_value = ((value.value() / MAX_PITCH) * SLIDER_PRECISION as f64) as usize;

    let value = value.clone();
    LinearLayout::horizontal().child(TextView::new(name)).child(
        SliderView::horizontal(SLIDER_PRECISION + 1)
            .value(initial_value)
            .on_change(move |_s, v| {
                value.set_value((v as f64) / SLIDER_PRECISION as f64 * MAX_PITCH);
            }),
    )
}

fn main() -> color_eyre::Result<()> {
    let mut siv = Cursive::new();

    let audio_running = Arc::new(AtomicBool::new(true));
    let parameters = DspParameters::default();
    let audio_thread = run_chain_in_thread(parameters.clone(), audio_running.clone())?;

    let reset_thread = {
        let audio_running = audio_running.clone();
        let control = parameters.control.clone();
        std::thread::spawn(move || {
            let mut last_time = std::time::Instant::now();

            loop {
                let now_time = std::time::Instant::now();
                if control.value() > 0.0 {
                    if (now_time - last_time).as_secs() >= MAX_DURATION as u64 {
                        control.set_value(-1.0);
                        last_time = now_time;
                    }
                } else {
                    last_time = now_time;
                }

                if !audio_running.load(Ordering::Relaxed) {
                    break;
                }

                std::thread::yield_now()
            }
        })
    };

    {
        let parameters = parameters.clone();

        siv.add_layer(
            LinearLayout::vertical()
                .child(TextView::new("esfxr"))
                .child(TextView::new("press 'p' to play, 'q' to quit"))
                .child(DummyView.fixed_height(1))
                .child(draw_unit_slider("Volume", &parameters.volume))
                .child(draw_pitch_slider("Pitch", &parameters.pitch))
                .child(DummyView.fixed_height(1))
                .child(TextView::new("waveform"))
                .child(draw_unit_slider("Sine", &parameters.waveform.sine_amount))
                .child(draw_unit_slider(
                    "Square",
                    &parameters.waveform.square_amount,
                ))
                .child(draw_unit_slider("Saw", &parameters.waveform.saw_amount))
                .child(draw_unit_slider("Noise", &parameters.waveform.noise_amount))
                .child(DummyView.fixed_height(1))
                .child(TextView::new("envelope"))
                .child(draw_unit_slider("Attack", &parameters.adsr.attack))
                .child(draw_unit_slider("Decay", &parameters.adsr.decay))
                .child(draw_unit_slider("Sustain", &parameters.adsr.sustain))
                .child(draw_unit_slider("Release", &parameters.adsr.release))
                .child(Button::new("Play", move |_s| {
                    parameters.control.set_value(1.0)
                })),
        );
    }

    siv.add_global_callback('q', |s| s.quit());

    {
        let parameters = parameters;
        siv.add_global_callback('p', move |_s| parameters.control.set_value(1.0));
    }

    siv.run();

    audio_running.store(false, Ordering::Relaxed);
    reset_thread.join().unwrap();
    audio_thread.join().unwrap();

    Ok(())
}
