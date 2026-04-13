//! ─── Batch Processing ───

use std::collections::VecDeque;
use std::sync::Arc;

use parking_lot::RwLock;
use tokio::sync::Notify;

use crate::{Embedding, EmbedResult, EmbeddingProvider};

// ═══════════════════════════════════════════════════════════════════════════════
//  BATCH CONFIGURATION
// ═══════════════════════════════════════════════════════════════════════════════

/// Batch processing configuration
#[derive(Debug, Clone)]
pub struct BatchConfig {
    /// Maximum batch size
    pub max_batch_size: usize,
    /// Maximum wait time in ms
    pub max_wait_ms: u64,
    /// Maximum concurrent batches
    pub max_concurrent: usize,
    /// Enable dynamic batching
    pub dynamic_batching: bool,
}

impl Default for BatchConfig {
    fn default() -> Self {
        Self {
            max_batch_size: 100,
            max_wait_ms: 50,
            max_concurrent: 10,
            dynamic_batching: true,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  BATCH REQUEST
// ═══════════════════════════════════════════════════════════════════════════════

/// Pending batch request
struct PendingRequest {
    text: String,
    model: String,
    result_tx: tokio::sync::oneshot::Sender<EmbedResult<Embedding>>,
}

// ═══════════════════════════════════════════════════════════════════════════════
//  BATCH PROCESSOR
// ═══════════════════════════════════════════════════════════════════════════════

/// Batch processing statistics
#[derive(Debug, Clone, Default)]
pub struct BatchStats {
    pub total_requests: u64,
    pub total_batches: u64,
    pub total_embeddings: u64,
    pub avg_batch_size: f64,
    pub avg_latency_ms: f64,
}

/// Batch processor for embeddings
pub struct BatchProcessor {
    config: BatchConfig,
    provider: Arc<dyn EmbeddingProvider + Send + Sync>,
    pending: Arc<RwLock<VecDeque<PendingRequest>>>,
    notify: Arc<Notify>,
    running: Arc<RwLock<bool>>,
    stats: Arc<RwLock<BatchStats>>,
}

impl BatchProcessor {
    /// Create new batch processor
    pub fn new(config: BatchConfig, provider: Arc<dyn EmbeddingProvider + Send + Sync>) -> Self {
        Self {
            config,
            provider,
            pending: Arc::new(RwLock::new(VecDeque::new())),
            notify: Arc::new(Notify::new()),
            running: Arc::new(RwLock::new(false)),
            stats: Arc::new(RwLock::new(BatchStats::default())),
        }
    }

    /// Start the batch processor
    pub fn start(&self) {
        *self.running.write() = true;
        let notify = self.notify.clone();
        let config = self.config.clone();
        let pending = self.pending.clone();
        let running = self.running.clone();
        let provider = self.provider.clone();
        let stats = self.stats.clone();
        
        tokio::spawn(async move {
            loop {
                if !*running.read() {
                    break;
                }
                
                // Wait for requests or timeout
                tokio::select! {
                    _ = notify.notified() => {}
                    _ = tokio::time::sleep(tokio::time::Duration::from_millis(config.max_wait_ms)) => {}
                }
                
                // Process batch
                let batch: Vec<PendingRequest> = {
                    let mut p = pending.write();
                    let count = p.len().min(config.max_batch_size);
                    p.drain(..count).collect()
                };
                
                if batch.is_empty() {
                    continue;
                }
                
                // Extract texts
                let texts: Vec<&str> = batch.iter().map(|r| r.text.as_str()).collect();
                let texts_len = texts.len();
                let model = batch.get(0).map(|r| r.model.clone()).unwrap_or_default();
                
                // Process batch
                let start = std::time::Instant::now();
                let result = provider.embed_batch(&texts.iter().map(|s| *s).collect::<Vec<_>>(), &model).await;
                let latency = start.elapsed().as_millis() as f64;
                
                // Send results
                match result {
                    Ok(embeddings) => {
                        for (i, req) in batch.into_iter().enumerate() {
                            let emb = embeddings.get(i).cloned().unwrap_or(Embedding {
                                vector: vec![],
                                model: model.clone(),
                                tokens: 0,
                                index: i,
                                text: Some(req.text.clone()),
                            });
                            let _ = req.result_tx.send(Ok(emb));
                        }
                        
                        let mut s = stats.write();
                        s.total_requests += texts_len as u64;
                        s.total_batches += 1;
                        s.total_embeddings += embeddings.len() as u64;
                        s.avg_batch_size = s.total_embeddings as f64 / s.total_batches as f64;
                        s.avg_latency_ms = (s.avg_latency_ms * (s.total_batches - 1) as f64 + latency) / s.total_batches as f64;
                    }
                    Err(e) => {
                        for req in batch {
                            let _ = req.result_tx.send(Err(e.clone()));
                        }
                    }
                }
            }
        });
    }

    /// Stop the batch processor
    pub fn stop(&self) {
        *self.running.write() = false;
        self.notify.notify_waiters();
    }

    /// Submit request for batch processing
    pub async fn submit(&self, text: String, model: String) -> EmbedResult<Embedding> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        
        {
            let mut pending = self.pending.write();
            pending.push_back(PendingRequest {
                text,
                model,
                result_tx: tx,
            });
        }
        
        self.notify.notify_one();
        
        rx.await.map_err(|_| crate::EmbedError::ChannelClosed)?
    }

    /// Get statistics
    pub fn stats(&self) -> BatchStats {
        self.stats.read().clone()
    }

    /// Check if running
    pub fn is_running(&self) -> bool {
        *self.running.read()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_batch_config_default() {
        let config = BatchConfig::default();
        assert_eq!(config.max_batch_size, 100);
        assert_eq!(config.max_wait_ms, 50);
    }
}
