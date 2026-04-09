//! ─── Whisper Wake Word Engine ───
//!
//! OpenAI's Whisper model for wake word detection.
//! More accurate but slower than Vosk.

use crate::{WakeError, WakeWordConfig};

/// Detect wake word using Whisper
#[cfg(feature = "whisper")]
pub fn detect(samples: &[f32], config: &WakeWordConfig) -> Result<Option<(f32, String)>, WakeError> {
    use whisper_rs::{WhisperContext, WhisperContextParameters, FullParams, SamplingStrategy};
    use std::sync::OnceLock;
    
    static CONTEXT: OnceLock<WhisperContext> = OnceLock::new();
    
    // Initialize context
    let ctx = CONTEXT.get_or_init(|| {
        let model_path = config.whisper_model_path.as_deref()
            .unwrap_or("ggml-tiny.en.bin");
        
        WhisperContext::new_with_params(
            model_path,
            WhisperContextParameters::default()
        ).expect("Failed to load Whisper model")
    });
    
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

#[cfg(not(feature = "whisper"))]
pub fn detect(_samples: &[f32], _config: &WakeWordConfig) -> Result<Option<(f32, String)>, WakeError> {
    Err(WakeError::Internal("Whisper feature not enabled".into()))
}
