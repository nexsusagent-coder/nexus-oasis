//! ─── Progress Tracking System ───

use crate::models::*;

/// Progress tracker
pub struct ProgressTracker {
    history: Vec<ProgressUpdate>,
}

impl ProgressTracker {
    pub fn new() -> Self {
        Self { history: vec![] }
    }
    
    /// Record a progress update
    pub fn record(&mut self, update: ProgressUpdate) {
        self.history.push(update);
    }
    
    /// Get progress history for a task
    pub fn get_history(&self, task_id: &str) -> Vec<&ProgressUpdate> {
        self.history.iter()
            .filter(|u| u.task_id == task_id)
            .collect()
    }
    
    /// Calculate velocity (tasks completed per day)
    pub fn velocity(&self, days: u32) -> f64 {
        let cutoff = chrono::Utc::now() - chrono::Duration::days(days as i64);
        let completed_in_period = self.history.iter()
            .filter(|u| u.timestamp > cutoff && u.status == TaskStatus::Completed)
            .count();
        
        completed_in_period as f64 / days as f64
    }
}

impl Default for ProgressTracker {
    fn default() -> Self {
        Self::new()
    }
}

/// Progress update
#[derive(Debug, Clone)]
pub struct ProgressUpdate {
    pub task_id: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub status: TaskStatus,
    pub progress_percent: f64,
    pub note: Option<String>,
}

impl ProgressUpdate {
    pub fn new(task_id: &str, status: TaskStatus, progress: f64) -> Self {
        Self {
            task_id: task_id.to_string(),
            timestamp: chrono::Utc::now(),
            status,
            progress_percent: progress,
            note: None,
        }
    }
    
    pub fn with_note(mut self, note: &str) -> Self {
        self.note = Some(note.to_string());
        self
    }
}
