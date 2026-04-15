//! ─── PWA (Progressive Web App) Interface ───

use serde::{Deserialize, Serialize};
use crate::{RemoteResult, RemoteError};

/// PWA configuration
#[derive(Debug, Clone)]
pub struct PwaConfig {
    pub port: u16,
    pub host: String,
    pub https: bool,
    pub cert_path: Option<String>,
    pub key_path: Option<String>,
}

impl Default for PwaConfig {
    fn default() -> Self {
        Self {
            port: 8443,
            host: "0.0.0.0".into(),
            https: true,
            cert_path: None,
            key_path: None,
        }
    }
}

/// PWA Server
pub struct PwaServer {
    config: PwaConfig,
    running: bool,
}

impl PwaServer {
    pub fn new(config: PwaConfig) -> Self {
        Self { config, running: false }
    }
    
    pub async fn start(&mut self) -> RemoteResult<()> {
        tracing::info!("Starting PWA server on {}:{}", self.config.host, self.config.port);
        self.running = true;
        Ok(())
    }
    
    pub fn stop(&mut self) {
        self.running = false;
    }
    
    pub fn is_running(&self) -> bool {
        self.running
    }
    
    /// Generate manifest.json
    pub fn generate_manifest(&self) -> PwaManifest {
        PwaManifest {
            name: "SENTIENT Remote".into(),
            short_name: "SENTIENT".into(),
            description: "SENTIENT OS Remote Control".into(),
            start_url: "/".into(),
            display: "standalone".into(),
            background_color: "#0F0F0F".into(),
            theme_color: "#1A1D21".into(),
            icons: vec![
                PwaIcon { src: "/icon-192.png".into(), sizes: "192x192".into(), r#type: "image/png".into() },
                PwaIcon { src: "/icon-512.png".into(), sizes: "512x512".into(), r#type: "image/png".into() },
            ],
        }
    }
    
    /// Generate service worker
    pub fn generate_service_worker(&self) -> String {
        r#"
const CACHE_NAME = 'sentient-remote-v1';
const urlsToCache = [
  '/',
  '/styles.css',
  '/app.js',
  '/icon-192.png',
  '/icon-512.png'
];

self.addEventListener('install', (event) => {
  event.waitUntil(
    caches.open(CACHE_NAME)
      .then((cache) => cache.addAll(urlsToCache))
  );
});

self.addEventListener('fetch', (event) => {
  event.respondWith(
    caches.match(event.request)
      .then((response) => response || fetch(event.request))
  );
});
"#.into()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PwaManifest {
    pub name: String,
    pub short_name: String,
    pub description: String,
    pub start_url: String,
    pub display: String,
    pub background_color: String,
    pub theme_color: String,
    pub icons: Vec<PwaIcon>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PwaIcon {
    pub src: String,
    pub sizes: String,
    #[serde(rename = "type")]
    pub r#type: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_pwa_manifest() {
        let server = PwaServer::new(PwaConfig::default());
        let manifest = server.generate_manifest();
        assert_eq!(manifest.short_name, "SENTIENT");
    }
}
