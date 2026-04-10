//! ─── Whisper Wake Word Engine ───
//!
//! OpenAI's Whisper model for wake word detection.
//! Features:
//! - More accurate than Vosk
//! - Multiple language support
//! - Can be run locally with whisper.cpp
//!
//! To use:
//! 1. Download a Whisper model (ggml-tiny.en.bin, ggml-base.bin, etc.)
//! 2. Set whisper_model_path in config
//! 3. Enable "whisper" feature

use crate::{WakeError, WakeWordConfig};

/// Detect wake word using Whisper
#[cfg(feature = "whisper")]
pub fn detect(samples: &[f32], config: &WakeWordConfig) -> Result<Option<(f32, String)>, WakeError> {
    use whisper_rs::{WhisperContext, WhisperContextParameters, FullParams, SamplingStrategy};
    use std::sync::OnceLock;
    
    static CONTEXT: OnceLock<Result<WhisperContext, String>> = OnceLock::new();
    
    // Initialize context
    let ctx = CONTEXT.get_or_init(|| {
        let model_path = config.whisper_model_path.as_deref()
            .unwrap_or("ggml-tiny.en.bin");
        
        log::info!("Loading Whisper model from: {}", model_path);
        
        WhisperContext::new_with_params(
            model_path,
            WhisperContextParameters::default()
        ).map_err(|e| format!("Failed to load Whisper model: {}", e))
    });
    
    let ctx = match ctx {
        Ok(c) => c,
        Err(e) => return Err(WakeError::ModelNotFound(e.clone())),
    };
    
    // Create parameters
    let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });
    params.set_language(Some("en"));
    params.set_translate(false);
    params.set_no_context(true);
    params.set_single_segment(true);
    
    // Run inference
    let state = ctx.create_state()
        .map_err(|e| WakeError::Internal(e.to_string()))?;
    
    state.full(params, samples)
        .map_err(|e| WakeError::Internal(e.to_string()))?;
    
    // Get result
    let num_segments = state.full_n_segments()
        .map_err(|e| WakeError::Internal(e.to_string()))?;
    
    if num_segments == 0 {
        return Ok(None);
    }
    
    let text = state.full_get_segment_text(0)
        .map_err(|e| WakeError::Internal(e.to_string()))?;
    
    let text_lower = text.to_lowercase();
    
    log::debug!("Whisper result: {}", text);
    
    // Check for wake word
    if text_lower.contains(&config.wake_word.to_lowercase()) {
        // Get probability
        let prob = state.full_get_segment_prob(0)
            .map_err(|e| WakeError::Internal(e.to_string()))?;
        
        Ok(Some((prob, config.wake_word.clone())))
    } else {
        Ok(None)
    }
}

/// Simulation mode - used when whisper feature is not enabled
#[cfg(not(feature = "whisper"))]
pub fn detect(samples: &[f32], config: &WakeWordConfig) -> Result<Option<(f32, String)>, WakeError> {
    // Calculate audio energy as a simple wake word simulation
    let energy: f32 = samples.iter().map(|s| s.abs()).sum::<f32>() / samples.len() as f32;
    
    // If audio energy is significant, simulate wake word detection
    if energy > 0.1 {
        // Simulated confidence based on energy
        let confidence = (energy * 5.0).min(0.9);
        
        if confidence > config.confidence_threshold {
            log::debug!("Whisper simulation mode: energy={}, confidence={}", energy, confidence);
            return Ok(Some((confidence, config.wake_word.clone())));
        }
    }
    
    Ok(None)
}

/// Check if Whisper is available
pub fn is_available() -> bool {
    #[cfg(feature = "whisper")]
    {
        true
    }
    #[cfg(not(feature = "whisper"))]
    {
        false
    }
}

/// Get Whisper status message
pub fn status() -> &'static str {
    #[cfg(feature = "whisper")]
    {
        "Whisper native library loaded"
    }
    #[cfg(not(feature = "whisper"))]
    {
        "Whisper simulation mode (enable 'whisper' feature for real detection)"
    }
}
