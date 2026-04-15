//! ─── SENTIENT CALENDAR INTEGRATION ───
//!
//! JARVIS-like calendar management
//!
//! # Features
//! - **Google Calendar API**: OAuth2, full calendar access
//! - **Outlook Calendar API**: Microsoft Graph API integration
//! - **Smart Reminders**: Voice-based meeting reminders
//! - **Meeting Prep**: Auto-prepare for upcoming meetings
//!
//! # Example
//! ```rust,ignore
//! use sentient_calendar::{CalendarClient, Event, Reminder};
//!
//! #[tokio::main]
//! async fn main() {
//!     let client = CalendarClient::google_from_env().await?;
//!     
//!     // Get today's events
//!     let events = client.get_events_today().await?;
//!     
//!     // Add reminder for next meeting
//!     if let Some(next) = events.first() {
//!         client.add_voice_reminder(next, "15 minutes before").await?;
//!     }
//! }
//! ```

pub mod client;
pub mod google;
pub mod outlook;
pub mod models;
pub mod reminder;
pub mod preparation;

pub use client::{CalendarClient, CalendarConfig, CalendarProvider};
pub use models::{Event, EventBuilder, Attendee, EventStatus, RecurrenceRule};
pub use reminder::{Reminder, ReminderType, VoiceReminder};
pub use preparation::{MeetingPrep, PreparationSuggestion};

pub mod prelude {
    pub use crate::{CalendarClient, Event, EventBuilder, Reminder};
}

/// Result type for calendar operations
pub type CalendarResult<T> = Result<T, CalendarError>;

/// Error type
#[derive(Debug, thiserror::Error)]
pub enum CalendarError {
    #[error("Authentication failed: {0}")]
    AuthFailed(String),
    
    #[error("Connection error: {0}")]
    ConnectionError(String),
    
    #[error("API error: {0}")]
    ApiError(String),
    
    #[error("Event not found: {0}")]
    NotFound(String),
    
    #[error("Invalid date/time: {0}")]
    DateTimeError(String),
    
    #[error(transparent)]
    Io(#[from] std::io::Error),
    
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    
    #[error(transparent)]
    Http(#[from] reqwest::Error),
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_event_creation() {
        let event = Event::new("Test Meeting")
            .starts(chrono::Utc::now())
            .with_duration(chrono::Duration::hours(1));
        
        assert_eq!(event.summary, "Test Meeting");
    }
}
