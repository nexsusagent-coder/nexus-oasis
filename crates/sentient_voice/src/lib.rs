//! ═══════════════════════════════════════════════════════════════════════════════
//!  SENTIENT Voice - Speech-to-Text and Text-to-Speech
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//!  Features:
//!  - Whisper STT (OpenAI API or local)
//!  - Multiple TTS providers (OpenAI, ElevenLabs, System)
//!  - Real-time audio streaming with VAD
//!  - Voice activity detection
//!  - Wake word detection
//!  - Multi-language support
//!  - Voice cloning (ElevenLabs)

pub mod stt;
pub mod tts;
pub mod audio;
pub mod config;
pub mod wake;
pub mod streaming;
pub mod diarization;

use std::sync::Arc;
use tokio::sync::{RwLock, mpsc};
pub use parking_lot::Mutex;

pub use config::VoiceConfig;
pub use stt::{SpeechToText, TranscriptionResult};
pub use tts::{TextToSpeech, SpeechResult};
pub use audio::{AudioBuffer, AudioFormat, VoiceActivityDetector};
pub use wake::{WakeWordDetector, WakeWord};
pub use streaming::{VoiceStream, StreamConfig, StreamEvent};

/// ─── Voice Engine ───

pub struct VoiceEngine {
    config: VoiceConfig,
    stt: Arc<RwLock<Box<dyn SpeechToText>>>,
    tts: Arc<RwLock<Box<dyn TextToSpeech>>>,
    wake_detector: Option<Arc<RwLock<WakeWordDetector>>>,
    vad: Arc<Mutex<VoiceActivityDetector>>,
}

impl VoiceEngine {
    /// Create new voice engine
    pub fn new(config: VoiceConfig) -> Self {
        let stt = create_stt(&config);
        let tts = create_tts(&config);
        let wake_detector = config.wake_word.as_ref().map(|w| {
            Arc::new(RwLock::new(WakeWordDetector::new(w.clone())))
        });
        let vad = Arc::new(Mutex::new(VoiceActivityDetector::new(
            config.vad_sensitivity,
            (config.sample_rate / 100) as usize, // 10ms frames
        )));
        
        Self {
            config,
            stt: Arc::new(RwLock::new(stt)),
            tts: Arc::new(RwLock::new(tts)),
            wake_detector,
            vad,
        }
    }
    
    /// Transcribe audio
    pub async fn transcribe(&self, audio: &[f32]) -> Result<TranscriptionResult, VoiceError> {
        let stt = self.stt.read().await;
        stt.transcribe(audio).await
    }
    
    /// Transcribe audio file
    pub async fn transcribe_file(&self, path: &str) -> Result<TranscriptionResult, VoiceError> {
        let stt = self.stt.read().await;
        stt.transcribe_file(path).await
    }
    
    /// Synthesize speech
    pub async fn synthesize(&self, text: &str) -> Result<SpeechResult, VoiceError> {
        let tts = self.tts.read().await;
        tts.synthesize(text).await
    }
    
    /// Synthesize with custom voice (voice cloning)
    pub async fn synthesize_with_voice(&self, text: &str, voice_id: &str) -> Result<SpeechResult, VoiceError> {
        let mut tts = self.tts.write().await;
        tts.set_voice(voice_id);
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
    
    /// Check voice activity
    pub fn detect_voice_activity(&self, frame: &[f32]) -> bool {
        let mut vad = self.vad.lock();
        vad.process(frame)
    }
    
    /// Reset VAD state
    pub fn reset_vad(&self) {
        let mut vad = self.vad.lock();
        vad.reset();
    }
    
    /// Create real-time streaming transcription session
    pub async fn create_stream(&self, config: StreamConfig) -> Result<VoiceStream, VoiceError> {
        VoiceStream::new(
            self.stt.clone(),
            self.vad.clone(),
            config,
            self.config.clone(),
        )
    }
    
    /// Get available TTS voices
    pub async fn get_voices(&self) -> Vec<String> {
        let tts = self.tts.read().await;
        tts.voices()
    }
    
    /// Get configuration
    pub fn config(&self) -> &VoiceConfig {
        &self.config
    }
}

/// Create STT engine
fn create_stt(config: &VoiceConfig) -> Box<dyn SpeechToText> {
    match &config.stt_provider {
        config::SttProvider::OpenAI { api_key } => {
            Box::new(stt::OpenAiWhisper::new(api_key.clone()))
        }
        config::SttProvider::Local { model_path: _ } => {
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
/// Implemented in streaming.rs
// pub use streaming::VoiceStream;

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
