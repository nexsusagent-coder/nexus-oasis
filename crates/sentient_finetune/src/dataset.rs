//! ─── Dataset Handling ───

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ═══════════════════════════════════════════════════════════════════════════════
//  DATASET
// ═══════════════════════════════════════════════════════════════════════════════

/// Training dataset
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dataset {
    /// Dataset name
    pub name: String,
    /// Dataset description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Format
    pub format: DatasetFormat,
    /// Training samples
    pub samples: Vec<TrainingSample>,
    /// Validation samples (optional)
    #[serde(default)]
    pub validation_samples: Vec<TrainingSample>,
    /// Metadata
    #[serde(default)]
    pub metadata: HashMap<String, String>,
}

impl Dataset {
    /// Create new dataset
    pub fn new(name: impl Into<String>, format: DatasetFormat) -> Self {
        Self {
            name: name.into(),
            description: None,
            format,
            samples: Vec::new(),
            validation_samples: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    /// Add training sample
    pub fn add_sample(mut self, sample: TrainingSample) -> Self {
        self.samples.push(sample);
        self
    }

    /// Add multiple samples
    pub fn add_samples(mut self, samples: Vec<TrainingSample>) -> Self {
        self.samples.extend(samples);
        self
    }

    /// Set validation samples
    pub fn validation(mut self, samples: Vec<TrainingSample>) -> Self {
        self.validation_samples = samples;
        self
    }

    /// Split into train/validation
    pub fn split(mut self, validation_ratio: f32) -> Self {
        if !self.validation_samples.is_empty() {
            return self;
        }

        let total = self.samples.len();
        let val_count = (total as f32 * validation_ratio) as usize;
        
        if val_count > 0 && total > val_count {
            self.validation_samples = self.samples.drain(total - val_count..).collect();
        }
        
        self
    }

    /// Get total token count (approximate)
    pub fn token_count(&self) -> usize {
        self.samples.iter()
            .map(|s| s.approximate_tokens())
            .sum()
    }

    /// Load from JSONL file
    pub fn from_jsonl(content: &str) -> Result<Self, DatasetError> {
        let samples: Vec<TrainingSample> = content
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|line| serde_json::from_str(line))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| DatasetError::ParseError(e.to_string()))?;

        Ok(Self {
            name: "loaded_dataset".into(),
            description: None,
            format: DatasetFormat::Jsonl,
            samples,
            validation_samples: Vec::new(),
            metadata: HashMap::new(),
        })
    }

    /// Export to JSONL format
    pub fn to_jsonl(&self) -> Result<String, DatasetError> {
        let lines: Vec<String> = self.samples
            .iter()
            .map(|s| serde_json::to_string(s))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| DatasetError::SerializeError(e.to_string()))?;

        Ok(lines.join("\n"))
    }

    /// Validate dataset
    pub fn validate(&self) -> Result<(), DatasetError> {
        if self.samples.is_empty() {
            return Err(DatasetError::EmptyDataset);
        }

        for (i, sample) in self.samples.iter().enumerate() {
            if sample.is_empty() {
                return Err(DatasetError::EmptySample(i));
            }
        }

        Ok(())
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  TRAINING SAMPLE
// ═══════════════════════════════════════════════════════════════════════════════

/// Single training sample
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingSample {
    /// Input prompt
    #[serde(default)]
    pub input: String,
    
    /// Expected output
    #[serde(default)]
    pub output: String,
    
    /// System message (for chat models)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,
    
    /// Messages (for conversation format)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<Message>>,
    
    /// Custom fields
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

impl TrainingSample {
    /// Create new sample
    pub fn new(input: impl Into<String>, output: impl Into<String>) -> Self {
        Self {
            input: input.into(),
            output: output.into(),
            system: None,
            messages: None,
            extra: HashMap::new(),
        }
    }

    /// Create chat sample
    pub fn chat(messages: Vec<Message>) -> Self {
        Self {
            input: String::new(),
            output: String::new(),
            system: None,
            messages: Some(messages),
            extra: HashMap::new(),
        }
    }

    /// Set system message
    pub fn system(mut self, system: impl Into<String>) -> Self {
        self.system = Some(system.into());
        self
    }

    /// Check if sample is empty
    pub fn is_empty(&self) -> bool {
        self.input.is_empty() 
            && self.output.is_empty() 
            && self.messages.as_ref().map_or(true, |m| m.is_empty())
    }

    /// Approximate token count
    pub fn approximate_tokens(&self) -> usize {
        let mut count = 0;
        
        count += self.input.len() / 4; // Rough estimate
        count += self.output.len() / 4;
        
        if let Some(ref system) = self.system {
            count += system.len() / 4;
        }
        
        if let Some(ref messages) = self.messages {
            for msg in messages {
                count += msg.content.len() / 4;
            }
        }
        
        count.max(1)
    }

    /// Format for specific provider
    pub fn format_for(&self, format: DatasetFormat) -> String {
        match format {
            DatasetFormat::Jsonl => {
                serde_json::to_string(self).unwrap_or_default()
            }
            DatasetFormat::Alpaca => {
                format!(
                    "### Instruction:\n{}\n\n### Response:\n{}",
                    self.input, self.output
                )
            }
            DatasetFormat::OpenAI => {
                if let Some(ref messages) = self.messages {
                    serde_json::to_string(&serde_json::json!({
                        "messages": messages
                    })).unwrap_or_default()
                } else {
                    serde_json::to_string(&serde_json::json!({
                        "messages": [
                            {"role": "user", "content": self.input},
                            {"role": "assistant", "content": self.output}
                        ]
                    })).unwrap_or_default()
                }
            }
            DatasetFormat::ShareGPT => {
                if let Some(ref messages) = self.messages {
                    serde_json::to_string(&serde_json::json!({
                        "conversations": messages
                    })).unwrap_or_default()
                } else {
                    serde_json::to_string(&serde_json::json!({
                        "conversations": [
                            {"from": "human", "value": self.input},
                            {"from": "gpt", "value": self.output}
                        ]
                    })).unwrap_or_default()
                }
            }
            _ => format!("{} -> {}", self.input, self.output),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  MESSAGE
// ═══════════════════════════════════════════════════════════════════════════════

/// Chat message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    /// Role (system, user, assistant)
    pub role: String,
    /// Content
    pub content: String,
}

impl Message {
    /// Create system message
    pub fn system(content: impl Into<String>) -> Self {
        Self {
            role: "system".into(),
            content: content.into(),
        }
    }

    /// Create user message
    pub fn user(content: impl Into<String>) -> Self {
        Self {
            role: "user".into(),
            content: content.into(),
        }
    }

    /// Create assistant message
    pub fn assistant(content: impl Into<String>) -> Self {
        Self {
            role: "assistant".into(),
            content: content.into(),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  DATASET FORMAT
// ═══════════════════════════════════════════════════════════════════════════════

/// Dataset format
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DatasetFormat {
    #[serde(rename = "jsonl")]
    Jsonl,
    #[serde(rename = "alpaca")]
    Alpaca,
    #[serde(rename = "openai")]
    OpenAI,
    #[serde(rename = "sharegpt")]
    ShareGPT,
    #[serde(rename = "custom")]
    Custom,
}

impl Default for DatasetFormat {
    fn default() -> Self {
        Self::Jsonl
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  DATASET SPLITTER
// ═══════════════════════════════════════════════════════════════════════════════

/// Dataset splitting utilities
pub struct DatasetSplitter;

impl DatasetSplitter {
    /// Split dataset into train/validation/test
    pub fn split_three(
        dataset: Dataset,
        val_ratio: f32,
        test_ratio: f32,
    ) -> (Dataset, Dataset, Dataset) {
        let total = dataset.samples.len();
        let test_count = (total as f32 * test_ratio) as usize;
        let val_count = (total as f32 * val_ratio) as usize;
        let train_count = total - test_count - val_count;

        let mut samples = dataset.samples.into_iter();
        
        let train_samples: Vec<_> = samples.by_ref().take(train_count).collect();
        let val_samples: Vec<_> = samples.by_ref().take(val_count).collect();
        let test_samples: Vec<_> = samples.collect();

        let train = Dataset::new(format!("{}_train", dataset.name), dataset.format.clone())
            .add_samples(train_samples);
        
        let val = Dataset::new(format!("{}_val", dataset.name), dataset.format.clone())
            .add_samples(val_samples);
        
        let test = Dataset::new(format!("{}_test", dataset.name), dataset.format)
            .add_samples(test_samples);

        (train, val, test)
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  DATASET ERROR
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DatasetError {
    ParseError(String),
    SerializeError(String),
    EmptyDataset,
    EmptySample(usize),
    InvalidFormat(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_training_sample() {
        let sample = TrainingSample::new("What is 2+2?", "4");
        assert_eq!(sample.input, "What is 2+2?");
        assert_eq!(sample.output, "4");
    }

    #[test]
    fn test_message_helpers() {
        let system = Message::system("You are helpful");
        assert_eq!(system.role, "system");
        
        let user = Message::user("Hello");
        assert_eq!(user.role, "user");
        
        let assistant = Message::assistant("Hi there!");
        assert_eq!(assistant.role, "assistant");
    }

    #[test]
    fn test_dataset_creation() {
        let dataset = Dataset::new("test", DatasetFormat::Jsonl)
            .add_sample(TrainingSample::new("Q1", "A1"))
            .add_sample(TrainingSample::new("Q2", "A2"));

        // After split with 0.5, we get 1 train and 1 validation
        let dataset = dataset.split(0.5);

        // At least 1 sample in each after split
        assert!(!dataset.samples.is_empty());
        assert!(!dataset.validation_samples.is_empty());
    }

    #[test]
    fn test_dataset_validation() {
        let valid = Dataset::new("test", DatasetFormat::Jsonl)
            .add_sample(TrainingSample::new("Q", "A"));
        
        assert!(valid.validate().is_ok());

        let empty = Dataset::new("empty", DatasetFormat::Jsonl);
        assert!(empty.validate().is_err());
    }

    #[test]
    fn test_jsonl_roundtrip() {
        let original = Dataset::new("test", DatasetFormat::Jsonl)
            .add_sample(TrainingSample::new("Q1", "A1"))
            .add_sample(TrainingSample::new("Q2", "A2"));

        let jsonl = original.to_jsonl().unwrap();
        let loaded = Dataset::from_jsonl(&jsonl).unwrap();

        assert_eq!(loaded.samples.len(), 2);
    }

    #[test]
    fn test_format_alpaca() {
        let sample = TrainingSample::new("What is AI?", "AI is...");
        let formatted = sample.format_for(DatasetFormat::Alpaca);
        
        assert!(formatted.contains("### Instruction:"));
        assert!(formatted.contains("### Response:"));
    }

    #[test]
    fn test_format_openai() {
        let sample = TrainingSample::new("Hello", "Hi!");
        let formatted = sample.format_for(DatasetFormat::OpenAI);
        
        assert!(formatted.contains("\"role\":"));
        assert!(formatted.contains("\"content\":"));
    }
}
