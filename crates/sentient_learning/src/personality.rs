//! ─── Adaptive Personality ───

use crate::LearningResult;
use serde::{Deserialize, Serialize};

/// Adaptive personality system
pub struct AdaptivePersonality {
    traits: PersonalityTraits,
    adaptation_history: Vec<AdaptationEvent>,
}

impl AdaptivePersonality {
    pub fn new() -> Self {
        Self {
            traits: PersonalityTraits::default(),
            adaptation_history: Vec::new(),
        }
    }
    
    pub fn adapt(&mut self, factor: AdaptationFactor) {
        match factor {
            AdaptationFactor::PositiveFeedback => {
                self.traits.friendliness = (self.traits.friendliness + 0.05).min(1.0);
            }
            AdaptationFactor::NegativeFeedback => {
                self.traits.formality = (self.traits.formality + 0.1).min(1.0);
            }
            AdaptationFactor::LongConversation => {
                self.traits.proactivity = (self.traits.proactivity + 0.03).min(1.0);
            }
            AdaptationFactor::QuickInteraction => {
                self.traits.brevity = (self.traits.brevity + 0.05).min(1.0);
            }
            AdaptationFactor::TechnicalQuery => {
                self.traits.technical_depth = (self.traits.technical_depth + 0.05).min(1.0);
            }
            AdaptationFactor::CreativeQuery => {
                self.traits.creativity = (self.traits.creativity + 0.05).min(1.0);
            }
        }
        
        self.adaptation_history.push(AdaptationEvent {
            factor,
            timestamp: chrono::Utc::now(),
        });
    }
    
    pub fn get_traits(&self) -> &PersonalityTraits {
        &self.traits
    }
    
    pub fn get_response_style(&self) -> ResponseStyle {
        ResponseStyle {
            formality: if self.traits.formality > 0.6 {
                "formal".to_string()
            } else {
                "casual".to_string()
            },
            length: if self.traits.brevity > 0.7 {
                "short".to_string()
            } else if self.traits.technical_depth > 0.7 {
                "detailed".to_string()
            } else {
                "medium".to_string()
            },
            tone: if self.traits.friendliness > 0.6 {
                "friendly".to_string()
            } else {
                "neutral".to_string()
            },
        }
    }
    
    /// Generate personality description for LLM prompt
    pub fn to_prompt(&self) -> String {
        let style = self.get_response_style();
        format!(
            "Personality traits:\n\
            - Formality: {:.0}%\n\
            - Friendliness: {:.0}%\n\
            - Proactivity: {:.0}%\n\
            - Technical depth: {:.0}%\n\
            - Creativity: {:.0}%\n\
            - Brevity: {:.0}%\n\n\
            Response style: {}, {}, {}",
            self.traits.formality * 100.0,
            self.traits.friendliness * 100.0,
            self.traits.proactivity * 100.0,
            self.traits.technical_depth * 100.0,
            self.traits.creativity * 100.0,
            self.traits.brevity * 100.0,
            style.formality,
            style.length,
            style.tone
        )
    }
}

impl Default for AdaptivePersonality {
    fn default() -> Self {
        Self::new()
    }
}

/// Personality traits (0.0 - 1.0 scale)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalityTraits {
    pub formality: f32,
    pub friendliness: f32,
    pub proactivity: f32,
    pub technical_depth: f32,
    pub creativity: f32,
    pub brevity: f32,
}

impl Default for PersonalityTraits {
    fn default() -> Self {
        Self {
            formality: 0.3,
            friendliness: 0.7,
            proactivity: 0.5,
            technical_depth: 0.5,
            creativity: 0.5,
            brevity: 0.3,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum AdaptationFactor {
    PositiveFeedback,
    NegativeFeedback,
    LongConversation,
    QuickInteraction,
    TechnicalQuery,
    CreativeQuery,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct AdaptationEvent {
    factor: AdaptationFactor,
    timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseStyle {
    pub formality: String,
    pub length: String,
    pub tone: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_adaptive_personality() {
        let mut personality = AdaptivePersonality::new();
        personality.adapt(AdaptationFactor::TechnicalQuery);
        
        let traits = personality.get_traits();
        assert!(traits.technical_depth > 0.5);
    }
}
