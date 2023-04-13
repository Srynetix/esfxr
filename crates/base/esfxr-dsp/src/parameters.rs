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

impl TimeEnvelope {
    pub fn total_duration(&self) -> f64 {
        self.attack_time.value() + self.sustain_time.value() + self.decay_time.value()
    }
}

impl Default for TimeEnvelope {
    fn default() -> Self {
        Self {
            attack_time: Shared::new(0.0),
            sustain_time: Shared::new(0.1),
            sustain_punch: Shared::new(0.0),
            decay_time: Shared::new(0.0),
        }
    }
}

#[derive(Clone)]
pub struct DspParameters {
    pub pitch: Shared<f64>,
    pub volume: Shared<f64>,
    pub envelope: TimeEnvelope,
    pub waveform: Waveform,
}

impl Default for DspParameters {
    fn default() -> Self {
        Self {
            pitch: Shared::new(220.0),
            volume: Shared::new(1.0),
            waveform: Waveform::default(),
            envelope: TimeEnvelope::default(),
        }
    }
}
