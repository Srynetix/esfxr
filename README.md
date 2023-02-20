# esfxr: extended-sfxr

**:warning: Warning: this is a Work in Progress!**

Simple tool to generate basic sounds, inspired from [DrPetter's sfxr](http://www.drpetter.se/project_sfxr.html).  
It's made in [Rust](https://www.rust-lang.org/).

## Project structure

The project is composed of multiple crates, separated in two groups:

- **Base crates (base)**:
    - [`esfxr-audio-driver`]: "Low-level" structures to handle audio using the [`cpal`] crate.
    - [`esfxr-dsp`]: DSP logic, where the magic happens, using the [`fundsp`] crate.
- **Frontend crates (frontends)**:
    - [`esfxr-egui-common`]: Common GUI code based on [`egui`] and [`eframe`].
    - [`esfxr-egui-desktop`]: Desktop application based on [`eframe`], using [`esfxr-egui-common`].
    - [`esfxr-egui-web`]: Web application based on [`eframe`], using [`esfxr-egui-common`] and [`Trunk`].
    - [`esfxr-tui`]: Terminal application based on [`cursive`].

[`cpal`]: https://github.com/RustAudio/cpal
[`cursive`]: https://github.com/gyscos/cursive
[`fundsp`]: https://github.com/SamiPerttu/fundsp
[`egui`]: https://github.com/emilk/egui
[`eframe`]: https://github.com/emilk/egui/tree/master/crates/eframe
[`Trunk`]: https://trunkrs.dev/

[`esfxr-audio-driver`]: ./crates/base/esfxr-audio-driver/Cargo.toml
[`esfxr-dsp`]: ./crates/base/esfxr-dsp/Cargo.toml
[`esfxr-egui-common`]: ./crates/frontends/esfxr-egui-common/Cargo.toml
[`esfxr-egui-desktop`]: ./crates/frontends/esfxr-egui-desktop/Cargo.toml
[`esfxr-egui-web`]: ./crates/frontends/esfxr-egui-web/Cargo.toml
[`esfxr-tui`]: ./crates/frontends/esfxr-tui/Cargo.toml
