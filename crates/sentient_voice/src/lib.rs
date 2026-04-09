//! ═══════════════════════════════════════════════════════════════════════════════
//!  SENTIENT Voice - Speech-to-Text and Text-to-Speech
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//!  Features:
//!  - Whisper STT (OpenAI API or local)
//!  - Multiple TTS providers (OpenAI, ElevenLabs, System)
//!  - Real-time audio streaming
//!  - Voice activity detection
//!  - Wake word detection
//!  - Multi-language support

pub mod stt;
pub mod tts;
pub mod audio;
pub mod config;
pub mod wake;

use serde::{Deserialize, Serialize};
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::RwLock;

pub use config::VoiceConfig;
pub use stt::{SpeechToText, TranscriptionResult};
pub use tts::{TextToSpeech, SpeechResult};
pub use audio::{AudioBuffer, AudioFormat};
pub use wake::{WakeWordDetector, WakeWord};

/// ─── Voice Engine ───

pub struct VoiceEngine {
    config: VoiceConfig,
    stt: Arc<RwLock<Box<dyn SpeechToText>>>,
    tts: Arc<RwLock<Box<dyn TextToSpeech>>>,
    wake_detector: Option<Arc<RwLock<WakeWordDetector>>>,
}

impl VoiceEngine {
    /// Create new voice engine
    pub fn new(config: VoiceConfig) -> Self {
        let stt = create_stt(&config);
        let tts = create_tts(&config);
        let wake_detector = config.wake_word.as_ref().map(|w| {
            Arc::new(RwLock::new(WakeWordDetector::new(w.clone())))
        });
        
        Self {
            config,
            stt: Arc::new(RwLock::new(stt)),
            tts: Arc::new(RwLock::new(tts)),
            wake_detector,
        }
    }
    
    /// Transcribe audio
    pub async fn transcribe(&self, audio: &[f32]) -> Result<TranscriptionResult, VoiceError> {
        let stt = self.stt.read().await;
        stt.transcribe(audio).await
    }
    
    /// Synthesize speech
    pub async fn synthesize(&self, text: &str) -> Result<SpeechResult, VoiceError> {
        let tts = self.tts.read().await;
        tts.synthesize(text).await
    }
    
    /// Check for wake word
    pub async fn check_wake_word(&self, audio: &[f32]) -> Option<WakeWord> {
        if let Some(detector) = &self.wake_detector {
            let det = detector.read().await;
            det.detect(audio).await
        } else {
            None
        }
    }
    
    /// Stream audio for real-time transcription
    pub async fn stream_transcribe(&self) -> Result<VoiceStream, VoiceError> {
        // TODO: Implement streaming
        Err(VoiceError::NotImplemented("Streaming not yet implemented".into()))
    }
}

/// Create STT engine
fn create_stt(config: &VoiceConfig) -> Box<dyn SpeechToText> {
    match &config.stt_provider {
        config::SttProvider::OpenAI { api_key } => {
            Box::new(stt::OpenAiWhisper::new(api_key.clone()))
        }
        config::SttProvider::Local { model_path } => {
            #[cfg(feature = "local-whisper")]
            {
                Box::new(stt::LocalWhisper::new(model_path.clone()))
            }
            #[cfg(not(feature = "local-whisper"))]
            {
                log::warn!("Local Whisper not compiled in, falling back to API");
                Box::new(stt::OpenAiWhisper::new(std::env::var("OPENAI_API_KEY").unwrap_or_default()))
            }
        }
    }
}

/// Create TTS engine
fn create_tts(config: &VoiceConfig) -> Box<dyn TextToSpeech> {
    match &config.tts_provider {
        config::TtsProvider::OpenAI { api_key, voice } => {
            Box::new(tts::OpenAiTts::new(api_key.clone(), voice.clone()))
        }
        config::TtsProvider::ElevenLabs { api_key, voice_id } => {
            Box::new(tts::ElevenLabsTts::new(api_key.clone(), voice_id.clone()))
        }
        config::TtsProvider::System => {
            Box::new(tts::SystemTts::new())
        }
    }
}

/// Voice stream for real-time processing
pub struct VoiceStream {
    // TODO: Implement
}

/// ─── Errors ───

#[derive(Debug, thiserror::Error)]
pub enum VoiceError {
    #[error("API error: {0}")]
    ApiError(String),
    
    #[error("Audio error: {0}")]
    AudioError(String),
    
    #[error("Network error: {0}")]
    NetworkError(String),
    
    #[error("Not implemented: {0}")]
    NotImplemented(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    
    #[error("Internal error: {0}")]
    Internal(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_voice_config() {
        let config = VoiceConfig::default();
        assert!(config.sample_rate > 0);
    }
}
