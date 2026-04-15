//! ─── Todo System Core ───

use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::RwLock;

use crate::models::*;
use crate::research::TaskResearcher;
use crate::decomposition::TaskDecomposer;
use crate::priority::PriorityEngine;
use crate::tracking::ProgressTracker;
use crate::{TodoResult, TodoError};

/// Todo system configuration
#[derive(Debug, Clone)]
pub struct TodoConfig {
    pub enable_self_research: bool,
    pub enable_auto_decomposition: bool,
    pub enable_priority_ai: bool,
    pub max_subtasks: usize,
    pub default_deadline_days: u32,
}

impl Default for TodoConfig {
    fn default() -> Self {
        Self {
            enable_self_research: true,
            enable_auto_decomposition: true,
            enable_priority_ai: true,
            max_subtasks: 10,
            default_deadline_days: 7,
        }
    }
}

/// Main todo system
pub struct TodoSystem {
    tasks: Arc<RwLock<HashMap<String, Task>>>,
    researcher: TaskResearcher,
    decomposer: TaskDecomposer,
    priority_engine: PriorityEngine,
    tracker: ProgressTracker,
    config: TodoConfig,
}

impl TodoSystem {
    /// Create new todo system
    pub fn new() -> Self {
        Self::with_config(TodoConfig::default())
    }
    
    /// Create with custom config
    pub fn with_config(config: TodoConfig) -> Self {
        Self {
            tasks: Arc::new(RwLock::new(HashMap::new())),
            researcher: TaskResearcher::new(),
            decomposer: TaskDecomposer::new(),
            priority_engine: PriorityEngine::new(),
            tracker: ProgressTracker::new(),
            config,
        }
    }
    
    /// Add a new task
    pub async fn add_task(&self, task: Task) -> TodoResult<String> {
        let id = task.id.clone();
        
        // Auto-decompose if enabled
        let task = if self.config.enable_auto_decomposition && task.subtasks.is_empty() {
            self.decomposer.decompose(&task).await
                .unwrap_or(task)
        } else {
            task
        };
        
        let mut tasks = self.tasks.write().await;
        tasks.insert(id.clone(), task);
        
        Ok(id)
    }
    
    /// Get task by ID
    pub async fn get_task(&self, id: &str) -> TodoResult<Task> {
        let tasks = self.tasks.read().await;
        tasks.get(id)
            .cloned()
            .ok_or_else(|| TodoError::NotFound(id.to_string()))
    }
    
    /// Update task
    pub async fn update_task(&self, task: Task) -> TodoResult<()> {
        let mut tasks = self.tasks.write().await;
        tasks.insert(task.id.clone(), task);
        Ok(())
    }
    
    /// Delete task
    pub async fn delete_task(&self, id: &str) -> TodoResult<()> {
        let mut tasks = self.tasks.write().await;
        tasks.remove(id)
            .ok_or_else(|| TodoError::NotFound(id.to_string()))?;
        Ok(())
    }
    
    /// Get all tasks
    pub async fn get_all_tasks(&self) -> Vec<Task> {
        let tasks = self.tasks.read().await;
        tasks.values().cloned().collect()
    }
    
    /// Get tasks by status
    pub async fn get_by_status(&self, status: TaskStatus) -> Vec<Task> {
        let tasks = self.tasks.read().await;
        tasks.values()
            .filter(|t| t.status == status)
            .cloned()
            .collect()
    }
    
    /// Get overdue tasks
    pub async fn get_overdue(&self) -> Vec<Task> {
        let tasks = self.tasks.read().await;
        tasks.values()
            .filter(|t| t.is_overdue())
            .cloned()
            .collect()
    }
    
    /// Mark task as complete
    pub async fn complete_task(&self, id: &str) -> TodoResult<()> {
        let mut tasks = self.tasks.write().await;
        let task = tasks.get_mut(id)
            .ok_or_else(|| TodoError::NotFound(id.to_string()))?;
        
        task.status = TaskStatus::Completed;
        task.completed = Some(chrono::Utc::now());
        
        Ok(())
    }
    
    /// Start task
    pub async fn start_task(&self, id: &str) -> TodoResult<()> {
        let mut tasks = self.tasks.write().await;
        let task = tasks.get_mut(id)
            .ok_or_else(|| TodoError::NotFound(id.to_string()))?;
        
        task.status = TaskStatus::InProgress;
        
        Ok(())
    }
    
    /// Process self-researching tasks
    pub async fn process_pending(&self) -> Vec<Task> {
        let mut processed = Vec::new();
        let mut tasks = self.tasks.write().await;
        
        for task in tasks.values_mut() {
            if task.self_researching && task.research_notes.is_none() {
                if let Ok(result) = self.researcher.research(&task.title).await {
                    task.research_notes = Some(result.summary);
                    
                    // Add research-based sub-tasks
                    for subtask in result.suggested_subtasks {
                        if task.subtasks.len() < self.config.max_subtasks {
                            task.subtasks.push(SubTask::new(&subtask, task.subtasks.len() as u32));
                        }
                    }
                    
                    processed.push(task.clone());
                }
            }
        }
        
        processed
    }
    
    /// Get prioritized task list
    pub async fn get_prioritized(&self) -> Vec<Task> {
        let tasks = self.tasks.read().await;
        let mut task_list: Vec<_> = tasks.values().cloned().collect();
        
        // Sort by priority
        self.priority_engine.sort_tasks(&mut task_list);
        
        task_list
    }
    
    /// Get progress summary
    pub async fn progress_summary(&self) -> ProgressSummary {
        let tasks = self.tasks.read().await;
        
        let total = tasks.len();
        let completed = tasks.values().filter(|t| t.status == TaskStatus::Completed).count();
        let in_progress = tasks.values().filter(|t| t.status == TaskStatus::InProgress).count();
        let overdue = tasks.values().filter(|t| t.is_overdue()).count();
        
        ProgressSummary {
            total,
            completed,
            in_progress,
            todo: total - completed - in_progress,
            overdue,
            completion_rate: if total > 0 { (completed as f64 / total as f64) * 100.0 } else { 0.0 },
        }
    }
}

impl Default for TodoSystem {
    fn default() -> Self {
        Self::new()
    }
}

/// Progress summary
#[derive(Debug, Clone)]
pub struct ProgressSummary {
    pub total: usize,
    pub completed: usize,
    pub in_progress: usize,
    pub todo: usize,
    pub overdue: usize,
    pub completion_rate: f64,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_add_get_task() {
        let todo = TodoSystem::new();
        let task = TaskBuilder::new("Test").build();
        let id = task.id.clone();
        
        todo.add_task(task).await.unwrap();
        let retrieved = todo.get_task(&id).await.unwrap();
        
        assert_eq!(retrieved.title, "Test");
    }
}
