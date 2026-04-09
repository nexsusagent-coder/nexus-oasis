//! ─── Wake Word Detection ───
//!
//! Supports multiple wake word engines:
//! - Porcupine (Picovoice) - cloud-based, high accuracy
//! - Vosk - offline, open source
//! - Whisper - OpenAI's model, local
//!
//! Usage:
//! ```rust
//! let detector = WakeWordDetector::new(WakeWordConfig::default())?;
//! detector.start(|event| {
//!     match event {
//!         WakeEvent::Detected { confidence } => {
//!             println!("Wake word detected! Confidence: {}", confidence);
//!         }
//!         WakeEvent::AudioLevel(level) => {
//!             // Update UI visualization
//!         }
//!     }
//! }).await?;
//! ```

pub mod config;
pub mod detector;
pub mod porcupine;
pub mod vosk_;
pub mod whisper_;
pub mod audio;

#[cfg(test)]
mod tests;

pub use config::WakeWordConfig;
pub use detector::{WakeWordDetector, WakeEvent};

pub const DEFAULT_WAKE_WORD: &str = "sentient";
pub const SAMPLE_RATE: u32 = 16000;
