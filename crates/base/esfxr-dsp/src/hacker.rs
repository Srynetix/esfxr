use fundsp::{
    prelude::{An, Envelope},
    shared::Shared,
};

#[inline]
pub fn sfxr_envelope(
    attack_time: Shared<f64>,
    sustain_time: Shared<f64>,
    sustain_punch: Shared<f64>,
    decay_time: Shared<f64>,
) -> An<Envelope<f64, f64, impl Fn(f64) -> f64 + Clone, f64>> {
    crate::envelope::sfxr_envelope(attack_time, sustain_time, sustain_punch, decay_time)
}
