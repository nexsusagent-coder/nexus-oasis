//! Main Daemon - Koordinasyon merkezi
//!
//! Voice → Parser → Action → TTS döngüsünü yönetir.

use crate::actions::VoiceActionExecutor;
use crate::commands::CommandParser;
use crate::error::DaemonResult;
use sentient_voice::assistant::VoiceAssistant;
use sentient_voice::types::VoiceProvider;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::{broadcast, RwLock};
use tracing::info;

/// Daemon configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaemonConfig {
    /// Assistant name (e.g., "Luna", "Jarvis")
    pub assistant_name: String,
    /// Wake word phrase (e.g., "Hey Luna")
    pub wake_word: String,
    /// Language code
    pub language: String,
    /// TTS provider
    pub tts_provider: VoiceProvider,
    /// STT provider
    pub stt_provider: VoiceProvider,
    /// Enable voice responses
    pub voice_responses: bool,
    /// Log level
    pub log_level: String,
}

impl Default for DaemonConfig {
    fn default() -> Self {
        Self {
            assistant_name: "Sentient".to_string(),
            wake_word: "Hey Sentient".to_string(),
            language: "tr".to_string(),
            tts_provider: VoiceProvider::OpenAI,
            stt_provider: VoiceProvider::OpenAI,
            voice_responses: true,
            log_level: "info".to_string(),
        }
    }
}

/// Daemon state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DaemonState {
    Stopped,
    Starting,
    Listening,
    Processing,
    Executing,
    Speaking,
    Error,
    ShuttingDown,
}

/// Main daemon struct
pub struct SentientDaemon {
    config: DaemonConfig,
    state: Arc<RwLock<DaemonState>>,
    parser: CommandParser,
    executor: VoiceActionExecutor,
    voice: Option<Arc<RwLock<VoiceAssistant>>>,
    shutdown_tx: broadcast::Sender<()>,
}

impl SentientDaemon {
    pub fn new(config: DaemonConfig) -> DaemonResult<Self> {
        let parser = CommandParser::new();
        let executor = VoiceActionExecutor::new(&config.assistant_name);
        let (shutdown_tx, _) = broadcast::channel(1);

        Ok(Self {
            voice: None,
            state: Arc::new(RwLock::new(DaemonState::Stopped)),
            shutdown_tx,
            config,
            parser,
            executor,
        })
    }

    pub fn with_voice(mut self, voice: VoiceAssistant) -> Self {
        self.voice = Some(Arc::new(RwLock::new(voice)));
        self
    }

    pub async fn start(&self) -> DaemonResult<()> {
        info!("🚀 {} Daemon başlatılıyor...", self.config.assistant_name);

        {
            let mut state = self.state.write().await;
            *state = DaemonState::Starting;
        }

        if let Some(ref voice) = self.voice {
            let _v = voice.write().await;
            info!("🎤 Voice assistant hazır");
        }

        {
            let mut state = self.state.write().await;
            *state = DaemonState::Listening;
        }

        info!("✅ {} hazır! Wake word: '{}'", self.config.assistant_name, self.config.wake_word);

        self.run_loop().await
    }

    async fn run_loop(&self) -> DaemonResult<()> {
        let mut shutdown_rx = self.shutdown_tx.subscribe();

        loop {
            if shutdown_rx.try_recv().is_ok() {
                info!("🛑 Shutdown sinyali alındı");
                break;
            }

            let state = *self.state.read().await;

            match state {
                DaemonState::Listening => {
                    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                }
                DaemonState::ShuttingDown => break,
                _ => tokio::time::sleep(tokio::time::Duration::from_millis(10)).await,
            }
        }

        {
            let mut state = self.state.write().await;
            *state = DaemonState::Stopped;
        }

        info!("👋 {} kapatıldı", self.config.assistant_name);
        Ok(())
    }

    pub async fn process_command(&self, text: &str) -> DaemonResult<crate::actions::ActionResult> {
        info!("📝 İşleniyor: '{}'", text);

        {
            let mut state = self.state.write().await;
            *state = DaemonState::Processing;
        }

        let parsed = self.parser.parse(text);
        info!("🎯 Intent: {:?}, Confidence: {:.2}", parsed.intent, parsed.confidence);

        {
            let mut state = self.state.write().await;
            *state = DaemonState::Executing;
        }

        let result = self.executor.execute(&parsed).await;

        {
            let mut state = self.state.write().await;
            *state = DaemonState::Speaking;
        }

        if let Ok(ref action_result) = result {
            if self.config.voice_responses {
                let response = self.executor.generate_response(action_result);
                info!("🗣️ Yanıt: '{}'", response);
            }
        }

        {
            let mut state = self.state.write().await;
            *state = DaemonState::Listening;
        }

        result
    }

    pub async fn stop(&self) -> DaemonResult<()> {
        info!("🛑 {} durduruluyor...", self.config.assistant_name);

        {
            let mut state = self.state.write().await;
            *state = DaemonState::ShuttingDown;
        }

        let _ = self.shutdown_tx.send(());
        Ok(())
    }

    pub async fn get_state(&self) -> DaemonState {
        *self.state.read().await
    }

    pub fn get_config(&self) -> &DaemonConfig {
        &self.config
    }

    pub async fn is_running(&self) -> bool {
        let state = *self.state.read().await;
        matches!(state, DaemonState::Listening | DaemonState::Processing | DaemonState::Executing)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let config = DaemonConfig::default();
        assert_eq!(config.assistant_name, "Sentient");
        assert_eq!(config.wake_word, "Hey Sentient");
    }

    #[test]
    fn test_daemon_creation() {
        let rt = tokio::runtime::Runtime::new().expect("failed");
        let config = DaemonConfig {
            assistant_name: "Luna".to_string(),
            wake_word: "Hey Luna".to_string(),
            ..Default::default()
        };

        let daemon = rt.block_on(async { SentientDaemon::new(config).expect("failed") });
        assert_eq!(daemon.config.assistant_name, "Luna");
    }
}
