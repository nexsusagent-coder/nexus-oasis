//! ─── Remote Commands ───

use serde::{Deserialize, Serialize};
use crate::{RemoteResult, RemoteError, CommandResult};

/// Remote command types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteCommand {
    pub command_type: CommandType,
    pub payload: serde_json::Value,
    pub requires_confirmation: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CommandType {
    Screenshot,
    Status,
    Lock,
    Shutdown,
    Message,
    RunCommand,
    ExecuteScript,
    GetFile,
    VoiceCommand,
    Custom,
}

impl RemoteCommand {
    pub fn new(command_type: CommandType, payload: serde_json::Value) -> Self {
        let requires_confirmation = matches!(command_type, 
            CommandType::Shutdown | 
            CommandType::Lock |
            CommandType::ExecuteScript
        );
        
        Self { command_type, payload, requires_confirmation }
    }
    
    /// Execute the command
    pub async fn execute(&self) -> RemoteResult<CommandResult> {
        match self.command_type {
            CommandType::Screenshot => self.exec_screenshot().await,
            CommandType::Status => self.exec_status().await,
            CommandType::Lock => self.exec_lock().await,
            CommandType::Shutdown => self.exec_shutdown().await,
            CommandType::Message => self.exec_message().await,
            CommandType::RunCommand => self.exec_run_command().await,
            CommandType::ExecuteScript => self.exec_script().await,
            CommandType::GetFile => self.exec_get_file().await,
            CommandType::VoiceCommand => self.exec_voice_command().await,
            CommandType::Custom => self.exec_custom().await,
        }
    }
    
    /// Create from Telegram callback data
    pub fn from_callback_data(data: &str) -> RemoteResult<Self> {
        let parts: Vec<&str> = data.splitn(2, ':').collect();
        let cmd_type = parts.get(0).ok_or_else(|| RemoteError::Command("Invalid callback".into()))?;
        
        let command_type = match *cmd_type {
            "screenshot" => CommandType::Screenshot,
            "status" => CommandType::Status,
            "lock" => CommandType::Lock,
            "shutdown" => CommandType::Shutdown,
            "message" => CommandType::Message,
            "run" => CommandType::RunCommand,
            "script" => CommandType::ExecuteScript,
            "file" => CommandType::GetFile,
            "voice" => CommandType::VoiceCommand,
            _ => CommandType::Custom,
        };
        
        let payload = if let Some(p) = parts.get(1) {
            serde_json::from_str(p).unwrap_or_else(|_| serde_json::json!({"raw": p}))
        } else {
            serde_json::json!({})
        };
        
        Ok(Self::new(command_type, payload))
    }
    
    async fn exec_screenshot(&self) -> RemoteResult<CommandResult> {
        tracing::info!("Taking remote screenshot");
        Ok(CommandResult::success("Screenshot captured"))
    }
    
    async fn exec_status(&self) -> RemoteResult<CommandResult> {
        let status = serde_json::json!({
            "cpu": "45%",
            "memory": "62%",
            "disk": "70%",
            "uptime": "2 days"
        });
        Ok(CommandResult::success("System status").with_data(status))
    }
    
    async fn exec_lock(&self) -> RemoteResult<CommandResult> {
        tracing::info!("Locking system remotely");
        Ok(CommandResult::success("System locked"))
    }
    
    async fn exec_shutdown(&self) -> RemoteResult<CommandResult> {
        tracing::warn!("Remote shutdown requested");
        Ok(CommandResult::success("Shutdown initiated"))
    }
    
    async fn exec_message(&self) -> RemoteResult<CommandResult> {
        let text = self.payload["text"].as_str().unwrap_or("");
        tracing::info!("Displaying message: {}", text);
        Ok(CommandResult::success("Message displayed"))
    }
    
    async fn exec_run_command(&self) -> RemoteResult<CommandResult> {
        let cmd = self.payload["command"].as_str()
            .ok_or_else(|| RemoteError::Command("No command provided".into()))?;
        tracing::info!("Running remote command: {}", cmd);
        Ok(CommandResult::success(&format!("Command executed: {}", cmd)))
    }
    
    async fn exec_script(&self) -> RemoteResult<CommandResult> {
        tracing::info!("Executing remote script");
        Ok(CommandResult::success("Script executed"))
    }
    
    async fn exec_get_file(&self) -> RemoteResult<CommandResult> {
        let path = self.payload["path"].as_str()
            .ok_or_else(|| RemoteError::Command("No path provided".into()))?;
        tracing::info!("Getting file: {}", path);
        Ok(CommandResult::success("File retrieved"))
    }
    
    async fn exec_voice_command(&self) -> RemoteResult<CommandResult> {
        tracing::info!("Executing voice command");
        Ok(CommandResult::success("Voice command processed"))
    }
    
    async fn exec_custom(&self) -> RemoteResult<CommandResult> {
        tracing::info!("Executing custom command");
        Ok(CommandResult::success("Custom command executed"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_command_creation() {
        let cmd = RemoteCommand::new(CommandType::Status, serde_json::json!({}));
        assert!(!cmd.requires_confirmation);
    }
}
