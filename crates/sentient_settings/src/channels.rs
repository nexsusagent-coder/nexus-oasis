//! Universal Channel Gateway - 20+ Mesajlaşma Platformu Desteği
//! WhatsApp, Signal, MS Teams, iMessage, Matrix, WeChat, Google Chat ve daha fazlası

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Mesajlaşma Kanalı Yapılandırması
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelConfig {
    /// Kanal türü
    pub channel_type: ChannelType,
    
    /// Aktif mi?
    pub enabled: bool,
    
    /// Kanal adı (kullanıcı tanımlı)
    pub name: String,
    
    /// Kimlik bilgileri
    pub credentials: HashMap<String, String>,
    
    /// Webhook URL (varsa)
    pub webhook_url: Option<String>,
    
    /// Kanal özellikleri
    pub features: ChannelFeatures,
    
    /// Rate limiting
    pub rate_limit: Option<RateLimitConfig>,
    
    /// Retry konfigürasyonu
    pub retry_config: RetryConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ChannelType {
    // === MOBILE MESSENGERS ===
    /// WhatsApp Business API
    WhatsApp,
    /// Signal Messenger
    Signal,
    /// Telegram Bot API
    Telegram,
    /// Apple iMessage (Mac only)
    IMessage,
    /// WeChat Work / Enterprise WeChat
    WeChat,
    /// LINE Messenger
    Line,
    /// Viber
    Viber,
    /// KakaoTalk
    KakaoTalk,
    /// Snapchat (via API)
    Snapchat,
    
    // === ENTERPRISE PLATFORMS ===
    /// Microsoft Teams
    MSTeams,
    /// Slack
    Slack,
    /// Google Chat
    GoogleChat,
    /// Discord
    Discord,
    /// Cisco Webex
    Webex,
    /// Zoom Chat
    ZoomChat,
    /// Mattermost
    Mattermost,
    /// RocketChat
    RocketChat,
    /// Flock
    Flock,
    /// Symphony
    Symphony,
    /// Workplace by Meta
    Workplace,
    
    // === DECENTRALIZED / FEDERATED ===
    /// Matrix (Element, etc.)
    Matrix,
    /// XMPP/Jabber
    Xmpp,
    /// Session
    Session,
    /// Session
    Wire,
    /// Threema
    Threema,
    
    // === SOCIAL PLATFORMS ===
    /// Twitter/X Direct Messages
    TwitterDM,
    /// Instagram DM
    InstagramDM,
    /// Facebook Messenger
    FacebookMessenger,
    /// LinkedIn Messaging
    LinkedInMessaging,
    /// Reddit Chat
    RedditChat,
    
    // === EMAIL & SMS ===
    /// Email SMTP
    Email,
    /// SMS (Twilio, etc.)
    Sms,
    /// RCS Messaging
    Rcs,
    
    // === DEVELOPER TOOLS ===
    /// GitHub Issues/PRs
    GitHub,
    /// GitLab
    GitLab,
    /// Jira
    Jira,
    /// PagerDuty
    PagerDuty,
    
    // === CUSTOM ===
    /// Özel webhook
    CustomWebhook,
    /// Özel API
    CustomApi,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ChannelFeatures {
    /// Metin mesajı
    pub text: bool,
    /// Resim gönderme
    pub images: bool,
    /// Video gönderme
    pub video: bool,
    /// Dosya eki
    pub files: bool,
    /// Audio mesaj
    pub audio: bool,
    /// Location sharing
    pub location: bool,
    /// Buttons/Quick replies
    pub buttons: bool,
    /// Carousel/Cards
    pub cards: bool,
    /// Markdown support
    pub markdown: bool,
    /// HTML support
    pub html: bool,
    /// Code blocks
    pub code_blocks: bool,
    /// Threads
    pub threads: bool,
    /// Reactions
    pub reactions: bool,
    /// Typing indicator
    pub typing_indicator: bool,
    /// Read receipts
    pub read_receipts: bool,
    /// Edit messages
    pub edit_messages: bool,
    /// Delete messages
    pub delete_messages: bool,
    /// Reply/Quote
    pub reply: bool,
    /// Forward
    pub forward: bool,
    /// Mentions (@user)
    pub mentions: bool,
    /// Channels/Groups
    pub groups: bool,
    /// Voice calls
    pub voice_calls: bool,
    /// Video calls
    pub video_calls: bool,
    /// Screen sharing
    pub screen_sharing: bool,
    /// End-to-end encryption
    pub e2e_encryption: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    /// Mesaj/dakika
    pub messages_per_minute: u32,
    /// Mesaj/saat
    pub messages_per_hour: u32,
    /// Mesaj/gün
    pub messages_per_day: u32,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            messages_per_minute: 30,
            messages_per_hour: 500,
            messages_per_day: 5000,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryConfig {
    /// Max deneme sayısı
    pub max_retries: u8,
    /// Başlangıç bekleme (ms)
    pub initial_delay_ms: u64,
    /// Maksimum bekleme (ms)
    pub max_delay_ms: u64,
    /// Çarpan (exponential backoff)
    pub multiplier: f32,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            initial_delay_ms: 1000,
            max_delay_ms: 30000,
            multiplier: 2.0,
        }
    }
}

impl ChannelType {
    /// Kanal için varsayılan özellikleri al
    pub fn default_features(&self) -> ChannelFeatures {
        match self {
            ChannelType::WhatsApp => ChannelFeatures {
                text: true, images: true, video: true, audio: true, 
                files: true, location: true, buttons: true, cards: true,
                e2e_encryption: true, groups: true, read_receipts: true,
                reactions: true, reply: true, mentions: true,
                ..Default::default()
            },
            ChannelType::Signal => ChannelFeatures {
                text: true, images: true, video: true, audio: true,
                files: true, location: true, e2e_encryption: true,
                groups: true, read_receipts: true, reactions: true,
                typing_indicator: true, reply: true, mentions: true,
                voice_calls: true, video_calls: true,
                ..Default::default()
            },
            ChannelType::Telegram => ChannelFeatures {
                text: true, images: true, video: true, audio: true,
                files: true, location: true, buttons: true, cards: true,
                markdown: true, code_blocks: true, threads: true,
                reactions: true, edit_messages: true, delete_messages: true,
                reply: true, forward: true, mentions: true, groups: true,
                voice_calls: true, video_calls: true, typing_indicator: true,
                ..Default::default()
            },
            ChannelType::Discord => ChannelFeatures {
                text: true, images: true, video: true, audio: true,
                files: true, markdown: true, code_blocks: true, threads: true,
                reactions: true, edit_messages: true, delete_messages: true,
                reply: true, forward: true, mentions: true, groups: true,
                voice_calls: true, video_calls: true, screen_sharing: true,
                ..Default::default()
            },
            ChannelType::MSTeams => ChannelFeatures {
                text: true, images: true, video: true, audio: true,
                files: true, cards: true, markdown: true, code_blocks: true,
                threads: true, reactions: true, edit_messages: true,
                reply: true, mentions: true, groups: true,
                voice_calls: true, video_calls: true, screen_sharing: true,
                ..Default::default()
            },
            ChannelType::Slack => ChannelFeatures {
                text: true, images: true, video: true, audio: true,
                files: true, cards: true, markdown: true, code_blocks: true,
                threads: true, reactions: true, edit_messages: true, delete_messages: true,
                reply: true, mentions: true, groups: true,
                voice_calls: true, video_calls: true, screen_sharing: true,
                ..Default::default()
            },
            ChannelType::Matrix => ChannelFeatures {
                text: true, images: true, video: true, audio: true,
                files: true, location: true, markdown: true, code_blocks: true,
                threads: true, reactions: true, edit_messages: true,
                reply: true, mentions: true, groups: true,
                e2e_encryption: true, voice_calls: true, video_calls: true,
                ..Default::default()
            },
            ChannelType::IMessage => ChannelFeatures {
                text: true, images: true, video: true, audio: true,
                files: true, location: true, reactions: true,
                read_receipts: true, reply: true, groups: true,
                e2e_encryption: true,
                ..Default::default()
            },
            ChannelType::WeChat => ChannelFeatures {
                text: true, images: true, video: true, audio: true,
                files: true, location: true, cards: true, buttons: true,
                groups: true, voice_calls: true, video_calls: true,
                ..Default::default()
            },
            ChannelType::GoogleChat => ChannelFeatures {
                text: true, images: true, video: true, files: true,
                cards: true, markdown: true, code_blocks: true, threads: true,
                reactions: true, reply: true, mentions: true, groups: true,
                ..Default::default()
            },
            ChannelType::Email => ChannelFeatures {
                text: true, images: true, files: true, html: true,
                reply: true, forward: true, groups: true,
                ..Default::default()
            },
            ChannelType::Sms => ChannelFeatures {
                text: true,
                ..Default::default()
            },
            _ => ChannelFeatures::default(),
        }
    }
    
    /// Kanal için ikon emoji
    pub fn icon(&self) -> &'static str {
        match self {
            ChannelType::WhatsApp => "📱",
            ChannelType::Signal => "🔐",
            ChannelType::Telegram => "✈️",
            ChannelType::IMessage => "💬",
            ChannelType::WeChat => "🟢",
            ChannelType::Line => "💚",
            ChannelType::Viber => "💜",
            ChannelType::KakaoTalk => "🟡",
            ChannelType::Snapchat => "👻",
            ChannelType::MSTeams => "👥",
            ChannelType::Slack => "💼",
            ChannelType::GoogleChat => "💬",
            ChannelType::Discord => "🎮",
            ChannelType::Webex => "📹",
            ChannelType::ZoomChat => "🎥",
            ChannelType::Mattermost => "🗣️",
            ChannelType::RocketChat => "🚀",
            ChannelType::Matrix => "🧮",
            ChannelType::Xmpp => "⚡",
            ChannelType::TwitterDM => "🐦",
            ChannelType::InstagramDM => "📸",
            ChannelType::FacebookMessenger => "📘",
            ChannelType::LinkedInMessaging => "💼",
            ChannelType::RedditChat => "🔴",
            ChannelType::Email => "📧",
            ChannelType::Sms => "📱",
            ChannelType::GitHub => "🐙",
            ChannelType::GitLab => "🦊",
            ChannelType::Jira => "📋",
            _ => "💬",
        }
    }
    
    /// Kanal adı
    pub fn display_name(&self) -> &'static str {
        match self {
            ChannelType::WhatsApp => "WhatsApp",
            ChannelType::Signal => "Signal",
            ChannelType::Telegram => "Telegram",
            ChannelType::IMessage => "iMessage",
            ChannelType::WeChat => "WeChat",
            ChannelType::Line => "LINE",
            ChannelType::Viber => "Viber",
            ChannelType::KakaoTalk => "KakaoTalk",
            ChannelType::Snapchat => "Snapchat",
            ChannelType::MSTeams => "Microsoft Teams",
            ChannelType::Slack => "Slack",
            ChannelType::GoogleChat => "Google Chat",
            ChannelType::Discord => "Discord",
            ChannelType::Webex => "Cisco Webex",
            ChannelType::ZoomChat => "Zoom Chat",
            ChannelType::Mattermost => "Mattermost",
            ChannelType::RocketChat => "RocketChat",
            ChannelType::Flock => "Flock",
            ChannelType::Symphony => "Symphony",
            ChannelType::Workplace => "Workplace",
            ChannelType::Matrix => "Matrix",
            ChannelType::Xmpp => "XMPP",
            ChannelType::Session => "Session",
            ChannelType::Wire => "Wire",
            ChannelType::Threema => "Threema",
            ChannelType::TwitterDM => "Twitter/X DM",
            ChannelType::InstagramDM => "Instagram DM",
            ChannelType::FacebookMessenger => "Facebook Messenger",
            ChannelType::LinkedInMessaging => "LinkedIn Messaging",
            ChannelType::RedditChat => "Reddit Chat",
            ChannelType::Email => "Email",
            ChannelType::Sms => "SMS",
            ChannelType::Rcs => "RCS",
            ChannelType::GitHub => "GitHub",
            ChannelType::GitLab => "GitLab",
            ChannelType::Jira => "Jira",
            ChannelType::PagerDuty => "PagerDuty",
            ChannelType::CustomWebhook => "Custom Webhook",
            ChannelType::CustomApi => "Custom API",
        }
    }
}

impl std::fmt::Display for ChannelType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.icon(), self.display_name())
    }
}

/// Kanal Kurulum Rehberi
pub struct ChannelSetupGuide;

impl ChannelSetupGuide {
    /// Kanal için kurulum rehberi
    pub fn get_guide(channel_type: &ChannelType) -> Vec<String> {
        match channel_type {
            ChannelType::WhatsApp => vec![
                "📱 WHATSAPP BUSINESS API KURULUMU".to_string(),
                "".to_string(),
                "1. WhatsApp Business API'ye kayıt olun:".to_string(),
                "   https://business.facebook.com".to_string(),
                "".to_string(),
                "2. Business Manager'da WhatsApp hesabı oluşturun".to_string(),
                "".to_string(),
                "3. Phone Number ID alın".to_string(),
                "".to_string(),
                "4. Permanent Access Token oluşturun:".to_string(),
                "   - Facebook Developer Console → WhatsApp → API Setup".to_string(),
                "".to_string(),
                "5. Webhook yapılandırın (opsiyonel)".to_string(),
            ],
            ChannelType::Signal => vec![
                "🔐 SIGNAL MESSENGER KURULUMU".to_string(),
                "".to_string(),
                "Signal, merkeziyetsizştir ve resmi bot API'si yoktur.".to_string(),
                "Alternatifler:".to_string(),
                "".to_string(),
                "1. signal-cli (Komut satırı aracı):".to_string(),
                "   https://github.com/AsamK/signal-cli".to_string(),
                "".to_string(),
                "2. signal-bot (Node.js):".to_string(),
                "   https://github.com/signalapp/libsignal".to_string(),
                "".to_string(),
                "3. Signal Desktop + Otomasyon".to_string(),
                "   SENTIENT GUI kontrolü ile Signal Desktop kullanılabilir".to_string(),
            ],
            ChannelType::IMessage => vec![
                "💬 IMESSAGE KURULUMU (macOS Only)".to_string(),
                "".to_string(),
                "iMessage sadece Apple cihazlarda çalışır.".to_string(),
                "".to_string(),
                "1. macOS gerekli (Apple Silicon veya Intel)".to_string(),
                "".to_string(),
                "2. SENTIENT GUI kontrol modunu aktif edin".to_string(),
                "   Level 3+: Klavye/Mouse kontrolü".to_string(),
                "".to_string(),
                "3. Messages.app otomatik kontrol edilecek".to_string(),
                "".to_string(),
                "4. Alternatif: AppleScript kullanımı".to_string(),
                "   osascript ile mesaj gönderme".to_string(),
            ],
            ChannelType::WeChat => vec![
                "🟢 WECHAT ENTERPRISE KURULUMU".to_string(),
                "".to_string(),
                "1. 企业微信 (WeChat Work) hesabı oluşturun:".to_string(),
                "   https://work.weixin.qq.com".to_string(),
                "".to_string(),
                "2. CorpID ve Secret alın".to_string(),
                "".to_string(),
                "3. Agent oluşturun ve AgentId alın".to_string(),
                "".to_string(),
                "4. Access Token API'si:".to_string(),
                "   https://qyapi.weixin.qq.com/cgi-bin/gettoken".to_string(),
            ],
            ChannelType::Matrix => vec![
                "🧮 MATRIX KURULUMU".to_string(),
                "".to_string(),
                "Matrix, açık kaynaklı ve federasyonlu bir mesajlaşma protokolüdür.".to_string(),
                "".to_string(),
                "1. Matrix hesabı oluşturun (herhangi bir homeserver'da):".to_string(),
                "   - matrix.org (varsayılan)".to_string(),
                "   - Kendi homeserver'ınız".to_string(),
                "".to_string(),
                "2. Access Token alın:".to_string(),
                "   Element → Settings → Help → Access Token".to_string(),
                "".to_string(),
                "3. Homeserver URL girin:".to_string(),
                "   https://matrix-client.matrix.org".to_string(),
                "".to_string(),
                "4. Bot için özel oda oluşturun".to_string(),
            ],
            ChannelType::MSTeams => vec![
                "👥 MICROSOFT TEAMS KURULUMU".to_string(),
                "".to_string(),
                "1. Azure Portal'da uygulama kaydedin:".to_string(),
                "   https://portal.azure.com".to_string(),
                "".to_string(),
                "2. App ID ve Secret oluşturun".to_string(),
                "".to_string(),
                "3. Teams Bot channel ekleyin".to_string(),
                "".to_string(),
                "4. Manifest oluşturun ve Teams'e yükleyin".to_string(),
                "".to_string(),
                "Alternatif: Incoming Webhook".to_string(),
                "   Teams → Channel → Connectors → Incoming Webhook".to_string(),
            ],
            ChannelType::GoogleChat => vec![
                "💬 GOOGLE CHAT KURULUMU".to_string(),
                "".to_string(),
                "1. Google Cloud Console'da proje oluşturun".to_string(),
                "   https://console.cloud.google.com".to_string(),
                "".to_string(),
                "2. Chat API'yi etkinleştirin".to_string(),
                "".to_string(),
                "3. Service Account oluşturun ve JSON key indirin".to_string(),
                "".to_string(),
                "4. Bot yapılandırın:".to_string(),
                "   Google Chat → Chat apps API → Configuration".to_string(),
                "".to_string(),
                "Alternatif: Incoming Webhook".to_string(),
                "   Room → Settings → Add webhooks".to_string(),
            ],
            ChannelType::Discord => vec![
                "🎮 DISCORD BOT KURULUMU".to_string(),
                "".to_string(),
                "1. Discord Developer Portal'a gidin:".to_string(),
                "   https://discord.com/developers/applications".to_string(),
                "".to_string(),
                "2. 'New Application' tıklayın".to_string(),
                "3. Bot ismi girin (ör: SENTIENT)".to_string(),
                "4. Sol menüden 'Bot' seçin".to_string(),
                "5. 'Add Bot' tıklayın".to_string(),
                "6. Token'ı kopyalayın".to_string(),
                "".to_string(),
                "7. Botu sunucuya ekleme:".to_string(),
                "   OAuth2 → URL Generator → bot scope".to_string(),
                "   Oluşan URL'yi açın ve sunucuya ekleyin".to_string(),
            ],
            ChannelType::Telegram => vec![
                "✈️ TELEGRAM BOT KURULUMU".to_string(),
                "".to_string(),
                "1. Telegram'da @BotFather'ı açın".to_string(),
                "2. /newbot komutunu gönderin".to_string(),
                "3. Bot ismi girin (ör: MySENTIENTBot)".to_string(),
                "4. Bot username girin (ör: my_sentient_bot)".to_string(),
                "5. Token'ı kopyalayın".to_string(),
                "".to_string(),
                "Chat ID alma:".to_string(),
                "   Botu gruba ekleyin veya direkt mesaj atın".to_string(),
                "   https://api.telegram.org/bot<TOKEN>/getUpdates".to_string(),
            ],
            ChannelType::Slack => vec![
                "💼 SLACK KURULUMU".to_string(),
                "".to_string(),
                "Yöntem 1: Incoming Webhook (Basit)".to_string(),
                "   1. https://api.slack.com/apps".to_string(),
                "   2. 'Create New App' → 'From scratch'".to_string(),
                "   3. 'Incoming Webhooks' aktif edin".to_string(),
                "   4. Webhook URL kopyalayın".to_string(),
                "".to_string(),
                "Yöntem 2: Bot User (Gelişmiş)".to_string(),
                "   1. App oluşturun ve Bot User ekleyin".to_string(),
                "   2. OAuth & Permissions'dan Bot Token alın".to_string(),
                "   3. Bot'u workspace'e install edin".to_string(),
            ],
            _ => vec![
                format!("{} KURULUMU", channel_type.display_name()),
                "".to_string(),
                "Bu kanal için özel kurulum rehberi hazırlanıyor.".to_string(),
                "Lütfen ilgili platformun dokümantasyonuna bakın.".to_string(),
            ],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_channel_features() {
        let whatsapp = ChannelType::WhatsApp;
        let features = whatsapp.default_features();
        
        assert!(features.text);
        assert!(features.images);
        assert!(features.e2e_encryption);
    }
    
    #[test]
    fn test_channel_display() {
        let discord = ChannelType::Discord;
        assert_eq!(discord.display_name(), "Discord");
        assert_eq!(discord.icon(), "🎮");
    }
}
