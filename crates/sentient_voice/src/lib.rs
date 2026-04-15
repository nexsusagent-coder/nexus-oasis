//! ═══════════════════════════════════════════════════════════════════════════════
//!  SENTIENT Voice - Voice Assistant Module
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//!  Sprint 5: Personal AI - Sesli asistan desteği
//!  
//!  Özellikler:
//!  - TTS (Text-to-Speech) - OpenAI, ElevenLabs, Piper
//!  - STT (Speech-to-Text) - OpenAI Whisper, Local Whisper
//!  - Wake Word Detection - "Hey SENTIENT", custom wake words
//!  - Audio capture and playback
//!  - VAD (Voice Activity Detection)

use serde::{Deserialize, Serialize};

pub mod error;
pub mod types;
pub mod tts;
pub mod stt;
pub mod wake;
pub mod audio;
pub mod assistant;
pub mod vad;

pub use error::{VoiceError, VoiceResult};
pub use types::{VoiceConfig, AudioFormat, VoiceProvider, VoiceInfo, VoiceGender, TtsSettings, SttSettings, WakeWordSettings, AudioSettings};
pub use tts::{TtsEngine, TtsConfig, TtsProvider};
pub use stt::{SttEngine, SttConfig, SttProvider, TranscriptSegment};
pub use wake::{WakeWordDetector, WakeWordConfig, WakeWord as WakeWordStruct};
pub use audio::{AudioCapture, AudioPlayback, AudioConfig, AudioDevice};
pub use assistant::{VoiceAssistant, AssistantState, SynthesisResult};
pub use vad::{VoiceActivityDetector, VadConfig, VadResult};

// Type aliases for compatibility
pub type WakeWord = WakeWordStruct;
pub type SpeechResult = TranscriptionResult;

/// StreamConfig alias for compatibility
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamConfig {
    pub sample_rate: u32,
    pub frame_size: u32,
    pub silence_timeout_ms: u32,
    pub min_audio_ms: u32,
    pub vad_enabled: bool,
    pub vad_sensitivity: f32,
}

impl Default for StreamConfig {
    fn default() -> Self {
        Self {
            sample_rate: 16000,
            frame_size: 480,
            silence_timeout_ms: 1500,
            min_audio_ms: 500,
            vad_enabled: true,
            vad_sensitivity: 0.5,
        }
    }
}

// Gateway compatibility types
pub use assistant::VoiceAssistant as VoiceEngine;

/// Stream event for gateway compatibility
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StreamEvent {
    Transcription { text: String, is_final: bool },
    Response { text: String },
    Audio { data: Vec<u8> },
    Error { message: String },
    End,
}

/// Transcription result for gateway compatibility
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscriptionResult {
    pub text: String,
    pub language: String,
    pub confidence: f32,
    pub duration_secs: f32,
}

impl Default for TranscriptionResult {
    fn default() -> Self {
        Self {
            text: String::new(),
            language: "tr".to_string(),
            confidence: 0.0,
            duration_secs: 0.0,
        }
    }
}
