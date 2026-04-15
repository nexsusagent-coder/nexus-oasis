//! Voice assistant - combines TTS, STT, and wake word

use std::sync::Arc;
use tokio::sync::{Mutex, mpsc};

use crate::{
    AudioCapture, AudioPlayback, AudioSettings,
    SttProvider, SttConfig, SttSettings, SttEngine,
    TtsProvider, TtsConfig, TtsSettings, TtsEngine,
    WakeWordDetector, WakeWordConfig, WakeWordSettings,
    VoiceError, VoiceResult, VoiceConfig, VoiceProvider,
};

/// Assistant state
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AssistantState {
    Idle,
    Listening,
    Processing,
    Speaking,
    Error,
}

/// Voice assistant
pub struct VoiceAssistant {
    config: VoiceConfig,
    state: Arc<Mutex<AssistantState>>,
    tts: TtsProvider,
    stt: SttProvider,
    wake: WakeWordDetector,
    capture: AudioCapture,
    playback: AudioPlayback,
    name: String,
}

impl VoiceAssistant {
    pub fn new(name: &str, config: VoiceConfig) -> Self {
        let tts = TtsProvider::new(TtsConfig::new(config.tts.clone()));
        let stt = SttProvider::new(SttConfig::new(config.stt.clone()));
        let wake = WakeWordDetector::new(
            WakeWordConfig::new(WakeWordSettings {
                phrase: format!("hey {}", name.to_lowercase()),
                ..Default::default()
            })
        );
        let capture = AudioCapture::new(config.audio.clone());
        let playback = AudioPlayback::new(config.audio.clone());

        Self {
            config,
            state: Arc::new(Mutex::new(AssistantState::Idle)),
            tts,
            stt,
            wake,
            capture,
            playback,
            name: name.to_string(),
        }
    }

    /// Initialize assistant
    pub async fn init(&self) -> VoiceResult<()> {
        log::info!("Initializing voice assistant: {}", self.name);
        self.set_state(AssistantState::Idle).await;
        Ok(())
    }

    /// Start listening for wake word
    pub async fn start(&self) -> VoiceResult<()> {
        log::info!("Starting voice assistant: '{}'", self.wake.phrase());
        self.wake.start().await?;
        self.capture.start().await?;
        self.set_state(AssistantState::Listening).await;
        Ok(())
    }

    /// Stop assistant
    pub async fn stop(&self) -> VoiceResult<()> {
        log::info!("Stopping voice assistant");
        self.wake.stop().await?;
        self.capture.stop().await?;
        self.set_state(AssistantState::Idle).await;
        Ok(())
    }

    /// Listen and process command
    pub async fn listen(&self, timeout_secs: f32) -> VoiceResult<Option<String>> {
        self.set_state(AssistantState::Listening).await;
        
        // Capture audio
        let audio = self.capture.capture(timeout_secs).await?;
        
        if audio.is_empty() {
            self.set_state(AssistantState::Idle).await;
            return Ok(None);
        }

        // Convert to bytes (simplified - real impl would use proper format)
        let audio_bytes: Vec<u8> = audio
            .iter()
            .flat_map(|&s| s.to_le_bytes())
            .collect();

        // Transcribe
        self.set_state(AssistantState::Processing).await;
        let text = self.stt.transcribe(&audio_bytes).await?;
        
        log::info!("Transcribed: '{}'", text);

        if text.is_empty() {
            self.set_state(AssistantState::Idle).await;
            return Ok(None);
        }

        Ok(Some(text))
    }

    /// Speak text
    pub async fn speak(&self, text: &str) -> VoiceResult<()> {
        self.set_state(AssistantState::Speaking).await;
        
        let audio = self.tts.synthesize(text).await?;
        self.playback.play(&audio).await?;
        
        self.set_state(AssistantState::Idle).await;
        Ok(())
    }

    /// Process interaction: listen, return text
    pub async fn interact(&self) -> VoiceResult<String> {
        // Wait for wake word
        while !self.wake.was_detected().await {
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        }

        self.wake.reset().await;

        // Listen for command
        let result = self.listen(5.0).await?;
        Ok(result.unwrap_or_default())
    }

    /// Set state
    async fn set_state(&self, state: AssistantState) {
        let mut current = self.state.lock().await;
        *current = state;
    }

    /// Get current state
    pub async fn state(&self) -> AssistantState {
        self.state.lock().await.clone()
    }

    /// Get assistant name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get wake word phrase
    pub fn wake_word(&self) -> &str {
        self.wake.phrase()
    }

    /// Trigger wake word manually (for testing)
    pub async fn trigger_wake(&self) {
        self.wake.trigger().await;
    }

    /// Update wake word
    pub fn set_wake_word(&mut self, phrase: &str) {
        self.wake.set_phrase(phrase);
    }

    /// Configure TTS provider
    pub fn set_tts_api_key(&mut self, key: &str) {
        self.config.tts = TtsSettings {
            provider: VoiceProvider::OpenAI,
            ..self.config.tts.clone()
        };
    }

    /// Configure STT provider
    pub fn set_stt_api_key(&mut self, key: &str) {
        self.config.stt = SttSettings {
            provider: VoiceProvider::OpenAI,
            ..self.config.stt.clone()
        };
    }
    
    // Gateway compatibility methods
    
    /// Transcribe audio (gateway compatibility)
    pub async fn transcribe(&self, audio: &[f32]) -> VoiceResult<crate::TranscriptionResult> {
        self.set_state(AssistantState::Processing).await;
        
        // Convert f32 to bytes
        let audio_bytes: Vec<u8> = audio
            .iter()
            .flat_map(|&s| {
                let sample = (s * 32767.0) as i16;
                sample.to_le_bytes().to_vec()
            })
            .collect();
        
        let text = self.stt.transcribe(&audio_bytes).await?;
        
        self.set_state(AssistantState::Idle).await;
        
        Ok(crate::TranscriptionResult {
            text,
            language: self.config.stt.language.clone(),
            confidence: 0.9,
            duration_secs: audio.len() as f32 / 16000.0,
        })
    }
    
    /// Synthesize speech (gateway compatibility)
    pub async fn synthesize(&self, text: &str) -> VoiceResult<SynthesisResult> {
        self.set_state(AssistantState::Speaking).await;
        
        let audio_bytes = self.tts.synthesize(text).await?;
        
        // Convert bytes to f32 samples
        let audio: Vec<f32> = audio_bytes
            .chunks_exact(2)
            .map(|chunk| {
                let sample = i16::from_le_bytes([chunk[0], chunk[1]]);
                sample as f32 / 32768.0
            })
            .collect();
        
        self.set_state(AssistantState::Idle).await;
        
        Ok(SynthesisResult { audio })
    }
    
    /// Synthesize with specific voice (gateway compatibility)
    pub async fn synthesize_with_voice(&self, text: &str, _voice_id: &str) -> VoiceResult<SynthesisResult> {
        self.synthesize(text).await
    }
    
    /// Detect voice activity (gateway compatibility)
    pub fn detect_voice_activity(&self, audio: &[f32]) -> bool {
        // Simple energy-based VAD
        let energy: f64 = audio.iter().map(|&s| (s as f64).powi(2)).sum::<f64>() / audio.len() as f64;
        energy > 0.01 // Threshold
    }
}

/// Synthesis result for gateway compatibility
#[derive(Debug, Clone)]
pub struct SynthesisResult {
    pub audio: Vec<f32>,
}

impl Default for VoiceAssistant {
    fn default() -> Self {
        Self::new("SENTIENT", VoiceConfig::default())
    }
}
