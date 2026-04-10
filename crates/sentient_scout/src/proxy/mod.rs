//! ─── PROXY YONETIMI ───
//!
//! Rotasyon, health check ve load balancing

use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

/// Proxy sunucu
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProxyServer {
    /// Host
    pub host: String,
    /// Port
    pub port: u16,
    /// Protokol
    pub protocol: ProxyProtocol,
    /// Kimlik bilgisi
    pub credentials: Option<ProxyCredentials>,
    /// Durum
    pub status: ProxyStatus,
    /// Health check URL
    pub health_url: Option<String>,
    /// Son kontrol zamani
    pub last_check: Option<chrono::DateTime<chrono::Utc>>,
    /// Ortalama yanit suresi (ms)
    pub avg_response_ms: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ProxyProtocol {
    Http,
    Https,
    Socks5,
    Socks5H,
}

impl ProxyProtocol {
    pub fn as_str(&self) -> &'static str {
        match self {
            ProxyProtocol::Http => "http",
            ProxyProtocol::Https => "https",
            ProxyProtocol::Socks5 => "socks5",
            ProxyProtocol::Socks5H => "socks5h",
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProxyCredentials {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ProxyStatus {
    Active,
    Inactive,
    Failed,
    Checking,
}

/// Proxy havuzu
pub struct ProxyPool {
    /// Proxy listesi
    proxies: VecDeque<ProxyServer>,
    /// Aktif proxy indeksi
    current_index: usize,
    /// Health check istemcisi
    client: reqwest::Client,
    /// Yapilandirma
    config: ProxyPoolConfig,
}

#[derive(Debug, Clone)]
pub struct ProxyPoolConfig {
    /// Health check araligi (saniye)
    pub health_check_interval_secs: u64,
    /// Maksimum hata sayisi
    pub max_failures: u32,
    /// Timeout (ms)
    pub timeout_ms: u64,
}

impl Default for ProxyPoolConfig {
    fn default() -> Self {
        Self {
            health_check_interval_secs: 60,
            max_failures: 3,
            timeout_ms: 10000,
        }
    }
}

impl ProxyPool {
    /// Yeni proxy havuzu olustur
    pub fn new(proxies: Vec<ProxyServer>, config: ProxyPoolConfig) -> Self {
        Self {
            proxies: proxies.into_iter().collect(),
            current_index: 0,
            client: reqwest::Client::new(),
            config,
        }
    }
    
    /// Bos havuz olustur
    pub fn empty() -> Self {
        Self::new(vec![], ProxyPoolConfig::default())
    }
    
    /// Proxy ekle
    pub fn add(&mut self, proxy: ProxyServer) {
        self.proxies.push_back(proxy);
    }
    
    /// Toplu proxy ekle
    pub fn add_batch(&mut self, proxies: Vec<ProxyServer>) {
        for proxy in proxies {
            self.add(proxy);
        }
    }
    
    /// Sonraki proxy'yi al (round-robin)
    pub fn next(&mut self) -> Option<&ProxyServer> {
        if self.proxies.is_empty() {
            return None;
        }
        
        // Sadece aktif proxy'leri kullan
        let start = self.current_index;
        loop {
            let proxy = &self.proxies[self.current_index % self.proxies.len()];
            self.current_index = (self.current_index + 1) % self.proxies.len().max(1);
            
            if proxy.status == ProxyStatus::Active {
                return Some(proxy);
            }
            
            if self.current_index == start {
                break;
            }
        }
        
        None
    }
    
    /// En iyi yanit suresine sahip proxy'yi al
    pub fn best(&self) -> Option<&ProxyServer> {
        self.proxies
            .iter()
            .filter(|p| p.status == ProxyStatus::Active)
            .min_by(|a, b| a.avg_response_ms.partial_cmp(&b.avg_response_ms).expect("operation failed"))
    }
    
    /// Health check yap
    pub async fn health_check(&mut self) {
        for proxy in self.proxies.iter_mut() {
            proxy.status = ProxyStatus::Checking;
            
            let url = proxy.health_url.clone().unwrap_or_else(|| "https://httpbin.org/ip".into());
            let proxy_url = format!("{}://{}:{}", proxy.protocol.as_str(), proxy.host, proxy.port);
            
            let start = Instant::now();
            
            let result = self.client
                .get(&url)
                .timeout(Duration::from_millis(self.config.timeout_ms))
                .send()
                .await;
            
            let response_time = start.elapsed().as_millis() as f64;
            
            match result {
                Ok(resp) if resp.status().is_success() => {
                    proxy.status = ProxyStatus::Active;
                    proxy.avg_response_ms = (proxy.avg_response_ms + response_time) / 2.0;
                }
                _ => {
                    proxy.status = ProxyStatus::Failed;
                }
            }
            
            proxy.last_check = Some(chrono::Utc::now());
        }
    }
    
    /// Proxy'yi sil
    pub fn remove(&mut self, host: &str, port: u16) {
        self.proxies.retain(|p| !(p.host == host && p.port == port));
    }
    
    /// Proxy sayisi
    pub fn len(&self) -> usize {
        self.proxies.len()
    }
    
    /// Bos mu?
    pub fn is_empty(&self) -> bool {
        self.proxies.is_empty()
    }
}

/// Proxy dosyasindan yukleme
pub fn load_proxies_from_file(path: &str) -> std::io::Result<Vec<ProxyServer>> {
    let content = std::fs::read_to_string(path)?;
    
    let proxies: Vec<ProxyServer> = content
        .lines()
        .filter(|line| !line.trim().is_empty() && !line.starts_with('#'))
        .filter_map(|line| parse_proxy_line(line))
        .collect();
    
    Ok(proxies)
}

fn parse_proxy_line(line: &str) -> Option<ProxyServer> {
    // Format: protocol://host:port veya host:port
    let parts: Vec<&str> = line.split("://").collect();
    
    let (protocol, addr) = if parts.len() == 2 {
        let proto = match parts[0].to_lowercase().as_str() {
            "http" => ProxyProtocol::Http,
            "https" => ProxyProtocol::Https,
            "socks5" => ProxyProtocol::Socks5,
            _ => return None,
        };
        (proto, parts[1])
    } else {
        (ProxyProtocol::Http, parts[0])
    };
    
    let addr_parts: Vec<&str> = addr.split(':').collect();
    if addr_parts.len() != 2 {
        return None;
    }
    
    let host = addr_parts[0].to_string();
    let port: u16 = addr_parts[1].parse().ok()?;
    
    Some(ProxyServer {
        host,
        port,
        protocol,
        credentials: None,
        status: ProxyStatus::Active,
        health_url: None,
        last_check: None,
        avg_response_ms: 0.0,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_proxy_line() {
        let proxy = parse_proxy_line("http://127.0.0.1:8080").expect("operation failed");
        assert_eq!(proxy.host, "127.0.0.1");
        assert_eq!(proxy.port, 8080);
    }
    
    #[test]
    fn test_proxy_pool_empty() {
        let pool = ProxyPool::empty();
        assert!(pool.is_empty());
    }
}
