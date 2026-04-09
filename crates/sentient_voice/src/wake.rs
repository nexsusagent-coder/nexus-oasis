//! ─── Wake Word Detection ───

use serde::{Deserialize, Serialize};
use crate::audio::AudioBuffer;
use crate::config::WakeWordConfig;

/// Wake word detection result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WakeWord {
    pub phrase: String,
    pub confidence: f32,
    pub timestamp: f32,
}

/// Wake word detector
pub struct WakeWordDetector {
    config: WakeWordConfig,
    phrase: String,
}

impl WakeWordDetector {
    pub fn new(config: WakeWordConfig) -> Self {
        Self {
            phrase: config.phrase.clone(),
            config,
        }
    }
    
    /// Detect wake word in audio
    pub async fn detect(&self, audio: &[f32]) -> Option<WakeWord> {
        // Simple phonetic matching (placeholder)
        // Real implementation would use Porcupine, snowboy, or similar
        
        // TODO: Implement actual wake word detection
        // For now, return None
        None
    }
    
    /// Get wake word phrase
    pub fn phrase(&self) -> &str {
        &self.phrase
    }
    
    /// Update sensitivity
    pub fn set_sensitivity(&mut self, sensitivity: f32) {
        self.config.sensitivity = sensitivity;
    }
}

/// Porcupine wake word detector (requires license)
#[cfg(feature = "porcupine")]
pub struct PorcupineDetector {
    porcupine: pvporcupine::Porcupine,
    frame_length: usize,
}

#[cfg(feature = "porcupine")]
impl PorcupineDetector {
    pub fn new(access_key: &str, keyword_path: &str) -> Result<Self, String> {
        let porcupine = pvporcupine::PorcupineBuilder::new()
            .access_key(access_key)
            .keyword_path(keyword_path)
            .init()
            .map_err(|e| e.to_string())?;
        
        Ok(Self {
            frame_length: porcupine.frame_length() as usize,
            porcupine,
        })
    }
    
    pub fn process(&mut self, frame: &[f32]) -> bool {
        self.porcupine.process(frame).unwrap_or(-1) >= 0
    }
}
