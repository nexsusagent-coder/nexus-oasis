//! Text-to-Speech engine

use async_trait::async_trait;

use crate::{AudioFormat, VoiceError, VoiceResult, VoiceProvider, TtsSettings, VoiceInfo};

/// TTS engine trait
#[async_trait]
pub trait TtsEngine: Send + Sync {
    /// Synthesize speech
    async fn synthesize(&self, text: &str) -> VoiceResult<Vec<u8>>;
    
    /// Synthesize to file
    async fn synthesize_to_file(&self, text: &str, path: &str) -> VoiceResult<()>;
    
    /// List available voices
    async fn list_voices(&self) -> VoiceResult<Vec<VoiceInfo>>;
    
    /// Get current voice
    fn current_voice(&self) -> &str;
}

/// TTS configuration
#[derive(Debug, Clone)]
pub struct TtsConfig {
    pub settings: TtsSettings,
    pub api_key: Option<String>,
    pub base_url: Option<String>,
}

impl TtsConfig {
    pub fn new(settings: TtsSettings) -> Self {
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

impl Default for TtsConfig {
    fn default() -> Self {
        Self::new(TtsSettings::default())
    }
}

/// TTS Provider implementation
pub struct TtsProvider {
    config: TtsConfig,
    client: reqwest::Client,
}

impl TtsProvider {
    pub fn new(config: TtsConfig) -> Self {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .unwrap_or_default();

        Self { config, client }
    }

    /// OpenAI TTS
    async fn synthesize_openai(&self, text: &str) -> VoiceResult<Vec<u8>> {
        let api_key = self.config.api_key.as_ref()
            .ok_or(VoiceError::AuthFailed)?;

        let url = "https://api.openai.com/v1/audio/speech";
        
        let body = serde_json::json!({
            "model": "tts-1",
            "input": text,
            "voice": self.config.settings.voice_id,
            "response_format": "mp3",
            "speed": self.config.settings.speed
        });

        let response = self.client
            .post(url)
            .header("Authorization", format!("Bearer {}", api_key))
            .json(&body)
            .send()
            .await
            .map_err(|e| VoiceError::ApiError(e.to_string()))?;

        if response.status().is_success() {
            response.bytes()
                .await
                .map(|b| b.to_vec())
                .map_err(|e| VoiceError::ApiError(e.to_string()))
        } else {
            let status = response.status();
            Err(VoiceError::ApiError(format!("OpenAI TTS error: {}", status)))
        }
    }

    /// ElevenLabs TTS
    async fn synthesize_elevenlabs(&self, text: &str) -> VoiceResult<Vec<u8>> {
        let api_key = self.config.api_key.as_ref()
            .ok_or(VoiceError::AuthFailed)?;

        let voice_id = &self.config.settings.voice_id;
        let url = format!("https://api.elevenlabs.io/v1/text-to-speech/{}", voice_id);
        
        let body = serde_json::json!({
            "text": text,
            "model_id": "eleven_multilingual_v2",
            "voice_settings": {
                "stability": 0.5,
                "similarity_boost": 0.75
            }
        });

        let response = self.client
            .post(&url)
            .header("xi-api-key", api_key)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| VoiceError::ApiError(e.to_string()))?;

        if response.status().is_success() {
            response.bytes()
                .await
                .map(|b| b.to_vec())
                .map_err(|e| VoiceError::ApiError(e.to_string()))
        } else {
            let status = response.status();
            Err(VoiceError::ApiError(format!("ElevenLabs error: {}", status)))
        }
    }
}

#[async_trait]
impl TtsEngine for TtsProvider {
    async fn synthesize(&self, text: &str) -> VoiceResult<Vec<u8>> {
        match self.config.settings.provider {
            VoiceProvider::OpenAI => self.synthesize_openai(text).await,
            VoiceProvider::ElevenLabs => self.synthesize_elevenlabs(text).await,
            VoiceProvider::Piper => {
                // Local Piper TTS would go here
                Err(VoiceError::TtsError("Piper not implemented".to_string()))
            }
            _ => Err(VoiceError::TtsError("Unsupported provider".to_string())),
        }
    }

    async fn synthesize_to_file(&self, text: &str, path: &str) -> VoiceResult<()> {
        let bytes = self.synthesize(text).await?;
        tokio::fs::write(path, bytes).await?;
        Ok(())
    }

    async fn list_voices(&self) -> VoiceResult<Vec<VoiceInfo>> {
        match self.config.settings.provider {
            VoiceProvider::OpenAI => {
                Ok(vec![
                    VoiceInfo {
                        id: "alloy".to_string(),
                        name: "Alloy".to_string(),
                        language: "en".to_string(),
                        gender: crate::VoiceGender::Neutral,
                        provider: VoiceProvider::OpenAI,
                    },
                    VoiceInfo {
                        id: "echo".to_string(),
                        name: "Echo".to_string(),
                        language: "en".to_string(),
                        gender: crate::VoiceGender::Male,
                        provider: VoiceProvider::OpenAI,
                    },
                    VoiceInfo {
                        id: "fable".to_string(),
                        name: "Fable".to_string(),
                        language: "en".to_string(),
                        gender: crate::VoiceGender::Neutral,
                        provider: VoiceProvider::OpenAI,
                    },
                    VoiceInfo {
                        id: "onyx".to_string(),
                        name: "Onyx".to_string(),
                        language: "en".to_string(),
                        gender: crate::VoiceGender::Male,
                        provider: VoiceProvider::OpenAI,
                    },
                    VoiceInfo {
                        id: "nova".to_string(),
                        name: "Nova".to_string(),
                        language: "en".to_string(),
                        gender: crate::VoiceGender::Female,
                        provider: VoiceProvider::OpenAI,
                    },
                    VoiceInfo {
                        id: "shimmer".to_string(),
                        name: "Shimmer".to_string(),
                        language: "en".to_string(),
                        gender: crate::VoiceGender::Female,
                        provider: VoiceProvider::OpenAI,
                    },
                ])
            }
            _ => Ok(Vec::new()),
        }
    }

    fn current_voice(&self) -> &str {
        &self.config.settings.voice_id
    }
}

impl Default for TtsProvider {
    fn default() -> Self {
        Self::new(TtsConfig::default())
    }
}
