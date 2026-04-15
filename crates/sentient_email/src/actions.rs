//! ─── Email Action Detection ───

use serde::{Deserialize, Serialize};

use crate::models::*;

/// Action detector
pub struct ActionDetector {
    patterns: Vec<ActionPattern>,
}

impl ActionDetector {
    pub fn new() -> Self {
        Self {
            patterns: default_action_patterns(),
        }
    }
    
    /// Detect actions from an email
    pub fn detect(&self, email: &Email) -> Vec<EmailAction> {
        let mut actions = Vec::new();
        
        let subject_lower = email.subject.to_lowercase();
        let body_lower = email.body_text.as_ref().map(|b| b.to_lowercase()).unwrap_or_default();
        let combined = format!("{} {}", subject_lower, body_lower);
        
        for pattern in &self.patterns {
            if pattern.matches(&combined) {
                actions.push(EmailAction {
                    action_type: pattern.action_type.clone(),
                    confidence: pattern.confidence,
                    description: pattern.description.clone(),
                    suggested_response: pattern.suggested_response.clone(),
                });
            }
        }
        
        // Sort by confidence
        actions.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap());
        
        actions
    }
}

impl Default for ActionDetector {
    fn default() -> Self {
        Self::new()
    }
}

/// Detected email action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailAction {
    /// Type of action needed
    pub action_type: ActionType,
    
    /// Confidence score (0.0-1.0)
    pub confidence: f64,
    
    /// Description of action
    pub description: String,
    
    /// Suggested response
    pub suggested_response: Option<String>,
}

/// Types of actions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionType {
    /// Needs a reply
    NeedsReply,
    
    /// Urgent attention needed
    Urgent,
    
    /// Calendar event to add
    AddToCalendar,
    
    /// Task to create
    CreateTask,
    
    /// Meeting to schedule
    ScheduleMeeting,
    
    /// Document to review
    ReviewDocument,
    
    /// Payment/bill to handle
    PaymentDue,
    
    /// Subscription to manage
    Subscription,
    
    /// Travel booking needed
    TravelBooking,
    
    /// Password reset required
    SecurityAlert,
    
    /// Newsletter/promotional
    Newsletter,
    
    /// Social media update
    SocialMedia,
    
    /// No action needed
    NoAction,
}

/// Action detection pattern
struct ActionPattern {
    action_type: ActionType,
    keywords: Vec<&'static str>,
    confidence: f64,
    description: String,
    suggested_response: Option<String>,
}

impl ActionPattern {
    fn matches(&self, text: &str) -> bool {
        self.keywords.iter().any(|k| text.contains(k))
    }
}

fn default_action_patterns() -> Vec<ActionPattern> {
    vec![
        // Urgent patterns
        ActionPattern {
            action_type: ActionType::Urgent,
            keywords: vec!["urgent", "asap", "emergency", "critical", "immediately"],
            confidence: 0.9,
            description: "This email requires urgent attention".into(),
            suggested_response: Some("I've marked this as urgent and will respond immediately.".into()),
        },
        
        // Needs reply
        ActionPattern {
            action_type: ActionType::NeedsReply,
            keywords: vec!["please reply", "let me know", "your thoughts", "what do you think"],
            confidence: 0.8,
            description: "This email needs a response".into(),
            suggested_response: Some("Thanks for reaching out. I'll get back to you soon.".into()),
        },
        
        // Calendar
        ActionPattern {
            action_type: ActionType::AddToCalendar,
            keywords: vec!["meeting", "appointment", "schedule", "calendar", "event"],
            confidence: 0.7,
            description: "Calendar event detected".into(),
            suggested_response: None,
        },
        
        // Task
        ActionPattern {
            action_type: ActionType::CreateTask,
            keywords: vec!["todo", "action item", "task:", "please complete", "deadline"],
            confidence: 0.75,
            description: "Task detected in email".into(),
            suggested_response: None,
        },
        
        // Payment
        ActionPattern {
            action_type: ActionType::PaymentDue,
            keywords: vec!["invoice", "payment", "bill", "due date", "receipt"],
            confidence: 0.8,
            description: "Payment-related email detected".into(),
            suggested_response: None,
        },
        
        // Security
        ActionPattern {
            action_type: ActionType::SecurityAlert,
            keywords: vec!["password", "security", "login attempt", "suspicious", "verify your"],
            confidence: 0.85,
            description: "Security-related email detected".into(),
            suggested_response: None,
        },
        
        // Newsletter
        ActionPattern {
            action_type: ActionType::Newsletter,
            keywords: vec!["unsubscribe", "newsletter", "this week", "digest", "updates"],
            confidence: 0.7,
            description: "Newsletter or digest".into(),
            suggested_response: None,
        },
        
        // Travel
        ActionPattern {
            action_type: ActionType::TravelBooking,
            keywords: vec!["flight", "hotel", "booking", "reservation", "itinerary", "check-in"],
            confidence: 0.8,
            description: "Travel booking detected".into(),
            suggested_response: None,
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_urgent_detection() {
        let detector = ActionDetector::new();
        let email = Email::new("1", "URGENT: Please respond", EmailAddress::new("test@test.com"));
        
        let actions = detector.detect(&email);
        assert!(actions.iter().any(|a| matches!(a.action_type, ActionType::Urgent)));
    }
    
    #[test]
    fn test_needs_reply_detection() {
        let detector = ActionDetector::new();
        let mut email = Email::new("1", "Question", EmailAddress::new("test@test.com"));
        email.body_text = Some("Please reply when you can".into());
        
        let actions = detector.detect(&email);
        assert!(actions.iter().any(|a| matches!(a.action_type, ActionType::NeedsReply)));
    }
}
