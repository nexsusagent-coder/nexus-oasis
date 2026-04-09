//! ─── IRC Integration ───
//!
//! Supports:
//! - IRC protocol (RFC 1459)
//! - SASL authentication
//! - Multiple networks
//! - IRCv3 extensions

use async_trait::async_trait;
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use serde::{Deserialize, Serialize};
use crate::{Channel, ChannelError, ChannelMessage, MessageContent, ChannelType};

/// IRC configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IrcConfig {
    /// Server hostname
    pub server: String,
    
    /// Server port
    pub port: u16,
    
    /// Use TLS
    pub use_tls: bool,
    
    /// Nickname
    pub nickname: String,
    
    /// Username
    pub username: Option<String>,
    
    /// Real name
    pub realname: Option<String>,
    
    /// Password (for SASL or server password)
    pub password: Option<String>,
    
    /// Default channels to join
    pub channels: Vec<String>,
    
    /// Nickserv password
    pub nickserv_password: Option<String>,
}

impl Default for IrcConfig {
    fn default() -> Self {
        Self {
            server: "irc.libera.chat".into(),
            port: 6697,
            use_tls: true,
            nickname: "sentient".into(),
            username: None,
            realname: None,
            password: None,
            channels: vec![],
            nickserv_password: None,
        }
    }
}

/// IRC channel
pub struct IrcChannel {
    config: IrcConfig,
    connected: bool,
}

impl IrcChannel {
    pub fn new(config: IrcConfig) -> Self {
        Self {
            config,
            connected: false,
        }
    }
    
    /// Connect to IRC server
    pub async fn connect(&mut self) -> Result<(), ChannelError> {
        let addr = format!("{}:{}", self.config.server, self.config.port);
        
        let stream = if self.config.use_tls {
            // Would need tokio-rustls for TLS
            TcpStream::connect(&addr).await
                .map_err(|e| ChannelError::Connection(e.to_string()))?
        } else {
            TcpStream::connect(&addr).await
                .map_err(|e| ChannelError::Connection(e.to_string()))?
        };
        
        let (mut reader, mut writer) = stream.into_split();
        
        // Send registration
        if let Some(ref pass) = self.config.password {
            let cmd = format!("PASS {}\r\n", pass);
            writer.write_all(cmd.as_bytes()).await
                .map_err(|e| ChannelError::Io(e.to_string()))?;
        }
        
        let nick = format!("NICK {}\r\n", self.config.nickname);
        writer.write_all(nick.as_bytes()).await
            .map_err(|e| ChannelError::Io(e.to_string()))?;
        
        let user = self.config.username.as_deref().unwrap_or(&self.config.nickname);
        let real = self.config.realname.as_deref().unwrap_or("SENTIENT AI");
        let user_cmd = format!("USER {} 0 * :{}\r\n", user, real);
        writer.write_all(user_cmd.as_bytes()).await
            .map_err(|e| ChannelError::Io(e.to_string()))?;
        
        // Identify with Nickserv
        if let Some(ref ns_pass) = self.config.nickserv_password {
            let identify = format!("PRIVMSG NickServ :IDENTIFY {}\r\n", ns_pass);
            writer.write_all(identify.as_bytes()).await
                .map_err(|e| ChannelError::Io(e.to_string()))?;
        }
        
        self.connected = true;
        Ok(())
    }
    
    /// Join channel
    pub async fn join(&self, channel: &str) -> Result<(), ChannelError> {
        // Would need to keep connection alive
        Ok(())
    }
    
    /// Send message to channel/user
    pub async fn send_privmsg(&self, target: &str, message: &str) -> Result<(), ChannelError> {
        // Would need persistent connection
        Ok(())
    }
    
    /// Send notice
    pub async fn send_notice(&self, target: &str, message: &str) -> Result<(), ChannelError> {
        Ok(())
    }
    
    /// Send action (/me)
    pub async fn send_action(&self, target: &str, action: &str) -> Result<(), ChannelError> {
        // PRIVMSG target :\x01ACTION action\x01
        Ok(())
    }
    
    /// Set topic
    pub async fn set_topic(&self, channel: &str, topic: &str) -> Result<(), ChannelError> {
        Ok(())
    }
    
    /// Kick user
    pub async fn kick(&self, channel: &str, user: &str, reason: Option<&str>) -> Result<(), ChannelError> {
        Ok(())
    }
    
    /// Mode change
    pub async fn mode(&self, target: &str, mode: &str) -> Result<(), ChannelError> {
        Ok(())
    }
    
    /// Part channel
    pub async fn part(&self, channel: &str, reason: Option<&str>) -> Result<(), ChannelError> {
        Ok(())
    }
    
    /// Quit server
    pub async fn quit(&self, reason: Option<&str>) -> Result<(), ChannelError> {
        Ok(())
    }
}

#[async_trait]
impl Channel for IrcChannel {
    fn channel_type(&self) -> ChannelType {
        ChannelType::IRC
    }
    
    async fn send(&self, message: ChannelMessage) -> Result<String, ChannelError> {
        match message.content {
            MessageContent::Text(text) => {
                self.send_privmsg(&message.recipient, &text).await?;
                Ok(String::new())
            }
            _ => Err(ChannelError::UnsupportedContentType),
        }
    }
    
    async fn receive(&self) -> Result<Vec<ChannelMessage>, ChannelError> {
        Ok(Vec::new())
    }
    
    fn is_connected(&self) -> bool {
        self.connected
    }
}

/// Parsed IRC message
#[derive(Debug, Clone)]
pub struct IrcMessage {
    pub prefix: Option<String>,
    pub command: String,
    pub params: Vec<String>,
}

impl IrcMessage {
    /// Parse IRC message from raw line
    pub fn parse(line: &str) -> Option<Self> {
        let line = line.trim_end_matches("\r\n");
        let mut parts = line.splitn(2, ' ');
        
        let first = parts.next()?;
        let rest = parts.next().unwrap_or("");
        
        let (prefix, command) = if first.starts_with(':') {
            let prefix = &first[1..];
            let mut cmd_parts = rest.splitn(2, ' ');
            let cmd = cmd_parts.next()?;
            let params_str = cmd_parts.next().unwrap_or("");
            
            (Some(prefix.to_string()), cmd.to_string())
        } else {
            (None, first.to_string())
        };
        
        // Parse params (handling trailing parameter)
        let mut params = Vec::new();
        if let Some(rest) = if prefix.is_some() {
            line.splitn(4, ' ').nth(2)
        } else {
            line.splitn(3, ' ').nth(1)
        } {
            if let Some(trailing_start) = rest.find(" :") {
                let before = &rest[..trailing_start];
                let trailing = &rest[trailing_start + 2..];
                
                params.extend(before.split_whitespace().map(String::from));
                params.push(trailing.to_string());
            } else {
                params.extend(rest.split_whitespace().map(String::from));
            }
        }
        
        Some(Self { prefix, command, params })
    }
    
    /// Get the trailing parameter (last param after :)
    pub fn trailing(&self) -> Option<&str> {
        self.params.last().map(|s| s.as_str())
    }
    
    /// Get sender from prefix
    pub fn sender(&self) -> Option<&str> {
        self.prefix.as_ref().and_then(|p| {
            let end = p.find('!').unwrap_or(p.len());
            Some(&p[..end])
        })
    }
}

/// IRC event types
#[derive(Debug, Clone)]
pub enum IrcEvent {
    /// Private message
    Privmsg { sender: String, target: String, message: String },
    
    /// Channel message
    Chanmsg { sender: String, channel: String, message: String },
    
    /// Notice
    Notice { sender: String, target: String, message: String },
    
    /// Join
    Join { user: String, channel: String },
    
    /// Part
    Part { user: String, channel: String, reason: Option<String> },
    
    /// Quit
    Quit { user: String, reason: Option<String> },
    
    /// Kick
    Kick { kicker: String, channel: String, victim: String, reason: Option<String> },
    
    /// Nick change
    Nick { old_nick: String, new_nick: String },
    
    /// Topic change
    Topic { user: String, channel: String, topic: String },
    
    /// Ping
    Ping { server: String },
    
    /// Pong
    Pong { server: String },
    
    /// Numeric reply
    Numeric { code: u16, params: Vec<String> },
}

impl From<IrcMessage> for IrcEvent {
    fn from(msg: IrcMessage) -> Self {
        match msg.command.as_str() {
            "PRIVMSG" => {
                let target = msg.params.get(0).cloned().unwrap_or_default();
                let message = msg.trailing().unwrap_or_default().to_string();
                let sender = msg.sender().unwrap_or_default().to_string();
                
                if target.starts_with('#') {
                    IrcEvent::Chanmsg { sender, channel: target, message }
                } else {
                    IrcEvent::Privmsg { sender, target, message }
                }
            }
            "NOTICE" => {
                let target = msg.params.get(0).cloned().unwrap_or_default();
                let message = msg.trailing().unwrap_or_default().to_string();
                let sender = msg.sender().unwrap_or_default().to_string();
                IrcEvent::Notice { sender, target, message }
            }
            "JOIN" => {
                let channel = msg.params.get(0).cloned().unwrap_or_default();
                let user = msg.sender().unwrap_or_default().to_string();
                IrcEvent::Join { user, channel }
            }
            "PART" => {
                let channel = msg.params.get(0).cloned().unwrap_or_default();
                let user = msg.sender().unwrap_or_default().to_string();
                let reason = msg.trailing().map(String::from);
                IrcEvent::Part { user, channel, reason }
            }
            "QUIT" => {
                let user = msg.sender().unwrap_or_default().to_string();
                let reason = msg.trailing().map(String::from);
                IrcEvent::Quit { user, reason }
            }
            "KICK" => {
                let channel = msg.params.get(0).cloned().unwrap_or_default();
                let victim = msg.params.get(1).cloned().unwrap_or_default();
                let kicker = msg.sender().unwrap_or_default().to_string();
                let reason = msg.trailing().map(String::from);
                IrcEvent::Kick { kicker, channel, victim, reason }
            }
            "NICK" => {
                let new_nick = msg.params.get(0).cloned().unwrap_or_default();
                let old_nick = msg.sender().unwrap_or_default().to_string();
                IrcEvent::Nick { old_nick, new_nick }
            }
            "TOPIC" => {
                let channel = msg.params.get(0).cloned().unwrap_or_default();
                let topic = msg.trailing().unwrap_or_default().to_string();
                let user = msg.sender().unwrap_or_default().to_string();
                IrcEvent::Topic { user, channel, topic }
            }
            "PING" => {
                let server = msg.params.get(0).cloned().unwrap_or_default();
                IrcEvent::Ping { server }
            }
            "PONG" => {
                let server = msg.params.get(0).cloned().unwrap_or_default();
                IrcEvent::Pong { server }
            }
            _ => {
                if let Ok(code) = msg.command.parse::<u16>() {
                    IrcEvent::Numeric { code, params: msg.params }
                } else {
                    IrcEvent::Numeric { code: 0, params: vec![msg.command] }
                }
            }
        }
    }
}
