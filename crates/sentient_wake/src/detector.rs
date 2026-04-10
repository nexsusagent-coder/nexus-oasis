//! ─── Wake Word Detector ───

use crate::{WakeEngine, WakeWordConfig};
use std::sync::Arc;
use tokio::sync::broadcast;

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
}

impl WakeWordDetector {
    /// Create new wake word detector
    pub fn new(config: WakeWordConfig) -> Result<Self, WakeError> {
        let (event_tx, _) = broadcast::channel(16);
        
        Ok(Self {
            config,
            event_tx,
            running: Arc::new(std::sync::atomic::AtomicBool::new(false)),
        })
    }
    
    /// Subscribe to wake events
    pub fn subscribe(&self) -> broadcast::Receiver<WakeEvent> {
        self.event_tx.subscribe()
    }
    
    /// Start listening for wake word
    pub fn start(&mut self) -> Result<(), WakeError> {
        if self.running.load(std::sync::atomic::Ordering::SeqCst) {
            return Err(WakeError::AlreadyRunning);
        }
        
        self.running.store(true, std::sync::atomic::Ordering::SeqCst);
        
        let running = self.running.clone();
        let config = self.config.clone();
        let event_tx = self.event_tx.clone();
        
        // Spawn detection in a separate thread
        std::thread::spawn(move || {
            if let Err(e) = run_detection_sync(running, config, event_tx) {
                log::error!("Detection error: {}", e);
            }
        });
        
        Ok(())
    }
    
    /// Stop listening
    pub fn stop(&mut self) -> Result<(), WakeError> {
        self.running.store(false, std::sync::atomic::Ordering::SeqCst);
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

/// Run the detection loop synchronously
fn run_detection_sync(
    running: Arc<std::sync::atomic::AtomicBool>,
    config: WakeWordConfig,
    event_tx: broadcast::Sender<WakeEvent>,
) -> Result<(), WakeError> {
    use cpal::traits::{HostTrait, DeviceTrait, StreamTrait};
    
    // Initialize audio capture
    let host = cpal::default_host();
    
    let device = match host.default_input_device() {
        Some(d) => d,
        None => return Err(WakeError::NoInputDevice),
    };
    
    let supported_config: cpal::SupportedStreamConfig = device
        .default_input_config()
        .map_err(|e| WakeError::AudioConfig(e.to_string()))?;
    
    log::info!("Using audio device: {}", device.name().unwrap_or_default());
    log::info!("Sample rate: {:?}", supported_config.sample_rate());
    
    // Log engine status
    log::info!("Wake engine: {:?}", config.engine);
    log::info!("Vosk: {}", crate::vosk_::status());
    log::info!("Whisper: {}", crate::whisper_::status());
    log::info!("Porcupine: {}", crate::porcupine::status());
    
    let running_clone = running.clone();
    let event_tx_clone = event_tx.clone();
    let config_clone = config.clone();
    
    // Audio buffer for accumulating samples
    let audio_buffer: Arc<std::sync::Mutex<Vec<f32>>> = Arc::new(std::sync::Mutex::new(Vec::new()));
    let buffer_clone = audio_buffer.clone();
    
    // Create audio stream
    let input_stream = device
        .build_input_stream(
            &supported_config.into(),
            move |data: &[f32], _: &cpal::InputCallbackInfo| {
                // Calculate audio level
                let level = calculate_audio_level(data);
                let _ = event_tx_clone.send(WakeEvent::AudioLevel(level));
                
                // Accumulate samples for detection
                let mut buf = buffer_clone.lock().expect("operation failed");
                buf.extend_from_slice(data);
                
                // Process when we have enough samples (at least 1 second)
                if buf.len() >= config_clone.sample_rate as usize {
                    let samples: Vec<f32> = buf.drain(..config_clone.sample_rate as usize).collect();
                    
                    // Try detection with available engine
                    let result = match config_clone.engine {
                        WakeEngine::Vosk => crate::vosk_::detect(&samples, &config_clone),
                        WakeEngine::Whisper => crate::whisper_::detect(&samples, &config_clone),
                        WakeEngine::Porcupine => crate::porcupine::detect(&samples, &config_clone),
                        WakeEngine::Simple => {
                            // Simple energy-based detection
                            if level > config_clone.confidence_threshold {
                                Ok(Some((level, config_clone.wake_word.clone())))
                            } else {
                                Ok(None)
                            }
                        }
                    };
                    
                    if let Ok(Some((confidence, word))) = result {
                        let _ = event_tx_clone.send(WakeEvent::Detected {
                            confidence,
                            timestamp: std::time::Instant::now(),
                        });
                        log::info!("Wake word '{}' detected with confidence {:.2}", word, confidence);
                    } else if let Ok(Some((_, text))) = crate::vosk_::detect(&samples, &config_clone) {
                        // Speech detected but not wake word
                        if !text.is_empty() {
                            let _ = event_tx_clone.send(WakeEvent::SpeechDetected { text });
                        }
                    }
                }
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
    
    // Keep stream alive while running
    while running.load(std::sync::atomic::Ordering::SeqCst) {
        std::thread::sleep(std::time::Duration::from_millis(100));
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
