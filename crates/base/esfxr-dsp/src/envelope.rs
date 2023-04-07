use fundsp::{
    prelude::{envelope, lerp, An, Envelope},
    shared::{Atomic, Shared},
    Real,
};

pub fn sfxr_envelope<F: Real + Atomic>(
    attack_time: Shared<F>,
    sustain_time: Shared<F>,
    sustain_punch: Shared<F>,
    decay_time: Shared<F>,
) -> An<Envelope<F, F, impl Fn(F) -> F + Clone, F>> {
    envelope(move |time| {
        sfxr_envelope_time(
            attack_time.clone(),
            sustain_time.clone(),
            sustain_punch.clone(),
            decay_time.clone(),
            time,
        )
    })
}

fn sfxr_envelope_time<F: Atomic + Real>(
    attack_time: Shared<F>,
    sustain_time: Shared<F>,
    sustain_punch: Shared<F>,
    decay_time: Shared<F>,
    time: F,
) -> F {
    let sustain_value_attack = F::from_f64(1.0) + sustain_punch.value() / F::from_f64(100.0);
    let sustain_value =
        F::from_f64(1.0) + (F::from_f64(4.0) * sustain_punch.value() / F::from_f64(100.0));

    if time < attack_time.value() {
        lerp(
            F::from_f64(0.0),
            sustain_value_attack,
            time / attack_time.value(),
        )
    } else {
        let time = time - attack_time.value();
        if time < sustain_time.value() {
            lerp(sustain_value, F::from_f64(1.0), time / sustain_time.value())
        } else {
            let time = time - sustain_time.value();
            if time < decay_time.value() {
                lerp(
                    F::from_f64(1.0),
                    F::from_f64(0.0),
                    time / decay_time.value(),
                )
            } else {
                F::from_f64(0.0)
            }
        }
    }
}
