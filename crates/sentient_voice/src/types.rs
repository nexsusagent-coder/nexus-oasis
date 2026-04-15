//! Voice types and configuration

use serde::{Deserialize, Serialize};

/// Voice configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceConfig {
    /// TTS configuration
    pub tts: TtsSettings,
    /// STT configuration
    pub stt: SttSettings,
    /// Wake word configuration
    pub wake_word: WakeWordSettings,
    /// Audio configuration
    pub audio: AudioSettings,
    /// Enable voice assistant
    pub enabled: bool,
}

impl Default for VoiceConfig {
    fn default() -> Self {
        Self {
            tts: TtsSettings::default(),
            stt: SttSettings::default(),
            wake_word: WakeWordSettings::default(),
            audio: AudioSettings::default(),
            enabled: true,
        }
    }
}

/// TTS settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TtsSettings {
    /// Provider
    pub provider: VoiceProvider,
    /// Voice ID
    pub voice_id: String,
    /// Sample rate
    pub sample_rate: u32,
    /// Speed (0.5 - 2.0)
    pub speed: f32,
    /// Language
    pub language: String,
}

impl Default for TtsSettings {
    fn default() -> Self {
        Self {
            provider: VoiceProvider::OpenAI,
            voice_id: "alloy".to_string(),
            sample_rate: 24000,
            speed: 1.0,
            language: "tr".to_string(),
        }
    }
}

/// STT settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SttSettings {
    /// Provider
    pub provider: VoiceProvider,
    /// Language
    pub language: String,
    /// Model
    pub model: String,
    /// Enable continuous listening
    pub continuous: bool,
}

impl Default for SttSettings {
    fn default() -> Self {
        Self {
            provider: VoiceProvider::OpenAI,
            language: "tr".to_string(),
            model: "whisper-1".to_string(),
            continuous: false,
        }
    }
}

/// Wake word settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WakeWordSettings {
    /// Wake word phrase
    pub phrase: String,
    /// Sensitivity (0.0 - 1.0)
    pub sensitivity: f32,
    /// Time to wait after wake word (seconds)
    pub listen_timeout: f32,
}

impl Default for WakeWordSettings {
    fn default() -> Self {
        Self {
            phrase: "hey sentient".to_string(),
            sensitivity: 0.5,
            listen_timeout: 5.0,
        }
    }
}

/// Audio settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioSettings {
    /// Input sample rate
    pub input_sample_rate: u32,
    /// Output sample rate
    pub output_sample_rate: u32,
    /// Channels
    pub channels: u16,
    /// Buffer size
    pub buffer_size: u32,
    /// Input device (None = default)
    pub input_device: Option<String>,
    /// Output device (None = default)
    pub output_device: Option<String>,
    // Legacy fields for compatibility
    /// Sample rate (alias for input_sample_rate)
    pub sample_rate: u32,
    /// Frame size
    pub frame_size: u32,
    /// Silence timeout (ms)
    pub silence_timeout_ms: u32,
    /// Min audio (ms)
    pub min_audio_ms: u32,
    /// VAD enabled
    pub vad_enabled: bool,
    /// VAD sensitivity
    pub vad_sensitivity: f32,
}

impl Default for AudioSettings {
    fn default() -> Self {
        Self {
            input_sample_rate: 16000,
            output_sample_rate: 24000,
            channels: 1,
            buffer_size: 1024,
            input_device: None,
            output_device: None,
            sample_rate: 16000,
            frame_size: 480,
            silence_timeout_ms: 1500,
            min_audio_ms: 500,
            vad_enabled: true,
            vad_sensitivity: 0.5,
        }
    }
}

/// Voice provider
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum VoiceProvider {
    OpenAI,
    ElevenLabs,
    Piper,
    WhisperLocal,
    Custom(String),
}

impl std::fmt::Display for VoiceProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VoiceProvider::OpenAI => write!(f, "openai"),
            VoiceProvider::ElevenLabs => write!(f, "elevenlabs"),
            VoiceProvider::Piper => write!(f, "piper"),
            VoiceProvider::WhisperLocal => write!(f, "whisper-local"),
            VoiceProvider::Custom(s) => write!(f, "{}", s),
        }
    }
}

/// Audio format
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AudioFormat {
    Wav,
    Mp3,
    Opus,
    Pcm,
    Raw,
}

impl std::fmt::Display for AudioFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AudioFormat::Wav => write!(f, "wav"),
            AudioFormat::Mp3 => write!(f, "mp3"),
            AudioFormat::Opus => write!(f, "opus"),
            AudioFormat::Pcm => write!(f, "pcm"),
            AudioFormat::Raw => write!(f, "raw"),
        }
    }
}

/// Voice gender
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum VoiceGender {
    Male,
    Female,
    Neutral,
}

/// Voice info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceInfo {
    pub id: String,
    pub name: String,
    pub language: String,
    pub gender: VoiceGender,
    pub provider: VoiceProvider,
}
