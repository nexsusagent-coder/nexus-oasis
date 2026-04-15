//! ─── Voice Reminder System ───

use serde::{Deserialize, Serialize};

use crate::models::*;

/// Voice reminder scheduler
pub struct VoiceReminder {
    scheduled: Vec<ScheduledReminder>,
}

#[derive(Debug, Clone)]
struct ScheduledReminder {
    event_id: String,
    event_summary: String,
    reminder_time: chrono::DateTime<chrono::Utc>,
    announced: bool,
}

impl VoiceReminder {
    pub fn new() -> Self {
        Self { scheduled: vec![] }
    }
    
    /// Schedule a voice reminder for an event
    pub async fn schedule(&self, event: &Event, before: &str) -> crate::CalendarResult<()> {
        let offset = parse_reminder_time(before);
        let reminder_time = event.start - offset;
        
        // TODO: Integrate with sentient_voice for TTS announcement
        tracing::info!(
            "Scheduled voice reminder for '{}' at {}",
            event.summary,
            reminder_time
        );
        
        Ok(())
    }
    
    /// Check for due reminders
    pub fn check_due(&mut self) -> Vec<Reminder> {
        let now = chrono::Utc::now();
        let mut due = Vec::new();
        
        for reminder in &mut self.scheduled {
            if !reminder.announced && reminder.reminder_time <= now {
                reminder.announced = true;
                due.push(Reminder {
                    event_id: reminder.event_id.clone(),
                    message: format!("You have {} starting soon", reminder.event_summary),
                    reminder_type: ReminderType::Voice,
                });
            }
        }
        
        due
    }
}

impl Default for VoiceReminder {
    fn default() -> Self {
        Self::new()
    }
}

/// Reminder
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reminder {
    pub event_id: String,
    pub message: String,
    pub reminder_type: ReminderType,
}

/// Reminder type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReminderType {
    Voice,
    Notification,
    Email,
    SMS,
}

fn parse_reminder_time(s: &str) -> chrono::Duration {
    let s = s.to_lowercase();
    
    if s.contains("min") {
        let num: i64 = s.chars().filter(|c| c.is_numeric()).collect::<String>().parse().unwrap_or(15);
        chrono::Duration::minutes(num)
    } else if s.contains("hour") {
        let num: i64 = s.chars().filter(|c| c.is_numeric()).collect::<String>().parse().unwrap_or(1);
        chrono::Duration::hours(num)
    } else {
        chrono::Duration::minutes(15)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_reminder_creation() {
        let reminder = VoiceReminder::new();
        assert!(reminder.scheduled.is_empty());
    }
}
