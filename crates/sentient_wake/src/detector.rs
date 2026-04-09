//! ─── Wake Word Detector ───

use crate::{WakeEngine, WakeWordConfig, SAMPLE_RATE};
use std::sync::Arc;
use tokio::sync::{broadcast, mpsc};
use tokio::task::JoinHandle;

/// Wake word event
#[derive(Debug, Clone)]
pub enum WakeEvent {
    /// Wake word detected
    Detected {
        confidence: f32,
        timestamp: std::time::Instant,
    },
    
    /// Audio level (0.0 - 1.0)
    AudioLevel(f32),
    
    /// Speech detected (but not wake word)
    SpeechDetected {
        text: String,
    },
    
    /// Error occurred
    Error(String),
}

/// Wake word detector
pub struct WakeWordDetector {
    config: WakeWordConfig,
    event_tx: broadcast::Sender<WakeEvent>,
    running: Arc<std::sync::atomic::AtomicBool>,
    task_handle: Option<JoinHandle<()>>,
}

impl WakeWordDetector {
    /// Create new wake word detector
    pub fn new(config: WakeWordConfig) -> Result<Self, WakeError> {
        let (event_tx, _) = broadcast::channel(16);
        
        Ok(Self {
            config,
            event_tx,
            running: Arc::new(std::sync::atomic::AtomicBool::new(false)),
            task_handle: None,
        })
    }
    
    /// Subscribe to wake events
    pub fn subscribe(&self) -> broadcast::Receiver<WakeEvent> {
        self.event_tx.subscribe()
    }
    
    /// Start listening for wake word
    pub async fn start(&mut self) -> Result<(), WakeError> {
        if self.running.load(std::sync::atomic::Ordering::SeqCst) {
            return Err(WakeError::AlreadyRunning);
        }
        
        self.running.store(true, std::sync::atomic::Ordering::SeqCst);
        
        let running = self.running.clone();
        let config = self.config.clone();
        let event_tx = self.event_tx.clone();
        
        // Spawn audio capture and detection task
        let handle = tokio::spawn(async move {
            if let Err(e) = run_detection(running, config, event_tx).await {
                log::error!("Detection error: {}", e);
            }
        });
        
        self.task_handle = Some(handle);
        
        Ok(())
    }
    
    /// Stop listening
    pub async fn stop(&mut self) -> Result<(), WakeError> {
        self.running.store(false, std::sync::atomic::Ordering::SeqCst);
        
        if let Some(handle) = self.task_handle.take() {
            handle.await.map_err(|e| WakeError::Internal(e.to_string()))?;
        }
        
        Ok(())
    }
    
    /// Check if detector is running
    pub fn is_running(&self) -> bool {
        self.running.load(std::sync::atomic::Ordering::SeqCst)
    }
    
    /// Update configuration
    pub fn update_config(&mut self, config: WakeWordConfig) {
        self.config = config;
    }
}

/// Run the detection loop
async fn run_detection(
    running: Arc<std::sync::atomic::AtomicBool>,
    config: WakeWordConfig,
    event_tx: broadcast::Sender<WakeEvent>,
) -> Result<(), WakeError> {
    // Initialize audio capture
    let host = cpal::default_host();
    let device = host
        .default_input_device()
        .ok_or_else(|| WakeError::NoInputDevice)?;
    
    let supported_config = device
        .default_input_config()
        .map_err(|e| WakeError::AudioConfig(e.to_string()))?;
    
    log::info!("Using audio device: {}", device.name().unwrap_or_default());
    log::info!("Sample rate: {:?}", supported_config.sample_rate());
    
    // Create audio stream
    let (audio_tx, mut audio_rx) = mpsc::channel::<Vec<f32>>(100);
    
    let input_stream = device
        .build_input_stream(
            &supported_config.into(),
            move |data: &[f32], _: &cpal::InputCallbackInfo| {
                // Resample if needed
                let samples = data.to_vec();
                let _ = audio_tx.blocking_send(samples);
            },
            |err| {
                log::error!("Audio stream error: {}", err);
            },
            None,
        )
        .map_err(|e| WakeError::AudioStream(e.to_string()))?;
    
    input_stream
        .play()
        .map_err(|e| WakeError::AudioStream(e.to_string()))?;
    
    // Detection loop
    let mut last_detection = std::time::Instant::now()
        - std::time::Duration::from_millis(config.cooldown_ms);
    
    while running.load(std::sync::atomic::Ordering::SeqCst) {
        // Receive audio samples
        let samples = match audio_rx.recv().await {
            Some(s) => s,
            None => break,
        };
        
        // Calculate audio level
        let level = calculate_audio_level(&samples);
        let _ = event_tx.send(WakeEvent::AudioLevel(level));
        
        // Skip if in cooldown
        if last_detection.elapsed().as_millis() < config.cooldown_ms as u128 {
            continue;
        }
        
        // Detect wake word based on engine
        let detected = match config.engine {
            WakeEngine::Porcupine => {
                #[cfg(feature = "porcupine")]
                {
                    crate::porcupine::detect(&samples, &config)?
                }
                #[cfg(not(feature = "porcupine"))]
                None
            }
            WakeEngine::Vosk => {
                #[cfg(feature = "vosk")]
                {
                    crate::vosk_::detect(&samples, &config)?
                }
                #[cfg(not(feature = "vosk"))]
                None
            }
            WakeEngine::Whisper => {
                #[cfg(feature = "whisper")]
                {
                    crate::whisper_::detect(&samples, &config)?
                }
                #[cfg(not(feature = "whisper"))]
                None
            }
            WakeEngine::Simple => {
                // Simple energy-based detection for testing
                if level > 0.5 {
                    Some((1.0, config.wake_word.clone()))
                } else {
                    None
                }
            }
        };
        
        if let Some((confidence, text)) = detected {
            if confidence >= config.confidence_threshold {
                log::info!("Wake word detected: {} (confidence: {})", text, confidence);
                let _ = event_tx.send(WakeEvent::Detected {
                    confidence,
                    timestamp: std::time::Instant::now(),
                });
                last_detection = std::time::Instant::now();
            }
        }
    }
    
    drop(input_stream);
    Ok(())
}

/// Calculate audio level (RMS)
fn calculate_audio_level(samples: &[f32]) -> f32 {
    if samples.is_empty() {
        return 0.0;
    }
    
    let sum: f32 = samples.iter().map(|s| s * s).sum();
    let rms = (sum / samples.len() as f32).sqrt();
    
    // Normalize to 0-1 range
    (rms * 10.0).min(1.0)
}

/// Wake word error
#[derive(Debug, thiserror::Error)]
pub enum WakeError {
    #[error("No input device available")]
    NoInputDevice,
    
    #[error("Audio config error: {0}")]
    AudioConfig(String),
    
    #[error("Audio stream error: {0}")]
    AudioStream(String),
    
    #[error("Model not found: {0}")]
    ModelNotFound(String),
    
    #[error("Invalid access key")]
    InvalidAccessKey,
    
    #[error("Detector already running")]
    AlreadyRunning,
    
    #[error("Internal error: {0}")]
    Internal(String),
}
