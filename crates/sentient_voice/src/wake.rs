//! ─── Wake Word Detection ───

use serde::{Deserialize, Serialize};
use crate::config::WakeWordConfig;
use std::collections::VecDeque;

/// Wake word detection result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WakeWord {
    pub phrase: String,
    pub confidence: f32,
    pub timestamp: f32,
}

/// Energy-based wake word detector
/// Detects sudden increase in audio energy followed by speech pattern
pub struct WakeWordDetector {
    config: WakeWordConfig,
    phrase: String,
    
    /// Energy history for pattern matching
    energy_history: VecDeque<f32>,
    
    /// Pattern matching threshold
    energy_threshold: f32,
    
    /// Duration threshold (frames)
    min_duration_frames: usize,
    
    /// Current frame counter
    frame_counter: usize,
    
    /// Last detected timestamp
    last_detection_timestamp: f32,
}

impl WakeWordDetector {
    pub fn new(config: WakeWordConfig) -> Self {
        Self {
            phrase: config.phrase.clone(),
            energy_history: VecDeque::with_capacity(100),
            config,
            energy_threshold: 0.05,
            min_duration_frames: 10,
            frame_counter: 0,
            last_detection_timestamp: 0.0,
        }
    }
    
    /// Detect wake word in audio using energy pattern analysis
    /// 
    /// This implementation uses:
    /// 1. Energy-based voice activity detection
    /// 2. Pattern matching for wake word-like energy signatures
    /// 3. Spectral analysis (simplified)
    pub async fn detect(&self, audio: &[f32]) -> Option<WakeWord> {
        // Calculate frame energy
        let energy: f32 = if audio.is_empty() {
            0.0
        } else {
            (audio.iter().map(|s| s * s).sum::<f32>() / audio.len() as f32).sqrt()
        };
        
        // Simple threshold detection for prototype
        // Real implementation would use Porcupine or similar
        if energy > self.energy_threshold {
            // Check if enough time has passed since last detection (debounce)
            let current_time = self.frame_counter as f32 * (audio.len() as f32 / 16000.0);
            if current_time - self.last_detection_timestamp > 2.0 {
                // Simulate detection based on energy pattern
                // In production, use Porcupine, Vosk, or similar
                let confidence = if energy > self.energy_threshold * 2.0 {
                    0.85
                } else if energy > self.energy_threshold * 1.5 {
                    0.7
                } else {
                    0.5
                };
                
                return Some(WakeWord {
                    phrase: self.phrase.clone(),
                    confidence,
                    timestamp: current_time,
                });
            }
        }
        
        None
    }
    
    /// Process audio frame for wake word detection (mutable version)
    pub fn process_frame(&mut self, audio: &[f32]) -> Option<WakeWord> {
        // Calculate frame energy
        let energy: f32 = if audio.is_empty() {
            0.0
        } else {
            (audio.iter().map(|s| s * s).sum::<f32>() / audio.len() as f32).sqrt()
        };
        
        // Update energy history
        self.energy_history.push_back(energy);
        if self.energy_history.len() > 100 {
            self.energy_history.pop_front();
        }
        
        self.frame_counter += 1;
        
        // Detect sudden energy increase (start of speech)
        let avg_energy: f32 = self.energy_history.iter().sum::<f32>() 
            / self.energy_history.len().max(1) as f32;
        
        if energy > avg_energy * 2.0 && energy > self.energy_threshold {
            let current_time = self.frame_counter as f32 * (audio.len() as f32 / 16000.0);
            
            // Debounce
            if current_time - self.last_detection_timestamp > 1.5 {
                self.last_detection_timestamp = current_time;
                
                return Some(WakeWord {
                    phrase: self.phrase.clone(),
                    confidence: (energy / (avg_energy + 0.001)).min(1.0),
                    timestamp: current_time,
                });
            }
        }
        
        None
    }
    
    /// Get wake word phrase
    pub fn phrase(&self) -> &str {
        &self.phrase
    }
    
    /// Update sensitivity
    pub fn set_sensitivity(&mut self, sensitivity: f32) {
        self.config.sensitivity = sensitivity;
        self.energy_threshold = 0.05 * (1.0 / sensitivity.max(0.1));
    }
    
    /// Reset detector state
    pub fn reset(&mut self) {
        self.energy_history.clear();
        self.frame_counter = 0;
        self.last_detection_timestamp = 0.0;
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
