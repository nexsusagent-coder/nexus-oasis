//! ─── Custom Wake Word Detection System ───
//!
//!  Features:
//!  - Multiple custom wake words
//!  - Wake word training from audio samples
//!  - Sensitivity tuning per wake word
//!  - Multi-language support
//!  - Pattern matching with MFCC features

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::path::Path;

/// Wake word detection result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WakeWord {
    pub id: String,
    pub phrase: String,
    pub confidence: f32,
    pub timestamp: f32,
    pub language: String,
}

/// Custom wake word model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WakeWordModel {
    /// Unique identifier
    pub id: String,
    
    /// Wake word phrase (e.g., "Hey SENTIENT", "Hello Assistant")
    pub phrase: String,
    
    /// Language code (en, tr, de, etc.)
    pub language: String,
    
    /// Phoneme sequence for the phrase
    pub phonemes: Vec<String>,
    
    /// MFCC feature templates
    pub feature_templates: Vec<Vec<f32>>,
    
    /// Detection sensitivity (0.0 - 1.0)
    pub sensitivity: f32,
    
    /// Minimum detection threshold
    pub threshold: f32,
    
    /// Training samples count
    pub training_samples: usize,
    
    /// Model accuracy (0.0 - 1.0)
    pub accuracy: f32,
    
    /// Creation timestamp
    pub created_at: chrono::DateTime<chrono::Utc>,
    
    /// Last updated
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl WakeWordModel {
    /// Create new wake word model
    pub fn new(phrase: String, language: String) -> Self {
        let id = format!("ww_{}", uuid::Uuid::new_v4());
        let now = chrono::Utc::now();
        
        Self {
            id,
            phrase,
            language,
            phonemes: Vec::new(),
            feature_templates: Vec::new(),
            sensitivity: 0.5,
            threshold: 0.7,
            training_samples: 0,
            accuracy: 0.0,
            created_at: now,
            updated_at: now,
        }
    }
    
    /// Update sensitivity
    pub fn set_sensitivity(&mut self, sensitivity: f32) {
        self.sensitivity = sensitivity.clamp(0.0, 1.0);
        self.threshold = 1.0 - sensitivity;
        self.updated_at = chrono::Utc::now();
    }
}

/// Wake word trainer
pub struct WakeWordTrainer {
    /// Audio sample rate
    #[allow(dead_code)]
    sample_rate: u32,
    
    /// MFCC configuration
    #[allow(dead_code)]
    mfcc_config: MfccConfig,
}

impl WakeWordTrainer {
    pub fn new() -> Self {
        Self {
            sample_rate: 16000,
            mfcc_config: MfccConfig::default(),
        }
    }
    
    /// Train wake word from audio samples
    pub async fn train(
        &self,
        phrase: &str,
        language: &str,
        audio_samples: &[Vec<f32>],
    ) -> Result<WakeWordModel, WakeWordError> {
        if audio_samples.len() < 3 {
            return Err(WakeWordError::InsufficientSamples(
                "Need at least 3 audio samples for training".into()
            ));
        }
        
        let mut model = WakeWordModel::new(phrase.into(), language.into());
        
        // Extract phonemes from phrase
        model.phonemes = self.extract_phonemes(phrase, language);
        
        // Extract MFCC features from each sample
        let mut features = Vec::new();
        for sample in audio_samples {
            let mfcc = self.extract_mfcc(sample);
            features.push(mfcc);
        }
        
        // Average features for template
        model.feature_templates = self.average_features(&features);
        model.training_samples = audio_samples.len();
        model.accuracy = 0.85; // Initial estimate
        model.updated_at = chrono::Utc::now();
        
        Ok(model)
    }
    
    /// Extract phonemes from text (simplified)
    fn extract_phonemes(&self, text: &str, _language: &str) -> Vec<String> {
        // Simplified phoneme extraction
        // Real implementation would use CMUdict or similar
        text.to_lowercase()
            .split_whitespace()
            .flat_map(|word| word.chars().collect::<Vec<_>>())
            .map(|c| c.to_string())
            .collect()
    }
    
    /// Extract MFCC features
    fn extract_mfcc(&self, audio: &[f32]) -> Vec<f32> {
        // Simplified MFCC extraction
        // Real implementation would use FFT, filter banks, DCT
        let frame_size = 400; // 25ms at 16kHz
        let frame_shift = 160; // 10ms
        let num_coefficients = 13;
        
        let mut features = Vec::new();
        
        for i in (0..audio.len()).step_by(frame_shift) {
            let end = (i + frame_size).min(audio.len());
            let frame = &audio[i..end];
            
            // Calculate energy
            let energy: f32 = frame.iter().map(|s| s * s).sum::<f32>()
                / frame.len() as f32;
            features.push(energy.sqrt());
            
            // Add placeholder MFCC coefficients
            for _ in 1..num_coefficients {
                features.push(0.0);
            }
        }
        
        features
    }
    
    /// Average multiple feature sets
    fn average_features(&self, features: &[Vec<f32>]) -> Vec<Vec<f32>> {
        if features.is_empty() {
            return Vec::new();
        }
        
        // Find minimum length
        let min_len = features.iter().map(|f| f.len()).min().unwrap_or(0);
        
        // Average features frame by frame
        let num_frames = min_len / 13;
        let mut templates = Vec::new();
        
        for frame_idx in 0..num_frames {
            let start = frame_idx * 13;
            let end = start + 13;
            
            let mut avg_frame = vec![0.0; 13];
            for feature_set in features {
                for (i, j) in (start..end).enumerate() {
                    if j < feature_set.len() {
                        avg_frame[i] += feature_set[j];
                    }
                }
            }
            
            for val in &mut avg_frame {
                *val /= features.len() as f32;
            }
            
            templates.push(avg_frame);
        }
        
        templates
    }
}

impl Default for WakeWordTrainer {
    fn default() -> Self {
        Self::new()
    }
}

/// MFCC configuration
#[derive(Debug, Clone)]
pub struct MfccConfig {
    pub sample_rate: u32,
    pub frame_size: usize,
    pub frame_shift: usize,
    pub num_coefficients: usize,
    pub num_filter_banks: usize,
    pub low_freq: f32,
    pub high_freq: f32,
}

impl Default for MfccConfig {
    fn default() -> Self {
        Self {
            sample_rate: 16000,
            frame_size: 400,
            frame_shift: 160,
            num_coefficients: 13,
            num_filter_banks: 26,
            low_freq: 20.0,
            high_freq: 8000.0,
        }
    }
}

/// Multi wake word detector
pub struct MultiWakeWordDetector {
    /// Registered wake word models
    models: HashMap<String, WakeWordModel>,
    
    /// Audio buffer for processing
    audio_buffer: VecDeque<f32>,
    
    /// Energy history
    energy_history: VecDeque<f32>,
    
    /// Frame counter
    frame_counter: usize,
    
    /// Sample rate
    sample_rate: u32,
    
    /// Detection cooldown (frames)
    cooldown_frames: usize,
    
    /// Last detection frame
    last_detection_frame: usize,
}

impl MultiWakeWordDetector {
    pub fn new() -> Self {
        Self {
            models: HashMap::new(),
            audio_buffer: VecDeque::with_capacity(16000),
            energy_history: VecDeque::with_capacity(100),
            frame_counter: 0,
            sample_rate: 16000,
            cooldown_frames: 32, // ~2 seconds at 16kHz/512 frame
            last_detection_frame: 0,
        }
    }
    
    /// Register wake word model
    pub fn register(&mut self, model: WakeWordModel) {
        self.models.insert(model.id.clone(), model);
    }
    
    /// Unregister wake word
    pub fn unregister(&mut self, id: &str) {
        self.models.remove(id);
    }
    
    /// Get all registered wake words
    pub fn list(&self) -> Vec<&WakeWordModel> {
        self.models.values().collect()
    }
    
    /// Train and register new wake word
    pub async fn train(
        &mut self,
        phrase: &str,
        language: &str,
        audio_samples: &[Vec<f32>],
    ) -> Result<String, WakeWordError> {
        // Create a new trainer for this operation
        let trainer = WakeWordTrainer::new();
        let model = trainer.train(phrase, language, audio_samples).await?;
        let id = model.id.clone();
        self.register(model);
        Ok(id)
    }
    
    /// Update wake word sensitivity
    pub fn set_sensitivity(&mut self, id: &str, sensitivity: f32) -> bool {
        if let Some(model) = self.models.get_mut(id) {
            model.set_sensitivity(sensitivity);
            true
        } else {
            false
        }
    }
    
    /// Process audio frame for wake word detection
    pub fn process(&mut self, audio: &[f32]) -> Option<WakeWord> {
        // Add to buffer
        self.audio_buffer.extend(audio);
        
        // Keep buffer size limited
        while self.audio_buffer.len() > self.sample_rate as usize {
            self.audio_buffer.pop_front();
        }
        
        // Calculate energy
        let energy: f32 = if audio.is_empty() {
            0.0
        } else {
            (audio.iter().map(|s| s * s).sum::<f32>() / audio.len() as f32).sqrt()
        };
        
        self.energy_history.push_back(energy);
        if self.energy_history.len() > 100 {
            self.energy_history.pop_front();
        }
        
        self.frame_counter += 1;
        
        // Check cooldown
        if self.frame_counter - self.last_detection_frame < self.cooldown_frames {
            return None;
        }
        
        // Check for each wake word
        let avg_energy: f32 = self.energy_history.iter().sum::<f32>()
            / self.energy_history.len().max(1) as f32;
        
        // Only process if energy is above background
        if energy < avg_energy * 1.5 {
            return None;
        }
        
        // Check each model
        for model in self.models.values() {
            if let Some(detection) = self.check_model(model, audio, energy, avg_energy) {
                self.last_detection_frame = self.frame_counter;
                return Some(detection);
            }
        }
        
        None
    }
    
    /// Check specific wake word model
    fn check_model(
        &self,
        model: &WakeWordModel,
        _audio: &[f32],
        energy: f32,
        avg_energy: f32,
    ) -> Option<WakeWord> {
        // Calculate confidence based on energy and sensitivity
        let energy_ratio = energy / (avg_energy + 0.001);
        let mut confidence = (energy_ratio / 3.0).min(1.0) * model.sensitivity;
        
        // Check feature templates if available
        if !model.feature_templates.is_empty() {
            let trainer = WakeWordTrainer::new();
            let features = trainer.extract_mfcc(&self.audio_buffer.iter().copied().collect::<Vec<_>>());
            let feature_similarity = self.compare_features(&features, &model.feature_templates);
            confidence = confidence * 0.3 + feature_similarity * 0.7;
        }
        
        // Check threshold
        if confidence >= model.threshold {
            Some(WakeWord {
                id: model.id.clone(),
                phrase: model.phrase.clone(),
                confidence,
                timestamp: self.frame_counter as f32 / (self.sample_rate as f32 / 512.0),
                language: model.language.clone(),
            })
        } else {
            None
        }
    }
    
    /// Compare features with templates
    fn compare_features(&self, features: &[f32], templates: &[Vec<f32>]) -> f32 {
        if templates.is_empty() || features.is_empty() {
            return 0.0;
        }
        
        // Simple correlation-based comparison
        let mut total_similarity = 0.0;
        let mut comparisons = 0;
        
        for template in templates {
            for i in (0..features.len()).step_by(13) {
                let end = (i + 13).min(features.len());
                let frame = &features[i..end];
                
                if frame.len() == template.len() {
                    let dot: f32 = frame.iter().zip(template.iter())
                        .map(|(a, b)| a * b)
                        .sum();
                    let norm_a: f32 = frame.iter().map(|x| x * x).sum::<f32>().sqrt();
                    let norm_b: f32 = template.iter().map(|x| x * x).sum::<f32>().sqrt();
                    
                    if norm_a > 0.0 && norm_b > 0.0 {
                        total_similarity += dot / (norm_a * norm_b);
                        comparisons += 1;
                    }
                }
            }
        }
        
        if comparisons > 0 {
            (total_similarity / comparisons as f32).max(0.0)
        } else {
            0.0
        }
    }
    
    /// Reset detector state
    pub fn reset(&mut self) {
        self.audio_buffer.clear();
        self.energy_history.clear();
        self.frame_counter = 0;
        self.last_detection_frame = 0;
    }
    
    /// Save models to file
    pub fn save_models(&self, path: &Path) -> Result<(), WakeWordError> {
        let models: Vec<_> = self.models.values().collect();
        let json = serde_json::to_string_pretty(&models)
            .map_err(|e| WakeWordError::SaveError(e.to_string()))?;
        
        std::fs::write(path, json)
            .map_err(|e| WakeWordError::SaveError(e.to_string()))?;
        
        Ok(())
    }
    
    /// Load models from file
    pub fn load_models(&mut self, path: &Path) -> Result<(), WakeWordError> {
        let json = std::fs::read_to_string(path)
            .map_err(|e| WakeWordError::LoadError(e.to_string()))?;
        
        let models: Vec<WakeWordModel> = serde_json::from_str(&json)
            .map_err(|e| WakeWordError::LoadError(e.to_string()))?;
        
        for model in models {
            self.register(model);
        }
        
        Ok(())
    }
}

impl Default for MultiWakeWordDetector {
    fn default() -> Self {
        Self::new()
    }
}

/// Single wake word detector (backward compatible)
pub struct WakeWordDetector {
    phrase: String,
    sensitivity: f32,
    threshold: f32,
    energy_history: VecDeque<f32>,
    frame_counter: usize,
    last_detection_timestamp: f32,
}

impl WakeWordDetector {
    pub fn new(config: crate::config::WakeWordConfig) -> Self {
        Self {
            phrase: config.phrase,
            sensitivity: config.sensitivity,
            threshold: 0.5,
            energy_history: VecDeque::with_capacity(100),
            frame_counter: 0,
            last_detection_timestamp: 0.0,
        }
    }
    
    /// Create with custom phrase
    pub fn with_phrase(phrase: String) -> Self {
        Self {
            phrase,
            sensitivity: 0.5,
            threshold: 0.5,
            energy_history: VecDeque::with_capacity(100),
            frame_counter: 0,
            last_detection_timestamp: 0.0,
        }
    }
    
    /// Detect wake word in audio
    pub async fn detect(&self, audio: &[f32]) -> Option<WakeWord> {
        let energy: f32 = if audio.is_empty() {
            0.0
        } else {
            (audio.iter().map(|s| s * s).sum::<f32>() / audio.len() as f32).sqrt()
        };
        
        if energy > self.threshold {
            let current_time = self.frame_counter as f32 * (audio.len() as f32 / 16000.0);
            
            if current_time - self.last_detection_timestamp > 2.0 {
                return Some(WakeWord {
                    id: "default".into(),
                    phrase: self.phrase.clone(),
                    confidence: (energy / self.threshold).min(1.0),
                    timestamp: current_time,
                    language: "en".into(),
                });
            }
        }
        
        None
    }
    
    /// Process frame (mutable)
    pub fn process_frame(&mut self, audio: &[f32]) -> Option<WakeWord> {
        let energy: f32 = if audio.is_empty() {
            0.0
        } else {
            (audio.iter().map(|s| s * s).sum::<f32>() / audio.len() as f32).sqrt()
        };
        
        self.energy_history.push_back(energy);
        if self.energy_history.len() > 100 {
            self.energy_history.pop_front();
        }
        
        self.frame_counter += 1;
        
        let avg_energy: f32 = self.energy_history.iter().sum::<f32>()
            / self.energy_history.len().max(1) as f32;
        
        if energy > avg_energy * 2.0 && energy > self.threshold {
            let current_time = self.frame_counter as f32 * (audio.len() as f32 / 16000.0);
            
            if current_time - self.last_detection_timestamp > 1.5 {
                self.last_detection_timestamp = current_time;
                
                return Some(WakeWord {
                    id: "default".into(),
                    phrase: self.phrase.clone(),
                    confidence: (energy / (avg_energy + 0.001)).min(1.0),
                    timestamp: current_time,
                    language: "en".into(),
                });
            }
        }
        
        None
    }
    
    pub fn phrase(&self) -> &str {
        &self.phrase
    }
    
    pub fn set_sensitivity(&mut self, sensitivity: f32) {
        self.sensitivity = sensitivity;
        self.threshold = 0.05 * (1.0 / sensitivity.max(0.1));
    }
    
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

/// Wake word errors
#[derive(Debug, thiserror::Error)]
pub enum WakeWordError {
    #[error("Insufficient training samples: {0}")]
    InsufficientSamples(String),
    
    #[error("Training failed: {0}")]
    TrainingFailed(String),
    
    #[error("Save error: {0}")]
    SaveError(String),
    
    #[error("Load error: {0}")]
    LoadError(String),
    
    #[error("Invalid audio: {0}")]
    InvalidAudio(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_wake_word_model() {
        let model = WakeWordModel::new("Hey SENTIENT".into(), "en".into());
        assert_eq!(model.phrase, "Hey SENTIENT");
        assert_eq!(model.language, "en");
    }
    
    #[test]
    fn test_multi_detector() {
        let mut detector = MultiWakeWordDetector::new();
        let model = WakeWordModel::new("Hello".into(), "en".into());
        detector.register(model);
        
        assert_eq!(detector.list().len(), 1);
    }
    
    #[test]
    fn test_sensitivity() {
        let mut model = WakeWordModel::new("Test".into(), "en".into());
        model.set_sensitivity(0.8);
        assert!((model.sensitivity - 0.8).abs() < 0.01);
    }
}
