//! ─── Calendar Models ───

use serde::{Deserialize, Serialize};

/// Calendar event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    /// Unique identifier
    pub id: String,
    
    /// Event title/summary
    pub summary: String,
    
    /// Description
    pub description: Option<String>,
    
    /// Location
    pub location: Option<String>,
    
    /// Start time
    pub start: chrono::DateTime<chrono::Utc>,
    
    /// End time
    pub end: chrono::DateTime<chrono::Utc>,
    
    /// Whether all-day
    pub all_day: bool,
    
    /// Attendees
    pub attendees: Vec<Attendee>,
    
    /// Status
    pub status: EventStatus,
    
    /// Recurrence rule
    pub recurrence: Option<RecurrenceRule>,
    
    /// Meeting URL (Zoom, Teams, etc.)
    pub meeting_url: Option<String>,
    
    /// Calendar/Provider ID
    pub calendar_id: String,
    
    /// Color ID (provider-specific)
    pub color_id: Option<String>,
    
    /// Reminders
    pub reminders: Vec<ReminderOffset>,
}

impl Event {
    /// Create new event
    pub fn new(summary: &str) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            summary: summary.to_string(),
            description: None,
            location: None,
            start: chrono::Utc::now(),
            end: chrono::Utc::now() + chrono::Duration::hours(1),
            all_day: false,
            attendees: vec![],
            status: EventStatus::Confirmed,
            recurrence: None,
            meeting_url: None,
            calendar_id: "primary".into(),
            color_id: None,
            reminders: vec![ReminderOffset::Minutes(15)],
        }
    }
    
    /// Set start time
    pub fn starts(mut self, time: chrono::DateTime<chrono::Utc>) -> Self {
        self.start = time;
        self.end = time + chrono::Duration::hours(1);
        self
    }
    
    /// Set end time
    pub fn ends(mut self, time: chrono::DateTime<chrono::Utc>) -> Self {
        self.end = time;
        self
    }
    
    /// Set duration
    pub fn with_duration(mut self, duration: chrono::Duration) -> Self {
        self.end = self.start + duration;
        self
    }
    
    /// Add attendee
    pub fn with_attendee(mut self, email: &str, name: Option<&str>) -> Self {
        self.attendees.push(Attendee {
            email: email.to_string(),
            name: name.map(|s| s.to_string()),
            response_status: ResponseStatus::NeedsAction,
            optional: false,
            organizer: false,
        });
        self
    }
    
    /// Set location
    pub fn at_location(mut self, location: &str) -> Self {
        self.location = Some(location.to_string());
        self
    }
    
    /// Set meeting URL
    pub fn with_meeting_url(mut self, url: &str) -> Self {
        self.meeting_url = Some(url.to_string());
        self
    }
    
    /// Get duration
    pub fn duration(&self) -> chrono::Duration {
        self.end - self.start
    }
    
    /// Check if event is happening now
    pub fn is_now(&self) -> bool {
        let now = chrono::Utc::now();
        now >= self.start && now <= self.end
    }
    
    /// Time until event
    pub fn time_until(&self) -> chrono::Duration {
        self.start - chrono::Utc::now()
    }
    
    /// Check if today
    pub fn is_today(&self) -> bool {
        let today = chrono::Utc::now().date_naive();
        self.start.date_naive() == today
    }
}

/// Event builder (alias for backward compat)
pub type EventBuilder = Event;

/// Attendee
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attendee {
    pub email: String,
    pub name: Option<String>,
    pub response_status: ResponseStatus,
    pub optional: bool,
    pub organizer: bool,
}

/// Response status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResponseStatus {
    NeedsAction,
    Declined,
    Tentative,
    Accepted,
}

/// Event status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EventStatus {
    Confirmed,
    Tentative,
    Cancelled,
}

/// Recurrence rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecurrenceRule {
    pub frequency: Frequency,
    pub interval: u32,
    pub count: Option<u32>,
    pub until: Option<chrono::DateTime<chrono::Utc>>,
    pub by_day: Vec<Weekday>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Frequency {
    Daily,
    Weekly,
    Monthly,
    Yearly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Weekday {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

/// Reminder offset
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReminderOffset {
    Minutes(u32),
    Hours(u32),
    Days(u32),
}

impl ReminderOffset {
    pub fn to_duration(&self) -> chrono::Duration {
        match self {
            Self::Minutes(m) => chrono::Duration::minutes(*m as i64),
            Self::Hours(h) => chrono::Duration::hours(*h as i64),
            Self::Days(d) => chrono::Duration::days(*d as i64),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_event_builder() {
        let event = Event::new("Test")
            .starts(chrono::Utc::now())
            .with_duration(chrono::Duration::hours(2))
            .with_attendee("test@example.com", Some("Test User"))
            .at_location("Conference Room A");
        
        assert_eq!(event.summary, "Test");
        assert_eq!(event.duration(), chrono::Duration::hours(2));
        assert_eq!(event.attendees.len(), 1);
    }
}
