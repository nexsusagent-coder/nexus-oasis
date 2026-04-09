//! ─── Porcupine Wake Word Engine ───
//!
//! High-accuracy wake word detection by Picovoice.
//! Requires an access key from https://picovoice.ai

use crate::{WakeError, WakeWordConfig};

/// Detect wake word using Porcupine
#[cfg(feature = "porcupine")]
pub fn detect(samples: &[f32], config: &WakeWordConfig) -> Result<Option<(f32, String)>, WakeError> {
    use pv_porcupine::{Porcupine, PorcupineBuilder};
    
    let access_key = config.porcupine_access_key.as_ref()
        .ok_or(WakeError::InvalidAccessKey)?;
    
    // Create Porcupine instance
    let porcupine = PorcupineBuilder::new_with_keywords(
        access_key,
        &[&config.wake_word],
    )
    .sensitivity(config.sensitivity)
    .init()
    .map_err(|e| WakeError::Internal(e.to_string()))?;
    
    // Process frame
    let frame_length = porcupine.frame_length() as usize;
    let frame = &samples[..frame_length.min(samples.len())];
    
    let result = porcupine.process(frame)
        .map_err(|e| WakeError::Internal(e.to_string()))?;
    
    match result {
        Some(index) => {
            // Wake word detected
            Ok(Some((1.0, config.wake_word.clone())))
        }
        None => Ok(None),
    }
}

#[cfg(not(feature = "porcupine"))]
pub fn detect(_samples: &[f32], _config: &WakeWordConfig) -> Result<Option<(f32, String)>, WakeError> {
    Err(WakeError::Internal("Porcupine feature not enabled".into()))
}
