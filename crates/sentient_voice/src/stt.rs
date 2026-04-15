//! Speech-to-Text engine

use async_trait::async_trait;

use crate::{VoiceError, VoiceResult, VoiceProvider, SttSettings};

/// STT engine trait
#[async_trait]
pub trait SttEngine: Send + Sync {
    /// Transcribe audio
    async fn transcribe(&self, audio: &[u8]) -> VoiceResult<String>;
    
    /// Transcribe audio file
    async fn transcribe_file(&self, path: &str) -> VoiceResult<String>;
    
    /// Transcribe with timestamp
    async fn transcribe_with_timestamps(&self, audio: &[u8]) -> VoiceResult<Vec<TranscriptSegment>>;
}

/// Transcript segment with timing
#[derive(Debug, Clone)]
pub struct TranscriptSegment {
    pub text: String,
    pub start: f32,
    pub end: f32,
    pub confidence: f32,
}

/// STT configuration
#[derive(Debug, Clone)]
pub struct SttConfig {
    pub settings: SttSettings,
    pub api_key: Option<String>,
    pub base_url: Option<String>,
}

impl SttConfig {
    pub fn new(settings: SttSettings) -> Self {
        Self {
            settings,
            api_key: None,
            base_url: None,
        }
    }

    pub fn with_api_key(mut self, key: &str) -> Self {
        self.api_key = Some(key.to_string());
        self
    }
}

impl Default for SttConfig {
    fn default() -> Self {
        Self::new(SttSettings::default())
    }
}

/// STT Provider implementation
pub struct SttProvider {
    config: SttConfig,
    client: reqwest::Client,
}

impl SttProvider {
    pub fn new(config: SttConfig) -> Self {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(60))
            .build()
            .unwrap_or_default();

        Self { config, client }
    }

    /// OpenAI Whisper
    async fn transcribe_openai(&self, audio: &[u8]) -> VoiceResult<String> {
        let api_key = self.config.api_key.as_ref()
            .ok_or(VoiceError::AuthFailed)?;

        let url = "https://api.openai.com/v1/audio/transcriptions";

        // Create multipart form
        let audio_part = reqwest::multipart::Part::bytes(audio.to_vec())
            .file_name("audio.wav")
            .mime_str("audio/wav")
            .map_err(|e| VoiceError::ApiError(e.to_string()))?;

        let form = reqwest::multipart::Form::new()
            .part("file", audio_part)
            .text("model", self.config.settings.model.clone())
            .text("language", self.config.settings.language.clone());

        let response = self.client
            .post(url)
            .header("Authorization", format!("Bearer {}", api_key))
            .multipart(form)
            .send()
            .await
            .map_err(|e| VoiceError::ApiError(e.to_string()))?;

        if response.status().is_success() {
            let result: serde_json::Value = response
                .json()
                .await
                .map_err(|e| VoiceError::ApiError(e.to_string()))?;

            Ok(result["text"]
                .as_str()
                .unwrap_or("")
                .to_string())
        } else {
            let status = response.status();
            Err(VoiceError::ApiError(format!("Whisper error: {}", status)))
        }
    }

    /// Local Whisper (would use whisper.cpp or similar)
    async fn transcribe_local(&self, audio: &[u8]) -> VoiceResult<String> {
        // Would integrate with whisper.cpp here
        log::debug!("Local STT: {} bytes", audio.len());
        Err(VoiceError::SttError("Local Whisper not implemented".to_string()))
    }
}

#[async_trait]
impl SttEngine for SttProvider {
    async fn transcribe(&self, audio: &[u8]) -> VoiceResult<String> {
        match self.config.settings.provider {
            VoiceProvider::OpenAI => self.transcribe_openai(audio).await,
            VoiceProvider::WhisperLocal => self.transcribe_local(audio).await,
            _ => Err(VoiceError::SttError("Unsupported provider".to_string())),
        }
    }

    async fn transcribe_file(&self, path: &str) -> VoiceResult<String> {
        let audio = tokio::fs::read(path).await?;
        self.transcribe(&audio).await
    }

    async fn transcribe_with_timestamps(&self, audio: &[u8]) -> VoiceResult<Vec<TranscriptSegment>> {
        // Would need to call API with timestamp_granularities
        let text = self.transcribe(audio).await?;
        Ok(vec![TranscriptSegment {
            text,
            start: 0.0,
            end: 0.0,
            confidence: 1.0,
        }])
    }
}

impl Default for SttProvider {
    fn default() -> Self {
        Self::new(SttConfig::default())
    }
}
