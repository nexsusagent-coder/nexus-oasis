//! ─── Training Monitor ───
//!
//! Monitor and log training progress

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

// ═══════════════════════════════════════════════════════════════════════════════
//  TRAINING MONITOR
// ═══════════════════════════════════════════════════════════════════════════════

/// Training progress monitor
pub struct TrainingMonitor {
    job_id: String,
    logs: Vec<LogEntry>,
    start_time: DateTime<Utc>,
    best_loss: Option<f32>,
}

impl TrainingMonitor {
    /// Create new monitor
    pub fn new(job_id: impl Into<String>) -> Self {
        Self {
            job_id: job_id.into(),
            logs: Vec::new(),
            start_time: Utc::now(),
            best_loss: None,
        }
    }

    /// Log a training event
    pub fn log(&mut self, entry: LogEntry) {
        log::info!("📊 TRAINING [{}]: {}", self.job_id, entry.message);
        self.logs.push(entry);
    }

    /// Log training step
    pub fn log_step(&mut self, step: u32, epoch: f32, loss: f32, lr: f32) {
        // Track best loss
        if self.best_loss.is_none() || loss < self.best_loss.unwrap() {
            self.best_loss = Some(loss);
        }

        let entry = LogEntry {
            timestamp: Utc::now(),
            level: LogLevel::Info,
            step: Some(step),
            epoch: Some(epoch),
            loss: Some(loss),
            learning_rate: Some(lr),
            message: format!("Step {} | Epoch {:.2} | Loss {:.4} | LR {:.2e}", step, epoch, loss, lr),
        };

        self.log(entry);
    }

    /// Log validation
    pub fn log_validation(&mut self, epoch: f32, val_loss: f32) {
        // Track best loss
        if self.best_loss.is_none() || val_loss < self.best_loss.unwrap() {
            self.best_loss = Some(val_loss);
        }

        let entry = LogEntry {
            timestamp: Utc::now(),
            level: LogLevel::Info,
            step: None,
            epoch: Some(epoch),
            loss: Some(val_loss),
            learning_rate: None,
            message: format!("Validation | Epoch {:.2} | Loss {:.4}", epoch, val_loss),
        };

        self.log(entry);
    }

    /// Log checkpoint
    pub fn log_checkpoint(&mut self, step: u32, checkpoint_id: &str) {
        let entry = LogEntry {
            timestamp: Utc::now(),
            level: LogLevel::Info,
            step: Some(step),
            epoch: None,
            loss: None,
            learning_rate: None,
            message: format!("Checkpoint saved: {}", checkpoint_id),
        };

        self.log(entry);
    }

    /// Log error
    pub fn log_error(&mut self, error: &str) {
        let entry = LogEntry {
            timestamp: Utc::now(),
            level: LogLevel::Error,
            step: None,
            epoch: None,
            loss: None,
            learning_rate: None,
            message: format!("Error: {}", error),
        };

        self.log(entry);
    }

    /// Log completion
    pub fn log_complete(&mut self, final_loss: f32, duration_secs: u64) {
        let entry = LogEntry {
            timestamp: Utc::now(),
            level: LogLevel::Info,
            step: None,
            epoch: None,
            loss: Some(final_loss),
            learning_rate: None,
            message: format!(
                "Training complete! Final loss: {:.4}, Duration: {}s",
                final_loss, duration_secs
            ),
        };

        self.log(entry);
    }

    /// Get all logs
    pub fn logs(&self) -> &[LogEntry] {
        &self.logs
    }

    /// Get elapsed time
    pub fn elapsed(&self) -> std::time::Duration {
        (Utc::now() - self.start_time).to_std().unwrap_or_default()
    }

    /// Get best loss
    pub fn best_loss(&self) -> Option<f32> {
        self.best_loss
    }

    /// Export logs as JSON
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(&self.logs)
    }

    /// Export logs as text
    pub fn to_text(&self) -> String {
        self.logs
            .iter()
            .map(|e| format!("[{}] {}", e.timestamp.format("%H:%M:%S"), e.message))
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Plot training curve (returns ASCII art)
    pub fn plot_loss_curve(&self, width: usize, height: usize) -> String {
        let losses: Vec<f32> = self.logs
            .iter()
            .filter_map(|e| e.loss)
            .collect();

        if losses.is_empty() {
            return "No loss data".to_string();
        }

        let min_loss = losses.iter().cloned().fold(f32::INFINITY, f32::min);
        let max_loss = losses.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
        let loss_range = max_loss - min_loss;

        if loss_range == 0.0 {
            return format!("Loss constant at {:.4}", min_loss);
        }

        let mut grid = vec![vec![' '; width]; height];

        for (i, &loss) in losses.iter().enumerate() {
            let x = (i * (width - 1) / losses.len().max(1)).min(width - 1);
            let y = ((max_loss - loss) / loss_range * (height - 1) as f32) as usize;
            let y = y.min(height - 1);
            grid[y][x] = '●';
        }

        let mut output = String::new();
        output.push_str(&format!("Loss: {:.4} - {:.4}\n", min_loss, max_loss));
        output.push_str(&format!("{}\n", "─".repeat(width + 2)));
        
        for row in grid {
            output.push_str(&format!("│{}│\n", row.into_iter().collect::<String>()));
        }
        
        output.push_str(&format!("{}\n", "─".repeat(width + 2)));

        output
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  LOG ENTRY
// ═══════════════════════════════════════════════════════════════════════════════

/// Log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    /// Log level
    pub level: LogLevel,
    /// Step number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step: Option<u32>,
    /// Epoch
    #[serde(skip_serializing_if = "Option::is_none")]
    pub epoch: Option<f32>,
    /// Loss value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loss: Option<f32>,
    /// Learning rate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub learning_rate: Option<f32>,
    /// Message
    pub message: String,
}

/// Log level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LogLevel {
    #[serde(rename = "debug")]
    Debug,
    #[serde(rename = "info")]
    Info,
    #[serde(rename = "warning")]
    Warning,
    #[serde(rename = "error")]
    Error,
}

// ═══════════════════════════════════════════════════════════════════════════════
//  TRAINING CALLBACKS
// ═══════════════════════════════════════════════════════════════════════════════

/// Callback trait for training events
pub trait TrainingCallback: Send + Sync {
    /// Called on each training step
    fn on_step(&self, step: u32, epoch: f32, loss: f32);
    
    /// Called on validation
    fn on_validation(&self, epoch: f32, val_loss: f32);
    
    /// Called on checkpoint
    fn on_checkpoint(&self, step: u32, checkpoint_id: &str);
    
    /// Called on completion
    fn on_complete(&self, final_loss: f32);
    
    /// Called on error
    fn on_error(&self, error: &str);
}

/// Simple console callback
pub struct ConsoleCallback;

impl TrainingCallback for ConsoleCallback {
    fn on_step(&self, step: u32, epoch: f32, loss: f32) {
        println!("Step {} | Epoch {:.2} | Loss {:.4}", step, epoch, loss);
    }

    fn on_validation(&self, epoch: f32, val_loss: f32) {
        println!("Validation | Epoch {:.2} | Loss {:.4}", epoch, val_loss);
    }

    fn on_checkpoint(&self, step: u32, checkpoint_id: &str) {
        println!("Checkpoint at step {}: {}", step, checkpoint_id);
    }

    fn on_complete(&self, final_loss: f32) {
        println!("Training complete! Final loss: {:.4}", final_loss);
    }

    fn on_error(&self, error: &str) {
        eprintln!("Training error: {}", error);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monitor_logging() {
        let mut monitor = TrainingMonitor::new("test-job");
        
        monitor.log_step(1, 0.1, 2.5, 5e-5);
        monitor.log_step(2, 0.2, 2.3, 5e-5);
        monitor.log_validation(1.0, 2.1);
        
        assert_eq!(monitor.logs().len(), 3);
        // Best loss is the minimum: 2.1 (validation) < 2.3 < 2.5
        assert_eq!(monitor.best_loss(), Some(2.1));
    }

    #[test]
    fn test_monitor_export() {
        let mut monitor = TrainingMonitor::new("test-job");
        monitor.log_step(1, 0.1, 2.5, 5e-5);
        
        let json = monitor.to_json().unwrap();
        assert!(json.contains("loss"));
        
        let text = monitor.to_text();
        assert!(text.contains("Step 1"));
    }

    #[test]
    fn test_loss_curve() {
        let mut monitor = TrainingMonitor::new("test-job");
        
        for i in 1..=10 {
            monitor.log_step(i, i as f32 / 10.0, 3.0 - i as f32 * 0.2, 5e-5);
        }
        
        let curve = monitor.plot_loss_curve(40, 10);
        assert!(curve.contains("Loss:"));
        assert!(curve.contains("●"));
    }

    #[test]
    fn test_console_callback() {
        let callback = ConsoleCallback;
        
        // These just print to console, no assertions needed
        callback.on_step(1, 0.1, 2.5);
        callback.on_validation(1.0, 2.0);
        callback.on_checkpoint(10, "ckpt-10");
        callback.on_complete(1.5);
        callback.on_error("Test error");
    }

    #[test]
    fn test_elapsed_time() {
        let monitor = TrainingMonitor::new("test-job");
        let elapsed = monitor.elapsed();
        assert!(elapsed.as_secs() < 1);
    }
}
