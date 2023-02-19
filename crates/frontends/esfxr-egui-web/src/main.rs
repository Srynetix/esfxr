// when compiling to web using trunk.
#[cfg(target_arch = "wasm32")]
fn main() -> color_eyre::Result<()> {
    use esfxr_egui_common::App;

    // Make sure panics are logged using `console.error`.
    console_error_panic_hook::set_once();

    // Redirect tracing to console.log and friends:
    tracing_wasm::set_as_global_default();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async move {
        eframe::start_web(
            "the_canvas_id", // hardcode it
            web_options,
            Box::new(|_cc| Box::<App>::default()),
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
