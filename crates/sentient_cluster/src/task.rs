//! ─── Task Dispatcher ───

use crate::crds::{SentientTask, SentientTaskSpec, TaskType, TaskPhase};
use std::collections::VecDeque;
use std::sync::Arc;
use tokio::sync::{Mutex, broadcast};

/// Task dispatcher for distributed execution
pub struct TaskDispatcher {
    client: kube::Client,
    namespace: String,
    queue: Arc<Mutex<VecDeque<SentientTask>>>,
    event_tx: broadcast::Sender<TaskEvent>,
}

/// Task event
#[derive(Debug, Clone)]
pub enum TaskEvent {
    Queued { task_id: String },
    Started { task_id: String, agent: String },
    Completed { task_id: String, result: serde_json::Value },
    Failed { task_id: String, error: String },
}

impl TaskDispatcher {
    /// Create new dispatcher
    pub async fn new(namespace: impl Into<String>) -> Result<Self, TaskError> {
        let config = kube::Config::infer().await
            .map_err(|e| TaskError::Config(e.to_string()))?;
        
        let client = kube::Client::try_from(config)
            .map_err(|e| TaskError::Config(e.to_string()))?;
        
        let (event_tx, _) = broadcast::channel(100);
        
        Ok(Self {
            client,
            namespace: namespace.into(),
            queue: Arc::new(Mutex::new(VecDeque::new())),
            event_tx,
        })
    }
    
    /// Subscribe to task events
    pub fn subscribe(&self) -> broadcast::Receiver<TaskEvent> {
        self.event_tx.subscribe()
    }
    
    /// Submit new task
    pub async fn submit(&self, task_type: TaskType, input: serde_json::Value) -> Result<String, TaskError> {
        let task = SentientTask {
            metadata: kube::api::ObjectMeta {
                name: Some(format!("task-{}", uuid::Uuid::new_v4())),
                namespace: Some(self.namespace.clone()),
                ..Default::default()
            },
            spec: SentientTaskSpec {
                task_type,
                input,
                target_agents: vec![],
                priority: 5,
                timeout: 300,
                retries: 3,
                callback_url: None,
            },
            status: None,
        };
        
        let task_id = task.metadata.name.clone().unwrap();
        
        // Create task CRD
        let api: kube::Api<SentientTask> = kube::Api::namespaced(
            self.client.clone(),
            &self.namespace,
        );
        
        api.create(&kube::api::PostParams::default(), &task).await?;
        
        // Add to queue
        self.queue.lock().await.push_back(task);
        
        // Notify subscribers
        let _ = self.event_tx.send(TaskEvent::Queued {
            task_id: task_id.clone(),
        });
        
        Ok(task_id)
    }
    
    /// Submit task to specific agent
    pub async fn submit_to(
        &self,
        task_type: TaskType,
        input: serde_json::Value,
        target_agents: Vec<&str>,
    ) -> Result<String, TaskError> {
        let task = SentientTask {
            metadata: kube::api::ObjectMeta {
                name: Some(format!("task-{}", uuid::Uuid::new_v4())),
                namespace: Some(self.namespace.clone()),
                ..Default::default()
            },
            spec: SentientTaskSpec {
                task_type,
                input,
                target_agents: target_agents.into_iter().map(String::from).collect(),
                priority: 5,
                timeout: 300,
                retries: 3,
                callback_url: None,
            },
            status: None,
        };
        
        let task_id = task.metadata.name.clone().unwrap();
        
        let api: kube::Api<SentientTask> = kube::Api::namespaced(
            self.client.clone(),
            &self.namespace,
        );
        
        api.create(&kube::api::PostParams::default(), &task).await?;
        
        let _ = self.event_tx.send(TaskEvent::Queued {
            task_id: task_id.clone(),
        });
        
        Ok(task_id)
    }
    
    /// Get task status
    pub async fn get_status(&self, task_id: &str) -> Result<Option<TaskPhase>, TaskError> {
        let api: kube::Api<SentientTask> = kube::Api::namespaced(
            self.client.clone(),
            &self.namespace,
        );
        
        let task = api.get_opt(task_id).await?;
        
        Ok(task.and_then(|t| t.status.map(|s| s.phase)))
    }
    
    /// Cancel task
    pub async fn cancel(&self, task_id: &str) -> Result<(), TaskError> {
        let api: kube::Api<SentientTask> = kube::Api::namespaced(
            self.client.clone(),
            &self.namespace,
        );
        
        api.delete(task_id, &kube::api::DeleteParams::default()).await?;
        
        Ok(())
    }
    
    /// List pending tasks
    pub async fn list_pending(&self) -> Result<Vec<SentientTask>, TaskError> {
        let api: kube::Api<SentientTask> = kube::Api::namespaced(
            self.client.clone(),
            &self.namespace,
        );
        
        let tasks = api.list(&kube::api::ListParams::default()).await?;
        
        Ok(tasks.items.into_iter()
            .filter(|t| {
                t.status.as_ref()
                    .map(|s| s.phase == TaskPhase::Pending || s.phase == TaskPhase::Queued)
                    .unwrap_or(true)
            })
            .collect())
    }
    
    /// Start worker to process tasks
    pub async fn start_worker(self) {
        let queue = self.queue.clone();
        let client = self.client.clone();
        let namespace = self.namespace.clone();
        let event_tx = self.event_tx.clone();
        
        tokio::spawn(async move {
            loop {
                // Process tasks from queue
                let task = queue.lock().await.pop_front();
                
                if let Some(task) = task {
                    let task_id = task.metadata.name.clone().unwrap();
                    
                    // Get available agent
                    // In production, this would query agent CRDs
                    
                    // Execute task
                    // This would dispatch to the actual agent
                    
                    let _ = event_tx.send(TaskEvent::Started {
                        task_id: task_id.clone(),
                        agent: "worker-1".into(),
                    });
                    
                    // Simulate execution
                    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                    
                    let _ = event_tx.send(TaskEvent::Completed {
                        task_id,
                        result: serde_json::json!({"success": true}),
                    });
                }
                
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            }
        });
    }
}

/// Task error
#[derive(Debug, thiserror::Error)]
pub enum TaskError {
    #[error("Config error: {0}")]
    Config(String),
    
    #[error("Kubernetes error: {0}")]
    Kube(#[from] kube::Error),
}
