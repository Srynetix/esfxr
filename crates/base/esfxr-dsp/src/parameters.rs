use fundsp::shared::Shared;

#[derive(Clone)]
pub struct Waveform {
    pub sine_amount: Shared<f64>,
    pub square_amount: Shared<f64>,
    pub saw_amount: Shared<f64>,
    pub noise_amount: Shared<f64>,
}

impl Default for Waveform {
    fn default() -> Self {
        Self {
            sine_amount: Shared::new(1.0),
            square_amount: Shared::new(0.0),
            saw_amount: Shared::new(0.0),
            noise_amount: Shared::new(0.0),
        }
    }
}

#[derive(Clone)]
pub struct TimeEnvelope {
    // 0.0s -> 2.268s
    pub attack_time: Shared<f64>,
    // 0.0s -> 2.268s
    pub sustain_time: Shared<f64>,
    // 0.0% -> 100.0%
    pub sustain_punch: Shared<f64>,
    // 0.0s -> 2.268s
    pub decay_time: Shared<f64>,
}

impl Default for TimeEnvelope {
    fn default() -> Self {
        Self {
            attack_time: Shared::new(0.0),
            sustain_time: Shared::new(0.0),
            sustain_punch: Shared::new(0.1),
            decay_time: Shared::new(0.0),
        }
    }
}

#[derive(Clone)]
pub struct AdsrEnvelope {
    pub attack: Shared<f64>,
    pub decay: Shared<f64>,
    pub sustain: Shared<f64>,
    pub release: Shared<f64>,
}

impl Default for AdsrEnvelope {
    fn default() -> Self {
        Self {
            attack: Shared::new(0.0),
            decay: Shared::new(0.0),
            sustain: Shared::new(1.0),
            release: Shared::new(0.0),
        }
    }
}

#[derive(Clone)]
pub struct DspParameters {
    pub pitch: Shared<f64>,
    pub volume: Shared<f64>,
    pub control: Shared<f64>,
    pub time: TimeEnvelope,
    pub waveform: Waveform,
    pub adsr: AdsrEnvelope,
}

impl Default for DspParameters {
    fn default() -> Self {
        Self {
            pitch: Shared::new(220.0),
            volume: Shared::new(1.0),
            control: Shared::new(-1.0),
            waveform: Waveform::default(),
            time: TimeEnvelope::default(),
            adsr: AdsrEnvelope::default(),
        }
    }
}
