//! ─── Remote Session Management ───

use serde::{Deserialize, Serialize};
use crate::{RemoteResult, RemoteError};

/// Remote session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteSession {
    pub id: String,
    pub device_id: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_activity: chrono::DateTime<chrono::Utc>,
    pub expires_at: chrono::DateTime<chrono::Utc>,
    pub permissions: Vec<Permission>,
}

impl RemoteSession {
    pub fn new(device_id: &str) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            device_id: device_id.to_string(),
            created_at: now,
            last_activity: now,
            expires_at: now + chrono::Duration::hours(24),
            permissions: vec![Permission::Read],
        }
    }
    
    pub fn with_permissions(mut self, perms: Vec<Permission>) -> Self {
        self.permissions = perms;
        self
    }
    
    pub fn is_valid(&self) -> bool {
        self.expires_at > chrono::Utc::now()
    }
    
    pub fn touch(&mut self) {
        self.last_activity = chrono::Utc::now();
    }
    
    pub fn refresh(&mut self, duration: chrono::Duration) {
        self.expires_at = chrono::Utc::now() + duration;
    }
    
    pub fn has_permission(&self, perm: &Permission) -> bool {
        self.permissions.contains(perm)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Permission {
    Read,
    Write,
    Execute,
    Admin,
    Voice,
}

/// Session manager
pub struct SessionManager {
    sessions: std::collections::HashMap<String, RemoteSession>,
    default_duration: chrono::Duration,
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            sessions: std::collections::HashMap::new(),
            default_duration: chrono::Duration::hours(24),
        }
    }
    
    pub fn create(&mut self, device_id: &str) -> RemoteSession {
        let session = RemoteSession::new(device_id)
            .with_permissions(vec![Permission::Read, Permission::Voice]);
        let id = session.id.clone();
        self.sessions.insert(id, session.clone());
        session
    }
    
    pub fn get(&self, session_id: &str) -> Option<&RemoteSession> {
        self.sessions.get(session_id)
    }
    
    pub fn validate(&self, session_id: &str) -> RemoteResult<&RemoteSession> {
        match self.sessions.get(session_id) {
            Some(session) if session.is_valid() => Ok(session),
            Some(_) => Err(RemoteError::SessionExpired),
            None => Err(RemoteError::Auth("Invalid session".into())),
        }
    }
    
    pub fn refresh(&mut self, session_id: &str) -> RemoteResult<()> {
        let session = self.sessions.get_mut(session_id)
            .ok_or_else(|| RemoteError::Auth("Invalid session".into()))?;
        
        if !session.is_valid() {
            return Err(RemoteError::SessionExpired);
        }
        
        session.refresh(self.default_duration);
        Ok(())
    }
    
    pub fn revoke(&mut self, session_id: &str) -> Option<RemoteSession> {
        self.sessions.remove(session_id)
    }
    
    pub fn cleanup_expired(&mut self) {
        let now = chrono::Utc::now();
        self.sessions.retain(|_, session| session.expires_at > now);
    }
    
    pub fn count(&self) -> usize {
        self.sessions.len()
    }
}

impl Default for SessionManager {
    fn default() -> Self {
        Self::new()
    }
}
