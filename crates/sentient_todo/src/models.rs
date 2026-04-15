//! ─── Todo Models ───

use serde::{Deserialize, Serialize};

/// Task
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    /// Unique identifier
    pub id: String,
    
    /// Task title
    pub title: String,
    
    /// Description
    pub description: Option<String>,
    
    /// Status
    pub status: TaskStatus,
    
    /// Priority
    pub priority: TaskPriority,
    
    /// Sub-tasks
    pub subtasks: Vec<SubTask>,
    
    /// Tags
    pub tags: Vec<String>,
    
    /// Created time
    pub created: chrono::DateTime<chrono::Utc>,
    
    /// Due date
    pub due: Option<chrono::DateTime<chrono::Utc>>,
    
    /// Completed time
    pub completed: Option<chrono::DateTime<chrono::Utc>>,
    
    /// Estimated minutes
    pub estimated_minutes: Option<u32>,
    
    /// Actual minutes spent
    pub actual_minutes: Option<u32>,
    
    /// Is self-researching
    pub self_researching: bool,
    
    /// Research notes
    pub research_notes: Option<String>,
    
    /// Dependencies (task IDs)
    pub dependencies: Vec<String>,
    
    /// Project ID
    pub project_id: Option<String>,
}

impl Task {
    /// Calculate progress percentage
    pub fn progress(&self) -> f64 {
        if self.subtasks.is_empty() {
            return match self.status {
                TaskStatus::Completed => 100.0,
                TaskStatus::InProgress => 50.0,
                _ => 0.0,
            };
        }
        
        let completed = self.subtasks.iter().filter(|s| s.completed).count();
        (completed as f64 / self.subtasks.len() as f64) * 100.0
    }
    
    /// Check if task is overdue
    pub fn is_overdue(&self) -> bool {
        if let Some(due) = self.due {
            return chrono::Utc::now() > due && self.status != TaskStatus::Completed;
        }
        false
    }
    
    /// Check if task can be started (dependencies met)
    pub fn can_start(&self, completed_ids: &[&str]) -> bool {
        self.dependencies.iter().all(|d| completed_ids.contains(&d.as_str()))
    }
    
    /// Get time until due
    pub fn time_until_due(&self) -> Option<chrono::Duration> {
        self.due.map(|d| d - chrono::Utc::now())
    }
}

/// Task builder
pub struct TaskBuilder {
    title: String,
    description: Option<String>,
    priority: TaskPriority,
    due: Option<chrono::DateTime<chrono::Utc>>,
    estimated_minutes: Option<u32>,
    self_researching: bool,
    tags: Vec<String>,
    dependencies: Vec<String>,
}

impl TaskBuilder {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            description: None,
            priority: TaskPriority::Medium,
            due: None,
            estimated_minutes: None,
            self_researching: false,
            tags: vec![],
            dependencies: vec![],
        }
    }
    
    pub fn with_description(mut self, desc: &str) -> Self {
        self.description = Some(desc.to_string());
        self
    }
    
    pub fn with_priority(mut self, priority: TaskPriority) -> Self {
        self.priority = priority;
        self
    }
    
    pub fn with_deadline(mut self, days: u32) -> Self {
        self.due = Some(chrono::Utc::now() + chrono::Duration::days(days as i64));
        self
    }
    
    pub fn with_due(mut self, due: chrono::DateTime<chrono::Utc>) -> Self {
        self.due = Some(due);
        self
    }
    
    pub fn with_estimate(mut self, minutes: u32) -> Self {
        self.estimated_minutes = Some(minutes);
        self
    }
    
    pub fn with_research(mut self, enabled: bool) -> Self {
        self.self_researching = enabled;
        self
    }
    
    pub fn with_tag(mut self, tag: &str) -> Self {
        self.tags.push(tag.to_string());
        self
    }
    
    pub fn with_dependency(mut self, task_id: &str) -> Self {
        self.dependencies.push(task_id.to_string());
        self
    }
    
    pub fn build(self) -> Task {
        Task {
            id: uuid::Uuid::new_v4().to_string(),
            title: self.title,
            description: self.description,
            status: TaskStatus::Todo,
            priority: self.priority,
            subtasks: vec![],
            tags: self.tags,
            created: chrono::Utc::now(),
            due: self.due,
            completed: None,
            estimated_minutes: self.estimated_minutes,
            actual_minutes: None,
            self_researching: self.self_researching,
            research_notes: None,
            dependencies: self.dependencies,
            project_id: None,
        }
    }
}

/// Task status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskStatus {
    Todo,
    InProgress,
    Blocked,
    Completed,
    Cancelled,
}

impl std::fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Todo => write!(f, "📋 Todo"),
            Self::InProgress => write!(f, "🔄 In Progress"),
            Self::Blocked => write!(f, "🚫 Blocked"),
            Self::Completed => write!(f, "✅ Completed"),
            Self::Cancelled => write!(f, "❌ Cancelled"),
        }
    }
}

/// Task priority
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskPriority {
    Critical,
    High,
    Medium,
    Low,
    Someday,
}

impl std::fmt::Display for TaskPriority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Critical => write!(f, "🔴 Critical"),
            Self::High => write!(f, "🟠 High"),
            Self::Medium => write!(f, "🟡 Medium"),
            Self::Low => write!(f, "🟢 Low"),
            Self::Someday => write!(f, "⚪ Someday"),
        }
    }
}

/// Sub-task
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubTask {
    pub id: String,
    pub title: String,
    pub completed: bool,
    pub order: u32,
}

impl SubTask {
    pub fn new(title: &str, order: u32) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            title: title.to_string(),
            completed: false,
            order,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_task_builder() {
        let task = TaskBuilder::new("Test")
            .with_priority(TaskPriority::High)
            .with_deadline(7)
            .with_research(true)
            .build();
        
        assert_eq!(task.title, "Test");
        assert!(task.self_researching);
        assert!(task.due.is_some());
    }
    
    #[test]
    fn test_progress() {
        let mut task = TaskBuilder::new("Test").build();
        task.subtasks = vec![
            SubTask::new("A", 0),
            SubTask::new("B", 1),
        ];
        
        assert_eq!(task.progress(), 0.0);
        
        task.subtasks[0].completed = true;
        assert_eq!(task.progress(), 50.0);
    }
}
