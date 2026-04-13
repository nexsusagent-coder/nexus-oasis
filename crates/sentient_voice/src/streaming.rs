//! ─── Real-time Voice Streaming ───

use std::sync::Arc;
use tokio::sync::{RwLock, mpsc};
#[allow(unused_imports)]
use tokio_stream::Stream;
use parking_lot::Mutex;
#[allow(unused_imports)]
use futures::StreamExt;

use crate::{
    SpeechToText, TranscriptionResult, VoiceActivityDetector,
    VoiceConfig, VoiceError,
};

/// Stream configuration
#[derive(Debug, Clone)]
pub struct StreamConfig {
    /// Sample rate
    pub sample_rate: u32,
    
    /// Frame size in samples (typically 10-50ms)
    pub frame_size: usize,
    
    /// Silence timeout in milliseconds before finalizing
    pub silence_timeout_ms: u64,
    
    /// Minimum audio length for transcription (ms)
    pub min_audio_ms: u64,
    
    /// Enable VAD-based segmentation
    pub vad_enabled: bool,
    
    /// VAD sensitivity
    pub vad_sensitivity: f32,
}

impl Default for StreamConfig {
    fn default() -> Self {
        Self {
            sample_rate: 16000,
            frame_size: 1600, // 100ms at 16kHz
            silence_timeout_ms: 1000,
            min_audio_ms: 500,
            vad_enabled: true,
            vad_sensitivity: 0.3,
        }
    }
}

/// Stream events
#[derive(Debug)]
pub enum StreamEvent {
    /// Audio chunk received
    AudioChunk(Vec<f32>),
    
    /// Voice activity started
    VoiceStart,
    
    /// Voice activity ended
    VoiceEnd,
    
    /// Partial transcription (if supported)
    PartialTranscript(String),
    
    /// Final transcription result
    Transcript(TranscriptionResult),
    
    /// Error occurred
    Error(VoiceError),
    
    /// Stream ended
    End,
}

/// Voice stream for real-time transcription
pub struct VoiceStream {
    stt: Arc<RwLock<Box<dyn SpeechToText>>>,
    vad: Arc<Mutex<VoiceActivityDetector>>,
    config: StreamConfig,
    #[allow(dead_code)]
    voice_config: VoiceConfig,
    
    /// Audio buffer for current utterance
    buffer: Arc<Mutex<Vec<f32>>>,
    
    /// State tracking
    is_speaking: Arc<Mutex<bool>>,
    silence_frames: Arc<Mutex<usize>>,
    
    /// Channel for sending events
    #[allow(dead_code)]
    event_tx: mpsc::Sender<StreamEvent>,
    event_rx: Option<mpsc::Receiver<StreamEvent>>,
    
    /// Stop signal
    stop_signal: Arc<Mutex<bool>>,
}

impl VoiceStream {
    /// Create new voice stream
    pub fn new(
        stt: Arc<RwLock<Box<dyn SpeechToText>>>,
        vad: Arc<Mutex<VoiceActivityDetector>>,
        config: StreamConfig,
        voice_config: VoiceConfig,
    ) -> Result<Self, VoiceError> {
        let (event_tx, event_rx) = mpsc::channel(100);
        
        Ok(Self {
            stt,
            vad,
            config,
            voice_config,
            buffer: Arc::new(Mutex::new(Vec::new())),
            is_speaking: Arc::new(Mutex::new(false)),
            silence_frames: Arc::new(Mutex::new(0)),
            event_tx,
            event_rx: Some(event_rx),
            stop_signal: Arc::new(Mutex::new(false)),
        })
    }
    
    /// Get event receiver
    pub fn take_receiver(&mut self) -> Option<mpsc::Receiver<StreamEvent>> {
        self.event_rx.take()
    }
    
    /// Process audio chunk
    pub async fn process_chunk(&self, audio: &[f32]) -> Result<(), VoiceError> {
        if *self.stop_signal.lock() {
            return Ok(());
        }
        
        // Send audio chunk event
        let _ = self.event_tx.send(StreamEvent::AudioChunk(audio.to_vec())).await;
        
        // Detect voice activity
        let voice_detected = if self.config.vad_enabled {
            let mut vad = self.vad.lock();
            vad.process(audio)
        } else {
            true // Always process if VAD disabled
        };
        
        let mut is_speaking = self.is_speaking.lock();
        let mut silence_frames = self.silence_frames.lock();
        let mut buffer = self.buffer.lock();
        
        if voice_detected {
            // Voice detected
            if !*is_speaking {
                *is_speaking = true;
                let _ = self.event_tx.send(StreamEvent::VoiceStart).await;
            }
            
            // Add to buffer
            buffer.extend_from_slice(audio);
            *silence_frames = 0;
            
            // Optional: partial transcription
            if buffer.len() > (self.config.sample_rate as f64 * 0.5) as usize {
                // Could send partial results here for streaming ASR
            }
        } else if *is_speaking {
            // Silence detected during speech
            *silence_frames += 1;
            buffer.extend_from_slice(audio);
            
            // Check if silence timeout reached
            let silence_duration_ms = (*silence_frames * self.config.frame_size * 1000) 
                / self.config.sample_rate as usize;
            
            if silence_duration_ms as u64 >= self.config.silence_timeout_ms {
                // Finalize utterance
                let _ = self.event_tx.send(StreamEvent::VoiceEnd).await;
                
                // Check minimum length
                let audio_duration_ms = (buffer.len() * 1000) / self.config.sample_rate as usize;
                
                if audio_duration_ms as u64 >= self.config.min_audio_ms {
                    // Transcribe
                    let audio_to_transcribe = buffer.clone();
                    buffer.clear();
                    *is_speaking = false;
                    *silence_frames = 0;
                    
                    // Drop locks before async operation
                    drop(buffer);
                    drop(is_speaking);
                    drop(silence_frames);
                    
                    let stt = self.stt.read().await;
                    match stt.transcribe(&audio_to_transcribe).await {
                        Ok(result) => {
                            let _ = self.event_tx.send(StreamEvent::Transcript(result)).await;
                        }
                        Err(e) => {
                            let _ = self.event_tx.send(StreamEvent::Error(e)).await;
                        }
                    }
                } else {
                    // Too short, discard
                    buffer.clear();
                    *is_speaking = false;
                    *silence_frames = 0;
                }
            }
        }
        
        Ok(())
    }
    
    /// Stop streaming and finalize
    pub async fn stop(&self) -> Result<(), VoiceError> {
        *self.stop_signal.lock() = true;
        
        let is_speaking = *self.is_speaking.lock();
        if is_speaking {
            let buffer = self.buffer.lock().clone();
            
            if !buffer.is_empty() {
                let stt = self.stt.read().await;
                match stt.transcribe(&buffer).await {
                    Ok(result) => {
                        let _ = self.event_tx.send(StreamEvent::Transcript(result)).await;
                    }
                    Err(e) => {
                        let _ = self.event_tx.send(StreamEvent::Error(e)).await;
                    }
                }
            }
        }
        
        let _ = self.event_tx.send(StreamEvent::End).await;
        Ok(())
    }
    
    /// Reset stream state
    pub fn reset(&self) {
        self.buffer.lock().clear();
        *self.is_speaking.lock() = false;
        *self.silence_frames.lock() = 0;
        *self.stop_signal.lock() = false;
        self.vad.lock().reset();
    }
    
    /// Get current buffer (for debugging)
    pub fn current_buffer(&self) -> Vec<f32> {
        self.buffer.lock().clone()
    }
    
    /// Check if currently speaking
    pub fn is_speaking(&self) -> bool {
        *self.is_speaking.lock()
    }
}

/// Audio recorder for capturing from microphone
pub struct AudioRecorder {
    #[allow(dead_code)]
    config: StreamConfig,
    #[allow(dead_code)]
    event_tx: mpsc::Sender<Vec<f32>>,
}

impl AudioRecorder {
    /// Create new recorder
    pub fn new(config: StreamConfig, event_tx: mpsc::Sender<Vec<f32>>) -> Self {
        Self { config, event_tx }
    }
    
    /// Start recording from microphone
    #[cfg(feature = "cpal")]
    pub async fn start(&self) -> Result<(), VoiceError> {
        use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
        
        let host = cpal::default_host();
        let device = host.default_input_device()
            .ok_or_else(|| VoiceError::AudioError("No input device found".into()))?;
        
        let supported_config = device.default_input_config()
            .map_err(|e| VoiceError::AudioError(e.to_string()))?;
        
        let sample_rate = self.config.sample_rate;
        let frame_size = self.config.frame_size;
        let tx = self.event_tx.clone();
        
        let stream_config = cpal::StreamConfig {
            channels: 1,
            sample_rate: cpal::SampleRate(sample_rate),
            buffer_size: cpal::BufferSize::Default,
        };
        
        let mut sample_buffer: Vec<f32> = Vec::with_capacity(frame_size);
        
        let stream = device.build_input_stream(
            &stream_config,
            move |data: &[f32], _: &cpal::InputCallbackInfo| {
                sample_buffer.extend_from_slice(data);
                
                if sample_buffer.len() >= frame_size {
                    let chunk: Vec<f32> = sample_buffer.drain(..frame_size).collect();
                    let _ = tx.try_send(chunk);
                }
            },
            |err| {
                log::error!("Audio error: {}", err);
            },
            None,
        ).map_err(|e| VoiceError::AudioError(e.to_string()))?;
        
        stream.play().map_err(|e| VoiceError::AudioError(e.to_string()))?;
        
        // Keep stream alive
        // In real implementation, return stream handle
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_stream_config() {
        let config = StreamConfig::default();
        assert!(config.sample_rate > 0);
        assert!(config.frame_size > 0);
    }
}
