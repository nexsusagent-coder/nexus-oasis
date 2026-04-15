//! ─── Task Decomposition System ───

use crate::models::*;
use crate::TodoResult;

/// Task decomposer
pub struct TaskDecomposer {
    min_words_for_decomposition: usize,
    max_subtasks: usize,
}

impl TaskDecomposer {
    pub fn new() -> Self {
        Self {
            min_words_for_decomposition: 5,
            max_subtasks: 10,
        }
    }
    
    /// Decompose task into subtasks
    pub async fn decompose(&self, task: &Task) -> TodoResult<Task> {
        let words: Vec<&str> = task.title.split_whitespace().collect();
        
        if words.len() < self.min_words_for_decomposition {
            return Ok(task.clone());
        }
        
        // Simple decomposition heuristics
        let subtasks = self.generate_subtasks(&task.title);
        
        let mut task = task.clone();
        for (i, subtask) in subtasks.into_iter().enumerate() {
            if i < self.max_subtasks {
                task.subtasks.push(SubTask::new(&subtask, i as u32));
            }
        }
        
        Ok(task)
    }
    
    fn generate_subtasks(&self, title: &str) -> Vec<String> {
        let title_lower = title.to_lowercase();
        
        // Check for common patterns
        if title_lower.contains("implement") || title_lower.contains("create") {
            return vec![
                "Research requirements".into(),
                "Design solution".into(),
                "Implement core functionality".into(),
                "Write tests".into(),
                "Review and polish".into(),
            ];
        }
        
        if title_lower.contains("learn") || title_lower.contains("study") {
            return vec![
                "Gather learning materials".into(),
                "Study fundamentals".into(),
                "Practice with examples".into(),
                "Create summary notes".into(),
            ];
        }
        
        if title_lower.contains("fix") || title_lower.contains("debug") {
            return vec![
                "Reproduce the issue".into(),
                "Identify root cause".into(),
                "Implement fix".into(),
                "Test the fix".into(),
                "Add regression test".into(),
            ];
        }
        
        if title_lower.contains("review") || title_lower.contains("analyze") {
            return vec![
                "Gather materials".into(),
                "Perform analysis".into(),
                "Document findings".into(),
                "Present results".into(),
            ];
        }
        
        // Default decomposition
        vec![
            "Break down task".into(),
            "Execute first step".into(),
            "Review progress".into(),
        ]
    }
}

impl Default for TaskDecomposer {
    fn default() -> Self {
        Self::new()
    }
}

/// Decomposition result
#[derive(Debug, Clone)]
pub struct DecompositionResult {
    pub original_task: String,
    pub subtasks: Vec<String>,
    pub complexity_score: f64,
}
