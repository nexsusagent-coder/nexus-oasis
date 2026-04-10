//! ─── Vosk Wake Word Engine ───
//!
//! Vosk is an offline open-source speech recognition toolkit.
//! Features:
//! - Completely offline (no internet required)
//! - Multiple language models available
//! - Lightweight and fast
//!
//! To use:
//! 1. Download a model from https://alphacephei.com/vosk/models
//! 2. Set vosk_model_path in config
//! 3. Enable "vosk" feature

use crate::{WakeError, WakeWordConfig};

/// Detect wake word using Vosk
#[cfg(feature = "vosk")]
pub fn detect(samples: &[f32], config: &WakeWordConfig) -> Result<Option<(f32, String)>, WakeError> {
    use vosk::{Model, Recognizer};
    use std::sync::OnceLock;
    
    static MODEL: OnceLock<Option<Model>> = OnceLock::new();
    
    // Initialize model
    let model = MODEL.get_or_init(|| {
        let model_path = config.vosk_model_path.as_deref()
            .unwrap_or("vosk-model-small-en-us-0.15");
        
        log::info!("Loading Vosk model from: {}", model_path);
        Model::new(model_path)
    });
    
    let model = match model {
        Some(m) => m,
        None => return Err(WakeError::ModelNotFound("Failed to load Vosk model".into())),
    };
    
    // Create recognizer
    let mut recognizer = Recognizer::new(model, config.sample_rate as f32)
        .ok_or_else(|| WakeError::Internal("Failed to create recognizer".into()))?;
    
    // Convert f32 samples to i16
    let samples_i16: Vec<i16> = samples
        .iter()
        .map(|&s| (s * 32767.0).clamp(-32768.0, 32767.0) as i16)
        .collect();
    
    // Process audio
    recognizer.accept_waveform(&samples_i16);
    
    // Get result
    let result = recognizer.final_result();
    let json_result = serde_json::to_string(&result)
        .unwrap_or_default();
    
    log::debug!("Vosk result: {}", json_result);
    
    // Parse the text from result
    let text = if let Ok(obj) = serde_json::from_str::<serde_json::Value>(&json_result) {
        obj["text"].as_str().unwrap_or("").to_lowercase()
    } else {
        String::new()
    };
    
    // Check for wake word
    if text.contains(&config.wake_word.to_lowercase()) {
        let confidence = if text.trim() == config.wake_word.to_lowercase() {
            0.95
        } else if text.starts_with(&config.wake_word.to_lowercase()) {
            0.85
        } else {
            0.7
        };
        
        Ok(Some((confidence, config.wake_word.clone())))
    } else if !text.is_empty() {
        Ok(None)
    } else {
        Ok(None)
    }
}

/// Simulation mode - used when vosk feature is not enabled
#[cfg(not(feature = "vosk"))]
pub fn detect(samples: &[f32], config: &WakeWordConfig) -> Result<Option<(f32, String)>, WakeError> {
    // Calculate audio energy as a simple wake word simulation
    let energy: f32 = samples.iter().map(|s| s.abs()).sum::<f32>() / samples.len() as f32;
    
    // If audio energy is significant, simulate wake word detection
    if energy > 0.1 {
        // Simulated confidence based on energy
        let confidence = (energy * 5.0).min(0.9);
        
        if confidence > config.confidence_threshold {
            log::debug!("Vosk simulation mode: energy={}, confidence={}", energy, confidence);
            return Ok(Some((confidence, config.wake_word.clone())));
        }
    }
    
    Ok(None)
}

/// Check if Vosk is available
pub fn is_available() -> bool {
    #[cfg(feature = "vosk")]
    {
        true
    }
    #[cfg(not(feature = "vosk"))]
    {
        false
    }
}

/// Get Vosk status message
pub fn status() -> &'static str {
    #[cfg(feature = "vosk")]
    {
        "Vosk native library loaded"
    }
    #[cfg(not(feature = "vosk"))]
    {
        "Vosk simulation mode (enable 'vosk' feature for real detection)"
    }
}
