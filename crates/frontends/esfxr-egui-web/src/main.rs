// when compiling to web using trunk.
#[cfg(target_arch = "wasm32")]
fn main() -> color_eyre::Result<()> {
    use esfxr_dsp::DspParameters;
    use esfxr_egui_common::App;
    use std::sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    };
    use tracing::info;

    // Make sure panics are logged using `console.error`.
    console_error_panic_hook::set_once();

    // Redirect tracing to console.log and friends:
    tracing_wasm::set_as_global_default();

    let web_options = eframe::WebOptions::default();
    let parameters = DspParameters::default();

    wasm_bindgen_futures::spawn_local(async move {
        eframe::start_web(
            "the_canvas_id", // hardcode it
            web_options,
            Box::new(|_cc| Box::new(App { parameters })),
        )
        .await
        .expect("failed to start eframe");
    });

    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    panic!("Built for wasm.")
}
