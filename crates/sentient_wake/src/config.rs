//! ─── Wake Word Configuration ───

use serde::{Deserialize, Serialize};

/// Wake word detection configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WakeWordConfig {
    /// Wake word to detect
    pub wake_word: String,
    
    /// Detection sensitivity (0.0 - 1.0)
    /// Higher = more sensitive but more false positives
    pub sensitivity: f32,
    
    /// Minimum confidence threshold
    pub confidence_threshold: f32,
    
    /// Detection engine
    pub engine: WakeEngine,
    
    /// Porcupine access key (if using Porcupine)
    pub porcupine_access_key: Option<String>,
    
    /// Vosk model path (if using Vosk)
    pub vosk_model_path: Option<String>,
    
    /// Whisper model path (if using Whisper)
    pub whisper_model_path: Option<String>,
    
    /// Audio input device index
    pub input_device_index: Option<u32>,
    
    /// Sample rate
    pub sample_rate: u32,
    
    /// Frame duration in milliseconds
    pub frame_duration_ms: u32,
    
    /// Enable continuous listening
    pub continuous: bool,
    
    /// Cooldown after detection (ms)
    pub cooldown_ms: u64,
}

/// Wake word detection engine
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WakeEngine {
    /// Porcupine by Picovoice (requires access key)
    Porcupine,
    
    /// Vosk - offline, open source
    Vosk,
    
    /// Whisper - OpenAI's model
    Whisper,
    
    /// Simple keyword matching (for testing)
    Simple,
}

impl Default for WakeWordConfig {
    fn default() -> Self {
        Self {
            wake_word: "sentient".into(),
            sensitivity: 0.5,
            confidence_threshold: 0.7,
            engine: WakeEngine::Vosk,
            porcupine_access_key: None,
            vosk_model_path: None,
            whisper_model_path: None,
            input_device_index: None,
            sample_rate: 16000,
            frame_duration_ms: 30,
            continuous: true,
            cooldown_ms: 2000,
        }
    }
}

impl WakeWordConfig {
    /// Create config for Porcupine engine
    pub fn porcupine(access_key: impl Into<String>) -> Self {
        Self {
            engine: WakeEngine::Porcupine,
            porcupine_access_key: Some(access_key.into()),
            ..Default::default()
        }
    }
    
    /// Create config for Vosk engine
    pub fn vosk(model_path: impl Into<String>) -> Self {
        Self {
            engine: WakeEngine::Vosk,
            vosk_model_path: Some(model_path.into()),
            ..Default::default()
        }
    }
    
    /// Create config for Whisper engine
    pub fn whisper(model_path: impl Into<String>) -> Self {
        Self {
            engine: WakeEngine::Whisper,
            whisper_model_path: Some(model_path.into()),
            ..Default::default()
        }
    }
    
    /// Simple engine for testing
    pub fn simple() -> Self {
        Self {
            engine: WakeEngine::Simple,
            ..Default::default()
        }
    }
}
