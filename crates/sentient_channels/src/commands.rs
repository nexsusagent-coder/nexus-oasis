//! ─── Command Processing ───

use regex::Regex;
use std::collections::HashMap;

/// Command definition
#[derive(Debug, Clone)]
pub struct Command {
    pub name: String,
    pub description: String,
    pub usage: String,
    pub handler: String,
    pub admin_only: bool,
    pub aliases: Vec<String>,
}

/// Command parser
pub struct CommandParser {
    commands: HashMap<String, Command>,
    prefix: String,
}

impl CommandParser {
    pub fn new(prefix: impl Into<String>) -> Self {
        Self {
            commands: HashMap::new(),
            prefix: prefix.into(),
        }
    }
    
    /// Register command
    pub fn register(&mut self, command: Command) {
        self.commands.insert(command.name.clone(), command);
    }
    
    /// Parse message
    pub fn parse(&self, message: &str) -> Option<ParsedCommand> {
        let message = message.trim();
        
        if !message.starts_with(&self.prefix) {
            return None;
        }
        
        let without_prefix = &message[self.prefix.len()..];
        let parts: Vec<&str> = without_prefix.splitn(2, ' ').collect();
        
        let name = parts.get(0)?;
        let args = parts.get(1).unwrap_or(&"").to_string();
        
        // Find command
        let command = self.commands.get(*name).or_else(|| {
            // Check aliases
            self.commands.values().find(|c| c.aliases.iter().any(|a| a == name))
        })?;
        
        Some(ParsedCommand {
            name: command.name.clone(),
            args,
            command: command.clone(),
        })
    }
}

/// Parsed command
#[derive(Debug, Clone)]
pub struct ParsedCommand {
    pub name: String,
    pub args: String,
    pub command: Command,
}

impl Default for CommandParser {
    fn default() -> Self {
        Self::new("/")
    }
}

/// Default commands
pub fn default_commands() -> Vec<Command> {
    vec![
        Command {
            name: "help".into(),
            description: "Show help".into(),
            usage: "/help".into(),
            handler: "help".into(),
            admin_only: false,
            aliases: vec!["h".into(), "?" .into()],
        },
        Command {
            name: "chat".into(),
            description: "Chat with AI".into(),
            usage: "/chat <message>".into(),
            handler: "chat".into(),
            admin_only: false,
            aliases: vec!["c".into()],
        },
        Command {
            name: "agent".into(),
            description: "Run autonomous agent".into(),
            usage: "/agent <goal>".into(),
            handler: "agent".into(),
            admin_only: false,
            aliases: vec!["a".into()],
        },
        Command {
            name: "status".into(),
            description: "Show system status".into(),
            usage: "/status".into(),
            handler: "status".into(),
            admin_only: false,
            aliases: vec!["s".into()],
        },
        Command {
            name: "clear".into(),
            description: "Clear conversation".into(),
            usage: "/clear".into(),
            handler: "clear".into(),
            admin_only: false,
            aliases: vec!["cls".into()],
        },
        Command {
            name: "model".into(),
            description: "Change model".into(),
            usage: "/model <name>".into(),
            handler: "model".into(),
            admin_only: false,
            aliases: vec!["m".into()],
        },
    ]
}
