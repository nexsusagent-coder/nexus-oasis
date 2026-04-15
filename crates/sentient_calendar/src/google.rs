//! ─── Google Calendar API ───

use crate::{CalendarResult, CalendarError};
use crate::models::*;

pub struct GoogleCalendar {
    access_token: String,
}

impl GoogleCalendar {
    pub fn new(access_token: String) -> Self {
        Self { access_token }
    }
    
    pub async fn list_events(&self, _date: chrono::NaiveDate) -> CalendarResult<Vec<Event>> {
        Ok(vec![])
    }
    
    pub async fn create_event(&self, _event: &Event) -> CalendarResult<String> {
        Ok(uuid::Uuid::new_v4().to_string())
    }
}
