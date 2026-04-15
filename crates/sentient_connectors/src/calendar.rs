//! Google Calendar connector

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    Connector, ConnectorError, ConnectorResult, ConnectorStatus, Credentials,
    CredentialType, Document, DocumentType, CalendarEvent, SyncConfig, SyncResult,
    OAuthToken,
};

/// Google Calendar connector
pub struct CalendarConnector {
    token: Option<OAuthToken>,
    calendar_id: String,
    status: ConnectorStatus,
    last_sync: Option<DateTime<Utc>>,
    config: SyncConfig,
    client: reqwest::Client,
}

impl CalendarConnector {
    pub fn new() -> Self {
        Self {
            token: None,
            calendar_id: "primary".to_string(),
            status: ConnectorStatus::Disconnected,
            last_sync: None,
            config: SyncConfig::default(),
            client: reqwest::Client::new(),
        }
    }

    pub fn with_calendar(mut self, calendar_id: &str) -> Self {
        self.calendar_id = calendar_id.to_string();
        self
    }

    fn auth_header(&self) -> ConnectorResult<String> {
        let token = self.token.as_ref()
            .ok_or_else(|| ConnectorError::AuthFailed("Not authenticated".to_string()))?;
        
        if token.is_expired() {
            return Err(ConnectorError::TokenExpired);
        }
        
        Ok(format!("Bearer {}", token.access_token))
    }

    /// Get events
    pub async fn get_events(&self, time_min: DateTime<Utc>, time_max: DateTime<Utc>, max_results: u32) -> ConnectorResult<Vec<CalendarEvent>> {
        let auth = self.auth_header()?;
        
        let url = format!(
            "https://www.googleapis.com/calendar/v3/calendars/{}/events?timeMin={}&timeMax={}&maxResults={}&singleEvents=true&orderBy=startTime",
            urlencoding::encode(&self.calendar_id),
            urlencoding::encode(&time_min.to_rfc3339()),
            urlencoding::encode(&time_max.to_rfc3339()),
            max_results
        );

        let response = self.client
            .get(&url)
            .header("Authorization", &auth)
            .send()
            .await?;

        if response.status().is_success() {
            let events: CalendarEventList = response.json().await?;
            Ok(events.items.into_iter().map(|e| self.parse_event(&e)).collect())
        } else {
            Err(ConnectorError::ApiError(format!("Calendar API error: {}", response.status())))
        }
    }

    /// Get today's events
    pub async fn get_today_events(&self) -> ConnectorResult<Vec<CalendarEvent>> {
        let now = Utc::now();
        let start = now.date_naive().and_hms_opt(0, 0, 0).unwrap();
        let end = now.date_naive().and_hms_opt(23, 59, 59).unwrap();
        
        self.get_events(
            DateTime::from_naive_utc_and_offset(start, Utc),
            DateTime::from_naive_utc_and_offset(end, Utc),
            50
        ).await
    }

    /// Get upcoming events (next N days)
    pub async fn get_upcoming_events(&self, days: u32) -> ConnectorResult<Vec<CalendarEvent>> {
        let now = Utc::now();
        let end = now + chrono::Duration::days(days as i64);
        
        self.get_events(now, end, 100).await
    }

    fn parse_event(&self, e: &GoogleCalendarEvent) -> CalendarEvent {
        let start = e.start.date_time.unwrap_or_else(|| {
            e.start.date.as_ref()
                .and_then(|d| chrono::NaiveDate::parse_from_str(d, "%Y-%m-%d").ok())
                .map(|d| DateTime::from_naive_utc_and_offset(d.and_hms_opt(0, 0, 0).unwrap(), Utc))
                .unwrap_or(Utc::now())
        });

        let end = e.end.date_time.unwrap_or_else(|| {
            e.end.date.as_ref()
                .and_then(|d| chrono::NaiveDate::parse_from_str(d, "%Y-%m-%d").ok())
                .map(|d| DateTime::from_naive_utc_and_offset(d.and_hms_opt(23, 59, 59).unwrap(), Utc))
                .unwrap_or(Utc::now())
        });

        CalendarEvent {
            id: e.id.clone(),
            summary: e.summary.clone().unwrap_or_default(),
            description: e.description.clone(),
            location: e.location.clone(),
            start,
            end,
            is_all_day: e.start.date.is_some(),
            attendees: e.attendees.as_ref().map(|a| a.iter().map(|att| crate::Attendee {
                email: att.email.clone(),
                name: att.display_name.clone(),
                response_status: att.response_status.clone().unwrap_or_default(),
            }).collect()).unwrap_or_default(),
            reminders: Vec::new(),
            recurrence: e.recurrence.as_ref().and_then(|r| r.first().cloned()),
        }
    }
}

impl Default for CalendarConnector {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Connector for CalendarConnector {
    fn connector_id(&self) -> &str { "calendar" }
    fn connector_name(&self) -> &str { "Google Calendar" }
    fn category(&self) -> &str { "calendar" }
    fn status(&self) -> ConnectorStatus { self.status.clone() }
    fn required_credentials(&self) -> Vec<String> { vec!["oauth2".to_string()] }

    async fn connect(&mut self, credentials: Credentials) -> ConnectorResult<()> {
        self.status = ConnectorStatus::Connecting;
        
        match credentials.cred_type {
            CredentialType::OAuth2 => {
                if let Some(access) = credentials.access_token {
                    let mut token = OAuthToken::new(&access, credentials.expires_at
                        .map(|e| (e - Utc::now()).num_seconds() as u64)
                        .unwrap_or(3600));
                    
                    if let Some(refresh) = credentials.refresh_token {
                        token = token.with_refresh(&refresh);
                    }
                    
                    self.token = Some(token);
                    self.status = ConnectorStatus::Connected;
                    Ok(())
                } else {
                    self.status = ConnectorStatus::Error("Missing access token".to_string());
                    Err(ConnectorError::AuthFailed("OAuth2 access token required".to_string()))
                }
            }
            _ => {
                self.status = ConnectorStatus::Error("Invalid credential type".to_string());
                Err(ConnectorError::AuthFailed("OAuth2 required for Calendar".to_string()))
            }
        }
    }

    async fn disconnect(&mut self) -> ConnectorResult<()> {
        self.token = None;
        self.status = ConnectorStatus::Disconnected;
        Ok(())
    }

    async fn test_connection(&self) -> ConnectorResult<bool> {
        let auth = self.auth_header()?;
        
        let response = self.client
            .get("https://www.googleapis.com/calendar/v3/users/me/calendarList")
            .header("Authorization", &auth)
            .send()
            .await?;

        Ok(response.status().is_success())
    }

    async fn sync(&self, since: Option<DateTime<Utc>>, config: &SyncConfig) -> ConnectorResult<SyncResult> {
        let mut result = SyncResult::new(self.connector_id());
        
        let events = self.get_upcoming_events(30).await?;
        result.items_synced = events.len();
        result.items_new = events.len();
        
        Ok(result)
    }

    async fn fetch(&self, query: &str, limit: usize) -> ConnectorResult<Vec<Document>> {
        let events = self.get_upcoming_events(30).await?;
        
        Ok(events.into_iter().take(limit).map(|e| {
            let content = format!(
                "Start: {}\nEnd: {}\nLocation: {}\nDescription: {}",
                e.start.format("%Y-%m-%d %H:%M"),
                e.end.format("%Y-%m-%d %H:%M"),
                e.location.unwrap_or_default(),
                e.description.unwrap_or_default()
            );
            
            Document::new("calendar", DocumentType::Event, &e.id, &e.summary)
                .with_content(&content)
                .with_metadata("start", serde_json::json!(e.start.to_rfc3339()))
                .with_metadata("end", serde_json::json!(e.end.to_rfc3339()))
        }).collect())
    }

    async fn get_document(&self, id: &str) -> ConnectorResult<Option<Document>> {
        let docs = self.fetch("", 100).await?;
        Ok(docs.into_iter().find(|d| d.id == id))
    }

    async fn search(&self, query: &str, limit: usize) -> ConnectorResult<Vec<Document>> {
        let docs = self.fetch("", 100).await?;
        let query_lower = query.to_lowercase();
        
        Ok(docs.into_iter()
            .filter(|d| d.title.to_lowercase().contains(&query_lower))
            .take(limit)
            .collect())
    }

    fn last_sync(&self) -> Option<DateTime<Utc>> { self.last_sync }
    fn set_config(&mut self, config: SyncConfig) { self.config = config; }
    fn config(&self) -> &SyncConfig { &self.config }
}

// Google Calendar API types
#[derive(Debug, Clone, Serialize, Deserialize)]
struct CalendarEventList {
    items: Vec<GoogleCalendarEvent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct GoogleCalendarEvent {
    id: String,
    summary: Option<String>,
    description: Option<String>,
    location: Option<String>,
    start: GoogleEventTime,
    end: GoogleEventTime,
    attendees: Option<Vec<GoogleAttendee>>,
    recurrence: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct GoogleEventTime {
    date_time: Option<DateTime<Utc>>,
    date: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct GoogleAttendee {
    email: String,
    #[serde(rename = "displayName")]
    display_name: Option<String>,
    #[serde(rename = "responseStatus")]
    response_status: Option<String>,
}

mod urlencoding {
    pub fn encode(s: &str) -> String {
        url::form_urlencoded::byte_serialize(s.as_bytes()).collect()
    }
}
