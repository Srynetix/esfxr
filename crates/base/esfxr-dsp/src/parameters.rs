use std::io::{Read, Write};

use base64::Engine;
use color_eyre::Result;
use serde::{Deserialize, Serialize};

use crate::serializable_shared::SerializableSharedF64;

#[derive(Clone, Serialize, Deserialize)]
pub struct WaveformParameters {
    pub sine_amount: SerializableSharedF64,
    pub square_amount: SerializableSharedF64,
    pub saw_amount: SerializableSharedF64,
    pub noise_amount: SerializableSharedF64,
}

impl Default for WaveformParameters {
    fn default() -> Self {
        Self {
            sine_amount: SerializableSharedF64::new(1.0),
            square_amount: SerializableSharedF64::new(0.0),
            saw_amount: SerializableSharedF64::new(0.0),
            noise_amount: SerializableSharedF64::new(0.0),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TimeEnvelope {
    // 0.0s -> 2.268s
    pub attack_time: SerializableSharedF64,
    // 0.0s -> 2.268s
    pub sustain_time: SerializableSharedF64,
    // 0.0% -> 100.0%
    pub sustain_punch: SerializableSharedF64,
    // 0.0s -> 2.268s
    pub decay_time: SerializableSharedF64,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct FrequencyParameters {
    // 20 Hz -> 20000 Hz
    pub start_frequency: SerializableSharedF64,
    // 0 Hz -> 20000 Hz
    pub min_frequency: SerializableSharedF64,
    // ??? (8va/sec)
    pub slide: SerializableSharedF64,
    // ??? (8va/sec^2)
    pub delta_slide: SerializableSharedF64,
}

impl Default for FrequencyParameters {
    fn default() -> Self {
        Self {
            start_frequency: SerializableSharedF64::new(220.0),
            min_frequency: SerializableSharedF64::new(0.0),
            slide: SerializableSharedF64::new(0.0),
            delta_slide: SerializableSharedF64::new(0.0),
        }
    }
}

impl TimeEnvelope {
    pub fn total_duration(&self) -> f64 {
        self.attack_time.value() + self.sustain_time.value() + self.decay_time.value()
    }
}

impl Default for TimeEnvelope {
    fn default() -> Self {
        Self {
            attack_time: SerializableSharedF64::new(0.0),
            sustain_time: SerializableSharedF64::new(0.1),
            sustain_punch: SerializableSharedF64::new(0.0),
            decay_time: SerializableSharedF64::new(0.0),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DspParameters {
    pub volume: SerializableSharedF64,
    pub envelope: TimeEnvelope,
    pub frequency: FrequencyParameters,
    pub waveform: WaveformParameters,
}

impl Default for DspParameters {
    fn default() -> Self {
        Self {
            volume: SerializableSharedF64::new(1.0),
            frequency: FrequencyParameters::default(),
            waveform: WaveformParameters::default(),
            envelope: TimeEnvelope::default(),
        }
    }
}

impl DspParameters {
    pub fn to_b64(&self) -> String {
        let binary = bincode::serialize(&self).unwrap();
        let mut encoder = flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::best());
        encoder.write_all(&binary).unwrap();
        let binary = encoder.finish().unwrap();

        base64::engine::general_purpose::URL_SAFE.encode(binary)
    }

    pub fn from_b64(input: &str) -> Result<Self> {
        let compressed_binary = base64::engine::general_purpose::URL_SAFE.decode(input)?;
        let mut decoder = flate2::bufread::ZlibDecoder::new(&compressed_binary[..]);
        let mut binary = Vec::new();
        decoder.read_to_end(&mut binary)?;

        bincode::deserialize(&binary).map_err(Into::into)
    }
}
