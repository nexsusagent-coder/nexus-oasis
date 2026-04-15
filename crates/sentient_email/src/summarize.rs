//! ─── Email Summarization ───

use serde::{Deserialize, Serialize};

use crate::models::*;

/// Email summarizer configuration
#[derive(Debug, Clone)]
pub struct SummaryConfig {
    pub max_emails: usize,
    pub include_urgent: bool,
    pub include_needs_reply: bool,
    pub group_by_sender: bool,
}

impl Default for SummaryConfig {
    fn default() -> Self {
        Self {
            max_emails: 20,
            include_urgent: true,
            include_needs_reply: true,
            group_by_sender: false,
        }
    }
}

/// Email summary result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailSummary {
    /// Overall summary
    pub overview: String,
    
    /// Number of urgent emails
    pub urgent_count: usize,
    
    /// Number of emails needing reply
    pub needs_reply_count: usize,
    
    /// Key topics detected
    pub topics: Vec<String>,
    
    /// Important emails highlighted
    pub important_emails: Vec<ImportantEmail>,
    
    /// Action items extracted
    pub action_items: Vec<String>,
    
    /// Suggested responses
    pub suggested_responses: Vec<SuggestedResponse>,
}

/// Important email highlight
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportantEmail {
    pub email_id: String,
    pub subject: String,
    pub from: String,
    pub reason: String,
}

/// Suggested response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuggestedResponse {
    pub email_id: String,
    pub subject: String,
    pub suggestion: String,
}

/// Email summarizer
pub struct EmailSummarizer {
    config: SummaryConfig,
}

impl EmailSummarizer {
    pub fn new() -> Self {
        Self {
            config: SummaryConfig::default(),
        }
    }
    
    pub fn with_config(config: SummaryConfig) -> Self {
        Self { config }
    }
    
    /// Summarize a batch of emails
    pub async fn summarize(&self, emails: &[Email]) -> crate::EmailResult<EmailSummary> {
        let mut urgent_count = 0;
        let mut needs_reply_count = 0;
        let mut topics = std::collections::HashSet::new();
        let mut important_emails = Vec::new();
        let mut action_items = Vec::new();
        let mut suggested_responses = Vec::new();
        
        for email in emails.iter().take(self.config.max_emails) {
            // Check urgency
            if is_urgent(email) {
                urgent_count += 1;
                important_emails.push(ImportantEmail {
                    email_id: email.id.clone(),
                    subject: email.subject.clone(),
                    from: email.from.display(),
                    reason: "Marked as urgent".into(),
                });
            }
            
            // Check needs reply
            if needs_reply(email) {
                needs_reply_count += 1;
                suggested_responses.push(SuggestedResponse {
                    email_id: email.id.clone(),
                    subject: email.subject.clone(),
                    suggestion: generate_reply_suggestion(email),
                });
            }
            
            // Extract topics
            if let Some(text) = &email.body_text {
                for topic in extract_topics(text) {
                    topics.insert(topic);
                }
                
                // Extract action items
                for action in extract_action_items(text) {
                    action_items.push(action);
                }
            }
        }
        
        // Generate overview
        let overview = generate_overview(emails.len(), urgent_count, needs_reply_count, &topics);
        
        Ok(EmailSummary {
            overview,
            urgent_count,
            needs_reply_count,
            topics: topics.into_iter().take(10).collect(),
            important_emails,
            action_items,
            suggested_responses,
        })
    }
}

impl Default for EmailSummarizer {
    fn default() -> Self {
        Self::new()
    }
}

// Helper functions
fn is_urgent(email: &Email) -> bool {
    let urgent_keywords = [
        "urgent", "asap", "emergency", "critical", "immediately",
        "deadline", "overdue", "action required", "important",
    ];
    
    let subject_lower = email.subject.to_lowercase();
    let body_lower = email.body_text.as_ref().map(|b| b.to_lowercase()).unwrap_or_default();
    
    urgent_keywords.iter().any(|k| {
        subject_lower.contains(k) || body_lower.contains(k)
    }) || email.priority.map_or(false, |p| p <= 2)
}

fn needs_reply(email: &Email) -> bool {
    let reply_keywords = [
        "please reply", "let me know", "your thoughts",
        "what do you think", "feedback", "confirm",
        "question", "?", "can you", "would you",
    ];
    
    // Don't include sent emails
    if email.folder == EmailFolder::Sent {
        return false;
    }
    
    // Check for questions
    let body = email.body_text.as_ref().map(|b| b.to_lowercase()).unwrap_or_default();
    
    reply_keywords.iter().any(|k| body.contains(k)) && !email.flags.answered
}

fn extract_topics(text: &str) -> Vec<String> {
    // Simple keyword extraction
    let words: Vec<&str> = text.split_whitespace().collect();
    let mut topics = Vec::new();
    
    // Look for capitalized words (likely proper nouns/topics)
    for word in words {
        if word.len() > 4 && word.chars().next().map_or(false, |c| c.is_uppercase()) {
            topics.push(word.to_string());
        }
    }
    
    topics
}

fn extract_action_items(text: &str) -> Vec<String> {
    let action_patterns = [
        "todo:", "action:", "action item:", "task:", 
        "please", "need to", "don't forget to",
    ];
    
    let mut actions = Vec::new();
    
    for line in text.lines() {
        let line_lower = line.to_lowercase();
        if action_patterns.iter().any(|p| line_lower.contains(p)) {
            actions.push(line.trim().to_string());
        }
    }
    
    actions
}

fn generate_reply_suggestion(email: &Email) -> String {
    let subject_lower = email.subject.to_lowercase();
    
    if subject_lower.contains("meeting") {
        "Thank you for the meeting invitation. I'll review my calendar and confirm shortly."
    } else if subject_lower.contains("question") {
        "Thanks for your question. Let me look into this and get back to you."
    } else if subject_lower.contains("request") {
        "I've received your request and will process it. I'll update you once complete."
    } else {
        "Thank you for your email. I'll review and respond with more details soon."
    }.to_string()
}

fn generate_overview(total: usize, urgent: usize, needs_reply: usize, topics: &std::collections::HashSet<String>) -> String {
    if total == 0 {
        return "No emails to summarize.".to_string();
    }
    
    let urgent_str = if urgent > 0 {
        format!(" {} urgent,", urgent)
    } else {
        String::new()
    };
    
    format!(
        "Analyzed {} emails:{}. {} need your attention.",
        total, urgent_str, needs_reply
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_urgent_detection() {
        let email = Email::new("1", "URGENT: Please respond", EmailAddress::new("test@test.com"));
        assert!(is_urgent(&email));
    }
    
    #[test]
    fn test_needs_reply() {
        let mut email = Email::new("1", "Question", EmailAddress::new("test@test.com"));
        email.body_text = Some("please reply to this question?".to_string());
        assert!(needs_reply(&email));
    }
}
