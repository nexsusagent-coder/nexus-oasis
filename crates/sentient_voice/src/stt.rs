//! ─── Speech-to-Text ───

use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use crate::VoiceError;

/// Transcription result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscriptionResult {
    /// Transcribed text
    pub text: String,
    
    /// Language detected
    pub language: String,
    
    /// Confidence score (0.0 - 1.0)
    pub confidence: f32,
    
    /// Duration in seconds
    pub duration_secs: f32,
    
    /// Timestamps (for whisper)
    pub segments: Vec<TranscriptionSegment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscriptionSegment {
    pub start: f32,
    pub end: f32,
    pub text: String,
}

/// Speech-to-Text trait
#[async_trait]
pub trait SpeechToText: Send + Sync {
    /// Transcribe audio buffer
    async fn transcribe(&self, audio: &[f32]) -> Result<TranscriptionResult, VoiceError>;
    
    /// Transcribe file
    async fn transcribe_file(&self, path: &str) -> Result<TranscriptionResult, VoiceError>;
    
    /// Get supported languages
    fn supported_languages(&self) -> Vec<&str>;
}

/// OpenAI Whisper API
pub struct OpenAiWhisper {
    api_key: String,
    client: Client,
    model: String,
}

impl OpenAiWhisper {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            client: Client::new(),
            model: "whisper-1".into(),
        }
    }
}

#[async_trait]
impl SpeechToText for OpenAiWhisper {
    async fn transcribe(&self, audio: &[f32]) -> Result<TranscriptionResult, VoiceError> {
        // Convert float samples to WAV
        let wav_data = float_to_wav(audio, 16000)?;
        
        // Create multipart form
        let part = reqwest::multipart::Part::bytes(wav_data)
            .file_name("audio.wav")
            .mime_str("audio/wav")
            .map_err(|e| VoiceError::Internal(e.to_string()))?;
        
        let form = reqwest::multipart::Form::new()
            .part("file", part)
            .text("model", self.model.clone())
            .text("response_format", "verbose_json");
        
        // Send request
        let response = self.client
            .post("https://api.openai.com/v1/audio/transcriptions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .multipart(form)
            .send()
            .await
            .map_err(|e| VoiceError::NetworkError(e.to_string()))?;
        
        if !response.status().is_success() {
            let error = response.text().await.unwrap_or_default();
            return Err(VoiceError::ApiError(error));
        }
        
        // Parse response
        let result: WhisperResponse = response.json().await
            .map_err(|e| VoiceError::ApiError(e.to_string()))?;
        
        Ok(TranscriptionResult {
            text: result.text,
            language: result.language,
            confidence: 0.9, // Whisper doesn't provide confidence
            duration_secs: result.duration,
            segments: result.segments.into_iter().map(|s| TranscriptionSegment {
                start: s.start,
                end: s.end,
                text: s.text,
            }).collect(),
        })
    }
    
    async fn transcribe_file(&self, path: &str) -> Result<TranscriptionResult, VoiceError> {
        let bytes = std::fs::read(path)?;
        let audio = wav_to_float(&bytes)?;
        self.transcribe(&audio).await
    }
    
    fn supported_languages(&self) -> Vec<&str> {
        vec!["en", "es", "fr", "de", "it", "pt", "zh", "ja", "ko", "ru", "tr", "ar"]
    }
}

/// Local Whisper (requires local-whisper feature)
#[cfg(feature = "local-whisper")]
pub struct LocalWhisper {
    model: whisper_rs::WhisperContext,
}

#[cfg(feature = "local-whisper")]
impl LocalWhisper {
    pub fn new(model_path: String) -> Self {
        let params = whisper_rs::WhisperContextParameters::default();
        let model = whisper_rs::WhisperContext::new_with_params(&model_path, params)
            .expect("Failed to load Whisper model");
        Self { model }
    }
}

/// Simulation mode Local Whisper (when native library not available)
#[cfg(not(feature = "local-whisper"))]
pub struct LocalWhisper {
    model_path: String,
}

#[cfg(not(feature = "local-whisper"))]
impl LocalWhisper {
    pub fn new(model_path: String) -> Self {
        log::info!("LocalWhisper simulation mode - no native library");
        log::info!("Model path: {}", model_path);
        Self { model_path }
    }
}

#[cfg(feature = "local-whisper")]
#[async_trait]
impl SpeechToText for LocalWhisper {
    async fn transcribe(&self, audio: &[f32]) -> Result<TranscriptionResult, VoiceError> {
        let mut state = self.model.create_state()
            .map_err(|e| VoiceError::Internal(e.to_string()))?;
        
        state.full(audio, whisper_rs::WhisperFullParams::default())
            .map_err(|e| VoiceError::Internal(e.to_string()))?;
        
        let num_segments = state.full_n_segments()
            .map_err(|e| VoiceError::Internal(e.to_string()))?;
        
        let mut text = String::new();
        let mut segments = Vec::new();
        
        for i in 0..num_segments {
            let segment_text = state.full_get_segment_text(i)
                .map_err(|e| VoiceError::Internal(e.to_string()))?;
            let start = state.full_get_segment_t0(i)
                .map_err(|e| VoiceError::Internal(e.to_string()))? as f32 / 100.0;
            let end = state.full_get_segment_t1(i)
                .map_err(|e| VoiceError::Internal(e.to_string()))? as f32 / 100.0;
            
            text.push_str(&segment_text);
            segments.push(TranscriptionSegment {
                start,
                end,
                text: segment_text,
            });
        }
        
        Ok(TranscriptionResult {
            text,
            language: "en".into(),
            confidence: 0.9,
            duration_secs: segments.last().map(|s| s.end).unwrap_or(0.0),
            segments,
        })
    }
    
    async fn transcribe_file(&self, path: &str) -> Result<TranscriptionResult, VoiceError> {
        let bytes = std::fs::read(path)?;
        let audio = wav_to_float(&bytes)?;
        self.transcribe(&audio).await
    }
    
    fn supported_languages(&self) -> Vec<&str> {
        vec!["en", "es", "fr", "de", "it", "pt", "zh", "ja", "ko", "ru", "tr", "ar"]
    }
}

/// ─── Helpers ───

fn float_to_wav(samples: &[f32], sample_rate: u32) -> Result<Vec<u8>, VoiceError> {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    
    let mut cursor = std::io::Cursor::new(Vec::new());
    {
        let mut writer = hound::WavWriter::new(&mut cursor, spec)
            .map_err(|e| VoiceError::AudioError(e.to_string()))?;
        
        for &sample in samples {
            let sample_i16 = (sample * 32767.0).clamp(-32768.0, 32767.0) as i16;
            writer.write_sample(sample_i16)
                .map_err(|e| VoiceError::AudioError(e.to_string()))?;
        }
        
        writer.finalize()
            .map_err(|e| VoiceError::AudioError(e.to_string()))?;
    }
    
    Ok(cursor.into_inner())
}

fn wav_to_float(bytes: &[u8]) -> Result<Vec<f32>, VoiceError> {
    let cursor = std::io::Cursor::new(bytes);
    let reader = hound::WavReader::new(cursor)
        .map_err(|e| VoiceError::AudioError(e.to_string()))?;
    
    let samples: Vec<f32> = reader.into_samples::<i16>()
        .filter_map(|s| s.ok())
        .map(|s| s as f32 / 32768.0)
        .collect();
    
    Ok(samples)
}

/// Whisper API response
#[derive(Debug, Deserialize)]
struct WhisperResponse {
    text: String,
    language: String,
    duration: f32,
    segments: Vec<WhisperSegment>,
}

#[derive(Debug, Deserialize)]
struct WhisperSegment {
    start: f32,
    end: f32,
    text: String,
}

/// Simulation mode implementation (when native library not available)
#[cfg(not(feature = "local-whisper"))]
#[async_trait]
impl SpeechToText for LocalWhisper {
    async fn transcribe(&self, audio: &[f32]) -> Result<TranscriptionResult, VoiceError> {
        log::debug!("LocalWhisper simulation mode - returning mock transcription");
        
        // Calculate audio properties for simulation
        let duration_secs = audio.len() as f32 / 16000.0;
        let energy: f32 = audio.iter().map(|s| s.abs()).sum::<f32>() / audio.len().max(1) as f32;
        
        // Simulate transcription based on audio energy
        let (text, confidence) = if energy < 0.01 {
            ("[silence]".to_string(), 0.5)
        } else if energy < 0.05 {
            ("[inaudible speech]".to_string(), 0.6)
        } else {
            (format!("[Simulated transcription - {} seconds of audio]", duration_secs as i32), 0.85)
        };
        
        Ok(TranscriptionResult {
            text,
            language: "en".into(),
            confidence,
            duration_secs,
            segments: vec![TranscriptionSegment {
                start: 0.0,
                end: duration_secs,
                text: "Simulated segment".into(),
            }],
        })
    }
    
    async fn transcribe_file(&self, path: &str) -> Result<TranscriptionResult, VoiceError> {
        log::debug!("LocalWhisper simulation mode - transcribing file: {}", path);
        let bytes = std::fs::read(path)?;
        let audio = wav_to_float(&bytes)?;
        self.transcribe(&audio).await
    }
    
    fn supported_languages(&self) -> Vec<&str> {
        vec!["en", "es", "fr", "de", "it", "pt", "zh", "ja", "ko", "ru", "tr", "ar"]
    }
}
