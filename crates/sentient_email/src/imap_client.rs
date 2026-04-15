//! ─── IMAP Client ───

use crate::models::*;
use crate::{EmailResult, EmailError};

/// IMAP email client
pub struct ImapClient {
    host: String,
    port: u16,
    username: String,
    password: String,
}

impl ImapClient {
    pub fn new(host: String, port: u16, username: String, password: String) -> Self {
        Self { host, port, username, password }
    }
    
    pub async fn get_unread(&self, limit: u32) -> EmailResult<Vec<Email>> {
        // TODO: Implement real IMAP connection
        // For now, return empty list
        Ok(vec![])
    }
    
    pub async fn get_folder(&self, folder: EmailFolder, limit: u32) -> EmailResult<Vec<Email>> {
        Ok(vec![])
    }
    
    pub async fn search(&self, query: &EmailQuery) -> EmailResult<Vec<Email>> {
        Ok(vec![])
    }
    
    pub async fn get_email(&self, id: &str) -> EmailResult<Email> {
        Err(EmailError::NotFound(id.to_string()))
    }
    
    pub async fn mark_read(&self, id: &str) -> EmailResult<()> {
        Ok(())
    }
    
    pub async fn move_to(&self, id: &str, folder: EmailFolder) -> EmailResult<()> {
        Ok(())
    }
}
