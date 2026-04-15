//! ─── Calendar Client ───

use crate::models::*;
use crate::reminder::VoiceReminder;
use crate::preparation::MeetingPrep;
use crate::{CalendarResult, CalendarError};

/// Calendar client configuration
#[derive(Debug, Clone)]
pub struct CalendarConfig {
    pub provider: CalendarProvider,
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub calendar_id: String,
    pub enable_reminders: bool,
    pub enable_preparation: bool,
}

impl Default for CalendarConfig {
    fn default() -> Self {
        Self {
            provider: CalendarProvider::Google,
            access_token: None,
            refresh_token: None,
            calendar_id: "primary".into(),
            enable_reminders: true,
            enable_preparation: true,
        }
    }
}

/// Calendar provider
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CalendarProvider {
    Google,
    Outlook,
    Apple,
    CalDAV,
}

/// Main calendar client
pub struct CalendarClient {
    config: CalendarConfig,
    http: reqwest::Client,
    voice_reminder: VoiceReminder,
    meeting_prep: MeetingPrep,
}

impl CalendarClient {
    /// Create new calendar client
    pub fn new(config: CalendarConfig) -> CalendarResult<Self> {
        Ok(Self {
            config,
            http: reqwest::Client::new(),
            voice_reminder: VoiceReminder::new(),
            meeting_prep: MeetingPrep::new(),
        })
    }
    
    /// Create Google client from environment
    pub async fn google_from_env() -> CalendarResult<Self> {
        let access_token = std::env::var("GOOGLE_CALENDAR_TOKEN")
            .map_err(|_| CalendarError::AuthFailed("GOOGLE_CALENDAR_TOKEN not set".into()))?;
        
        Self::new(CalendarConfig {
            provider: CalendarProvider::Google,
            access_token: Some(access_token),
            ..Default::default()
        })
    }
    
    /// Get events for a specific date
    pub async fn get_events(&self, date: chrono::NaiveDate) -> CalendarResult<Vec<Event>> {
        match self.config.provider {
            CalendarProvider::Google => self.get_google_events(date).await,
            CalendarProvider::Outlook => self.get_outlook_events(date).await,
            _ => Ok(vec![]),
        }
    }
    
    /// Get today's events
    pub async fn get_events_today(&self) -> CalendarResult<Vec<Event>> {
        self.get_events(chrono::Utc::now().date_naive()).await
    }
    
    /// Get upcoming events
    pub async fn get_upcoming(&self, limit: usize) -> CalendarResult<Vec<Event>> {
        let today = self.get_events_today().await?;
        let tomorrow = self.get_events(chrono::Utc::now().date_naive() + chrono::Duration::days(1)).await?;
        
        let mut events: Vec<Event> = today.into_iter()
            .chain(tomorrow.into_iter())
            .filter(|e| e.start > chrono::Utc::now())
            .take(limit)
            .collect();
        
        events.sort_by_key(|e| e.start);
        Ok(events)
    }
    
    /// Get next event
    pub async fn get_next_event(&self) -> CalendarResult<Option<Event>> {
        let events = self.get_upcoming(1).await?;
        Ok(events.into_iter().next())
    }
    
    /// Create new event
    pub async fn create_event(&self, event: &Event) -> CalendarResult<String> {
        match self.config.provider {
            CalendarProvider::Google => self.create_google_event(event).await,
            CalendarProvider::Outlook => self.create_outlook_event(event).await,
            _ => Err(CalendarError::ApiError("Provider not supported".into())),
        }
    }
    
    /// Update event
    pub async fn update_event(&self, event: &Event) -> CalendarResult<()> {
        match self.config.provider {
            CalendarProvider::Google => self.update_google_event(event).await,
            CalendarProvider::Outlook => self.update_outlook_event(event).await,
            _ => Err(CalendarError::ApiError("Provider not supported".into())),
        }
    }
    
    /// Delete event
    pub async fn delete_event(&self, id: &str) -> CalendarResult<()> {
        match self.config.provider {
            CalendarProvider::Google => self.delete_google_event(id).await,
            CalendarProvider::Outlook => self.delete_outlook_event(id).await,
            _ => Err(CalendarError::ApiError("Provider not supported".into())),
        }
    }
    
    /// Add voice reminder for event
    pub async fn add_voice_reminder(&self, event: &Event, before: &str) -> CalendarResult<()> {
        if !self.config.enable_reminders {
            return Ok(());
        }
        
        self.voice_reminder.schedule(event, before).await
    }
    
    /// Get preparation suggestions for event
    pub fn get_prep_suggestions(&self, event: &Event) -> Vec<String> {
        if !self.config.enable_preparation {
            return vec![];
        }
        
        self.meeting_prep.suggest(event)
    }
    
    /// Quick schedule check
    pub async fn schedule_summary(&self) -> CalendarResult<String> {
        let events = self.get_events_today().await?;
        
        if events.is_empty() {
            return Ok("📅 No events scheduled for today! Enjoy your free time.".into());
        }
        
        let now = chrono::Utc::now();
        let upcoming: Vec<_> = events.iter()
            .filter(|e| e.start > now)
            .collect();
        
        let current: Vec<_> = events.iter()
            .filter(|e| e.is_now())
            .collect();
        
        let mut summary = format!("📅 Today's schedule: {} events\n", events.len());
        
        if !current.is_empty() {
            summary.push_str("\n🔴 Happening now:\n");
            for e in current {
                summary.push_str(&format!("  • {} (until {})\n", e.summary, e.end.format("%H:%M")));
            }
        }
        
        if !upcoming.is_empty() {
            summary.push_str("\n⏰ Upcoming:\n");
            for e in upcoming.iter().take(5) {
                let time_until = e.time_until();
                let minutes = time_until.num_minutes();
                let time_str = if minutes < 60 {
                    format!("in {}m", minutes)
                } else {
                    format!("at {}", e.start.format("%H:%M"))
                };
                summary.push_str(&format!("  • {} ({})\n", e.summary, time_str));
            }
        }
        
        Ok(summary)
    }
    
    // Google API methods
    async fn get_google_events(&self, date: chrono::NaiveDate) -> CalendarResult<Vec<Event>> {
        // TODO: Implement real Google Calendar API
        Ok(vec![])
    }
    
    async fn create_google_event(&self, event: &Event) -> CalendarResult<String> {
        Ok(event.id.clone())
    }
    
    async fn update_google_event(&self, event: &Event) -> CalendarResult<()> {
        Ok(())
    }
    
    async fn delete_google_event(&self, id: &str) -> CalendarResult<()> {
        Ok(())
    }
    
    // Outlook API methods
    async fn get_outlook_events(&self, date: chrono::NaiveDate) -> CalendarResult<Vec<Event>> {
        Ok(vec![])
    }
    
    async fn create_outlook_event(&self, event: &Event) -> CalendarResult<String> {
        Ok(event.id.clone())
    }
    
    async fn update_outlook_event(&self, event: &Event) -> CalendarResult<()> {
        Ok(())
    }
    
    async fn delete_outlook_event(&self, id: &str) -> CalendarResult<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_config_default() {
        let config = CalendarConfig::default();
        assert_eq!(config.provider, CalendarProvider::Google);
    }
}
