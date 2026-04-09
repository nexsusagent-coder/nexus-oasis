//! ─── Voice Configuration ───

use serde::{Deserialize, Serialize};

/// Voice configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceConfig {
    /// Sample rate (default: 16000)
    pub sample_rate: u32,
    
    /// Channels (default: 1)
    pub channels: u16,
    
    /// STT provider
    pub stt_provider: SttProvider,
    
    /// TTS provider
    pub tts_provider: TtsProvider,
    
    /// Wake word configuration
    pub wake_word: Option<WakeWordConfig>,
    
    /// Voice activity detection
    pub vad_enabled: bool,
    
    /// VAD sensitivity (0.0 - 1.0)
    pub vad_sensitivity: f32,
    
    /// Silence timeout (ms)
    pub silence_timeout_ms: u32,
    
    /// Enable voice commands
    pub voice_commands: bool,
    
    /// Language for STT
    pub language: String,
}

impl Default for VoiceConfig {
    fn default() -> Self {
        Self {
            sample_rate: 16000,
            channels: 1,
            stt_provider: SttProvider::OpenAI {
                api_key: std::env::var("OPENAI_API_KEY").unwrap_or_default(),
            },
            tts_provider: TtsProvider::System,
            wake_word: None,
            vad_enabled: true,
            vad_sensitivity: 0.5,
            silence_timeout_ms: 1500,
            voice_commands: true,
            language: "en".into(),
        }
    }
}

/// Speech-to-Text provider
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SttProvider {
    /// OpenAI Whisper API
    OpenAI {
        api_key: String,
    },
    
    /// Local Whisper model
    Local {
        model_path: String,
    },
}

/// Text-to-Speech provider
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TtsProvider {
    /// OpenAI TTS API
    OpenAI {
        api_key: String,
        voice: String, // alloy, echo, fable, onyx, nova, shimmer
    },
    
    /// ElevenLabs API
    ElevenLabs {
        api_key: String,
        voice_id: String,
    },
    
    /// System TTS
    System,
}

/// Wake word configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WakeWordConfig {
    /// Wake word phrase
    pub phrase: String,
    
    /// Sensitivity (0.0 - 1.0)
    pub sensitivity: f32,
    
    /// Model path (for Porcupine/snowboy)
    pub model_path: Option<String>,
    
    /// Access key (for Porcupine)
    pub access_key: Option<String>,
}

impl WakeWordConfig {
    pub fn sentient() -> Self {
        Self {
            phrase: "hey sentient".into(),
            sensitivity: 0.5,
            model_path: None,
            access_key: None,
        }
    }
}
