//! Voice error types

use thiserror::Error;

#[derive(Debug, Error)]
pub enum VoiceError {
    #[error("TTS error: {0}")]
    TtsError(String),

    #[error("STT error: {0}")]
    SttError(String),

    #[error("Wake word error: {0}")]
    WakeWordError(String),

    #[error("Audio capture error: {0}")]
    CaptureError(String),

    #[error("Audio playback error: {0}")]
    PlaybackError(String),

    #[error("No audio input device found")]
    NoInputDevice,

    #[error("No audio output device found")]
    NoOutputDevice,

    #[error("Voice activity detection error: {0}")]
    VadError(String),

    #[error("API error: {0}")]
    ApiError(String),

    #[error("Authentication failed")]
    AuthFailed,

    #[error("Timeout")]
    Timeout,

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),

    // Additional variants for compatibility
    #[error("Internal error: {0}")]
    Internal(String),

    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("Not implemented: {0}")]
    NotImplemented(String),

    #[error("Audio error: {0}")]
    AudioError(String),
}

pub type VoiceResult<T> = Result<T, VoiceError>;
