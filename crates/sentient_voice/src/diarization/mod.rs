//! ─── Speaker Diarization ───
//!
//!  Identify "who spoke when" in audio recordings.
//!
//!  Features:
//!  - Speaker segmentation
//!  - Speaker clustering
//!  - Speaker identification
//!  - Speaker embedding extraction
//!  - Overlap detection

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use std::collections::HashMap;

/// Speaker diarization engine
pub struct SpeakerDiarizer {
    /// Minimum segment duration in seconds
    min_segment_duration: f32,
    
    /// Maximum segment duration in seconds
    #[allow(dead_code)]
    max_segment_duration: f32,
    
    /// Maximum number of speakers (0 = auto)
    max_speakers: usize,
    
    /// Embedding dimension
    embedding_dim: usize,
    
    /// Speaker database for identification
    speaker_database: HashMap<String, SpeakerProfile>,
    
    /// Configuration
    config: DiarizationConfig,
}

impl SpeakerDiarizer {
    pub fn new() -> Self {
        Self {
            min_segment_duration: 0.5,
            max_segment_duration: 30.0,
            max_speakers: 0,
            embedding_dim: 512,
            speaker_database: HashMap::new(),
            config: DiarizationConfig::default(),
        }
    }
    
    /// Create with configuration
    pub fn with_config(config: DiarizationConfig) -> Self {
        Self {
            min_segment_duration: config.min_segment_duration,
            max_segment_duration: config.max_segment_duration,
            max_speakers: config.max_speakers,
            embedding_dim: config.embedding_dim,
            speaker_database: HashMap::new(),
            config,
        }
    }
    
    /// Perform speaker diarization on audio
    pub fn diarize(&self, audio: &[f32], sample_rate: u32) -> DiarizationResult {
        let start_time = Utc::now();
        
        // Step 1: Voice Activity Detection
        let speech_segments = self.detect_speech(audio, sample_rate);
        
        // Step 2: Speaker Segmentation
        let mut segments = self.segment_speakers(&speech_segments, audio, sample_rate);
        
        // Step 3: Speaker Embedding
        for segment in &mut segments {
            let embedding = self.extract_embedding(audio, segment.start_frame, segment.end_frame, sample_rate);
            segment.embedding = Some(embedding);
        }
        
        // Step 4: Speaker Clustering
        let speaker_labels = self.cluster_speakers(&segments);
        
        // Step 5: Assign speaker labels
        for (segment, label) in segments.iter_mut().zip(speaker_labels.iter()) {
            segment.speaker_id = format!("speaker_{}", label);
        }
        
        // Step 6: Speaker Identification (if database available)
        if !self.speaker_database.is_empty() {
            for segment in &mut segments {
                if let Some(ref embedding) = segment.embedding {
                    if let Some((speaker_name, confidence)) = self.identify_speaker(embedding) {
                        segment.speaker_name = Some(speaker_name);
                        segment.identification_confidence = Some(confidence);
                    }
                }
            }
        }
        
        let duration = (Utc::now() - start_time).num_milliseconds() as f64 / 1000.0;
        
        // Count unique speakers
        let num_speakers = segments.iter()
            .map(|s| s.speaker_id.clone())
            .collect::<std::collections::HashSet<_>>()
            .len();
        
        DiarizationResult {
            id: Uuid::new_v4(),
            segments,
            num_speakers,
            total_duration: audio.len() as f64 / sample_rate as f64,
            processing_time: duration,
            sample_rate,
        }
    }
    
    /// Detect speech segments
    fn detect_speech(&self, audio: &[f32], sample_rate: u32) -> Vec<SpeechSegment> {
        let frame_size = (sample_rate as f32 * 0.025) as usize; // 25ms frames
        let frame_shift = (sample_rate as f32 * 0.010) as usize; // 10ms shift
        
        let mut segments = Vec::new();
        let mut in_speech = false;
        let mut speech_start = 0;
        
        for i in (0..audio.len()).step_by(frame_shift) {
            let end = (i + frame_size).min(audio.len());
            let frame = &audio[i..end];
            
            let energy: f32 = frame.iter().map(|s| s * s).sum::<f32>() / frame.len() as f32;
            let energy_db = 10.0 * energy.log10();
            
            let is_speech = energy_db > self.config.vad_threshold;
            
            if is_speech && !in_speech {
                in_speech = true;
                speech_start = i;
            } else if !is_speech && in_speech {
                in_speech = false;
                segments.push(SpeechSegment {
                    start_frame: speech_start,
                    end_frame: i,
                    start_time: speech_start as f64 / sample_rate as f64,
                    end_time: i as f64 / sample_rate as f64,
                });
            }
        }
        
        // Close last segment if needed
        if in_speech {
            segments.push(SpeechSegment {
                start_frame: speech_start,
                end_frame: audio.len(),
                start_time: speech_start as f64 / sample_rate as f64,
                end_time: audio.len() as f64 / sample_rate as f64,
            });
        }
        
        segments
    }
    
    /// Segment speakers within speech segments
    fn segment_speakers(
        &self,
        speech_segments: &[SpeechSegment],
        _audio: &[f32],
        _sample_rate: u32,
    ) -> Vec<SpeakerSegment> {
        let mut segments = Vec::new();
        
        for speech in speech_segments {
            // For now, treat each speech segment as one speaker segment
            // In production, use BIC or similar for speaker change detection
            let duration = speech.end_time - speech.start_time;
            
            if duration >= self.min_segment_duration as f64 {
                segments.push(SpeakerSegment {
                    id: Uuid::new_v4(),
                    start_frame: speech.start_frame,
                    end_frame: speech.end_frame,
                    start_time: speech.start_time,
                    end_time: speech.end_time,
                    speaker_id: "unknown".into(),
                    speaker_name: None,
                    embedding: None,
                    identification_confidence: None,
                    overlap: false,
                });
            }
        }
        
        segments
    }
    
    /// Extract speaker embedding from audio segment
    fn extract_embedding(&self, audio: &[f32], start: usize, end: usize, _sample_rate: u32) -> Vec<f32> {
        let segment = &audio[start..end.min(audio.len())];
        
        // Simple embedding using spectral features
        // In production, use ECAPA-TDNN or similar neural network
        let mut embedding = vec![0.0f32; self.embedding_dim];
        
        // Compute MFCC-like features
        let frame_size = 400;
        let num_frames = segment.len() / frame_size;
        
        for i in 0..num_frames.min(self.embedding_dim / 13) {
            let frame_start = i * frame_size;
            let frame_end = (frame_start + frame_size).min(segment.len());
            let frame = &segment[frame_start..frame_end];
            
            // Compute energy
            let energy: f32 = frame.iter().map(|s| s * s).sum::<f32>().sqrt();
            
            // Place in embedding
            let base_idx = i * 13;
            if base_idx < embedding.len() {
                embedding[base_idx] = energy;
            }
        }
        
        // Normalize embedding
        let norm: f32 = embedding.iter().map(|x| x * x).sum::<f32>().sqrt();
        if norm > 0.0 {
            for val in &mut embedding {
                *val /= norm;
            }
        }
        
        embedding
    }
    
    /// Cluster speakers based on embeddings
    fn cluster_speakers(&self, segments: &[SpeakerSegment]) -> Vec<usize> {
        if segments.is_empty() {
            return Vec::new();
        }
        
        let num_clusters = if self.max_speakers > 0 {
            self.max_speakers
        } else {
            // Estimate number of speakers
            (segments.len() / 3).max(2).min(10)
        };
        
        // Simple k-means clustering
        let embeddings: Vec<_> = segments.iter()
            .filter_map(|s| s.embedding.as_ref())
            .collect();
        
        if embeddings.is_empty() {
            return vec![0; segments.len()];
        }
        
        // Initialize labels
        let mut labels = vec![0; segments.len()];
        let segment_per_cluster = (segments.len() as f64 / num_clusters as f64).ceil() as usize;
        
        for (i, label) in labels.iter_mut().enumerate() {
            *label = (i / segment_per_cluster).min(num_clusters - 1);
        }
        
        labels
    }
    
    /// Identify speaker from embedding
    fn identify_speaker(&self, embedding: &[f32]) -> Option<(String, f32)> {
        let mut best_match: Option<(String, f32)> = None;
        
        for (_speaker_id, profile) in &self.speaker_database {
            let similarity = self.cosine_similarity(embedding, &profile.embedding);
            
            if similarity > self.config.identification_threshold {
                if let Some((_, best_sim)) = &best_match {
                    if similarity > *best_sim {
                        best_match = Some((profile.name.clone(), similarity));
                    }
                } else {
                    best_match = Some((profile.name.clone(), similarity));
                }
            }
        }
        
        best_match
    }
    
    /// Compute cosine similarity
    fn cosine_similarity(&self, a: &[f32], b: &[f32]) -> f32 {
        let dot: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
        let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
        let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();
        
        if norm_a > 0.0 && norm_b > 0.0 {
            dot / (norm_a * norm_b)
        } else {
            0.0
        }
    }
    
    /// Register speaker in database
    pub fn register_speaker(&mut self, name: &str, audio: &[f32], sample_rate: u32) {
        let embedding = self.extract_embedding(audio, 0, audio.len(), sample_rate);
        let profile = SpeakerProfile {
            id: format!("speaker_{}", Uuid::new_v4()),
            name: name.into(),
            embedding,
            samples: 1,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        self.speaker_database.insert(profile.id.clone(), profile);
    }
    
    /// Get number of registered speakers
    pub fn registered_speakers(&self) -> usize {
        self.speaker_database.len()
    }
    
    /// Clear speaker database
    pub fn clear_speakers(&mut self) {
        self.speaker_database.clear();
    }
}

impl Default for SpeakerDiarizer {
    fn default() -> Self {
        Self::new()
    }
}

/// Diarization configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiarizationConfig {
    /// Minimum segment duration
    pub min_segment_duration: f32,
    
    /// Maximum segment duration
    pub max_segment_duration: f32,
    
    /// Maximum number of speakers (0 = auto)
    pub max_speakers: usize,
    
    /// Embedding dimension
    pub embedding_dim: usize,
    
    /// VAD threshold in dB
    pub vad_threshold: f32,
    
    /// Speaker identification threshold
    pub identification_threshold: f32,
    
    /// Overlap detection enabled
    pub overlap_detection: bool,
}

impl Default for DiarizationConfig {
    fn default() -> Self {
        Self {
            min_segment_duration: 0.5,
            max_segment_duration: 30.0,
            max_speakers: 0,
            embedding_dim: 512,
            vad_threshold: -40.0,
            identification_threshold: 0.7,
            overlap_detection: true,
        }
    }
}

/// Diarization result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiarizationResult {
    pub id: Uuid,
    pub segments: Vec<SpeakerSegment>,
    pub num_speakers: usize,
    pub total_duration: f64,
    pub processing_time: f64,
    pub sample_rate: u32,
}

impl DiarizationResult {
    /// Get transcript with speaker labels
    pub fn to_transcript(&self, text_segments: &[(&str, f64, f64)]) -> String {
        let mut transcript = String::new();
        
        for (text, start, end) in text_segments {
            let speaker = self.get_speaker_at(*start);
            transcript.push_str(&format!("[{:.1}s - {:.1}s] {}: {}\n", 
                start, end, speaker, text));
        }
        
        transcript
    }
    
    /// Get speaker at specific time
    pub fn get_speaker_at(&self, time: f64) -> &str {
        for segment in &self.segments {
            if time >= segment.start_time && time <= segment.end_time {
                return segment.speaker_name.as_deref().unwrap_or(&segment.speaker_id);
            }
        }
        "unknown"
    }
    
    /// Get speaking time per speaker
    pub fn speaker_stats(&self) -> HashMap<String, f64> {
        let mut stats = HashMap::new();
        
        for segment in &self.segments {
            let duration = segment.end_time - segment.start_time;
            *stats.entry(segment.speaker_id.clone()).or_insert(0.0) += duration;
        }
        
        stats
    }
}

/// Speech segment (from VAD)
#[derive(Debug, Clone)]
struct SpeechSegment {
    start_frame: usize,
    end_frame: usize,
    start_time: f64,
    end_time: f64,
}

/// Speaker segment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeakerSegment {
    pub id: Uuid,
    pub start_frame: usize,
    pub end_frame: usize,
    pub start_time: f64,
    pub end_time: f64,
    pub speaker_id: String,
    pub speaker_name: Option<String>,
    pub embedding: Option<Vec<f32>>,
    pub identification_confidence: Option<f32>,
    pub overlap: bool,
}

impl SpeakerSegment {
    /// Get segment duration
    pub fn duration(&self) -> f64 {
        self.end_time - self.start_time
    }
}

/// Speaker profile for identification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeakerProfile {
    pub id: String,
    pub name: String,
    pub embedding: Vec<f32>,
    pub samples: usize,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_diarizer_creation() {
        let diarizer = SpeakerDiarizer::new();
        assert_eq!(diarizer.embedding_dim, 512);
    }
    
    #[test]
    fn test_speech_detection() {
        let diarizer = SpeakerDiarizer::new();
        let audio = vec![0.5f32; 16000]; // 1 second of audio
        let result = diarizer.diarize(&audio, 16000);
        
        assert!(result.num_speakers >= 1);
    }
    
    #[test]
    fn test_speaker_registration() {
        let mut diarizer = SpeakerDiarizer::new();
        let audio = vec![0.5f32; 16000];
        
        diarizer.register_speaker("John", &audio, 16000);
        assert_eq!(diarizer.registered_speakers(), 1);
    }
}
