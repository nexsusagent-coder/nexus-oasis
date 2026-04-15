//! ─── Cron Scheduling System ───
//!
//! Parses and evaluates cron expressions

use serde::{Deserialize, Serialize};
use chrono::{Datelike, Timelike};

/// Cron expression parser
pub struct CronParser;

/// Parsed cron schedule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CronSchedule {
    /// Minute (0-59)
    pub minute: CronField,
    
    /// Hour (0-23)
    pub hour: CronField,
    
    /// Day of month (1-31)
    pub day_of_month: CronField,
    
    /// Month (1-12)
    pub month: CronField,
    
    /// Day of week (0-6, Sunday=0)
    pub day_of_week: CronField,
}

/// A single cron field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CronField {
    /// Values that match
    pub values: Vec<u32>,
    
    /// Whether this matches all values
    pub is_wildcard: bool,
}

impl CronField {
    /// Create a wildcard field (matches all)
    pub fn wildcard() -> Self {
        Self {
            values: Vec::new(),
            is_wildcard: true,
        }
    }
    
    /// Create a specific value field
    pub fn specific(values: Vec<u32>) -> Self {
        Self {
            values,
            is_wildcard: false,
        }
    }
    
    /// Check if a value matches
    pub fn matches(&self, value: u32) -> bool {
        self.is_wildcard || self.values.contains(&value)
    }
}

impl CronParser {
    /// Parse a cron expression
    /// 
    /// Format: minute hour day_of_month month day_of_week
    /// Example: "0 9 * * 1-5" = 9:00 AM on weekdays
    pub fn parse(expression: &str) -> Result<CronSchedule, crate::ProactiveError> {
        let parts: Vec<&str> = expression.split_whitespace().collect();
        
        if parts.len() != 5 {
            return Err(crate::ProactiveError::InvalidCron(
                format!("Expected 5 fields, got {}", parts.len())
            ));
        }
        
        Ok(CronSchedule {
            minute: Self::parse_field(parts[0], 0, 59)?,
            hour: Self::parse_field(parts[1], 0, 23)?,
            day_of_month: Self::parse_field(parts[2], 1, 31)?,
            month: Self::parse_field(parts[3], 1, 12)?,
            day_of_week: Self::parse_field(parts[4], 0, 6)?,
        })
    }
    
    /// Parse a single field
    fn parse_field(field: &str, min: u32, max: u32) -> Result<CronField, crate::ProactiveError> {
        if field == "*" {
            return Ok(CronField::wildcard());
        }
        
        let mut values = Vec::new();
        
        // Handle ranges (e.g., "1-5")
        if field.contains('-') {
            let parts: Vec<&str> = field.split('-').collect();
            if parts.len() != 2 {
                return Err(crate::ProactiveError::InvalidCron(
                    format!("Invalid range: {}", field)
                ));
            }
            let start: u32 = parts[0].parse()
                .map_err(|_| crate::ProactiveError::InvalidCron(format!("Invalid number: {}", parts[0])))?;
            let end: u32 = parts[1].parse()
                .map_err(|_| crate::ProactiveError::InvalidCron(format!("Invalid number: {}", parts[1])))?;
            
            for v in start..=end {
                if v >= min && v <= max {
                    values.push(v);
                }
            }
        }
        // Handle lists (e.g., "1,3,5")
        else if field.contains(',') {
            for part in field.split(',') {
                let v: u32 = part.parse()
                    .map_err(|_| crate::ProactiveError::InvalidCron(format!("Invalid number: {}", part)))?;
                if v >= min && v <= max {
                    values.push(v);
                }
            }
        }
        // Handle step (e.g., "*/5")
        else if field.starts_with("*/") {
            let step: u32 = field[2..].parse()
                .map_err(|_| crate::ProactiveError::InvalidCron(format!("Invalid step: {}", field)))?;
            for v in (min..=max).step_by(step as usize) {
                values.push(v);
            }
        }
        // Single value
        else {
            let v: u32 = field.parse()
                .map_err(|_| crate::ProactiveError::InvalidCron(format!("Invalid number: {}", field)))?;
            if v >= min && v <= max {
                values.push(v);
            }
        }
        
        Ok(CronField::specific(values))
    }
}

impl CronSchedule {
    /// Check if this schedule matches a given time
    pub fn matches(&self, time: &chrono::DateTime<chrono::Utc>) -> bool {
        self.minute.matches(time.minute())
            && self.hour.matches(time.hour())
            && self.day_of_month.matches(time.day())
            && self.month.matches(time.month())
            && self.day_of_week.matches(time.weekday().num_days_from_sunday())
    }
    
    /// Calculate next occurrence
    pub fn next_occurrence(&self, from: &chrono::DateTime<chrono::Utc>) -> Option<chrono::DateTime<chrono::Utc>> {
        let mut current = from.clone();
        
        // Search up to 1 year ahead
        for _ in 0..525600 { // minutes in a year
            current = current + chrono::Duration::minutes(1);
            
            if self.matches(&current) {
                return Some(current);
            }
        }
        
        None
    }
    
    /// Get human-readable description
    pub fn describe(&self) -> String {
        if self.minute.is_wildcard && self.hour.is_wildcard && self.day_of_month.is_wildcard 
            && self.month.is_wildcard && self.day_of_week.is_wildcard {
            return "Every minute".to_string();
        }
        
        let mut parts = Vec::new();
        
        if !self.minute.is_wildcard && self.minute.values.len() == 1 {
            parts.push(format!("at minute {}", self.minute.values[0]));
        }
        
        if !self.hour.is_wildcard && self.hour.values.len() == 1 {
            parts.push(format!("at hour {}", self.hour.values[0]));
        }
        
        if !self.day_of_week.is_wildcard {
            let days: Vec<&str> = self.day_of_week.values.iter()
                .map(|d| match d {
                    0 => "Sunday",
                    1 => "Monday",
                    2 => "Tuesday",
                    3 => "Wednesday",
                    4 => "Thursday",
                    5 => "Friday",
                    6 => "Saturday",
                    _ => "Unknown",
                })
                .collect();
            parts.push(format!("on {}", days.join(", ")));
        }
        
        if parts.is_empty() {
            "Custom schedule".to_string()
        } else {
            parts.join(", ")
        }
    }
}

/// Common cron patterns
pub struct CronPatterns;

impl CronPatterns {
    /// Every minute
    pub fn every_minute() -> &'static str { "* * * * *" }
    
    /// Every hour
    pub fn hourly() -> &'static str { "0 * * * *" }
    
    /// Every day at midnight
    pub fn daily() -> &'static str { "0 0 * * *" }
    
    /// Weekdays at 9 AM
    pub fn weekday_morning() -> &'static str { "0 9 * * 1-5" }
    
    /// Every Friday at 5 PM
    pub fn friday_evening() -> &'static str { "0 17 * * 5" }
    
    /// First of every month
    pub fn monthly() -> &'static str { "0 0 1 * *" }
    
    /// Every 5 minutes
    pub fn every_5_minutes() -> &'static str { "*/5 * * * *" }
    
    /// Every 15 minutes
    pub fn every_15_minutes() -> &'static str { "*/15 * * * *" }
    
    /// Twice daily (9 AM and 5 PM)
    pub fn twice_daily() -> &'static str { "0 9,17 * * *" }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_wildcard() {
        let schedule = CronParser::parse("* * * * *").unwrap();
        assert!(schedule.minute.is_wildcard);
        assert!(schedule.hour.is_wildcard);
    }
    
    #[test]
    fn test_parse_specific() {
        let schedule = CronParser::parse("30 9 * * *").unwrap();
        assert!(!schedule.minute.is_wildcard);
        assert!(schedule.minute.values.contains(&30));
        assert!(schedule.hour.values.contains(&9));
    }
    
    #[test]
    fn test_parse_range() {
        let schedule = CronParser::parse("0 9 * * 1-5").unwrap();
        assert!(schedule.day_of_week.values.contains(&1));
        assert!(schedule.day_of_week.values.contains(&5));
        assert!(!schedule.day_of_week.values.contains(&0));
    }
    
    #[test]
    fn test_parse_step() {
        let schedule = CronParser::parse("*/15 * * * *").unwrap();
        assert!(schedule.minute.values.contains(&0));
        assert!(schedule.minute.values.contains(&15));
        assert!(schedule.minute.values.contains(&30));
        assert!(schedule.minute.values.contains(&45));
    }
    
    #[test]
    fn test_matches() {
        let schedule = CronParser::parse("30 9 * * *").unwrap();
        let time = chrono::TimeZone::with_ymd_and_hms(&chrono::Utc, 2024, 1, 15, 9, 30, 0).unwrap();
        assert!(schedule.matches(&time));
        
        let other_time = chrono::TimeZone::with_ymd_and_hms(&chrono::Utc, 2024, 1, 15, 10, 0, 0).unwrap();
        assert!(!schedule.matches(&other_time));
    }
    
    #[test]
    fn test_patterns() {
        assert!(CronParser::parse(CronPatterns::every_minute()).is_ok());
        assert!(CronParser::parse(CronPatterns::weekday_morning()).is_ok());
    }
}
