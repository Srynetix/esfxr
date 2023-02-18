use fundsp::{
    prelude::{An, EnvelopeIn, Frame, U1},
    shared::Shared,
};

use crate::adsr::adsr_v_generic;

#[inline]
pub fn adsr_v(
    attack: Shared<f64>,
    decay: Shared<f64>,
    sustain: Shared<f64>,
    release: Shared<f64>,
) -> An<EnvelopeIn<f64, f64, impl Fn(f64, &Frame<f64, U1>) -> f64 + Sized + Clone, U1, f64>> {
    adsr_v_generic(attack, decay, sustain, release)
}
