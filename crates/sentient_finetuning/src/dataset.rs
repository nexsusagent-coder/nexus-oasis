//! Dataset handling for fine-tuning

use crate::types::*;
use crate::{FinetuningError, Result};
use serde_json::Value;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Dataset format
#[derive(Debug, Clone, Copy)]
pub enum DatasetFormat {
    /// JSON array format
    JsonArray,
    /// JSONL (one JSON per line)
    Jsonl,
    /// CSV format
    Csv,
    /// Parquet format
    Parquet,
    /// Conversation format (ShareGPT-style)
    Conversation,
}

impl DatasetFormat {
    /// Detect format from file extension
    pub fn from_path(path: &Path) -> Self {
        match path.extension().and_then(|e| e.to_str()) {
            Some("json") => DatasetFormat::JsonArray,
            Some("jsonl") => DatasetFormat::Jsonl,
            Some("csv") => DatasetFormat::Csv,
            Some("parquet") => DatasetFormat::Parquet,
            _ => DatasetFormat::Jsonl, // Default
        }
    }
}

/// Dataset loader
pub struct DatasetLoader {
    /// Input path
    path: PathBuf,
    /// Format
    format: DatasetFormat,
    /// Input field name
    input_field: String,
    /// Output field name
    output_field: String,
    /// Max samples to load (None = all)
    max_samples: Option<usize>,
    /// Shuffle samples
    shuffle: bool,
}

impl DatasetLoader {
    /// Create new loader for path
    pub fn new(path: impl AsRef<Path>) -> Self {
        let path = path.as_ref().to_path_buf();
        let format = DatasetFormat::from_path(&path);

        Self {
            path,
            format,
            input_field: "input".to_string(),
            output_field: "output".to_string(),
            max_samples: None,
            shuffle: false,
        }
    }

    /// Set format explicitly
    pub fn with_format(mut self, format: DatasetFormat) -> Self {
        self.format = format;
        self
    }

    /// Set input field name
    pub fn with_input_field(mut self, field: impl Into<String>) -> Self {
        self.input_field = field.into();
        self
    }

    /// Set output field name
    pub fn with_output_field(mut self, field: impl Into<String>) -> Self {
        self.output_field = field.into();
        self
    }

    /// Set max samples
    pub fn with_max_samples(mut self, max: usize) -> Self {
        self.max_samples = Some(max);
        self
    }

    /// Enable shuffling
    pub fn with_shuffle(mut self, shuffle: bool) -> Self {
        self.shuffle = shuffle;
        self
    }

    /// Load dataset
    pub fn load(&self) -> Result<Dataset> {
        if !self.path.exists() {
            return Err(FinetuningError::DatasetNotFound(self.path.clone()));
        }

        let name = self.path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("dataset")
            .to_string();

        let mut dataset = Dataset::new(name);

        let samples = match self.format {
            DatasetFormat::JsonArray => self.load_json_array()?,
            DatasetFormat::Jsonl => self.load_jsonl()?,
            DatasetFormat::Csv => self.load_csv()?,
            DatasetFormat::Conversation => self.load_conversation()?,
            DatasetFormat::Parquet => {
                return Err(FinetuningError::Unsupported(
                    "Parquet format requires 'polars' feature".to_string(),
                ));
            }
        };

        dataset.add_samples(samples);

        if self.shuffle {
            shuffle_samples(&mut dataset.samples);
        }

        if let Some(max) = self.max_samples {
            dataset.samples.truncate(max);
        }

        Ok(dataset)
    }

    /// Load from JSON array
    fn load_json_array(&self) -> Result<Vec<TrainingSample>> {
        let content = std::fs::read_to_string(&self.path)?;
        let json: Vec<Value> = serde_json::from_str(&content)?;

        json.into_iter()
            .enumerate()
            .map(|(i, v)| self.parse_sample(&v, i))
            .collect()
    }

    /// Load from JSONL
    fn load_jsonl(&self) -> Result<Vec<TrainingSample>> {
        let file = File::open(&self.path)?;
        let reader = BufReader::new(file);

        reader
            .lines()
            .enumerate()
            .map(|(i, line)| {
                let line = line?;
                let json: Value = serde_json::from_str(&line)?;
                self.parse_sample(&json, i)
            })
            .collect()
    }

    /// Load from CSV
    fn load_csv(&self) -> Result<Vec<TrainingSample>> {
        // Simple CSV parsing without polars
        let file = File::open(&self.path)?;
        let reader = BufReader::new(file);

        let mut samples = Vec::new();

        for (i, line) in reader.lines().enumerate() {
            if i == 0 {
                // Skip header
                continue;
            }

            let line = line?;
            let parts: Vec<&str> = line.splitn(3, ',').collect();

            if parts.len() >= 3 {
                // Remove quotes if present
                let input = parts[1].trim_matches('"').to_string();
                let output = parts[2].trim_matches('"').to_string();

                samples.push(
                    TrainingSample::new(input, output)
                        .with_id(format!("sample-{}", i))
                );
            }
        }

        Ok(samples)
    }

    /// Load conversation format (ShareGPT-style)
    fn load_conversation(&self) -> Result<Vec<TrainingSample>> {
        let content = std::fs::read_to_string(&self.path)?;

        // Try JSON array first
        let conversations: Vec<Value> = serde_json::from_str(&content)?;

        let mut samples = Vec::new();

        for (conv_idx, conv) in conversations.iter().enumerate() {
            if let Some(messages) = conv.get("conversations").and_then(|m| m.as_array()) {
                // Extract user-assistant pairs
                let mut user_msg: Option<String> = None;

                for msg in messages {
                    let role = msg.get("from").and_then(|r| r.as_str()).unwrap_or("");
                    let content = msg.get("value").and_then(|v| v.as_str()).unwrap_or("");

                    match role {
                        "human" | "user" => {
                            user_msg = Some(content.to_string());
                        }
                        "gpt" | "assistant" => {
                            if let Some(input) = user_msg.take() {
                                samples.push(
                                    TrainingSample::new(input, content)
                                        .with_id(format!("conv-{}-{}", conv_idx, samples.len()))
                                );
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        Ok(samples)
    }

    /// Parse sample from JSON value
    fn parse_sample(&self, value: &Value, index: usize) -> Result<TrainingSample> {
        let input = value.get(&self.input_field)
            .and_then(|v| v.as_str())
            .ok_or_else(|| FinetuningError::InvalidDatasetFormat(
                format!("Missing field: {}", self.input_field),
                "json".to_string(),
            ))?;

        let output = value.get(&self.output_field)
            .and_then(|v| v.as_str())
            .ok_or_else(|| FinetuningError::InvalidDatasetFormat(
                format!("Missing field: {}", self.output_field),
                "json".to_string(),
            ))?;

        let mut sample = TrainingSample::new(input, output)
            .with_id(format!("sample-{}", index));

        // Add metadata
        if let Some(obj) = value.as_object() {
            for (key, val) in obj {
                if key != &self.input_field && key != &self.output_field {
                    if let Some(s) = val.as_str() {
                        sample.metadata.insert(key.clone(), s.to_string());
                    }
                }
            }
        }

        Ok(sample)
    }
}

/// Dataset saver
pub struct DatasetSaver {
    /// Output path
    path: PathBuf,
    /// Format
    format: DatasetFormat,
}

impl DatasetSaver {
    /// Create new saver
    pub fn new(path: impl AsRef<Path>) -> Self {
        let path = path.as_ref().to_path_buf();
        let format = DatasetFormat::from_path(&path);

        Self { path, format }
    }

    /// Set format
    pub fn with_format(mut self, format: DatasetFormat) -> Self {
        self.format = format;
        self
    }

    /// Save dataset
    pub fn save(&self, dataset: &Dataset) -> Result<()> {
        // Create parent directories
        if let Some(parent) = self.path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        match self.format {
            DatasetFormat::JsonArray => self.save_json_array(dataset),
            DatasetFormat::Jsonl => self.save_jsonl(dataset),
            DatasetFormat::Csv => self.save_csv(dataset),
            _ => Err(FinetuningError::Unsupported(
                format!("Cannot save in {:?} format", self.format),
            )),
        }
    }

    fn save_json_array(&self, dataset: &Dataset) -> Result<()> {
        let json = serde_json::to_string_pretty(&dataset.samples)?;
        let mut file = File::create(&self.path)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }

    fn save_jsonl(&self, dataset: &Dataset) -> Result<()> {
        let mut file = File::create(&self.path)?;
        for sample in &dataset.samples {
            let json = serde_json::to_string(sample)?;
            writeln!(file, "{}", json)?;
        }
        Ok(())
    }

    fn save_csv(&self, dataset: &Dataset) -> Result<()> {
        let mut file = File::create(&self.path)?;
        writeln!(file, "id,input,output")?;

        for sample in &dataset.samples {
            // Simple CSV escaping
            let input = sample.input.replace("\"", "\"\"");
            let output = sample.output.replace("\"", "\"\"");
            writeln!(file, "{},\"{}\",\"{}\"", sample.id, input, output)?;
        }
        Ok(())
    }
}

/// Dataset utilities
pub struct DatasetUtils;

impl DatasetUtils {
    /// Load all datasets from directory
    pub fn load_directory(path: impl AsRef<Path>) -> Result<Vec<Dataset>> {
        let mut datasets = Vec::new();

        for entry in WalkDir::new(path)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
        {
            let ext = entry.path().extension().and_then(|e| e.to_str());
            if matches!(ext, Some("json") | Some("jsonl") | Some("csv")) {
                match DatasetLoader::new(entry.path()).load() {
                    Ok(dataset) => datasets.push(dataset),
                    Err(e) => {
                        tracing::warn!("Failed to load {}: {}", entry.path().display(), e);
                    }
                }
            }
        }

        Ok(datasets)
    }

    /// Merge multiple datasets
    pub fn merge(datasets: Vec<Dataset>, name: impl Into<String>) -> Dataset {
        let mut merged = Dataset::new(name);

        for dataset in datasets {
            merged.add_samples(dataset.samples);

            if let Some(val) = dataset.validation_samples {
                if let Some(ref mut merged_val) = merged.validation_samples {
                    merged_val.extend(val);
                } else {
                    merged.validation_samples = Some(val);
                }
            }
        }

        merged
    }

    /// Calculate dataset statistics
    pub fn stats(dataset: &Dataset) -> DatasetStats {
        let mut stats = DatasetStats::default();

        stats.total_samples = dataset.len();

        let mut input_lens = Vec::with_capacity(dataset.len());
        let mut output_lens = Vec::with_capacity(dataset.len());

        for sample in &dataset.samples {
            let input_len = sample.input.chars().count();
            let output_len = sample.output.chars().count();

            input_lens.push(input_len);
            output_lens.push(output_len);

            stats.total_input_chars += input_len;
            stats.total_output_chars += output_len;
        }

        if !input_lens.is_empty() {
            input_lens.sort_unstable();
            output_lens.sort_unstable();

            stats.min_input_len = input_lens[0];
            stats.max_input_len = *input_lens.last().unwrap_or(&0);
            stats.avg_input_len = stats.total_input_chars as f32 / stats.total_samples as f32;

            stats.min_output_len = output_lens[0];
            stats.max_output_len = *output_lens.last().unwrap_or(&0);
            stats.avg_output_len = stats.total_output_chars as f32 / stats.total_samples as f32;
        }

        stats
    }
}

/// Dataset statistics
#[derive(Debug, Clone, Default)]
pub struct DatasetStats {
    /// Total sample count
    pub total_samples: usize,
    /// Total input characters
    pub total_input_chars: usize,
    /// Total output characters
    pub total_output_chars: usize,
    /// Minimum input length
    pub min_input_len: usize,
    /// Maximum input length
    pub max_input_len: usize,
    /// Average input length
    pub avg_input_len: f32,
    /// Minimum output length
    pub min_output_len: usize,
    /// Maximum output length
    pub max_output_len: usize,
    /// Average output length
    pub avg_output_len: f32,
}

/// Shuffle samples in place
fn shuffle_samples(samples: &mut [TrainingSample]) {
    use rand::seq::SliceRandom;
    use rand::thread_rng;

    let mut rng = thread_rng();
    samples.shuffle(&mut rng);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_dataset_format_detection() {
        assert!(matches!(
            DatasetFormat::from_path(Path::new("data.json")),
            DatasetFormat::JsonArray
        ));
        assert!(matches!(
            DatasetFormat::from_path(Path::new("data.jsonl")),
            DatasetFormat::Jsonl
        ));
        assert!(matches!(
            DatasetFormat::from_path(Path::new("data.csv")),
            DatasetFormat::Csv
        ));
    }

    #[test]
    fn test_load_jsonl() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("test.jsonl");

        let mut file = File::create(&path).unwrap();
        writeln!(file, r#"{{"input": "Q1", "output": "A1"}}"#).unwrap();
        writeln!(file, r#"{{"input": "Q2", "output": "A2"}}"#).unwrap();

        let dataset = DatasetLoader::new(&path)
            .load()
            .unwrap();

        assert_eq!(dataset.len(), 2);
        assert_eq!(dataset.samples[0].input, "Q1");
        assert_eq!(dataset.samples[1].output, "A2");
    }

    #[test]
    fn test_load_json_array() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("test.json");

        let mut file = File::create(&path).unwrap();
        write!(file, r#"[{{"input": "Q1", "output": "A1"}}, {{"input": "Q2", "output": "A2"}}]"#).unwrap();

        let dataset = DatasetLoader::new(&path)
            .with_format(DatasetFormat::JsonArray)
            .load()
            .unwrap();

        assert_eq!(dataset.len(), 2);
    }

    #[test]
    fn test_max_samples() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("test.jsonl");

        let mut file = File::create(&path).unwrap();
        for i in 0..10 {
            writeln!(file, r#"{{"input": "Q{}", "output": "A{}"}}"#, i, i).unwrap();
        }

        let dataset = DatasetLoader::new(&path)
            .with_max_samples(5)
            .load()
            .unwrap();

        assert_eq!(dataset.len(), 5);
    }

    #[test]
    fn test_save_jsonl() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("output.jsonl");

        let mut dataset = Dataset::new("test");
        dataset.add_sample(TrainingSample::new("Q1", "A1"));
        dataset.add_sample(TrainingSample::new("Q2", "A2"));

        DatasetSaver::new(&path)
            .save(&dataset)
            .unwrap();

        // Verify
        let loaded = DatasetLoader::new(&path).load().unwrap();
        assert_eq!(loaded.len(), 2);
    }

    #[test]
    fn test_save_json_array() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("output.json");

        let mut dataset = Dataset::new("test");
        dataset.add_sample(TrainingSample::new("Q1", "A1"));

        DatasetSaver::new(&path)
            .with_format(DatasetFormat::JsonArray)
            .save(&dataset)
            .unwrap();

        let loaded = DatasetLoader::new(&path)
            .with_format(DatasetFormat::JsonArray)
            .load()
            .unwrap();

        assert_eq!(loaded.len(), 1);
    }

    #[test]
    fn test_dataset_stats() {
        let mut dataset = Dataset::new("test");
        dataset.add_sample(TrainingSample::new("Hello", "World"));
        dataset.add_sample(TrainingSample::new("Test", "Response"));

        let stats = DatasetUtils::stats(&dataset);

        assert_eq!(stats.total_samples, 2);
        assert!(stats.total_input_chars > 0);
        assert!(stats.total_output_chars > 0);
    }

    #[test]
    fn test_dataset_split() {
        let mut dataset = Dataset::new("test");
        for i in 0..100 {
            dataset.add_sample(TrainingSample::new(format!("Q{}", i), format!("A{}", i)));
        }

        dataset.split_validation(0.2);
        assert_eq!(dataset.len(), 80);
        assert_eq!(dataset.validation_len(), 20);
    }

    #[test]
    fn test_load_conversation() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("conv.json");

        let mut file = File::create(&path).unwrap();
        write!(file, r#"[{{"conversations": [
            {{"from": "human", "value": "Hello"}},
            {{"from": "gpt", "value": "Hi there!"}}
        ]}}]"#).unwrap();

        let dataset = DatasetLoader::new(&path)
            .with_format(DatasetFormat::Conversation)
            .load()
            .unwrap();

        assert_eq!(dataset.len(), 1);
        assert_eq!(dataset.samples[0].input, "Hello");
        assert_eq!(dataset.samples[0].output, "Hi there!");
    }
}
