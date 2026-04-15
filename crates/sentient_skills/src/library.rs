//! ─── Skill Library ───

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::models::*;
use crate::{SkillResult, SkillError};

/// Skill statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillStats {
    pub total_executions: u64,
    pub successful_executions: u64,
    pub failed_executions: u64,
    pub average_duration_ms: u64,
    pub last_execution: Option<chrono::DateTime<chrono::Utc>>,
}

impl Default for SkillStats {
    fn default() -> Self {
        Self {
            total_executions: 0,
            successful_executions: 0,
            failed_executions: 0,
            average_duration_ms: 0,
            last_execution: None,
        }
    }
}

/// Skill library
pub struct SkillLibrary {
    skills: HashMap<String, Skill>,
    stats: HashMap<String, SkillStats>,
}

impl SkillLibrary {
    pub fn new() -> Self {
        Self {
            skills: HashMap::new(),
            stats: HashMap::new(),
        }
    }
    
    pub fn register(&mut self, skill: Skill) {
        let id = skill.id.clone();
        self.stats.insert(id.clone(), SkillStats::default());
        self.skills.insert(id, skill);
    }
    
    pub fn unregister(&mut self, id: &str) -> Option<Skill> {
        self.stats.remove(id);
        self.skills.remove(id)
    }
    
    pub fn get(&self, id: &str) -> Option<&Skill> {
        self.skills.get(id)
    }
    
    pub fn get_mut(&mut self, id: &str) -> Option<&mut Skill> {
        self.skills.get_mut(id)
    }
    
    pub fn get_all(&self) -> Vec<&Skill> {
        self.skills.values().collect()
    }
    
    pub fn search(&self, query: &str) -> Vec<&Skill> {
        let query_lower = query.to_lowercase();
        self.skills.values()
            .filter(|s| s.name.to_lowercase().contains(&query_lower))
            .collect()
    }
    
    pub fn record_execution(&mut self, id: &str, success: bool, duration_ms: u64) {
        if let Some(stats) = self.stats.get_mut(id) {
            stats.total_executions += 1;
            if success {
                stats.successful_executions += 1;
            } else {
                stats.failed_executions += 1;
            }
            stats.average_duration_ms = (stats.average_duration_ms + duration_ms) / 2;
            stats.last_execution = Some(chrono::Utc::now());
        }
        
        if let Some(skill) = self.skills.get_mut(id) {
            skill.mark_used();
        }
    }
    
    pub fn get_stats(&self, id: &str) -> Option<&SkillStats> {
        self.stats.get(id)
    }
    
    pub fn count(&self) -> usize {
        self.skills.len()
    }
    
    pub fn load_defaults(&mut self) {
        let morning = Skill::new("morning_routine", "Morning routine")
            .with_category(SkillCategory::Automation);
        self.register(morning);
        
        let focus = Skill::new("focus_mode", "Focus mode")
            .with_category(SkillCategory::Productivity);
        self.register(focus);
    }
}

impl Default for SkillLibrary {
    fn default() -> Self { Self::new() }
}
