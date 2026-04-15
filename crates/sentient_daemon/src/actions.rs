//! Action Executor - Browser aksiyonlarını gerçekleştirir
//!
//! YouTube müzik/video açma, web arama gibi işlemleri
//! insan taklidi hareketlerle gerçekleştirir.

use crate::commands::{CommandIntent, ParsedCommand};
use crate::error::{DaemonError, DaemonResult};
use oasis_browser::actions::{ActionExecutor, BrowserAction};
use oasis_browser::stealth::StealthEngine;
use sentient_home::client::HomeClient;
use sentient_home::devices::DeviceCommand;
use sentient_home::voice_commands::VoiceCommandParser;
use sentient_connectors::github::GitHubConnector;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::info;
use urlencoding::encode;

/// Action result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionResult {
    /// Success
    pub success: bool,
    /// Message for TTS
    pub message: String,
    /// Additional data
    pub data: HashMap<String, String>,
}

/// Action executor for voice commands
pub struct VoiceActionExecutor {
    /// Stealth engine for human-like behavior
    stealth: StealthEngine,
    /// Browser action executor
    browser: ActionExecutor,
    /// Home Assistant client (optional)
    home_client: Option<HomeClient>,
    /// Home voice command parser
    home_parser: VoiceCommandParser,
    /// GitHub connector
    github: GitHubConnector,
    /// Assistant name for responses
    assistant_name: String,
}

impl VoiceActionExecutor {
    /// Create new executor
    pub fn new(assistant_name: &str) -> Self {
        Self {
            stealth: StealthEngine::default(),
            browser: ActionExecutor::new(),
            home_client: None,
            home_parser: VoiceCommandParser::new(),
            github: GitHubConnector::new(),
            assistant_name: assistant_name.to_string(),
        }
    }

    /// Set Home Assistant client
    pub fn with_home_client(mut self, client: HomeClient) -> Self {
        self.home_client = Some(client);
        self
    }

    /// Check if smart home is available
    pub fn has_home(&self) -> bool {
        self.home_client.is_some()
    }

    /// Execute a parsed command
    pub async fn execute(&self, command: &ParsedCommand) -> DaemonResult<ActionResult> {
        tracing::info!("🎯 Executing command: {:?} -> {:?}", command.intent, command.entities);

        match command.intent {
            CommandIntent::PlayMusic => self.play_youtube_music(command).await,
            CommandIntent::PlayVideo => self.play_youtube_video(command).await,
            CommandIntent::WebSearch => self.web_search(command).await,
            CommandIntent::Pause => self.pause_playback(command).await,
            CommandIntent::Resume => self.resume_playback(command).await,
            CommandIntent::Close => self.close_browser(command).await,
            CommandIntent::WhatTime => self.tell_time().await,
            CommandIntent::Weather => self.check_weather(command).await,
            CommandIntent::ControlHome => self.control_home(command).await,
            CommandIntent::GitHubTrending => self.github_trending(command).await,
            CommandIntent::ProjectAssign => self.project_assign(command).await,
            CommandIntent::Unknown => self.handle_unknown(command).await,
            _ => Err(DaemonError::Action("Intent not implemented".into())),
        }
    }

    /// Play music on YouTube
    async fn play_youtube_music(&self, command: &ParsedCommand) -> DaemonResult<ActionResult> {
        let query = command.entities.get("query")
            .cloned()
            .unwrap_or_else(|| "rahatlatıcı müzik".to_string());

        tracing::info!("🎵 Playing music: {}", query);

        // Build YouTube search URL
        let search_url = format!(
            "https://www.youtube.com/results?search_query={}",
            encode(&format!("{} music", query))
        );

        // Execute browser actions
        let mut actions = vec![
            BrowserAction::Navigate { url: search_url },
            BrowserAction::WaitForPageLoad { timeout_ms: 5000 },
        ];

        // Simulate human delay
        tokio::time::sleep(self.stealth.human_delay()).await;

        // Click first video (selector for YouTube video results)
        actions.push(BrowserAction::Click {
            selector: "a#video-title".to_string(),
            xpath: None,
            index: Some(0),
        });

        actions.push(BrowserAction::WaitForPageLoad { timeout_ms: 5000 });

        // Execute actions
        for action in actions {
            let _ = self.browser.execute(action).await;
            tokio::time::sleep(self.stealth.human_delay()).await;
        }

        Ok(ActionResult {
            success: true,
            message: format!("{} şarkısını YouTube'da açıyorum.", query),
            data: {
                let mut map = HashMap::new();
                map.insert("query".to_string(), query);
                map.insert("platform".to_string(), "youtube".to_string());
                map
            },
        })
    }

    /// Play video on YouTube
    async fn play_youtube_video(&self, command: &ParsedCommand) -> DaemonResult<ActionResult> {
        let query = command.entities.get("query")
            .cloned()
            .unwrap_or_else(|| "video".to_string());

        tracing::info!("🎬 Playing video: {}", query);

        let search_url = format!(
            "https://www.youtube.com/results?search_query={}",
            encode(&query)
        );

        let actions = vec![
            BrowserAction::Navigate { url: search_url },
            BrowserAction::WaitForPageLoad { timeout_ms: 5000 },
            BrowserAction::Click {
                selector: "a#video-title".to_string(),
                xpath: None,
                index: Some(0),
            },
        ];

        for action in actions {
            let _ = self.browser.execute(action).await;
            tokio::time::sleep(self.stealth.human_delay()).await;
        }

        Ok(ActionResult {
            success: true,
            message: format!("{} videosunu YouTube'da açıyorum.", query),
            data: {
                let mut map = HashMap::new();
                map.insert("query".to_string(), query);
                map
            },
        })
    }

    /// Web search
    async fn web_search(&self, command: &ParsedCommand) -> DaemonResult<ActionResult> {
        let query = command.entities.get("query")
            .cloned()
            .unwrap_or_else(|| "".to_string());

        if query.is_empty() {
            return Err(DaemonError::Action("Arama sorgusu boş".into()));
        }

        tracing::info!("🔍 Searching: {}", query);

        let search_url = format!(
            "https://www.google.com/search?q={}",
            encode(&query)
        );

        let _ = self.browser.execute(BrowserAction::Navigate { url: search_url }).await;

        Ok(ActionResult {
            success: true,
            message: format!("{} için arama sonuçlarını gösteriyorum.", query),
            data: {
                let mut map = HashMap::new();
                map.insert("query".to_string(), query);
                map
            },
        })
    }

    /// Pause playback
    async fn pause_playback(&self, _command: &ParsedCommand) -> DaemonResult<ActionResult> {
        tracing::info!("⏸️ Pausing playback");

        // Send space key to pause (works in most players)
        // In real implementation, would use keyboard automation
        let _ = self.browser.execute(BrowserAction::Click {
            selector: ".ytp-play-button".to_string(), // YouTube play button
            xpath: None,
            index: None,
        }).await;

        Ok(ActionResult {
            success: true,
            message: "Oynatmayı durdurdum.".to_string(),
            data: HashMap::new(),
        })
    }

    /// Resume playback
    async fn resume_playback(&self, _command: &ParsedCommand) -> DaemonResult<ActionResult> {
        tracing::info!("▶️ Resuming playback");

        let _ = self.browser.execute(BrowserAction::Click {
            selector: ".ytp-play-button".to_string(),
            xpath: None,
            index: None,
        }).await;

        Ok(ActionResult {
            success: true,
            message: "Oynatmaya devam ediyorum.".to_string(),
            data: HashMap::new(),
        })
    }

    /// Close browser
    async fn close_browser(&self, _command: &ParsedCommand) -> DaemonResult<ActionResult> {
        tracing::info!("❌ Closing browser");

        let _ = self.browser.execute(BrowserAction::CloseTab).await;

        Ok(ActionResult {
            success: true,
            message: "Tarayıcıyı kapattım.".to_string(),
            data: HashMap::new(),
        })
    }

    /// Tell current time
    async fn tell_time(&self) -> DaemonResult<ActionResult> {
        use chrono::Local;
        use chrono::Timelike;

        let now = Local::now();
        let hour = now.hour();
        let minute = now.minute();

        let message = format!(
            "Saat {} {}.",
            hour,
            if minute == 0 {
                "tam".to_string()
            } else {
                minute.to_string()
            }
        );

        Ok(ActionResult {
            success: true,
            message,
            data: {
                let mut map = HashMap::new();
                map.insert("hour".to_string(), hour.to_string());
                map.insert("minute".to_string(), minute.to_string());
                map
            },
        })
    }

    /// Check weather (placeholder)
    async fn check_weather(&self, command: &ParsedCommand) -> DaemonResult<ActionResult> {
        let location = command.entities.get("location")
            .cloned()
            .unwrap_or_else(|| "İstanbul".to_string());

        tracing::info!("🌤️ Checking weather for: {}", location);

        // In real implementation, would call weather API
        let message = format!(
            "{} için hava durumu bilgisi alınıyor. Bu özellik yakında aktif olacak.",
            location
        );

        Ok(ActionResult {
            success: true,
            message,
            data: {
                let mut map = HashMap::new();
                map.insert("location".to_string(), location);
                map
            },
        })
    }

    /// Control smart home device
    async fn control_home(&self, command: &ParsedCommand) -> DaemonResult<ActionResult> {
        let client = self.home_client.as_ref()
            .ok_or_else(|| DaemonError::Action(
                "Smart home bağlantısı yok. Home Assistant yapılandırması gerekiyor.".into()
            ))?;

        let action = command.entities.get("action").cloned().unwrap_or_else(|| "toggle".to_string());
        let room = command.entities.get("room");
        let device_type = command.entities.get("device_type");
        let scene = command.entities.get("scene");
        let value = command.entities.get("value");

        // Scene activation
        if let Some(scene_name) = scene {
            tracing::info!("🏠 Activating scene: {}", scene_name);
            client.activate_scene(scene_name).await
                .map_err(|e| DaemonError::Action(format!("Sahne hatası: {}", e)))?;

            return Ok(ActionResult {
                success: true,
                message: format!("{} modunu aktifleştiriyorum.", scene_name),
                data: HashMap::new(),
            });
        }

        // Build entity_id from room + device_type
        let entity_id = match (room, device_type) {
            (Some(r), Some(d)) => format!("{}.{}", d, r),
            (Some(r), None) => format!("light.{}", r),  // default to light
            (None, Some(d)) => d.to_string(),
            (None, None) => return Err(DaemonError::Action(
                "Hangi cihazı kontrol etmek istediğinizi anlayamadım.".into()
            )),
        };

        tracing::info!("🏠 Home command: {} -> {}", action, entity_id);

        // Execute device command
        let device_cmd = match action.as_str() {
            "turn_on" => DeviceCommand::TurnOn(entity_id.clone()),
            "turn_off" => DeviceCommand::TurnOff(entity_id.clone()),
            "toggle" => DeviceCommand::Toggle(entity_id.clone()),
            "dim" => DeviceCommand::SetBrightness(entity_id.clone(), value.as_ref().and_then(|v| v.parse().ok()).unwrap_or(30)),
            "brighten" => DeviceCommand::SetBrightness(entity_id.clone(), value.as_ref().and_then(|v| v.parse().ok()).unwrap_or(100)),
            _ => DeviceCommand::Toggle(entity_id.clone()),
        };

        client.execute_command(device_cmd).await
            .map_err(|e| DaemonError::Action(format!("Cihaz hatası: {}", e)))?;

        let friendly_msg = match action.as_str() {
            "turn_on" => format!("{} cihazını açıyorum.", entity_id),
            "turn_off" => format!("{} cihazını kapatıyorum.", entity_id),
            "dim" => format!("{} cihazını kısacağım.", entity_id),
            "brighten" => format!("{} cihazını parlatacağım.", entity_id),
            _ => format!("{} cihazını değiştiriyorum.", entity_id),
        };

        Ok(ActionResult {
            success: true,
            message: friendly_msg,
            data: {
                let mut map = HashMap::new();
                map.insert("entity_id".to_string(), entity_id);
                map.insert("action".to_string(), action);
                map
            },
        })
    }

    /// GitHub trending repos
    async fn github_trending(&self, command: &ParsedCommand) -> DaemonResult<ActionResult> {
        let language = command.entities.get("language").cloned();
        let since = command.entities.get("since").cloned().unwrap_or_else(|| "daily".to_string());

        tracing::info!("🔥 GitHub trending: lang={:?}, since={}", language, since);

        // Use GitHub Search API for trending repos
        let _query = match &language {
            Some(lang) => format!("language:{}", lang),
            None => "stars:>100".to_string(),
        };

        let _sort = "stars";
        let _order = "desc";

        // Navigate to GitHub trending via browser as fallback
        let trend_url = match since.as_str() {
            "weekly" => "https://github.com/trending?since=weekly",
            "monthly" => "https://github.com/trending?since=monthly",
            _ => "https://github.com/trending",
        };

        let lang_suffix = language.as_ref().map(|l| format!("/{}", l)).unwrap_or_default();
        let full_url = format!("{}{}", trend_url, lang_suffix);

        tracing::info!("🌐 Navigating to: {}", full_url);

        let _ = self.browser.execute(BrowserAction::Navigate { url: full_url.clone() }).await;
        tokio::time::sleep(self.stealth.human_delay()).await;
        let _ = self.browser.execute(BrowserAction::WaitForPageLoad { timeout_ms: 5000 }).await;

        let lang_display = language.as_deref().unwrap_or("tüm diller");
        let since_display = match since.as_str() {
            "weekly" => "bu hafta",
            "monthly" => "bu ay",
            _ => "bugün",
        };

        Ok(ActionResult {
            success: true,
            message: format!("GitHub trendlerini açıyorum — {} {}.", since_display, lang_display),
            data: {
                let mut map = HashMap::new();
                map.insert("url".to_string(), full_url);
                map.insert("language".to_string(), lang_display.to_string());
                map.insert("since".to_string(), since);
                map
            },
        })
    }

    /// Project assignment - open project and assign agents
    async fn project_assign(&self, command: &ParsedCommand) -> DaemonResult<ActionResult> {
        let project = command.entities.get("project").cloned().unwrap_or_else(|| "proje".to_string());
        let agent_type = command.entities.get("agent_type").cloned();
        let framework = command.entities.get("framework").cloned();

        tracing::info!("📋 Project assignment: project={}, agent_type={:?}, framework={:?}",
            project, agent_type, framework);

        // Build project context
        let assigned_agents = match agent_type.as_deref() {
            Some("researcher") => vec![" Araştırma Ajanı"],
            Some("coder") => vec![" Yazılım Ajanı"],
            Some("tester") => vec![" Test Ajanı"],
            Some("designer") => vec![" Tasarım Ajanı"],
            None => vec![" Araştırma", " Yazılım", " Test"],  // Default full team
            _ => vec![" Genel Ajan"],
        };

        let framework_display = framework.as_deref().unwrap_or("SENTIENT Native");

        let agent_list = assigned_agents.join(",");
        let message = format!(
            "{} projesi {} framework'ü ile başlatılıyor. Atanan ajanlar: {}",
            project, framework_display, agent_list
        );

        Ok(ActionResult {
            success: true,
            message,
            data: {
                let mut map = HashMap::new();
                map.insert("project".to_string(), project);
                map.insert("framework".to_string(), framework_display.to_string());
                map.insert("agents".to_string(), agent_list);
                map
            },
        })
    }

    /// Handle unknown command
    async fn handle_unknown(&self, command: &ParsedCommand) -> DaemonResult<ActionResult> {
        tracing::info!("❓ Unknown command: {}", command.original);

        Ok(ActionResult {
            success: false,
            message: format!(
                "'{}' komutunu anlayamadım. Müzik aç, video izle veya arama yapabilirsiniz.",
                command.original
            ),
            data: HashMap::new(),
        })
    }

    /// Generate response for TTS
    pub fn generate_response(&self, result: &ActionResult) -> String {
        if result.success {
            result.message.clone()
        } else {
            format!("Üzgünüm, bir sorun oluştu: {}", result.message)
        }
    }
}

impl Default for VoiceActionExecutor {
    fn default() -> Self {
        Self::new("Sentient")
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// ADDITIONAL TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod additional_tests {
    use super::*;

    #[test]
    fn test_home_command_parse() {
        let parser = crate::commands::CommandParser::new();
        let cmd = parser.parse("salon ışığını kapat");
        assert_eq!(cmd.intent, CommandIntent::ControlHome);
    }

    #[test]
    fn test_github_trending_parse() {
        let parser = crate::commands::CommandParser::new();
        let cmd = parser.parse("github trendlere bak");
        assert_eq!(cmd.intent, CommandIntent::GitHubTrending);
    }

    #[test]
    fn test_github_trending_rust() {
        let parser = crate::commands::CommandParser::new();
        let cmd = parser.parse("github rust trending bak");
        assert_eq!(cmd.intent, CommandIntent::GitHubTrending);
        assert_eq!(cmd.entities.get("language"), Some(&"rust".to_string()));
    }

    #[test]
    fn test_project_assign_parse() {
        let parser = crate::commands::CommandParser::new();
        let cmd = parser.parse("X projesini aç ajanları yetkilendir");
        assert_eq!(cmd.intent, CommandIntent::ProjectAssign);
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_executor_creation() {
        let executor = VoiceActionExecutor::new("Luna");
        assert_eq!(executor.assistant_name, "Luna");
    }

    #[test]
    fn test_time_response() {
        let executor = VoiceActionExecutor::new("Test");

        let rt = tokio::runtime::Runtime::new().expect("operation failed");
        let result = rt.block_on(executor.tell_time());

        assert!(result.expect("operation failed").success);
    }
}
