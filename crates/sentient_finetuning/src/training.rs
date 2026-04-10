//! Training engine for fine-tuning

use crate::dataset::*;
use crate::types::*;
use crate::{FinetuningError, Result};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::{broadcast, mpsc, RwLock};
use tracing::{info, warn};

/// Training status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrainingStatus {
    /// Not started
    Idle,
    /// Training in progress
    Training,
    /// Paused
    Paused,
    /// Completed successfully
    Completed,
    /// Failed with error
    Failed,
    /// Cancelled by user
    Cancelled,
}

/// Training event
#[derive(Debug, Clone)]
pub enum TrainingEvent {
    /// Training started
    Started {
        config: TrainingConfig,
        total_steps: usize,
    },
    /// Progress update
    Progress {
        metrics: TrainingMetrics,
    },
    /// Validation completed
    Validation {
        step: Step,
        val_loss: Loss,
    },
    /// Checkpoint saved
    CheckpointSaved {
        path: String,
        step: Step,
    },
    /// Training completed
    Completed {
        final_loss: Loss,
        duration_secs: u64,
    },
    /// Training failed
    Failed {
        error: String,
    },
    /// Training cancelled
    Cancelled {
        step: Step,
    },
}

/// Training handle for monitoring and control
pub struct TrainingHandle {
    /// Status
    status: Arc<RwLock<TrainingStatus>>,
    /// Metrics
    metrics: Arc<RwLock<TrainingMetrics>>,
    /// Event sender
    event_tx: broadcast::Sender<TrainingEvent>,
    /// Cancel signal
    cancel_tx: mpsc::Sender<()>,
}

impl TrainingHandle {
    /// Get current status
    pub async fn status(&self) -> TrainingStatus {
        *self.status.read().await
    }

    /// Get current metrics
    pub async fn metrics(&self) -> TrainingMetrics {
        self.metrics.read().await.clone()
    }

    /// Subscribe to events
    pub fn subscribe(&self) -> broadcast::Receiver<TrainingEvent> {
        self.event_tx.subscribe()
    }

    /// Cancel training
    pub async fn cancel(&self) -> Result<()> {
        let status = self.status().await;
        if status == TrainingStatus::Training {
            self.cancel_tx.send(()).await
                .map_err(|_| FinetuningError::Training("Failed to send cancel signal".to_string()))?;
        }
        Ok(())
    }
}

/// Training engine
pub struct TrainingEngine {
    /// Training configuration
    config: TrainingConfig,
    /// Dataset
    dataset: Dataset,
    /// Current status
    status: Arc<RwLock<TrainingStatus>>,
    /// Current metrics
    metrics: Arc<RwLock<TrainingMetrics>>,
    /// Event broadcaster
    event_tx: broadcast::Sender<TrainingEvent>,
    /// Cancel receiver (stored for training)
    cancel_rx: Option<mpsc::Receiver<()>>,
    /// Training state
    state: TrainingState,
}

impl TrainingEngine {
    /// Create new training engine
    pub fn new(config: TrainingConfig, dataset: Dataset) -> Self {
        let (event_tx, _) = broadcast::channel(100);

        Self {
            config,
            dataset,
            status: Arc::new(RwLock::new(TrainingStatus::Idle)),
            metrics: Arc::new(RwLock::new(TrainingMetrics::default())),
            event_tx,
            cancel_rx: None,
            state: TrainingState::new(),
        }
    }

    /// Validate configuration and dataset
    pub fn validate(&self) -> Result<()> {
        self.config.validate()?;

        if self.dataset.is_empty() {
            return Err(FinetuningError::Dataset("Dataset is empty".to_string()));
        }

        Ok(())
    }

    /// Create training handle
    pub fn handle(&self) -> TrainingHandle {
        let (cancel_tx, cancel_rx) = mpsc::channel(1);

        // Store cancel_rx for training
        // Note: In a real implementation, we'd pass this to the training loop

        TrainingHandle {
            status: self.status.clone(),
            metrics: self.metrics.clone(),
            event_tx: self.event_tx.clone(),
            cancel_tx,
        }
    }

    /// Start training (async)
    pub async fn train(&mut self) -> Result<FinetunedModel> {
        // Validate
        self.validate()?;

        // Calculate total steps
        let samples_per_epoch = self.dataset.len();
        let total_steps = samples_per_epoch * self.config.hyperparameters.epochs;

        // Update status
        *self.status.write().await = TrainingStatus::Training;

        // Emit start event
        let _ = self.event_tx.send(TrainingEvent::Started {
            config: self.config.clone(),
            total_steps,
        });

        let start_time = Instant::now();

        // Simulated training loop
        // In real implementation, this would use candle/transformers
        let result = self.training_loop(start_time).await;

        match result {
            Ok(model) => {
                *self.status.write().await = TrainingStatus::Completed;
                let _ = self.event_tx.send(TrainingEvent::Completed {
                    final_loss: model.final_train_loss,
                    duration_secs: model.training_duration_secs,
                });
                Ok(model)
            }
            Err(e) => {
                *self.status.write().await = TrainingStatus::Failed;
                let _ = self.event_tx.send(TrainingEvent::Failed {
                    error: e.to_string(),
                });
                Err(e)
            }
        }
    }

    /// Training loop (placeholder for actual ML training)
    async fn training_loop(&mut self, start_time: Instant) -> Result<FinetunedModel> {
        let hp = &self.config.hyperparameters;
        let total_steps = self.dataset.len() * hp.epochs;

        // Create output directory
        std::fs::create_dir_all(&self.config.output_dir)?;

        let mut current_loss = 1.0; // Starting loss
        let mut best_val_loss = f32::MAX;

        for epoch in 0..hp.epochs {
            self.state.epoch = epoch;

            for step in 0..self.dataset.len() {
                // Check for cancellation
                // In real implementation, check cancel_rx here

                self.state.step = epoch * self.dataset.len() + step;

                // Simulate training step
                // In real implementation: forward pass, backward pass, optimizer step
                let step_loss = self.simulate_training_step().await?;

                // Update loss (exponential moving average)
                current_loss = 0.9 * current_loss + 0.1 * step_loss;

                // Update metrics
                let mut metrics = self.metrics.write().await;
                metrics.epoch = epoch;
                metrics.step = self.state.step;
                metrics.total_steps = total_steps;
                metrics.train_loss = current_loss;
                metrics.learning_rate = self.get_lr(self.state.step, total_steps);
                metrics.eta_seconds = Some(self.estimate_eta(self.state.step, total_steps, start_time));

                // Log progress
                if self.state.step % self.config.log_steps == 0 {
                    let _ = self.event_tx.send(TrainingEvent::Progress {
                        metrics: metrics.clone(),
                    });

                    info!(
                        "Step {}/{} | Loss: {:.4} | LR: {:.2e}",
                        self.state.step, total_steps, current_loss, metrics.learning_rate
                    );
                }

                // Save checkpoint
                if self.state.step > 0 && self.state.step % self.config.save_steps == 0 {
                    self.save_checkpoint(current_loss, None).await?;
                }

                // Validation
                if self.state.step > 0 && self.state.step % self.config.eval_steps == 0 {
                    if let Some(val_samples) = &self.dataset.validation_samples {
                        let val_loss = self.run_validation(val_samples).await?;

                        if val_loss < best_val_loss {
                            best_val_loss = val_loss;
                            self.save_checkpoint(current_loss, Some(val_loss)).await?;
                        }

                        let _ = self.event_tx.send(TrainingEvent::Validation {
                            step: self.state.step,
                            val_loss,
                        });
                    }
                }

                // Yield to allow cancellation
                tokio::time::sleep(Duration::from_millis(1)).await;
            }
        }

        // Save final checkpoint
        let checkpoint_path = self.save_checkpoint(current_loss, None).await?;

        // Create finetuned model info
        let model = FinetunedModel {
            id: format!("{}-finetuned", self.config.model_id),
            base_model: self.config.model_id.clone(),
            method: self.config.method.clone(),
            config: self.config.clone(),
            final_train_loss: current_loss,
            final_val_loss: if best_val_loss < f32::MAX { Some(best_val_loss) } else { None },
            training_duration_secs: start_time.elapsed().as_secs(),
            num_samples: self.dataset.len(),
            checkpoint_path,
            created_at: chrono::Utc::now().timestamp(),
        };

        Ok(model)
    }

    /// Simulate training step (placeholder)
    async fn simulate_training_step(&self) -> Result<f32> {
        // Simulate computation time
        tokio::time::sleep(Duration::from_micros(100)).await;

        // Return simulated loss
        use rand::Rng;
        let mut rng = rand::thread_rng();
        Ok(rng.gen_range(0.1..1.0))
    }

    /// Run validation on validation set
    async fn run_validation(&self, _val_samples: &[TrainingSample]) -> Result<f32> {
        // Simulate validation
        tokio::time::sleep(Duration::from_millis(10)).await;

        // Return simulated validation loss
        use rand::Rng;
        let mut rng = rand::thread_rng();
        Ok(rng.gen_range(0.1..0.5))
    }

    /// Get learning rate for current step
    fn get_lr(&self, step: usize, total_steps: usize) -> f32 {
        let base_lr = self.config.hyperparameters.learning_rate;
        let warmup_steps = self.config.hyperparameters.warmup_steps;

        // Warmup phase
        if step < warmup_steps {
            return base_lr * (step as f32 / warmup_steps as f32);
        }

        // Apply scheduler
        match self.config.hyperparameters.lr_scheduler {
            LrScheduler::Constant => base_lr,
            LrScheduler::Linear => {
                let progress = step as f32 / total_steps as f32;
                base_lr * (1.0 - progress)
            }
            LrScheduler::Cosine => {
                let progress = (step - warmup_steps) as f32 / (total_steps - warmup_steps) as f32;
                base_lr * 0.5 * (1.0 + (std::f32::consts::PI * progress).cos())
            }
            _ => base_lr,
        }
    }

    /// Estimate time remaining
    fn estimate_eta(&self, step: usize, total_steps: usize, start_time: Instant) -> u64 {
        if step == 0 {
            return 0;
        }

        let elapsed = start_time.elapsed().as_secs();
        let steps_per_sec = step as f64 / elapsed as f64;
        let remaining_steps = total_steps - step;

        (remaining_steps as f64 / steps_per_sec) as u64
    }

    /// Save checkpoint
    async fn save_checkpoint(&self, train_loss: f32, val_loss: Option<f32>) -> Result<String> {
        let checkpoint_dir = PathBuf::from(&self.config.output_dir)
            .join(format!("checkpoint-{}", self.state.step));

        std::fs::create_dir_all(&checkpoint_dir)?;

        // Save training state
        let state_path = checkpoint_dir.join("training_state.json");
        let state_json = serde_json::to_string_pretty(&self.state)?;
        std::fs::write(&state_path, state_json)?;

        // Save config
        let config_path = checkpoint_dir.join("config.json");
        let config_json = serde_json::to_string_pretty(&self.config)?;
        std::fs::write(&config_path, config_json)?;

        // Emit checkpoint event
        let _ = self.event_tx.send(TrainingEvent::CheckpointSaved {
            path: checkpoint_dir.to_string_lossy().to_string(),
            step: self.state.step,
        });

        info!("Saved checkpoint to {}", checkpoint_dir.display());

        Ok(checkpoint_dir.to_string_lossy().to_string())
    }

    /// Load checkpoint
    pub fn load_checkpoint(&mut self, path: impl AsRef<Path>) -> Result<()> {
        let path = path.as_ref();

        // Load training state
        let state_path = path.join("training_state.json");
        let state_json = std::fs::read_to_string(state_path)?;
        self.state = serde_json::from_str(&state_json)?;

        // Load config
        let config_path = path.join("config.json");
        let config_json = std::fs::read_to_string(config_path)?;
        self.config = serde_json::from_str(&config_json)?;

        info!("Loaded checkpoint from step {}", self.state.step);

        Ok(())
    }
}

/// Resume training from checkpoint
pub async fn resume_training(
    checkpoint_path: impl AsRef<Path>,
    dataset: Dataset,
) -> Result<TrainingEngine> {
    let path = checkpoint_path.as_ref();

    // Load config
    let config_path = path.join("config.json");
    let config_json = std::fs::read_to_string(&config_path)?;
    let config: TrainingConfig = serde_json::from_str(&config_json)?;

    let mut engine = TrainingEngine::new(config, dataset);
    engine.load_checkpoint(path)?;

    Ok(engine)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn create_test_dataset() -> Dataset {
        let mut dataset = Dataset::new("test");
        for i in 0..10 {
            dataset.add_sample(TrainingSample::new(
                format!("Question {}", i),
                format!("Answer {}", i),
            ));
        }
        dataset
    }

    fn create_test_config() -> TrainingConfig {
        TrainingConfig {
            model_id: "test-model".to_string(),
            output_dir: "./test-output".to_string(),
            hyperparameters: Hyperparameters {
                epochs: 2,
                batch_size: 2,
                ..Default::default()
            },
            save_steps: 5,
            eval_steps: 5,
            log_steps: 2,
            ..Default::default()
        }
    }

    #[test]
    fn test_engine_creation() {
        let config = create_test_config();
        let dataset = create_test_dataset();

        let engine = TrainingEngine::new(config, dataset);
        assert!(engine.validate().is_ok());
    }

    #[test]
    fn test_engine_validate_empty_dataset() {
        let config = create_test_config();
        let dataset = Dataset::new("empty");

        let engine = TrainingEngine::new(config, dataset);
        assert!(engine.validate().is_err());
    }

    #[test]
    fn test_engine_handle() {
        let config = create_test_config();
        let dataset = create_test_dataset();

        let engine = TrainingEngine::new(config, dataset);
        let handle = engine.handle();

        // Should be able to subscribe to events
        let _rx = handle.subscribe();
    }

    #[tokio::test]
    async fn test_training_status() {
        let config = create_test_config();
        let dataset = create_test_dataset();

        let engine = TrainingEngine::new(config, dataset);
        let handle = engine.handle();

        let status = handle.status().await;
        assert_eq!(status, TrainingStatus::Idle);
    }

    #[tokio::test]
    async fn test_full_training() {
        let config = create_test_config();
        let dataset = create_test_dataset();

        let mut engine = TrainingEngine::new(config, dataset);
        let handle = engine.handle();

        // Subscribe to events
        let mut event_rx = handle.subscribe();

        // Run training in background
        let engine_handle = tokio::spawn(async move {
            engine.train().await
        });

        // Collect some events
        let mut events = Vec::new();
        for _ in 0..3 {
            if let Ok(event) = event_rx.recv().await {
                events.push(event);
            }
        }

        // Wait for completion
        let result = engine_handle.await.unwrap();

        // Check events
        assert!(!events.is_empty());
        assert!(result.is_ok());
    }

    #[test]
    fn test_lr_scheduler() {
        let config = create_test_config();
        let dataset = create_test_dataset();

        let engine = TrainingEngine::new(config, dataset);

        // Test warmup
        let lr_warmup = engine.get_lr(50, 1000);
        assert!(lr_warmup > 0.0);

        // Test post-warmup
        let lr = engine.get_lr(500, 1000);
        assert!(lr > 0.0);
    }

    #[test]
    fn test_eta_estimation() {
        let config = create_test_config();
        let dataset = create_test_dataset();

        let engine = TrainingEngine::new(config, dataset);
        let start = Instant::now() - Duration::from_secs(10);

        let eta = engine.estimate_eta(100, 1000, start);
        assert!(eta > 0);
    }
}
