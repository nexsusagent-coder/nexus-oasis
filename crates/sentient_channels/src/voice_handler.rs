//! ═══════════════════════════════════════════════════════════════════════════════
//!  SENTIENT Channels Voice Handler - Telegram/Discord Voice Messages
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//!  Telegram ve Discord'dan gelen sesli mesajları işler:
//!  - Voice message download
//!  - STT transcription
//!  - LLM processing
//!  - TTS response generation
//!  - Voice message reply
//!
//!  Akış:
//!  [Telegram Voice] -> [Download] -> [STT] -> [LLM] -> [TTS] -> [Voice Reply]

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

use sentient_voice::{
    VoiceEngine, VoiceConfig, VoiceError,
    TranscriptionResult, SpeechResult,
};

// ─────────────────────────────────────────────────────────────────────────────
// VOICE MESSAGE TYPES
// ─────────────────────────────────────────────────────────────────────────────

/// Gelen sesli mesaj
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceMessage {
    /// Mesaj ID
    pub message_id: String,
    
    /// Kanal (telegram, discord, etc.)
    pub channel: VoiceChannel,
    
    /// Kullanıcı ID
    pub user_id: String,
    
    /// Kullanıcı adı
    pub username: Option<String>,
    
    /// Chat/Channel ID
    pub chat_id: String,
    
    /// Ses dosyası URL veya path
    pub file_url: Option<String>,
    
    /// Dosya ID (platform-specific)
    pub file_id: String,
    
    /// Süre (saniye)
    pub duration_secs: Option<u32>,
    
    /// MIME type
    pub mime_type: Option<String>,
    
    /// Zaman damgası
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Voice channel türleri
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum VoiceChannel {
    Telegram,
    Discord,
    Slack,
    WhatsApp,
    Signal,
    Web,
    Desktop,
}

/// Voice yanıt
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceResponse {
    /// Orijinal mesaj ID
    pub reply_to: String,
    
    /// Transkripsiyon
    pub transcript: String,
    
    /// LLM yanıtı metni
    pub response_text: String,
    
    /// TTS ses verisi
    pub audio_data: Option<Vec<u8>>,
    
    /// Ses formatı
    pub audio_format: String,
    
    /// Dil
    pub language: String,
}

/// Voice handler konfigürasyonu
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceHandlerConfig {
    /// Maksimum ses süresi (saniye)
    pub max_duration_secs: u32,
    
    /// Desteklenen formatlar
    pub supported_formats: Vec<String>,
    
    /// Otomatik yanıt gönderilsin mi?
    pub auto_reply: bool,
    
    /// Sesli yanıt gönderilsin mi?
    pub voice_reply: bool,
    
    /// Dil
    pub default_language: String,
    
    /// TTS sesi
    pub tts_voice: Option<String>,
}

impl Default for VoiceHandlerConfig {
    fn default() -> Self {
        Self {
            max_duration_secs: 120, // 2 dakika
            supported_formats: vec!["ogg".into(), "mp3".into(), "wav".into(), "m4a".into()],
            auto_reply: true,
            voice_reply: true,
            default_language: "tr".into(),
            tts_voice: None,
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// VOICE HANDLER TRAIT
// ─────────────────────────────────────────────────────────────────────────────

/// Voice handler trait - her platform için ayrı implementasyon
#[async_trait]
pub trait VoiceHandler: Send + Sync {
    /// Sesi indir
    async fn download_voice(&self, message: &VoiceMessage) -> Result<Vec<u8>, VoiceError>;
    
    /// Sesli mesajı işle
    async fn process_voice(&self, message: VoiceMessage) -> Result<VoiceResponse, VoiceError>;
    
    /// Yanıtı gönder
    async fn send_response(&self, response: VoiceResponse, chat_id: &str) -> Result<(), VoiceError>;
    
    /// Sesli yanıt gönder
    async fn send_voice_response(
        &self,
        audio_data: Vec<u8>,
        chat_id: &str,
        reply_to: &str,
    ) -> Result<(), VoiceError>;
}

// ─────────────────────────────────────────────────────────────────────────────
// TELEGRAM VOICE HANDLER
// ─────────────────────────────────────────────────────────────────────────────

/// Telegram voice handler
pub struct TelegramVoiceHandler {
    voice_engine: Arc<VoiceEngine>,
    config: VoiceHandlerConfig,
    bot_token: String,
    http_client: reqwest::Client,
}

impl TelegramVoiceHandler {
    pub fn new(bot_token: String, voice_engine: Arc<VoiceEngine>) -> Self {
        Self {
            voice_engine,
            config: VoiceHandlerConfig::default(),
            bot_token,
            http_client: reqwest::Client::new(),
        }
    }
    
    pub fn with_config(mut self, config: VoiceHandlerConfig) -> Self {
        self.config = config;
        self
    }
    
    /// Telegram'dan dosya indir
    async fn download_telegram_file(&self, file_id: &str) -> Result<Vec<u8>, VoiceError> {
        // Önce file path al
        let file_info_url = format!(
            "https://api.telegram.org/bot{}/getFile?file_id={}",
            self.bot_token, file_id
        );
        
        let response = self.http_client
            .get(&file_info_url)
            .send()
            .await
            .map_err(|e| VoiceError::NetworkError(e.to_string()))?;
        
        let file_info: TelegramFileResponse = response.json().await
            .map_err(|e| VoiceError::ApiError(e.to_string()))?;
        
        let file_path = file_info.result.file_path
            .ok_or_else(|| VoiceError::ApiError("File path not found".into()))?;
        
        // Dosyayı indir
        let download_url = format!(
            "https://api.telegram.org/file/bot{}/{}",
            self.bot_token, file_path
        );
        
        let response = self.http_client
            .get(&download_url)
            .send()
            .await
            .map_err(|e| VoiceError::NetworkError(e.to_string()))?;
        
        let bytes = response.bytes().await
            .map_err(|e| VoiceError::NetworkError(e.to_string()))?;
        
        Ok(bytes.to_vec())
    }
    
    /// OGG sesi WAV'a dönüştür (basit implementasyon)
    fn convert_ogg_to_wav(&self, ogg_data: &[u8]) -> Result<Vec<f32>, VoiceError> {
        // Gerçek implementasyonda ffmpeg veya symphonia kullanılacak
        // Şimdilik mock
        log::debug!("Converting OGG to WAV ({} bytes)", ogg_data.len());
        
        // Simulated audio samples
        let sample_count = ogg_data.len() / 2;
        let mut samples = Vec::with_capacity(sample_count);
        
        for i in (0..ogg_data.len().saturating_sub(2)).step_by(2) {
            let sample = i16::from_le_bytes([ogg_data[i], ogg_data[i + 1]]);
            samples.push(sample as f32 / 32768.0);
        }
        
        Ok(samples)
    }
    
    /// Mesaj gönder
    async fn send_message(&self, chat_id: &str, text: &str, reply_to: Option<&str>) -> Result<(), VoiceError> {
        let mut url = format!(
            "https://api.telegram.org/bot{}/sendMessage?chat_id={}&text={}",
            self.bot_token, chat_id, urlencoding::encode(text)
        );
        
        if let Some(reply_to_id) = reply_to {
            url.push_str(&format!("&reply_to_message_id={}", reply_to_id));
        }
        
        let _ = self.http_client.get(&url).send().await
            .map_err(|e| VoiceError::NetworkError(e.to_string()))?;
        
        Ok(())
    }
    
    /// Sesli mesaj gönder
    async fn send_voice(
        &self,
        chat_id: &str,
        audio_data: Vec<u8>,
        reply_to: Option<&str>,
    ) -> Result<(), VoiceError> {
        let url = format!(
            "https://api.telegram.org/bot{}/sendVoice",
            self.bot_token
        );
        
        let part = reqwest::multipart::Part::bytes(audio_data)
            .file_name("voice.ogg")
            .mime_str("audio/ogg")
            .map_err(|e| VoiceError::Internal(e.to_string()))?;
        
        let mut form = reqwest::multipart::Form::new()
            .part("voice", part)
            .text("chat_id", chat_id.to_string());
        
        if let Some(reply_to_id) = reply_to {
            form = form.text("reply_to_message_id", reply_to_id.to_string());
        }
        
        let _ = self.http_client.post(&url).multipart(form).send().await
            .map_err(|e| VoiceError::NetworkError(e.to_string()))?;
        
        Ok(())
    }
}

#[async_trait]
impl VoiceHandler for TelegramVoiceHandler {
    async fn download_voice(&self, message: &VoiceMessage) -> Result<Vec<u8>, VoiceError> {
        log::info!("🎙️ Telegram voice indiriliyor: {}", message.file_id);
        self.download_telegram_file(&message.file_id).await
    }
    
    async fn process_voice(&self, message: VoiceMessage) -> Result<VoiceResponse, VoiceError> {
        log::info!("🎙️ Telegram voice işleniyor: {}", message.message_id);
        
        // Süre kontrolü
        if let Some(duration) = message.duration_secs {
            if duration > self.config.max_duration_secs {
                return Err(VoiceError::AudioError(
                    format!("Ses çok uzun: {} saniye (max: {})", duration, self.config.max_duration_secs)
                ));
            }
        }
        
        // Sesi indir
        let ogg_data = self.download_voice(&message).await?;
        
        // WAV'a dönüştür
        let audio_samples = self.convert_ogg_to_wav(&ogg_data)?;
        
        // Transcribe et
        let transcription = self.voice_engine.transcribe(&audio_samples).await?;
        
        log::info!("🎙️ Transkripsiyon: {}", transcription.text);
        
        // LLM yanıtı (simulated)
        let response_text = format!("Anlaşıldı: {}", transcription.text);
        
        // TTS
        let audio_response = if self.config.voice_reply {
            let speech = self.voice_engine.synthesize(&response_text).await?;
            // Convert f32 audio to bytes
            let audio_bytes: Vec<u8> = speech.audio.iter()
                .flat_map(|s| s.to_le_bytes())
                .collect();
            Some(audio_bytes)
        } else {
            None
        };
        
        Ok(VoiceResponse {
            reply_to: message.message_id,
            transcript: transcription.text,
            response_text,
            audio_data: audio_response,
            audio_format: "wav".into(),
            language: transcription.language,
        })
    }
    
    async fn send_response(&self, response: VoiceResponse, chat_id: &str) -> Result<(), VoiceError> {
        if let Some(audio) = response.audio_data.clone() {
            self.send_voice(chat_id, audio, Some(&response.reply_to)).await?;
        } else {
            self.send_message(chat_id, &response.response_text, Some(&response.reply_to)).await?;
        }
        Ok(())
    }
    
    async fn send_voice_response(
        &self,
        audio_data: Vec<u8>,
        chat_id: &str,
        reply_to: &str,
    ) -> Result<(), VoiceError> {
        self.send_voice(chat_id, audio_data, Some(reply_to)).await
    }
}

// Telegram API response
#[derive(Debug, Deserialize)]
struct TelegramFileResponse {
    result: TelegramFile,
}

#[derive(Debug, Deserialize)]
struct TelegramFile {
    file_path: Option<String>,
}

// ─────────────────────────────────────────────────────────────────────────────
// DISCORD VOICE HANDLER (STUB)
// ─────────────────────────────────────────────────────────────────────────────

/// Discord voice handler
pub struct DiscordVoiceHandler {
    voice_engine: Arc<VoiceEngine>,
    config: VoiceHandlerConfig,
}

impl DiscordVoiceHandler {
    pub fn new(voice_engine: Arc<VoiceEngine>) -> Self {
        Self {
            voice_engine,
            config: VoiceHandlerConfig::default(),
        }
    }
}

#[async_trait]
impl VoiceHandler for DiscordVoiceHandler {
    async fn download_voice(&self, message: &VoiceMessage) -> Result<Vec<u8>, VoiceError> {
        // Discord'dan ses indirme implementasyonu
        log::info!("🎙️ Discord voice indiriliyor: {}", message.file_id);
        Err(VoiceError::NotImplemented("Discord voice download".into()))
    }
    
    async fn process_voice(&self, message: VoiceMessage) -> Result<VoiceResponse, VoiceError> {
        log::info!("🎙️ Discord voice işleniyor: {}", message.message_id);
        Err(VoiceError::NotImplemented("Discord voice processing".into()))
    }
    
    async fn send_response(&self, response: VoiceResponse, chat_id: &str) -> Result<(), VoiceError> {
        log::info!("🎙️ Discord yanıt gönderiliyor: {}", chat_id);
        Err(VoiceError::NotImplemented("Discord voice response".into()))
    }
    
    async fn send_voice_response(
        &self,
        audio_data: Vec<u8>,
        chat_id: &str,
        reply_to: &str,
    ) -> Result<(), VoiceError> {
        Err(VoiceError::NotImplemented("Discord voice send".into()))
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// VOICE HANDLER MANAGER
// ─────────────────────────────────────────────────────────────────────────────

/// Tüm platformlar için voice handler yöneticisi
pub struct VoiceHandlerManager {
    telegram: Option<Arc<TelegramVoiceHandler>>,
    discord: Option<Arc<DiscordVoiceHandler>>,
}

impl VoiceHandlerManager {
    pub fn new() -> Self {
        Self {
            telegram: None,
            discord: None,
        }
    }
    
    pub fn with_telegram(mut self, handler: TelegramVoiceHandler) -> Self {
        self.telegram = Some(Arc::new(handler));
        self
    }
    
    pub fn with_discord(mut self, handler: DiscordVoiceHandler) -> Self {
        self.discord = Some(Arc::new(handler));
        self
    }
    
    /// Platform'a göre handler al
    pub fn get_handler(&self, channel: &VoiceChannel) -> Option<Arc<dyn VoiceHandler>> {
        match channel {
            VoiceChannel::Telegram => self.telegram.as_ref().map(|h| h.clone() as Arc<dyn VoiceHandler>),
            VoiceChannel::Discord => self.discord.as_ref().map(|h| h.clone() as Arc<dyn VoiceHandler>),
            _ => None,
        }
    }
    
    /// Mesajı işle
    pub async fn process(&self, message: VoiceMessage) -> Result<VoiceResponse, VoiceError> {
        if let Some(handler) = self.get_handler(&message.channel) {
            let response = handler.process_voice(message.clone()).await?;
            
            // Otomatik yanıt
            if handler.get_auto_reply() {
                handler.send_response(response.clone(), &message.chat_id).await?;
            }
            
            Ok(response)
        } else {
            Err(VoiceError::NotImplemented(
                format!("Voice handler for {:?}", message.channel)
            ))
        }
    }
}

impl Default for VoiceHandlerManager {
    fn default() -> Self {
        Self::new()
    }
}

// Helper trait for config access
trait VoiceHandlerExt: VoiceHandler {
    fn get_auto_reply(&self) -> bool;
}

impl VoiceHandlerExt for TelegramVoiceHandler {
    fn get_auto_reply(&self) -> bool {
        self.config.auto_reply
    }
}

impl VoiceHandlerExt for DiscordVoiceHandler {
    fn get_auto_reply(&self) -> bool {
        self.config.auto_reply
    }
}

impl VoiceHandlerExt for dyn VoiceHandler {
    fn get_auto_reply(&self) -> bool {
        // Default
        true
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// TESTS
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_voice_message_creation() {
        let msg = VoiceMessage {
            message_id: "test123".into(),
            channel: VoiceChannel::Telegram,
            user_id: "user123".into(),
            username: Some("test_user".into()),
            chat_id: "chat456".into(),
            file_url: None,
            file_id: "file789".into(),
            duration_secs: Some(10),
            mime_type: Some("audio/ogg".into()),
            timestamp: chrono::Utc::now(),
        };
        
        assert_eq!(msg.message_id, "test123");
        assert_eq!(msg.channel, VoiceChannel::Telegram);
    }
    
    #[test]
    fn test_voice_handler_config() {
        let config = VoiceHandlerConfig::default();
        assert_eq!(config.max_duration_secs, 120);
        assert!(config.auto_reply);
        assert!(config.voice_reply);
    }
    
    #[test]
    fn test_voice_channel_serialization() {
        let channel = VoiceChannel::Telegram;
        let json = serde_json::to_string(&channel).unwrap();
        assert_eq!(json, "\"telegram\"");
    }
    
    #[test]
    fn test_voice_response() {
        let response = VoiceResponse {
            reply_to: "msg123".into(),
            transcript: "Merhaba".into(),
            response_text: "Size de merhaba!".into(),
            audio_data: Some(vec![0u8; 100]),
            audio_format: "wav".into(),
            language: "tr".into(),
        };
        
        assert_eq!(response.language, "tr");
        assert!(response.audio_data.is_some());
    }
}

// URL encoding module (simple implementation)
mod urlencoding {
    pub fn encode(s: &str) -> String {
        url::form_urlencoded::byte_serialize(s.as_bytes()).collect()
    }
}
