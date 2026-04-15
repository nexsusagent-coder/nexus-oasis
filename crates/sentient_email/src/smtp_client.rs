//! ─── SMTP Client ───

use crate::models::*;
use crate::{EmailResult, EmailError};

/// SMTP email sender
pub struct SmtpClient {
    host: String,
    port: u16,
    username: String,
    password: String,
}

impl SmtpClient {
    pub fn new(host: String, port: u16, username: String, password: String) -> Self {
        Self { host, port, username, password }
    }
    
    pub async fn send(&self, email: &ComposeEmail) -> EmailResult<String> {
        // TODO: Implement real SMTP sending using lettre
        // For now, return a mock message ID
        Ok(format!("msg-{}", uuid::Uuid::new_v4()))
    }
}
