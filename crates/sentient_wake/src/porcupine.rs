//! ─── Porcupine Wake Word Engine ───
//!
//! High-accuracy wake word detection by Picovoice.
//! Features:
//! - Industry-leading accuracy
//! - Low latency
//! - Custom wake word training available
//!
//! To use:
//! 1. Get an access key from https://picovoice.ai
//! 2. Set porcupine_access_key in config
//! 3. Enable "porcupine" feature
//!
//! Note: pvporcupine crate requires native library installation.
//! Check Picovoice documentation for setup instructions.

use crate::{WakeError, WakeWordConfig};

/// Detect wake word using Porcupine
#[cfg(feature = "porcupine")]
pub fn detect(samples: &[f32], config: &WakeWordConfig) -> Result<Option<(f32, String)>, WakeError> {
    use pv_porcupine::{Porcupine, PorcupineBuilder};
    use std::sync::OnceLock;
    
    static PORCUPINE: OnceLock<Result<Porcupine, String>> = OnceLock::new();
    
    let access_key = config.porcupine_access_key.as_ref()
        .ok_or(WakeError::InvalidAccessKey)?;
    
    // Initialize Porcupine (cached)
    let porcupine = PORCUPINE.get_or_init(|| {
        PorcupineBuilder::new_with_keywords(
            access_key,
            &[&config.wake_word],
        )
        .sensitivity(config.sensitivity)
        .init()
        .map_err(|e| format!("Porcupine init failed: {}", e))
    });
    
    let porcupine = match porcupine {
        Ok(p) => p,
        Err(e) => return Err(WakeError::Internal(e.clone())),
    };
    
    // Process frame
    let frame_length = porcupine.frame_length() as usize;
    
    // Ensure we have enough samples
    if samples.len() < frame_length {
        return Ok(None);
    }
    
    // Convert f32 to i16
    let frame_i16: Vec<i16> = samples[..frame_length]
        .iter()
        .map(|&s| (s * 32767.0).clamp(-32768.0, 32767.0) as i16)
        .collect();
    
    let result = porcupine.process(&frame_i16)
        .map_err(|e| WakeError::Internal(e.to_string()))?;
    
    match result {
        Some(_index) => {
            log::debug!("Porcupine detected wake word: {}", config.wake_word);
            Ok(Some((1.0, config.wake_word.clone())))
        }
        None => Ok(None),
    }
}

/// Simulation mode - used when porcupine feature is not enabled
#[cfg(not(feature = "porcupine"))]
pub fn detect(samples: &[f32], config: &WakeWordConfig) -> Result<Option<(f32, String)>, WakeError> {
    // Calculate audio energy as a simple wake word simulation
    let energy: f32 = samples.iter().map(|s| s.abs()).sum::<f32>() / samples.len() as f32;
    
    // If audio energy is significant, simulate wake word detection
    if energy > 0.1 {
        // Simulated confidence based on energy
        let confidence = (energy * 5.0).min(0.95);
        
        if confidence > config.confidence_threshold {
            log::debug!("Porcupine simulation mode: energy={}, confidence={}", energy, confidence);
            return Ok(Some((confidence, config.wake_word.clone())));
        }
    }
    
    Ok(None)
}

/// Check if Porcupine is available
pub fn is_available() -> bool {
    #[cfg(feature = "porcupine")]
    {
        true
    }
    #[cfg(not(feature = "porcupine"))]
    {
        false
    }
}

/// Get Porcupine status message
pub fn status() -> &'static str {
    #[cfg(feature = "porcupine")]
    {
        "Porcupine native library loaded"
    }
    #[cfg(not(feature = "porcupine"))]
    {
        "Porcupine simulation mode (enable 'porcupine' feature for real detection)"
    }
}
