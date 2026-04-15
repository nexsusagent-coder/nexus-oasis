//! GitHub connector - Notifications, Issues, PRs

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    Connector, ConnectorError, ConnectorResult, ConnectorStatus, Credentials,
    CredentialType, Document, DocumentType, GitHubNotification, SyncConfig, SyncResult,
};

/// GitHub connector
pub struct GitHubConnector {
    token: Option<String>,
    status: ConnectorStatus,
    last_sync: Option<DateTime<Utc>>,
    config: SyncConfig,
    client: reqwest::Client,
}

impl GitHubConnector {
    pub fn new() -> Self {
        Self {
            token: None,
            status: ConnectorStatus::Disconnected,
            last_sync: None,
            config: SyncConfig::default(),
            client: reqwest::Client::new(),
        }
    }

    /// Get authenticated user
    pub async fn get_user(&self) -> ConnectorResult<GitHubUser> {
        let token = self.token.as_ref()
            .ok_or_else(|| ConnectorError::AuthFailed("Token not set".to_string()))?;

        let response = self.client
            .get("https://api.github.com/user")
            .header("Authorization", format!("token {}", token))
            .header("User-Agent", "SENTIENT-Connectors/1.0")
            .send()
            .await?;

        if response.status().is_success() {
            response.json().await.map_err(Into::into)
        } else {
            Err(ConnectorError::ApiError(format!("GitHub API error: {}", response.status())))
        }
    }

    /// Get notifications
    pub async fn get_notifications(&self, all: bool) -> ConnectorResult<Vec<GitHubNotification>> {
        let token = self.token.as_ref()
            .ok_or_else(|| ConnectorError::AuthFailed("Token not set".to_string()))?;

        let url = if all {
            "https://api.github.com/notifications?all=true"
        } else {
            "https://api.github.com/notifications"
        };

        let response = self.client
            .get(url)
            .header("Authorization", format!("token {}", token))
            .header("User-Agent", "SENTIENT-Connectors/1.0")
            .send()
            .await?;

        if response.status().is_success() {
            let raw: Vec<GitHubNotificationRaw> = response.json().await?;
            Ok(raw.into_iter().map(|n| n.into()).collect())
        } else {
            Err(ConnectorError::ApiError(format!("GitHub API error: {}", response.status())))
        }
    }

    /// Get repository issues
    pub async fn get_issues(&self, repo: &str, state: &str) -> ConnectorResult<Vec<GitHubIssue>> {
        let token = self.token.as_ref()
            .ok_or_else(|| ConnectorError::AuthFailed("Token not set".to_string()))?;

        let url = format!("https://api.github.com/repos/{}/issues?state={}&per_page=100", repo, state);

        let response = self.client
            .get(&url)
            .header("Authorization", format!("token {}", token))
            .header("User-Agent", "SENTIENT-Connectors/1.0")
            .send()
            .await?;

        if response.status().is_success() {
            response.json().await.map_err(Into::into)
        } else {
            Err(ConnectorError::ApiError(format!("GitHub API error: {}", response.status())))
        }
    }

    /// Get trending repositories
    /// Scrapes GitHub trending page via search API
    pub async fn get_trending(&self, language: Option<&str>, since: Option<&str>) -> ConnectorResult<Vec<TrendingRepo>> {
        // Use GitHub Search API as primary method
        let query = match language {
            Some(lang) => format!("language:{} stars:>100", lang),
            None => "stars:>100".to_string(),
        };

        let sort = "stars";
        let order = "desc";

        let url = format!(
            "https://api.github.com/search/repositories?q={}&sort={}&order={}&per_page=25",
            urlencoding::encode(&query),
            sort,
            order
        );

        let response = self.client
            .get(&url)
            .header("User-Agent", "SENTIENT-Connectors/1.0")
            .send()
            .await?;

        if response.status().is_success() {
            let search_result: GitHubSearchResult = response.json().await?;
            let repos = search_result.items.into_iter().map(|r| {
                TrendingRepo {
                    name: r.full_name.clone(),
                    description: r.description.clone(),
                    language: r.language.clone(),
                    stars: r.stargazers_count,
                    forks: r.forks_count,
                    url: r.html_url.clone(),
                    today_stars: 0, // Not available via search API
                }
            }).collect();
            Ok(repos)
        } else {
            Err(ConnectorError::ApiError(format!("GitHub Search API error: {}", response.status())))
        }
    }

    /// Get trending repositories (unauthenticated, public API)
    pub async fn get_trending_public(&self, language: Option<&str>, since: Option<&str>) -> ConnectorResult<Vec<TrendingRepo>> {
        // Use GitHub Search API without auth - limited to 10 req/min
        let query = match language {
            Some(lang) => format!("language:{} stars:>50", lang),
            None => "stars:>50".to_string(),
        };

        let url = format!(
            "https://api.github.com/search/repositories?q={}&sort=stars&order=desc&per_page=20",
            urlencoding::encode(&query),
        );

        let response = self.client
            .get(&url)
            .header("User-Agent", "SENTIENT-Connectors/1.0")
            .header("Accept", "application/vnd.github.v3+json")
            .send()
            .await?;

        if response.status().is_success() {
            let search_result: GitHubSearchResult = response.json().await?;
            Ok(search_result.items.into_iter().map(|r| {
                TrendingRepo {
                    name: r.full_name,
                    description: r.description.clone(),
                    language: r.language.clone(),
                    stars: r.stargazers_count,
                    forks: r.forks_count,
                    url: r.html_url,
                    today_stars: 0,
                }
            }).collect())
        } else {
            Err(ConnectorError::ApiError(format!("GitHub API error: {}", response.status())))
        }
    }

    /// Mark notification as read
    pub async fn mark_notification_read(&self, notification_id: &str) -> ConnectorResult<()> {
        let token = self.token.as_ref()
            .ok_or_else(|| ConnectorError::AuthFailed("Token not set".to_string()))?;

        let url = format!("https://api.github.com/notifications/threads/{}", notification_id);

        let response = self.client
            .patch(&url)
            .header("Authorization", format!("token {}", token))
            .header("User-Agent", "SENTIENT-Connectors/1.0")
            .send()
            .await?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(ConnectorError::ApiError(format!("GitHub API error: {}", response.status())))
        }
    }
}

impl Default for GitHubConnector {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Connector for GitHubConnector {
    fn connector_id(&self) -> &str { "github" }
    fn connector_name(&self) -> &str { "GitHub" }
    fn category(&self) -> &str { "development" }
    fn status(&self) -> ConnectorStatus { self.status.clone() }
    fn required_credentials(&self) -> Vec<String> { vec!["token".to_string()] }

    async fn connect(&mut self, credentials: Credentials) -> ConnectorResult<()> {
        self.status = ConnectorStatus::Connecting;
        
        let token = match credentials.cred_type {
            CredentialType::Token | CredentialType::ApiKey => {
                credentials.api_key.or(credentials.access_token)
            }
            CredentialType::OAuth2 => credentials.access_token,
            _ => None,
        };

        if let Some(token) = token {
            self.token = Some(token);
            // Test the token
            match self.get_user().await {
                Ok(_) => {
                    self.status = ConnectorStatus::Connected;
                    Ok(())
                }
                Err(e) => {
                    self.status = ConnectorStatus::Error(e.to_string());
                    Err(e)
                }
            }
        } else {
            self.status = ConnectorStatus::Error("Token required".to_string());
            Err(ConnectorError::AuthFailed("GitHub token required".to_string()))
        }
    }

    async fn disconnect(&mut self) -> ConnectorResult<()> {
        self.token = None;
        self.status = ConnectorStatus::Disconnected;
        Ok(())
    }

    async fn test_connection(&self) -> ConnectorResult<bool> {
        self.get_user().await.map(|_| true)
    }

    async fn sync(&self, since: Option<DateTime<Utc>>, config: &SyncConfig) -> ConnectorResult<SyncResult> {
        let mut result = SyncResult::new(self.connector_id());
        
        let notifications = self.get_notifications(true).await?;
        result.items_new = notifications.iter().filter(|n| !n.is_read).count();
        result.items_synced = notifications.len();
        
        Ok(result)
    }

    async fn fetch(&self, query: &str, limit: usize) -> ConnectorResult<Vec<Document>> {
        let notifications = self.get_notifications(true).await?;
        
        Ok(notifications.into_iter().take(limit).map(|n| {
            Document::new("github", DocumentType::Notification, &n.id, &n.subject)
                .with_content(&format!("{}: {}", n.repository, n.reason))
                .with_url(n.url.as_deref().unwrap_or(""))
                .with_tag(&n.notification_type)
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

// GitHub API types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubUser {
    pub login: String,
    pub id: u64,
    pub name: Option<String>,
    pub email: Option<String>,
    pub avatar_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubIssue {
    pub id: u64,
    pub number: u32,
    pub title: String,
    pub body: Option<String>,
    pub state: String,
    pub html_url: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub user: GitHubUser,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct GitHubNotificationRaw {
    id: String,
    #[serde(rename = "type")]
    notification_type: String,
    repository: GitHubRepoRef,
    subject: GitHubSubject,
    reason: String,
    updated_at: DateTime<Utc>,
    unread: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct GitHubRepoRef {
    full_name: String,
}

/// Trending repository
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrendingRepo {
    /// Repository full name (owner/repo)
    pub name: String,
    /// Description
    pub description: Option<String>,
    /// Primary language
    pub language: Option<String>,
    /// Total stars
    pub stars: u64,
    /// Total forks
    pub forks: u64,
    /// Repository URL
    pub url: String,
    /// Stars gained today (only from trending page)
    pub today_stars: u64,
}

impl TrendingRepo {
    /// Format for TTS/speech output
    pub fn to_speech_summary(&self, index: usize) -> String {
        let lang = self.language.as_deref().unwrap_or("N/A");
        let desc = self.description.as_deref().unwrap_or("");
        let today = if self.today_stars > 0 {
            format!(" (+{} bugün)", self.today_stars)
        } else {
            String::new()
        };
        format!("{}. {} - {} yıldız{}, {} forks. {} {}",
            index + 1, self.name, self.stars, today, self.forks, lang, desc)
    }
}

/// GitHub Search API result
#[derive(Debug, Clone, Serialize, Deserialize)]
struct GitHubSearchResult {
    total_count: u64,
    incomplete_results: bool,
    items: Vec<GitHubSearchRepo>,
}

/// GitHub search result item
#[derive(Debug, Clone, Serialize, Deserialize)]
struct GitHubSearchRepo {
    full_name: String,
    description: Option<String>,
    language: Option<String>,
    stargazers_count: u64,
    forks_count: u64,
    html_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct GitHubSubject {
    title: String,
    url: Option<String>,
}

impl From<GitHubNotificationRaw> for GitHubNotification {
    fn from(raw: GitHubNotificationRaw) -> Self {
        GitHubNotification {
            id: raw.id,
            notification_type: raw.notification_type,
            repository: raw.repository.full_name,
            subject: raw.subject.title,
            url: raw.subject.url,
            reason: raw.reason,
            updated_at: raw.updated_at,
            is_read: !raw.unread,
        }
    }
}
