//! ─── Skill Weaver ───

use std::collections::HashMap;

use crate::models::*;
use crate::patterns::{ActionPattern, PatternDetector};
use crate::watcher::{UserAction, WatcherEvent};
use crate::{SkillResult, SkillError};

/// Weaver configuration
#[derive(Debug, Clone)]
pub struct WeaverConfig {
    pub min_pattern_occurrences: u32,
    pub min_confidence: f64,
    pub max_actions_per_skill: usize,
    pub auto_register: bool,
}

impl Default for WeaverConfig {
    fn default() -> Self {
        Self {
            min_pattern_occurrences: 3,
            min_confidence: 0.7,
            max_actions_per_skill: 20,
            auto_register: true,
        }
    }
}

/// Skill weaver
pub struct SkillWeaver {
    config: WeaverConfig,
    pattern_detector: PatternDetector,
    action_buffer: Vec<UserAction>,
    generated_skills: HashMap<String, Skill>,
}

impl SkillWeaver {
    pub fn new() -> Self {
        Self {
            config: WeaverConfig::default(),
            pattern_detector: PatternDetector::new(),
            action_buffer: vec![],
            generated_skills: HashMap::new(),
        }
    }
    
    pub fn with_config(config: WeaverConfig) -> Self {
        Self {
            config,
            pattern_detector: PatternDetector::new(),
            action_buffer: vec![],
            generated_skills: HashMap::new(),
        }
    }
    
    pub async fn process_action(&mut self, action: UserAction) -> SkillResult<Option<Skill>> {
        self.action_buffer.push(action.clone());
        
        if self.action_buffer.len() > 1000 {
            self.action_buffer.remove(0);
        }
        
        let patterns = self.pattern_detector.detect(&self.action_buffer);
        
        for pattern in patterns {
            if pattern.occurrences >= self.config.min_pattern_occurrences 
                && pattern.confidence >= self.config.min_confidence 
            {
                if let Some(skill) = self.generate_skill_from_pattern(&pattern) {
                    if self.config.auto_register {
                        self.generated_skills.insert(skill.id.clone(), skill.clone());
                    }
                    return Ok(Some(skill));
                }
            }
        }
        
        Ok(None)
    }
    
    pub async fn process_event(&mut self, event: WatcherEvent) -> SkillResult<Option<Skill>> {
        match event {
            WatcherEvent::Action(action) => self.process_action(action).await,
            WatcherEvent::PatternDetected(pattern) => {
                if let Some(skill) = self.generate_skill_from_pattern(&pattern) {
                    self.generated_skills.insert(skill.id.clone(), skill.clone());
                    return Ok(Some(skill));
                }
                Ok(None)
            }
            _ => Ok(None),
        }
    }
    
    fn generate_skill_from_pattern(&self, pattern: &ActionPattern) -> Option<Skill> {
        if pattern.actions.is_empty() { return None; }
        
        let name = self.generate_skill_name(pattern);
        let description = format!("Auto-generated from {} repeated actions", pattern.actions.len());
        
        let mut skill = Skill::new(&name, &description)
            .with_category(self.categorize_pattern(pattern));
        
        skill.auto_generated = true;
        
        for (i, action) in pattern.actions.iter().enumerate() {
            if i < self.config.max_actions_per_skill {
                skill.actions.push(self.convert_action(action, i as u32));
            }
        }
        
        Some(skill)
    }
    
    fn generate_skill_name(&self, pattern: &ActionPattern) -> String {
        format!("auto_skill_{}", uuid::Uuid::new_v4().to_string()[..8].to_string())
    }
    
    fn categorize_pattern(&self, pattern: &ActionPattern) -> SkillCategory {
        SkillCategory::Automation
    }
    
    fn convert_action(&self, action: &UserAction, order: u32) -> SkillAction {
        SkillAction::new(ActionType::Custom, &action.description)
            .with_order(order)
    }
    
    pub async fn generate_skill(&self, name: &str) -> SkillResult<Skill> {
        let patterns = self.pattern_detector.detect(&self.action_buffer);
        
        if let Some(pattern) = patterns.first() {
            if let Some(mut skill) = self.generate_skill_from_pattern(pattern) {
                skill.name = name.to_string();
                return Ok(skill);
            }
        }
        
        let recent: Vec<_> = self.action_buffer.iter().rev().take(10).rev().cloned().collect();
        
        let mut skill = Skill::new(name, "Generated from recent actions")
            .with_category(SkillCategory::Custom);
        
        for (i, action) in recent.iter().enumerate() {
            skill.actions.push(self.convert_action(action, i as u32));
        }
        
        Ok(skill)
    }
    
    pub fn get_skills(&self) -> Vec<&Skill> {
        self.generated_skills.values().collect()
    }
    
    pub fn get_skill(&self, id: &str) -> Option<&Skill> {
        self.generated_skills.get(id)
    }
    
    pub fn clear_buffer(&mut self) {
        self.action_buffer.clear();
    }
    
    pub fn set_config(&mut self, config: WeaverConfig) {
        self.config = config;
    }
}

impl Default for SkillWeaver {
    fn default() -> Self { Self::new() }
}
