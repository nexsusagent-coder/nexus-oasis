//! ─── Vosk Wake Word Engine ───
//!
//! Offline, open-source speech recognition.
//! Downloads models from https://alphacephei.com/vosk/models

use crate::{WakeError, WakeWordConfig};

/// Detect wake word using Vosk
#[cfg(feature = "vosk")]
pub fn detect(samples: &[f32], config: &WakeWordConfig) -> Result<Option<(f32, String)>, WakeError> {
    use vosk::{Model, Recognizer, DecodingState};
    use std::sync::OnceLock;
    
    static MODEL: OnceLock<Model> = OnceLock::new();
    
    // Initialize model
    let model = MODEL.get_or_init(|| {
        let model_path = config.vosk_model_path.as_deref()
            .unwrap_or("vosk-model-small-en-us-0.15");
        
        Model::new(model_path)
            .expect("Failed to load Vosk model")
    });
    
    // Create recognizer
    let mut recognizer = Recognizer::new(model, config.sample_rate as f32)
        .ok_or_else(|| WakeError::Internal("Failed to create recognizer".into()))?;
    
    // Convert f32 samples to i16
    let samples_i16: Vec<i16> = samples
        .iter()
        .map(|&s| (s * 32767.0) as i16)
        .collect();
    
    // Process audio
    let state = recognizer.accept_waveform(&samples_i16);
    
    match state {
        DecodingState::Running => Ok(None),
        DecodingState::EndOfUtterance => {
            let result = recognizer.final_result();
            let text = result.text.to_lowercase();
            
            log::debug!("Vosk result: {}", text);
            
            // Check for wake word
            if text.contains(&config.wake_word.to_lowercase()) {
                // Calculate confidence (simple heuristic)
                let confidence = if text.trim() == config.wake_word.to_lowercase() {
                    0.95
                } else if text.starts_with(&config.wake_word.to_lowercase()) {
                    0.85
                } else {
                    0.7
                };
                
                Ok(Some((confidence, config.wake_word.clone())))
            } else {
                // Speech detected but not wake word
                Ok(None)
            }
        }
        _ => Ok(None),
    }
}

#[cfg(not(feature = "vosk"))]
pub fn detect(_samples: &[f32], _config: &WakeWordConfig) -> Result<Option<(f32, String)>, WakeError> {
    Err(WakeError::Internal("Vosk feature not enabled".into()))
}
