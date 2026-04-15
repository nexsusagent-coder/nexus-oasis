//! Weather connector - OpenWeatherMap integration

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::{
    Connector, ConnectorError, ConnectorResult, ConnectorStatus, Credentials,
    CredentialType, Document, DocumentType, SyncConfig, SyncResult,
    WeatherData, WeatherForecast,
};

/// Weather connector
pub struct WeatherConnector {
    api_key: Option<String>,
    location: String,
    status: ConnectorStatus,
    last_sync: Option<DateTime<Utc>>,
    config: SyncConfig,
    client: reqwest::Client,
}

impl WeatherConnector {
    pub fn new(location: &str) -> Self {
        Self {
            api_key: None,
            location: location.to_string(),
            status: ConnectorStatus::Disconnected,
            last_sync: None,
            config: SyncConfig::default(),
            client: reqwest::Client::new(),
        }
    }

    /// Get current weather
    pub async fn get_current_weather(&self) -> ConnectorResult<WeatherData> {
        let api_key = self.api_key.as_ref()
            .ok_or_else(|| ConnectorError::AuthFailed("API key not set".to_string()))?;

        let url = format!(
            "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric",
            urlencoding::encode(&self.location),
            api_key
        );

        let response = self.client
            .get(&url)
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        self.parse_weather_response(&response)
    }

    /// Get forecast
    pub async fn get_forecast(&self, days: u32) -> ConnectorResult<Vec<WeatherForecast>> {
        let api_key = self.api_key.as_ref()
            .ok_or_else(|| ConnectorError::AuthFailed("API key not set".to_string()))?;

        let url = format!(
            "https://api.openweathermap.org/data/2.5/forecast?q={}&appid={}&units=metric&cnt={}",
            urlencoding::encode(&self.location),
            api_key,
            days * 8 // 8 readings per day (3-hour intervals)
        );

        let response = self.client
            .get(&url)
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        self.parse_forecast_response(&response)
    }

    fn parse_weather_response(&self, json: &serde_json::Value) -> ConnectorResult<WeatherData> {
        let main = json.get("main")
            .ok_or_else(|| ConnectorError::ParseError("Missing main data".to_string()))?;
        
        let weather = json.get("weather")
            .and_then(|w| w.get(0))
            .ok_or_else(|| ConnectorError::ParseError("Missing weather data".to_string()))?;

        let wind = json.get("wind").unwrap_or(&serde_json::Value::Null);

        Ok(WeatherData {
            location: self.location.clone(),
            timestamp: Utc::now(),
            temperature: main["temp"].as_f64().unwrap_or(0.0),
            feels_like: main["feels_like"].as_f64().unwrap_or(0.0),
            humidity: main["humidity"].as_u64().unwrap_or(0) as u32,
            wind_speed: wind["speed"].as_f64().unwrap_or(0.0),
            wind_direction: self.degrees_to_direction(wind["deg"].as_f64().unwrap_or(0.0)),
            condition: weather["main"].as_str().unwrap_or("Unknown").to_string(),
            condition_icon: weather["icon"].as_str().map(|s| format!(
                "https://openweathermap.org/img/wn/{}@2x.png", s
            )),
            precipitation: json.get("rain")
                .and_then(|r| r.get("1h"))
                .and_then(|v| v.as_f64())
                .unwrap_or(0.0),
            visibility: json.get("visibility")
                .and_then(|v| v.as_f64())
                .unwrap_or(10000.0) / 1000.0, // km
            forecast: Vec::new(),
        })
    }

    fn parse_forecast_response(&self, json: &serde_json::Value) -> ConnectorResult<Vec<WeatherForecast>> {
        let list = json.get("list")
            .and_then(|l| l.as_array())
            .ok_or_else(|| ConnectorError::ParseError("Missing forecast list".to_string()))?;

        let mut forecasts = Vec::new();
        for item in list.iter().step_by(8) { // One per day
            if let (Some(dt), Some(main)) = (
                item.get("dt").and_then(|d| d.as_i64()),
                item.get("main")
            ) {
                forecasts.push(WeatherForecast {
                    date: DateTime::from_timestamp(dt, 0).unwrap_or(Utc::now()),
                    temp_high: main["temp_max"].as_f64().unwrap_or(0.0),
                    temp_low: main["temp_min"].as_f64().unwrap_or(0.0),
                    condition: item.get("weather")
                        .and_then(|w| w.get(0))
                        .and_then(|w| w.get("main"))
                        .and_then(|m| m.as_str())
                        .unwrap_or("Unknown")
                        .to_string(),
                });
            }
        }

        Ok(forecasts)
    }

    fn degrees_to_direction(&self, deg: f64) -> String {
        let directions = ["N", "NE", "E", "SE", "S", "SW", "W", "NW"];
        let index = ((deg + 22.5) / 45.0) as usize % 8;
        directions[index].to_string()
    }
}

#[async_trait]
impl Connector for WeatherConnector {
    fn connector_id(&self) -> &str {
        "weather"
    }

    fn connector_name(&self) -> &str {
        "Weather (OpenWeatherMap)"
    }

    fn category(&self) -> &str {
        "weather"
    }

    fn status(&self) -> ConnectorStatus {
        self.status.clone()
    }

    fn required_credentials(&self) -> Vec<String> {
        vec!["api_key".to_string()]
    }

    async fn connect(&mut self, credentials: Credentials) -> ConnectorResult<()> {
        self.status = ConnectorStatus::Connecting;
        
        match credentials.cred_type {
            CredentialType::ApiKey => {
                if let Some(key) = credentials.api_key {
                    self.api_key = Some(key);
                    self.status = ConnectorStatus::Connected;
                    Ok(())
                } else {
                    self.status = ConnectorStatus::Error("Missing API key".to_string());
                    Err(ConnectorError::AuthFailed("API key required".to_string()))
                }
            }
            _ => {
                self.status = ConnectorStatus::Error("Invalid credential type".to_string());
                Err(ConnectorError::AuthFailed("API key required".to_string()))
            }
        }
    }

    async fn disconnect(&mut self) -> ConnectorResult<()> {
        self.api_key = None;
        self.status = ConnectorStatus::Disconnected;
        Ok(())
    }

    async fn test_connection(&self) -> ConnectorResult<bool> {
        if self.api_key.is_none() {
            return Ok(false);
        }
        
        match self.get_current_weather().await {
            Ok(_) => Ok(true),
            Err(e) => Err(e),
        }
    }

    async fn sync(&self, since: Option<DateTime<Utc>>, config: &SyncConfig) -> ConnectorResult<SyncResult> {
        let mut result = SyncResult::new(self.connector_id());
        
        match self.get_current_weather().await {
            Ok(_) => {
                result.items_new = 1;
                result.items_synced = 1;
            }
            Err(e) => {
                result.errors.push(e.to_string());
            }
        }

        Ok(result)
    }

    async fn fetch(&self, query: &str, limit: usize) -> ConnectorResult<Vec<Document>> {
        let weather = self.get_current_weather().await?;
        
        let doc = Document::new("weather", DocumentType::Weather, &self.location, 
            &format!("Weather in {}", self.location))
            .with_content(&format!(
                "Temperature: {:.1}°C (feels like {:.1}°C)\n\
                 Condition: {}\n\
                 Humidity: {}%\n\
                 Wind: {:.1} m/s {}",
                weather.temperature, weather.feels_like,
                weather.condition, weather.humidity,
                weather.wind_speed, weather.wind_direction
            ))
            .with_metadata("temperature", serde_json::json!(weather.temperature))
            .with_metadata("condition", serde_json::json!(weather.condition))
            .with_metadata("humidity", serde_json::json!(weather.humidity));

        Ok(vec![doc])
    }

    async fn get_document(&self, id: &str) -> ConnectorResult<Option<Document>> {
        let docs = self.fetch(id, 1).await?;
        Ok(docs.into_iter().next())
    }

    async fn search(&self, query: &str, limit: usize) -> ConnectorResult<Vec<Document>> {
        self.fetch(query, limit).await
    }

    fn last_sync(&self) -> Option<DateTime<Utc>> {
        self.last_sync
    }

    fn set_config(&mut self, config: SyncConfig) {
        self.config = config;
    }

    fn config(&self) -> &SyncConfig {
        &self.config
    }
}

// URL encoding helper
mod urlencoding {
    pub fn encode(s: &str) -> String {
        url::form_urlencoded::byte_serialize(s.as_bytes()).collect()
    }
}
