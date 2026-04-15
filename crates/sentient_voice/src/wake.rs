//! Wake word detection

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{VoiceError, VoiceResult, WakeWordSettings};

/// Wake word detector
pub struct WakeWordDetector {
    config: WakeWordSettings,
    state: Arc<Mutex<DetectorState>>,
}

/// Detector state
#[derive(Debug, Clone)]
pub struct DetectorState {
    pub is_listening: bool,
    pub wake_word_detected: bool,
    pub last_detection: Option<chrono::DateTime<chrono::Utc>>,
    pub detections: u64,
}

impl Default for DetectorState {
    fn default() -> Self {
        Self {
            is_listening: false,
            wake_word_detected: false,
            last_detection: None,
            detections: 0,
        }
    }
}

/// Wake word configuration
#[derive(Debug, Clone)]
pub struct WakeWordConfig {
    pub settings: WakeWordSettings,
    pub access_key: Option<String>,  // For Picovoice
    pub model_path: Option<String>,  // Custom model
}

impl WakeWordConfig {
    pub fn new(settings: WakeWordSettings) -> Self {
        Self {
            settings,
            access_key: None,
            model_path: None,
        }
    }

    pub fn with_phrase(mut self, phrase: &str) -> Self {
        self.settings.phrase = phrase.to_string();
        self
    }
}

impl Default for WakeWordConfig {
    fn default() -> Self {
        Self::new(WakeWordSettings::default())
    }
}

impl WakeWordDetector {
    pub fn new(config: WakeWordConfig) -> Self {
        Self {
            config: config.settings,
            state: Arc::new(Mutex::new(DetectorState::default())),
        }
    }

    /// Start listening for wake word
    pub async fn start(&self) -> VoiceResult<()> {
        let mut state = self.state.lock().await;
        state.is_listening = true;
        log::info!("Wake word detector started: '{}'", self.config.phrase);
        Ok(())
    }

    /// Stop listening
    pub async fn stop(&self) -> VoiceResult<()> {
        let mut state = self.state.lock().await;
        state.is_listening = false;
        log::info!("Wake word detector stopped");
        Ok(())
    }

    /// Process audio frame for wake word detection
    pub async fn process(&self, audio_frame: &[i16]) -> VoiceResult<bool> {
        let mut state = self.state.lock().await;
        
        if !state.is_listening {
            return Ok(false);
        }

        // In a real implementation, this would use:
        // - Porcupine (Picovoice) for accurate wake word detection
        // - Vosk or similar for open-source alternative
        // - Custom neural network model
        
        // For now, simulate detection based on audio energy
        let energy: f64 = audio_frame
            .iter()
            .map(|&s| (s as f64).powi(2))
            .sum::<f64>()
            / audio_frame.len() as f64;

        let energy_level = energy.sqrt() / 32768.0;
        
        // Simple threshold-based detection (placeholder)
        if energy_level > self.config.sensitivity as f64 * 0.1 {
            log::debug!("Audio energy: {:.4}", energy_level);
        }

        // Placeholder: return false (no wake word detected)
        Ok(false)
    }

    /// Check if wake word was detected
    pub async fn was_detected(&self) -> bool {
        let state = self.state.lock().await;
        state.wake_word_detected
    }

    /// Reset detection state
    pub async fn reset(&self) {
        let mut state = self.state.lock().await;
        state.wake_word_detected = false;
    }

    /// Manually trigger wake word (for testing)
    pub async fn trigger(&self) {
        let mut state = self.state.lock().await;
        state.wake_word_detected = true;
        state.last_detection = Some(chrono::Utc::now());
        state.detections += 1;
    }

    /// Get detection count
    pub async fn detection_count(&self) -> u64 {
        let state = self.state.lock().await;
        state.detections
    }

    /// Get wake word phrase
    pub fn phrase(&self) -> &str {
        &self.config.phrase
    }

    /// Update wake word phrase
    pub fn set_phrase(&mut self, phrase: &str) {
        self.config.phrase = phrase.to_string();
    }

    /// Get state
    pub async fn state(&self) -> DetectorState {
        self.state.lock().await.clone()
    }
}

impl Default for WakeWordDetector {
    fn default() -> Self {
        Self::new(WakeWordConfig::default())
    }
}

/// Create wake word detector with custom phrase
pub fn create_wake_word_detector(assistant_name: &str) -> WakeWordDetector {
    let phrase = format!("hey {}", assistant_name.to_lowercase());
    WakeWordDetector::new(
        WakeWordConfig::default().with_phrase(&phrase)
    )
}

/// Wake word struct for compatibility
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WakeWord {
    pub id: String,
    pub phrase: String,
    pub confidence: f32,
    pub timestamp: f64,
    pub language: String,
    pub sensitivity: f32,
    pub enabled: bool,
}

impl Default for WakeWord {
    fn default() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            phrase: "hey sentient".to_string(),
            confidence: 0.0,
            timestamp: 0.0,
            language: "tr".to_string(),
            sensitivity: 0.5,
            enabled: true,
        }
    }
}

impl From<WakeWordSettings> for WakeWord {
    fn from(settings: WakeWordSettings) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            phrase: settings.phrase,
            confidence: 0.0,
            timestamp: 0.0,
            language: "tr".to_string(),
            sensitivity: settings.sensitivity,
            enabled: true,
        }
    }
}
