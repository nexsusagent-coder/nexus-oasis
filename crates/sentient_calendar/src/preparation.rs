//! ─── Meeting Preparation System ───

use crate::models::*;

/// Meeting preparation assistant
pub struct MeetingPrep {
    templates: Vec<PrepTemplate>,
}

impl MeetingPrep {
    pub fn new() -> Self {
        Self {
            templates: default_prep_templates(),
        }
    }
    
    /// Get preparation suggestions for an event
    pub fn suggest(&self, event: &Event) -> Vec<String> {
        let mut suggestions = Vec::new();
        let summary_lower = event.summary.to_lowercase();
        
        // Check for common meeting types
        if summary_lower.contains("interview") {
            suggestions.push("📋 Review candidate's resume and portfolio".into());
            suggestions.push("📝 Prepare interview questions".into());
            suggestions.push("🔍 Check candidate's LinkedIn/GitHub".into());
        }
        
        if summary_lower.contains("standup") || summary_lower.contains("daily") {
            suggestions.push("📝 Prepare your updates: What did you do yesterday?".into());
            suggestions.push("🚧 Note any blockers you're facing".into());
        }
        
        if summary_lower.contains("review") || summary_lower.contains("demo") {
            suggestions.push("📊 Review the materials/demo beforehand".into());
            suggestions.push("✍️ Prepare feedback points".into());
        }
        
        if summary_lower.contains("1:1") || summary_lower.contains("one-on-one") {
            suggestions.push("📋 Review previous meeting notes".into());
            suggestions.push("💭 Think about topics to discuss".into());
            suggestions.push("🎯 Check goals/OKRs progress".into());
        }
        
        if summary_lower.contains("planning") || summary_lower.contains("sprint") {
            suggestions.push("📊 Review backlog items".into());
            suggestions.push("📈 Check team velocity metrics".into());
        }
        
        // Add attendees-based suggestions
        if event.attendees.len() > 5 {
            suggestions.push("👥 Large meeting - prepare talking points".into());
        }
        
        // Add location-based suggestions
        if let Some(url) = &event.meeting_url {
            suggestions.push(format!("🔗 Test meeting link: {}", url));
        }
        
        if event.duration() > chrono::Duration::hours(1) {
            suggestions.push("⏰ Long meeting - plan breaks".into());
        }
        
        // Apply templates
        for template in &self.templates {
            if template.matches(&event.summary) {
                suggestions.extend(template.suggestions.clone());
            }
        }
        
        suggestions.truncate(5); // Limit to 5 suggestions
        suggestions
    }
}

impl Default for MeetingPrep {
    fn default() -> Self {
        Self::new()
    }
}

/// Preparation suggestion
#[derive(Debug, Clone)]
pub struct PreparationSuggestion {
    pub event_id: String,
    pub suggestion: String,
    pub priority: u8,
}

struct PrepTemplate {
    keywords: Vec<&'static str>,
    suggestions: Vec<String>,
}

impl PrepTemplate {
    fn matches(&self, summary: &str) -> bool {
        let summary_lower = summary.to_lowercase();
        self.keywords.iter().any(|k| summary_lower.contains(k))
    }
}

fn default_prep_templates() -> Vec<PrepTemplate> {
    vec![
        PrepTemplate {
            keywords: vec!["client", "customer"],
            suggestions: vec![
                "📊 Review client history and previous interactions".into(),
                "📝 Prepare case studies relevant to the client".into(),
            ],
        },
        PrepTemplate {
            keywords: vec!["pitch", "presentation"],
            suggestions: vec![
                "🎯 Practice your pitch".into(),
                "📊 Prepare slides and demos".into(),
                "❓ Anticipate questions and prepare answers".into(),
            ],
        },
        PrepTemplate {
            keywords: vec!["onboard", "onboarding"],
            suggestions: vec![
                "📋 Prepare welcome materials".into(),
                "🔧 Set up accounts and access".into(),
                "📅 Schedule introductory meetings".into(),
            ],
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_prep_suggestions() {
        let prep = MeetingPrep::new();
        let event = Event::new("Team standup")
            .starts(chrono::Utc::now() + chrono::Duration::hours(1));
        
        let suggestions = prep.suggest(&event);
        assert!(!suggestions.is_empty());
    }
}
