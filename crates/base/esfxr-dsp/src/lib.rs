mod chain;
pub mod envelope;
pub mod hacker;
pub mod math;
mod parameters;
mod serializable_shared;

pub use esfxr_audio_driver::cpal;
pub use fundsp;

pub use chain::DspChain;
pub use parameters::DspParameters;
