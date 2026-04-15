//! ─── Task Research System ───

use crate::models::*;
use crate::TodoResult;

/// Task researcher
pub struct TaskResearcher;

impl TaskResearcher {
    pub fn new() -> Self { Self }
    
    /// Research a task topic
    pub async fn research(&self, topic: &str) -> TodoResult<ResearchResult> {
        // TODO: Integrate with sentient_research crate
        Ok(ResearchResult {
            topic: topic.to_string(),
            summary: format!("Research summary for: {}", topic),
            sources: vec!["Wikipedia".into(), "Documentation".into()],
            suggested_subtasks: generate_research_subtasks(topic),
            confidence: 0.8,
        })
    }
}

impl Default for TaskResearcher {
    fn default() -> Self { Self::new() }
}

/// Research result
#[derive(Debug, Clone)]
pub struct ResearchResult {
    pub topic: String,
    pub summary: String,
    pub sources: Vec<String>,
    pub suggested_subtasks: Vec<String>,
    pub confidence: f64,
}

fn generate_research_subtasks(topic: &str) -> Vec<String> {
    vec![
        format!("Research {} basics", topic),
        format!("Find best practices for {}", topic),
        format!("Identify key resources for {}", topic),
        "Create summary document".into(),
    ]
}
