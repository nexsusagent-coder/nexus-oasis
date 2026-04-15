//! ═══════════════════════════════════════════════════════════════════════════════
//!  SENTIENT Gateway Voice - WebSocket Voice Streaming Handler
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//!  Dashboard ve web istemcilerinden gelen ses akışını işler:
//!  - WebSocket üzerinden gerçek zamanlı ses streaming
//!  - STT (Speech-to-Text) transcription
//!  - LLM response generation
//!  - TTS (Text-to-Speech) response streaming
//!
//!  Akış:
//!  [Browser Mic] -> [WebSocket] -> [VoiceHandler] -> [STT] -> [LLM] -> [TTS] -> [Browser Speaker]

use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Extension,
    },
    response::Response,
};
use futures::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use uuid::Uuid;
use chrono::{DateTime, Utc};

use sentient_voice::{
    VoiceEngine, VoiceConfig, VoiceError,
    StreamConfig, StreamEvent, TranscriptionResult,
};

use crate::dispatcher::TaskDispatcher;

// ─────────────────────────────────────────────────────────────────────────────
// VOICE WEBSOCKET MESSAGE TYPES
// ─────────────────────────────────────────────────────────────────────────────

/// Client -> Server mesajları
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum VoiceClientMessage {
    /// Ses chunk'ı (base64 encoded)
    AudioChunk {
        /// Base64 encoded audio data (16kHz, mono, f32)
        data: String,
        /// Chunk sequence number
        seq: u32,
    },
    
    /// Ses akışı başlat
    StartStream {
        /// Sample rate
        sample_rate: u32,
        /// Language code (tr, en, etc.)
        language: Option<String>,
        /// Wake word ile başlatılsın mı?
        wake_word: Option<bool>,
    },
    
    /// Ses akışını durdur
    StopStream,
    
    /// Metin gönder (text-to-speech için)
    TextToSpeech {
        /// Metin
        text: String,
        /// Ses ID (opsiyonel)
        voice_id: Option<String>,
    },
    
    /// Ping
    Ping,
}

/// Server -> Client mesajları
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum VoiceServerMessage {
    /// Ses akışı başladı
    StreamStarted {
        session_id: Uuid,
    },
    
    /// Ses aktivitesi başladı
    VoiceActivityStart {
        timestamp: DateTime<Utc>,
    },
    
    /// Ses aktivitesi bitti
    VoiceActivityEnd {
        timestamp: DateTime<Utc>,
    },
    
    /// Kısmi transcription (real-time)
    PartialTranscript {
        text: String,
        is_final: bool,
    },
    
    /// Final transcription
    Transcript {
        text: String,
        language: String,
        confidence: f32,
        duration_secs: f32,
    },
    
    /// LLM yanıtı
    LlmResponse {
        text: String,
        is_streaming: bool,
    },
    
    /// TTS ses verisi (base64 encoded)
    AudioResponse {
        data: String,
        format: String, // "wav", "mp3", etc.
    },
    
    /// Hata
    Error {
        code: u16,
        message: String,
    },
    
    /// Pong
    Pong {
        timestamp: DateTime<Utc>,
    },
    
    /// Durum güncellemesi
    Status {
        state: VoiceState,
    },
    
    /// Oturum sonlandı
    SessionEnded {
        reason: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum VoiceState {
    Idle,
    Listening,
    Processing,
    Speaking,
    Error,
}

// ─────────────────────────────────────────────────────────────────────────────
// VOICE SESSION MANAGER
// ─────────────────────────────────────────────────────────────────────────────

/// Aktif voice oturumlarını yönetir
pub struct VoiceSessionManager {
    sessions: Arc<RwLock<Vec<VoiceSession>>>,
    voice_engine: Arc<VoiceEngine>,
    task_dispatcher: Arc<TaskDispatcher>,
}

impl VoiceSessionManager {
    pub fn new(voice_engine: Arc<VoiceEngine>, task_dispatcher: Arc<TaskDispatcher>) -> Self {
        Self {
            sessions: Arc::new(RwLock::new(Vec::new())),
            voice_engine,
            task_dispatcher,
        }
    }
    
    /// Yeni oturum oluştur
    pub async fn create_session(&self, config: VoiceSessionConfig) -> VoiceSession {
        let session = VoiceSession {
            id: Uuid::new_v4(),
            config,
            state: VoiceState::Idle,
            created_at: Utc::now(),
        };
        
        self.sessions.write().await.push(session.clone());
        session
    }
    
    /// Oturumu kaldır
    pub async fn remove_session(&self, session_id: Uuid) {
        self.sessions.write().await.retain(|s| s.id != session_id);
    }
    
    /// Aktif oturum sayısı
    pub async fn active_count(&self) -> usize {
        self.sessions.read().await.len()
    }
}

#[derive(Debug, Clone)]
pub struct VoiceSession {
    pub id: Uuid,
    pub config: VoiceSessionConfig,
    pub state: VoiceState,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceSessionConfig {
    pub sample_rate: u32,
    pub language: String,
    pub wake_word_enabled: bool,
    pub vad_sensitivity: f32,
}

impl Default for VoiceSessionConfig {
    fn default() -> Self {
        Self {
            sample_rate: 16000,
            language: "tr".into(),
            wake_word_enabled: true,
            vad_sensitivity: 0.3,
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// WEBSOCKET HANDLER
// ─────────────────────────────────────────────────────────────────────────────

/// Voice WebSocket yükseltme
pub async fn voice_ws_upgrade(
    ws: WebSocketUpgrade,
    Extension(manager): Extension<Arc<VoiceSessionManager>>,
) -> Response {
    ws.on_upgrade(move |socket| handle_voice_websocket(socket, manager))
}

/// Ana WebSocket handler
async fn handle_voice_websocket(socket: WebSocket, manager: Arc<VoiceSessionManager>) {
    let (mut ws_sender, mut ws_receiver) = socket.split();
    let session_id = Uuid::new_v4();
    
    log::info!("🎙️  Yeni voice WebSocket bağlantısı: {}", session_id);
    
    // Session oluştur
    let session = manager.create_session(VoiceSessionConfig::default()).await;
    
    // Yanıt kanalı
    let (tx, mut rx) = mpsc::channel::<VoiceServerMessage>(32);
    
    // Session ID'yi gönder
    let _ = tx.send(VoiceServerMessage::StreamStarted { session_id }).await;
    
    // Giden mesajlar için task
    let send_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            let json = serde_json::to_string(&msg).unwrap_or_default();
            if ws_sender.send(Message::Text(json)).await.is_err() {
                break;
            }
        }
    });
    
    // Ses buffer'ı
    let mut audio_buffer: Vec<f32> = Vec::new();
    let mut current_state = VoiceState::Idle;
    
    // Gelen mesajları işle
    let recv_task = tokio::spawn(async move {
        while let Some(msg) = ws_receiver.next().await {
            match msg {
                Ok(Message::Text(text)) => {
                    if let Ok(client_msg) = serde_json::from_str::<VoiceClientMessage>(&text) {
                        handle_voice_message(
                            client_msg,
                            &tx,
                            &mut audio_buffer,
                            &mut current_state,
                            &manager.voice_engine,
                            &manager.task_dispatcher,
                        ).await;
                    }
                }
                Ok(Message::Binary(data)) => {
                    // Raw binary audio data
                    if current_state == VoiceState::Listening {
                        // Convert bytes to f32 samples
                        let samples = bytes_to_f32(&data);
                        audio_buffer.extend_from_slice(&samples);
                        
                        // VAD check
                        if let Some(event) = check_voice_activity(&samples, &manager.voice_engine).await {
                            let _ = tx.send(event).await;
                        }
                    }
                }
                Ok(Message::Ping(_)) => {
                    let _ = tx.send(VoiceServerMessage::Pong {
                        timestamp: Utc::now(),
                    }).await;
                }
                Ok(Message::Close(_)) => {
                    log::info!("🎙️  Voice WebSocket kapandı: {}", session_id);
                    break;
                }
                Err(e) => {
                    log::error!("Voice WebSocket hatası: {}", e);
                    break;
                }
                _ => {}
            }
        }
        
        // Session'ı temizle
        manager.remove_session(session_id).await;
    });
    
    // Task'ları bekle
    tokio::select! {
        _ = send_task => {},
        _ = recv_task => {},
    }
}

/// Mesaj işleyici
async fn handle_voice_message(
    msg: VoiceClientMessage,
    tx: &mpsc::Sender<VoiceServerMessage>,
    audio_buffer: &mut Vec<f32>,
    state: &mut VoiceState,
    voice_engine: &Arc<VoiceEngine>,
    task_dispatcher: &Arc<TaskDispatcher>,
) {
    match msg {
        VoiceClientMessage::StartStream { sample_rate, language, wake_word } => {
            *state = VoiceState::Listening;
            audio_buffer.clear();
            
            let _ = tx.send(VoiceServerMessage::Status {
                state: VoiceState::Listening,
            }).await;
            
            log::info!("🎙️  Voice stream başladı ({}Hz, {})", sample_rate, language.unwrap_or_else(|| "tr".into()));
        }
        
        VoiceClientMessage::AudioChunk { data, seq: _ } => {
            if *state == VoiceState::Listening {
                // Base64 decode
                if let Ok(bytes) = base64_decode(&data) {
                    let samples = bytes_to_f32(&bytes);
                    audio_buffer.extend_from_slice(&samples);
                    
                    // Her 1 saniyede bir kısmi transcription gönder
                    if audio_buffer.len() >= 16000 {
                        let partial_audio = audio_buffer.clone();
                        if let Ok(result) = voice_engine.transcribe(&partial_audio).await {
                            let _ = tx.send(VoiceServerMessage::PartialTranscript {
                                text: result.text.clone(),
                                is_final: false,
                            }).await;
                        }
                    }
                }
            }
        }
        
        VoiceClientMessage::StopStream => {
            *state = VoiceState::Processing;
            let _ = tx.send(VoiceServerMessage::Status {
                state: VoiceState::Processing,
            }).await;
            
            // Final transcription
            if !audio_buffer.is_empty() {
                match voice_engine.transcribe(audio_buffer).await {
                    Ok(result) => {
                        let _ = tx.send(VoiceServerMessage::Transcript {
                            text: result.text.clone(),
                            language: result.language.clone(),
                            confidence: result.confidence,
                            duration_secs: result.duration_secs,
                        }).await;
                        
                        // LLM'e gönder
                        if !result.text.is_empty() {
                            process_with_llm(result.text, tx, task_dispatcher).await;
                        }
                    }
                    Err(e) => {
                        let _ = tx.send(VoiceServerMessage::Error {
                            code: 500,
                            message: e.to_string(),
                        }).await;
                    }
                }
            }
            
            audio_buffer.clear();
            *state = VoiceState::Idle;
            let _ = tx.send(VoiceServerMessage::Status {
                state: VoiceState::Idle,
            }).await;
        }
        
        VoiceClientMessage::TextToSpeech { text, voice_id } => {
            *state = VoiceState::Speaking;
            let _ = tx.send(VoiceServerMessage::Status {
                state: VoiceState::Speaking,
            }).await;
            
            // TTS
            match if let Some(vid) = voice_id {
                voice_engine.synthesize_with_voice(&text, &vid).await
            } else {
                voice_engine.synthesize(&text).await
            } {
                Ok(result) => {
                    // Convert f32 audio to bytes
                    let audio_bytes: Vec<u8> = result.audio.iter()
                        .flat_map(|s| s.to_le_bytes())
                        .collect();
                    let audio_b64 = base64_encode(&audio_bytes);
                    let _ = tx.send(VoiceServerMessage::AudioResponse {
                        data: audio_b64,
                        format: "wav".into(),
                    }).await;
                }
                Err(e) => {
                    let _ = tx.send(VoiceServerMessage::Error {
                        code: 500,
                        message: e.to_string(),
                    }).await;
                }
            }
            
            *state = VoiceState::Idle;
            let _ = tx.send(VoiceServerMessage::Status {
                state: VoiceState::Idle,
            }).await;
        }
        
        VoiceClientMessage::Ping => {
            let _ = tx.send(VoiceServerMessage::Pong {
                timestamp: Utc::now(),
            }).await;
        }
    }
}

/// LLM ile işle
async fn process_with_llm(
    text: String,
    tx: &mpsc::Sender<VoiceServerMessage>,
    task_dispatcher: &Arc<TaskDispatcher>,
) {
    // Streaming response için mock
    // Gerçek implementasyonda sentient_llm kullanılacak
    
    let _ = tx.send(VoiceServerMessage::LlmResponse {
        text: format!("Anlaşıldı: {}", text),
        is_streaming: true,
    }).await;
    
    // Task dispatch et
    let request = crate::GatewayRequest::new(
        text.clone(),
        crate::RequestSource::Internal,
    );
    
    match task_dispatcher.dispatch(request).await {
        Ok(result) => {
            if result.accepted {
                let _ = tx.send(VoiceServerMessage::LlmResponse {
                    text: format!("Görev oluşturuldu: {}", result.task_id),
                    is_streaming: false,
                }).await;
            }
        }
        Err(e) => {
            let _ = tx.send(VoiceServerMessage::Error {
                code: 500,
                message: e.to_string(),
            }).await;
        }
    }
}

/// VAD kontrolü
async fn check_voice_activity(audio: &[f32], voice_engine: &Arc<VoiceEngine>) -> Option<VoiceServerMessage> {
    if voice_engine.detect_voice_activity(audio) {
        Some(VoiceServerMessage::VoiceActivityStart {
            timestamp: Utc::now(),
        })
    } else {
        None
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// HELPER FUNCTIONS
// ─────────────────────────────────────────────────────────────────────────────

fn bytes_to_f32(bytes: &[u8]) -> Vec<f32> {
    // Assuming 16-bit PCM
    bytes.chunks_exact(2)
        .map(|chunk| {
            let sample = i16::from_le_bytes([chunk[0], chunk[1]]);
            sample as f32 / 32768.0
        })
        .collect()
}

fn base64_decode(s: &str) -> Result<Vec<u8>, base64::DecodeError> {
    use base64::{Engine as _, engine::general_purpose};
    general_purpose::STANDARD.decode(s)
}

fn base64_encode(data: &[u8]) -> String {
    use base64::{Engine as _, engine::general_purpose};
    general_purpose::STANDARD.encode(data)
}

// ─────────────────────────────────────────────────────────────────────────────
// API ROUTE: /voice/status
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct VoiceStatus {
    pub active_sessions: usize,
    pub engine_ready: bool,
    pub sample_rate: u32,
    pub supported_languages: Vec<&'static str>,
}

pub async fn get_voice_status(
    Extension(manager): Extension<Arc<VoiceSessionManager>>,
) -> axum::Json<VoiceStatus> {
    axum::Json(VoiceStatus {
        active_sessions: manager.active_count().await,
        engine_ready: true,
        sample_rate: 16000,
        supported_languages: vec!["tr", "en", "de", "fr", "es"],
    })
}

// ─────────────────────────────────────────────────────────────────────────────
// TESTS
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_voice_client_message_serialization() {
        let msg = VoiceClientMessage::StartStream {
            sample_rate: 16000,
            language: Some("tr".into()),
            wake_word: Some(true),
        };
        
        let json = serde_json::to_string(&msg).unwrap();
        assert!(json.contains("start_stream"));
    }
    
    #[test]
    fn test_voice_server_message_serialization() {
        let msg = VoiceServerMessage::Transcript {
            text: "Merhaba dünya".into(),
            language: "tr".into(),
            confidence: 0.95,
            duration_secs: 2.5,
        };
        
        let json = serde_json::to_string(&msg).unwrap();
        assert!(json.contains("transcript"));
        assert!(json.contains("Merhaba dünya"));
    }
    
    #[test]
    fn test_bytes_to_f32_conversion() {
        // 16-bit PCM sample: 16384 -> 0.5
        let bytes = [0x00, 0x40]; // little-endian 16384
        let samples = bytes_to_f32(&bytes);
        assert!((samples[0] - 0.5).abs() < 0.01);
    }
}
