//! Setup Configuration - Yapılandırma yapıları v4.0.0
//! Universal Omni-Gateway & Full Channel Support

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use anyhow::Result;

/// Ana yapılandırma
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetupConfig {
    pub language: String,
    pub config_path: String,
    pub llm: LlmConfig,
    pub api_keys: ApiKeyConfig,
    pub integrations: IntegrationConfigs,
    pub permissions: PermissionConfig,
    pub dashboard: DashboardConfig,
    /// Custom LLM Providers (Universal Gateway)
    #[serde(default)]
    pub custom_providers: Vec<serde_json::Value>,
    
    /// Dynamic Routing Mode (v4.0.0 - Human-in-the-Loop)
    #[serde(default)]
    pub routing_mode: RoutingModeConfig,
}

impl Default for SetupConfig {
    fn default() -> Self {
        Self {
            language: "tr".to_string(),
            config_path: dirs::config_dir()
                .unwrap_or_else(|| PathBuf::from(".sentient"))
                .join("config.toml")
                .to_string_lossy()
                .to_string(),
            llm: LlmConfig::default(),
            api_keys: ApiKeyConfig::default(),
            integrations: IntegrationConfigs::default(),
            permissions: PermissionConfig::default(),
            dashboard: DashboardConfig::default(),
            custom_providers: vec![],
            routing_mode: RoutingModeConfig::default(),
        }
    }
}

impl SetupConfig {
    pub fn save(&self) -> Result<()> {
        let config_path = PathBuf::from(&self.config_path);
        
        // Dizini oluştur
        if let Some(parent) = config_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        
        // TOML olarak kaydet
        let content = toml::to_string_pretty(self)?;
        std::fs::write(&config_path, content)?;
        
        // .env dosyası oluştur (API anahtarları için)
        self.save_env_file()?;
        
        Ok(())
    }
    
    pub fn load(path: &str) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: SetupConfig = toml::from_str(&content)?;
        Ok(config)
    }
    
    fn save_env_file(&self) -> Result<()> {
        let mut env_content = String::new();
        
        env_content.push_str("# ═══════════════════════════════════════════════════════════════════\n");
        env_content.push_str("#  SENTIENT NEXUS OS v1.1.0 - Universal Omni-Gateway\n");
        env_content.push_str("# ═══════════════════════════════════════════════════════════════════\n");
        env_content.push_str("#  ⚠️  Bu dosyayı ASLA public yapmayın! .gitignore'a ekleyin!\n");
        env_content.push_str("# ═══════════════════════════════════════════════════════════════════\n\n");
        
        // LLM API Keys
        env_content.push_str("# ═══ LLM PROVIDERS ═══\n");
        if let Some(ref key) = self.api_keys.openai {
            env_content.push_str(&format!("OPENAI_API_KEY={}\n", key));
        }
        if let Some(ref key) = self.api_keys.anthropic {
            env_content.push_str(&format!("ANTHROPIC_API_KEY={}\n", key));
        }
        if let Some(ref key) = self.api_keys.google {
            env_content.push_str(&format!("GOOGLE_AI_API_KEY={}\n", key));
        }
        if let Some(ref key) = self.api_keys.groq {
            env_content.push_str(&format!("GROQ_API_KEY={}\n", key));
        }
        if let Some(ref key) = self.api_keys.mistral {
            env_content.push_str(&format!("MISTRAL_API_KEY={}\n", key));
        }
        if let Some(ref key) = self.api_keys.together {
            env_content.push_str(&format!("TOGETHER_API_KEY={}\n", key));
        }
        if let Some(ref key) = self.api_keys.deepseek {
            env_content.push_str(&format!("DEEPSEEK_API_KEY={}\n", key));
        }
        
        // Custom Provider Keys
        env_content.push_str("\n# ═══ CUSTOM PROVIDERS ═══\n");
        for (name, key) in &self.api_keys.extra {
            env_content.push_str(&format!("{}_API_KEY={}\n", name.to_uppercase().replace(" ", "_"), key));
        }
        
        // Messaging Channels
        env_content.push_str("\n# ═══ MESSAGING CHANNELS ═══\n");
        if let Some(ref key) = self.api_keys.telegram {
            env_content.push_str(&format!("TELEGRAM_BOT_TOKEN={}\n", key));
        }
        if let Some(ref key) = self.api_keys.discord {
            env_content.push_str(&format!("DISCORD_BOT_TOKEN={}\n", key));
        }
        if let Some(ref key) = self.api_keys.slack {
            env_content.push_str(&format!("SLACK_WEBHOOK_URL={}\n", key));
        }
        if let Some(ref key) = self.api_keys.whatsapp {
            env_content.push_str(&format!("WHATSAPP_TOKEN={}\n", key));
        }
        if let Some(ref key) = self.api_keys.matrix {
            env_content.push_str(&format!("MATRIX_ACCESS_TOKEN={}\n", key));
        }
        
        // Developer Tools
        env_content.push_str("\n# ═══ DEVELOPER TOOLS ═══\n");
        if let Some(ref key) = self.api_keys.github {
            env_content.push_str(&format!("GITHUB_TOKEN={}\n", key));
        }
        if let Some(ref key) = self.api_keys.gitlab {
            env_content.push_str(&format!("GITLAB_TOKEN={}\n", key));
        }
        if let Some(ref key) = self.api_keys.jira {
            env_content.push_str(&format!("JIRA_API_TOKEN={}\n", key));
        }
        
        // Social Platforms
        env_content.push_str("\n# ═══ SOCIAL PLATFORMS ═══\n");
        if let Some(ref key) = self.api_keys.twitter {
            env_content.push_str(&format!("TWITTER_BEARER_TOKEN={}\n", key));
        }
        if let Some(ref key) = self.api_keys.linkedin {
            env_content.push_str(&format!("LINKEDIN_ACCESS_TOKEN={}\n", key));
        }
        
        // .env dosyasını kaydet
        let env_path = PathBuf::from(&self.config_path).parent()
            .map(|p| p.join(".env"))
            .unwrap_or_else(|| PathBuf::from(".sentient/.env"));
        
        std::fs::write(env_path, env_content)?;
        
        Ok(())
    }
}

/// LLM Yapılandırması
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmConfig {
    pub provider: String,
    pub model: String,
    pub temperature: f32,
    pub max_tokens: usize,
    pub system_prompt: Option<String>,
    /// Custom provider base URL (for universal gateway)
    #[serde(default)]
    pub base_url: Option<String>,
    /// API format (openai, anthropic, custom)
    #[serde(default)]
    pub api_format: Option<String>,
}

impl Default for LlmConfig {
    fn default() -> Self {
        Self {
            provider: "ollama".to_string(),
            model: "qwen2.5-coder:7b".to_string(),
            temperature: 0.7,
            max_tokens: 4096,
            system_prompt: Some("Sen SENTIENT NEXUS OS asistanısın. Türkçe ve İngilizce yanıt verebilirsin.".to_string()),
            base_url: None,
            api_format: None,
        }
    }
}

/// API Anahtarları - Genişletilmiş
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ApiKeyConfig {
    // LLM Providers
    pub openai: Option<String>,
    pub anthropic: Option<String>,
    pub google: Option<String>,
    pub groq: Option<String>,
    pub mistral: Option<String>,
    pub together: Option<String>,
    pub deepseek: Option<String>,
    
    // Developer Tools
    pub github: Option<String>,
    pub gitlab: Option<String>,
    pub jira: Option<String>,
    pub cloudflare: Option<String>,
    
    // Messaging Channels
    pub telegram: Option<String>,
    pub discord: Option<String>,
    pub slack: Option<String>,
    pub whatsapp: Option<String>,
    pub matrix: Option<String>,
    
    // Social Platforms
    pub twitter: Option<String>,
    pub linkedin: Option<String>,
    pub reddit: Option<String>,
    
    /// Extra keys for custom providers
    #[serde(default)]
    pub extra: HashMap<String, String>,
}

/// Entegrasyon Yapılandırması
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationConfig {
    pub enabled: bool,
    pub token: Option<String>,
    pub extra: HashMap<String, String>,
}

impl Default for IntegrationConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            token: None,
            extra: HashMap::new(),
        }
    }
}

/// Tüm Entegrasyonlar - 20+ Platform Desteği
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IntegrationConfigs {
    // Mobile Messengers
    pub telegram: Option<IntegrationConfig>,
    pub whatsapp: Option<IntegrationConfig>,
    pub signal: Option<IntegrationConfig>,
    pub imessage: Option<IntegrationConfig>,
    pub wechat: Option<IntegrationConfig>,
    pub line: Option<IntegrationConfig>,
    pub viber: Option<IntegrationConfig>,
    pub kakaotalk: Option<IntegrationConfig>,
    
    // Enterprise Platforms
    pub discord: Option<IntegrationConfig>,
    pub slack: Option<IntegrationConfig>,
    pub ms_teams: Option<IntegrationConfig>,
    pub google_chat: Option<IntegrationConfig>,
    pub webex: Option<IntegrationConfig>,
    pub zoom: Option<IntegrationConfig>,
    pub mattermost: Option<IntegrationConfig>,
    pub rocketchat: Option<IntegrationConfig>,
    
    // Decentralized
    pub matrix: Option<IntegrationConfig>,
    pub xmpp: Option<IntegrationConfig>,
    pub session: Option<IntegrationConfig>,
    pub wire: Option<IntegrationConfig>,
    pub threema: Option<IntegrationConfig>,
    
    // Social Platforms
    pub twitter: Option<IntegrationConfig>,
    pub instagram: Option<IntegrationConfig>,
    pub facebook: Option<IntegrationConfig>,
    pub linkedin: Option<IntegrationConfig>,
    pub reddit: Option<IntegrationConfig>,
    
    // Email & SMS
    pub email: Option<IntegrationConfig>,
    pub sms: Option<IntegrationConfig>,
    pub rcs: Option<IntegrationConfig>,
    
    // Developer Tools
    pub github: Option<IntegrationConfig>,
    pub gitlab: Option<IntegrationConfig>,
    pub jira: Option<IntegrationConfig>,
    pub pagerduty: Option<IntegrationConfig>,
    
    /// Extra integrations (key-value pairs for custom configs)
    #[serde(default)]
    pub extra: HashMap<String, String>,
}

/// Yetki Yapılandırması - Agent-S3 Hardware Permissions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionConfig {
    /// Default permission level (1-5)
    pub default_level: u8,
    
    /// Allow keyboard/mouse control (Agent-S3)
    pub allow_gui_control: bool,
    
    /// Allow screen recording for visual AI
    #[serde(default)]
    pub allow_screen_recording: bool,
    
    /// Allow file system operations
    pub allow_file_system: bool,
    
    /// Allow network access
    pub allow_network: bool,
    
    /// Require confirmation for dangerous operations
    pub require_confirmation: bool,
    
    /// Safe mode (restrict dangerous operations)
    pub safe_mode: bool,
    
    /// Skill execution mode: "manual", "auto_safe", "full_auto"
    #[serde(default = "default_skill_mode")]
    pub skill_mode: String,
    
    /// Allowed skills (empty = all)
    #[serde(default)]
    pub allowed_skills: Vec<String>,
    
    /// Blocked skills
    #[serde(default)]
    pub blocked_skills: Vec<String>,
}

fn default_skill_mode() -> String {
    "auto_safe".to_string()
}

impl Default for PermissionConfig {
    fn default() -> Self {
        Self {
            default_level: 2,
            allow_gui_control: false,
            allow_screen_recording: false,
            allow_file_system: true,
            allow_network: true,
            require_confirmation: true,
            safe_mode: true,
            skill_mode: default_skill_mode(),
            allowed_skills: vec![],
            blocked_skills: vec![],
        }
    }
}

/// Dashboard Yapılandırması
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardConfig {
    pub port: u16,
    pub host: String,
    pub theme: String,
    pub auto_open: bool,
    /// Enable SSL
    #[serde(default)]
    pub ssl: bool,
    /// SSL certificate path
    #[serde(default)]
    pub ssl_cert: Option<String>,
    /// SSL key path
    #[serde(default)]
    pub ssl_key: Option<String>,
}

impl Default for DashboardConfig {
    fn default() -> Self {
        Self {
            port: 8080,
            host: "0.0.0.0".to_string(),
            theme: "dark".to_string(),
            auto_open: true,
            ssl: false,
            ssl_cert: None,
            ssl_key: None,
        }
    }
}

/// Setup Durumu
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SetupStatus {
    NotStarted,
    InProgress,
    Completed,
    Failed(String),
}

/// Setup Sonucu
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetupResult {
    pub status: SetupStatus,
    pub config_path: String,
    pub integrations_enabled: Vec<String>,
    pub warnings: Vec<String>,
}

/// Dynamic Routing Mode (v4.0.0)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RoutingModeConfig {
    /// Routing mode: "fully_autonomous", "require_approval", "approval_with_override"
    pub mode: String,
    /// Cost optimization enabled
    pub cost_optimization: bool,
    /// Prefer free models when possible
    pub prefer_free: bool,
    /// Maximum latency threshold (ms)
    pub max_latency_ms: u32,
    /// Minimum quality tier: "mini", "standard", "advanced", "premium"
    pub min_quality_tier: String,
}

impl RoutingModeConfig {
    pub fn new(mode: &str) -> Self {
        Self {
            mode: mode.to_string(),
            cost_optimization: true,
            prefer_free: true,
            max_latency_ms: 5000,
            min_quality_tier: "mini".to_string(),
        }
    }
}
