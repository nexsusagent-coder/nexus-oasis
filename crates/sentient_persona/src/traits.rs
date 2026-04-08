//! ─── PERSONA TRAITS ───
//!
//! Kişilik özellikleri ve davranış kalıpları

use serde::{Deserialize, Serialize};

/// Tek bir özellik
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trait {
    pub name: String,
    pub value: TraitValue,
    pub description: String,
}

/// Özellik değeri
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TraitValue {
    Numeric(f32),
    Boolean(bool),
    Text(String),
    Level(u8), // 1-10
}

/// Davranış kalıbı
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehaviorPattern {
    pub name: String,
    pub pattern_type: PatternType,
    pub conditions: Vec<Condition>,
    pub responses: Vec<Response>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PatternType {
    Trigger,
    Contextual,
    Scheduled,
    Reactive,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Condition {
    pub field: String,
    pub operator: Operator,
    pub value: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Operator {
    Equals,
    NotEquals,
    Contains,
    GreaterThan,
    LessThan,
    Matches,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    pub action: String,
    pub params: std::collections::HashMap<String, serde_json::Value>,
}
