//! ─── Priority Engine ───

use crate::models::*;

/// Priority engine for task prioritization
pub struct PriorityEngine {
    weights: PriorityWeights,
}

#[derive(Debug, Clone)]
struct PriorityWeights {
    deadline: f64,
    importance: f64,
    age: f64,
    dependencies: f64,
}

impl PriorityEngine {
    pub fn new() -> Self {
        Self {
            weights: PriorityWeights {
                deadline: 0.4,
                importance: 0.3,
                age: 0.15,
                dependencies: 0.15,
            },
        }
    }
    
    /// Calculate priority score for a task
    pub fn score(&self, task: &Task) -> PriorityScore {
        let deadline_score = self.deadline_score(task);
        let importance_score = self.importance_score(task);
        let age_score = self.age_score(task);
        let dep_score = self.dependency_score(task);
        
        let total = (deadline_score * self.weights.deadline)
            + (importance_score * self.weights.importance)
            + (age_score * self.weights.age)
            + (dep_score * self.weights.dependencies);
        
        PriorityScore {
            total: total.min(100.0),
            deadline: deadline_score,
            importance: importance_score,
            age: age_score,
            dependencies: dep_score,
        }
    }
    
    /// Sort tasks by priority
    pub fn sort_tasks(&self, tasks: &mut Vec<Task>) {
        tasks.sort_by(|a, b| {
            let score_a = self.score(a).total;
            let score_b = self.score(b).total;
            score_b.partial_cmp(&score_a).unwrap_or(std::cmp::Ordering::Equal)
        });
    }
    
    fn deadline_score(&self, task: &Task) -> f64 {
        if let Some(due) = task.due {
            let hours_until = (due - chrono::Utc::now()).num_hours() as f64;
            
            if hours_until < 0.0 {
                return 100.0; // Overdue
            } else if hours_until < 24.0 {
                return 90.0;
            } else if hours_until < 72.0 {
                return 70.0;
            } else if hours_until < 168.0 {
                return 50.0; // Within a week
            } else if hours_until < 720.0 {
                return 30.0; // Within a month
            } else {
                return 10.0;
            }
        }
        20.0 // No deadline
    }
    
    fn importance_score(&self, task: &Task) -> f64 {
        match task.priority {
            TaskPriority::Critical => 100.0,
            TaskPriority::High => 80.0,
            TaskPriority::Medium => 50.0,
            TaskPriority::Low => 30.0,
            TaskPriority::Someday => 10.0,
        }
    }
    
    fn age_score(&self, task: &Task) -> f64 {
        let age_days = (chrono::Utc::now() - task.created).num_days() as f64;
        
        if age_days < 1.0 {
            30.0
        } else if age_days < 7.0 {
            40.0 + (age_days * 2.0)
        } else if age_days < 30.0 {
            50.0 + (age_days / 2.0)
        } else {
            70.0
        }
    }
    
    fn dependency_score(&self, task: &Task) -> f64 {
        // Tasks with more dependencies should be done first
        let dep_count = task.dependencies.len() as f64;
        30.0 + (dep_count * 10.0).min(50.0)
    }
}

impl Default for PriorityEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Priority score breakdown
#[derive(Debug, Clone)]
pub struct PriorityScore {
    pub total: f64,
    pub deadline: f64,
    pub importance: f64,
    pub age: f64,
    pub dependencies: f64,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_priority_scoring() {
        let engine = PriorityEngine::new();
        let task = TaskBuilder::new("Test")
            .with_priority(TaskPriority::High)
            .with_deadline(1)
            .build();
        
        let score = engine.score(&task);
        assert!(score.total > 50.0);
    }
}
