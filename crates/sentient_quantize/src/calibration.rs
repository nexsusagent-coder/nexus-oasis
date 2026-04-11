//! ─── Calibration Module ───
//!
//! Calibration data handling for GPTQ/AWQ

use serde::{Deserialize, Serialize};

// ═══════════════════════════════════════════════════════════════════════════════
//  CALIBRATION DATA
// ═══════════════════════════════════════════════════════════════════════════════

/// Calibration dataset
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalibrationData {
    /// Dataset name
    pub name: String,
    
    /// Samples
    pub samples: Vec<CalibrationSample>,
    
    /// Tokenizer info
    pub tokenizer: String,
    
    /// Max length
    pub max_length: usize,
    
    /// Statistics
    pub stats: CalibrationStats,
}

impl CalibrationData {
    /// Create new calibration data
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            samples: Vec::new(),
            tokenizer: "unknown".into(),
            max_length: 2048,
            stats: CalibrationStats::default(),
        }
    }

    /// Add sample
    pub fn add_sample(&mut self, sample: CalibrationSample) {
        self.samples.push(sample);
        self.update_stats();
    }

    /// Add text samples
    pub fn add_texts(&mut self, texts: &[&str]) {
        for text in texts {
            self.samples.push(CalibrationSample::from_text(text));
        }
        self.update_stats();
    }

    /// Load from dataset
    pub async fn load(dataset_name: &str, num_samples: usize) -> Result<Self, CalibrationError> {
        log::info!("📊 Loading calibration data from {} ({} samples)", dataset_name, num_samples);

        // In a real implementation, load from HuggingFace datasets
        let mut data = Self::new(dataset_name);
        
        // Simulate loading
        for i in 0..num_samples {
            data.samples.push(CalibrationSample {
                id: format!("sample-{}", i),
                text: format!("Sample calibration text {}", i),
                tokens: vec![1, 2, 3, 4, 5],
                token_count: 5,
                source: dataset_name.into(),
            });
        }
        
        data.update_stats();
        Ok(data)
    }

    /// Update statistics
    fn update_stats(&mut self) {
        if self.samples.is_empty() {
            return;
        }

        let total_tokens: usize = self.samples.iter().map(|s| s.token_count).sum();
        let avg_tokens = total_tokens / self.samples.len();
        
        let min_tokens = self.samples.iter().map(|s| s.token_count).min().unwrap_or(0);
        let max_tokens = self.samples.iter().map(|s| s.token_count).max().unwrap_or(0);

        self.stats = CalibrationStats {
            num_samples: self.samples.len(),
            total_tokens,
            avg_tokens,
            min_tokens,
            max_tokens,
        };
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  CALIBRATION SAMPLE
// ═══════════════════════════════════════════════════════════════════════════════

/// Single calibration sample
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalibrationSample {
    /// Sample ID
    pub id: String,
    
    /// Text content
    pub text: String,
    
    /// Token IDs
    pub tokens: Vec<u32>,
    
    /// Token count
    pub token_count: usize,
    
    /// Source dataset
    pub source: String,
}

impl CalibrationSample {
    /// Create from text
    pub fn from_text(text: &str) -> Self {
        // Rough token estimation
        let token_count = text.split_whitespace().count();
        
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            text: text.into(),
            tokens: vec![1; token_count], // Placeholder
            token_count,
            source: "custom".into(),
        }
    }

    /// Create from tokens
    pub fn from_tokens(tokens: Vec<u32>, text: impl Into<String>) -> Self {
        let token_count = tokens.len();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            text: text.into(),
            tokens,
            token_count,
            source: "custom".into(),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  CALIBRATION STATS
// ═══════════════════════════════════════════════════════════════════════════════

/// Calibration statistics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CalibrationStats {
    pub num_samples: usize,
    pub total_tokens: usize,
    pub avg_tokens: usize,
    pub min_tokens: usize,
    pub max_tokens: usize,
}

// ═══════════════════════════════════════════════════════════════════════════════
//  CALIBRATION ERROR
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone)]
pub enum CalibrationError {
    DatasetNotFound(String),
    TokenizationError(String),
    InsufficientData,
}

// ═══════════════════════════════════════════════════════════════════════════════
//  STANDARD DATASETS
// ═══════════════════════════════════════════════════════════════════════════════

pub struct StandardCalibrationDatasets;

impl StandardCalibrationDatasets {
    /// Get recommended dataset for model
    pub fn for_model(model_id: &str) -> CalibrationDatasetInfo {
        let model_lower = model_id.to_lowercase();
        
        // Check code first (before llama, since CodeLlama contains both)
        if model_lower.contains("code") {
            CalibrationDatasetInfo {
                name: "codeparrot/codeparrot-clean".into(),
                description: "Code dataset for code models".into(),
                recommended_samples: 256,
                max_length: 1024,
            }
        } else if model_lower.contains("llama") || model_lower.contains("mistral") {
            CalibrationDatasetInfo {
                name: "wikitext".into(),
                description: "WikiText-2 dataset, good for language models".into(),
                recommended_samples: 512,
                max_length: 2048,
            }
        } else {
            CalibrationDatasetInfo {
                name: "c4".into(),
                description: "Colossal Clean Crawled Corpus".into(),
                recommended_samples: 512,
                max_length: 2048,
            }
        }
    }

    /// List all standard datasets
    pub fn all() -> Vec<CalibrationDatasetInfo> {
        vec![
            CalibrationDatasetInfo {
                name: "wikitext".into(),
                description: "WikiText-2, standard for LLM calibration".into(),
                recommended_samples: 512,
                max_length: 2048,
            },
            CalibrationDatasetInfo {
                name: "pileval".into(),
                description: "Pile validation set".into(),
                recommended_samples: 512,
                max_length: 2048,
            },
            CalibrationDatasetInfo {
                name: "c4".into(),
                description: "Colossal Clean Crawled Corpus".into(),
                recommended_samples: 1024,
                max_length: 2048,
            },
            CalibrationDatasetInfo {
                name: "ptb".into(),
                description: "Penn Treebank".into(),
                recommended_samples: 256,
                max_length: 512,
            },
        ]
    }
}

/// Calibration dataset info
#[derive(Debug, Clone)]
pub struct CalibrationDatasetInfo {
    pub name: String,
    pub description: String,
    pub recommended_samples: usize,
    pub max_length: usize,
}

// ═══════════════════════════════════════════════════════════════════════════════
//  TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calibration_data() {
        let mut data = CalibrationData::new("test");
        data.add_sample(CalibrationSample::from_text("Hello world"));
        
        assert_eq!(data.samples.len(), 1);
        assert!(data.stats.num_samples > 0);
    }

    #[test]
    fn test_add_texts() {
        let mut data = CalibrationData::new("test");
        data.add_texts(&["Hello", "World", "Test"]);
        
        assert_eq!(data.samples.len(), 3);
    }

    #[test]
    fn test_sample_from_text() {
        let sample = CalibrationSample::from_text("Hello world this is a test");
        assert!(sample.token_count > 0);
    }

    #[test]
    fn test_standard_datasets() {
        let datasets = StandardCalibrationDatasets::all();
        assert!(!datasets.is_empty());
    }

    #[test]
    fn test_dataset_for_model() {
        let llama = StandardCalibrationDatasets::for_model("meta-llama/Llama-2-7b");
        assert!(llama.name.contains("wiki"));

        let code = StandardCalibrationDatasets::for_model("codellama/CodeLlama-7b");
        assert!(code.name.contains("code"));
    }

    #[tokio::test]
    async fn test_load_calibration() {
        let data = CalibrationData::load("wikitext", 10).await.unwrap();
        assert_eq!(data.samples.len(), 10);
    }
}
