//! ═══════════════════════════════════════════════════════════════════════════════
//!  MANUS CONTAINER - Docker Container Havuzu
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! Docker container havuzu yönetimi.
//! Önceden oluşturulmuş container'lar ile hızlı çalıştırma.

use crate::error::{ManusError, ManusResult, translate_error};
use crate::sovereign::SandboxPolicy;
use crate::{Language, PoolStatus};
use bollard::Docker;
use bollard::container::{
    Config, CreateContainerOptions, RemoveContainerOptions,
    StartContainerOptions, StopContainerOptions,
};
use bollard::models::HostConfig;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

/// ─── CONTAINER POOL ───

pub struct ContainerPool {
    /// Docker client
    docker: Option<Arc<Docker>>,
    /// Aktif container'lar
    containers: Arc<Mutex<HashMap<String, ContainerInfo>>>,
    /// Varsayılan policy
    default_policy: SandboxPolicy,
    /// Simülasyon modu
    simulation: bool,
}

/// Container bilgisi
#[derive(Debug, Clone)]
pub struct ContainerInfo {
    /// Container ID
    pub id: String,
    /// Container adı
    pub name: String,
    /// Dil
    pub language: Language,
    /// Durum
    pub status: ContainerStatus,
    /// Oluşturulma zamanı
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Son kullanım
    pub last_used: Option<chrono::DateTime<chrono::Utc>>,
    /// Kullanım sayısı
    pub usage_count: u64,
}

/// Container durumu
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContainerStatus {
    Creating,
    Ready,
    Busy,
    Stopped,
    Error,
}

/// Container yapılandırması
#[derive(Debug, Clone)]
pub struct ContainerConfig {
    /// Dil
    pub language: Language,
    /// Bellek limiti (MB)
    pub memory_mb: u32,
    /// CPU kotası
    pub cpu_quota: f32,
    /// Ağ erişimi
    pub network_enabled: bool,
    /// Timeout (saniye)
    pub timeout_secs: u64,
    /// Ortam değişkenleri
    pub env_vars: HashMap<String, String>,
}

impl Default for ContainerConfig {
    fn default() -> Self {
        Self {
            language: Language::Python,
            memory_mb: 256,
            cpu_quota: 0.5,
            network_enabled: false,
            timeout_secs: 60,
            env_vars: HashMap::new(),
        }
    }
}

impl ContainerConfig {
    pub fn from_policy(policy: &SandboxPolicy, language: Language) -> Self {
        Self {
            language,
            memory_mb: policy.resources.memory_mb,
            cpu_quota: policy.resources.cpu_quota,
            network_enabled: policy.network_enabled,
            timeout_secs: policy.max_timeout_secs,
            env_vars: policy.env_vars.clone(),
        }
    }
}

impl ContainerPool {
    /// Yeni havuz oluştur
    pub async fn new() -> ManusResult<Self> {
        log::info!("🐳  MANUS-POOL: Container havuzu oluşturuluyor...");
        
        let docker = match Docker::connect_with_socket_defaults() {
            Ok(d) => {
                log::info!("🐳  MANUS-POOL: Docker bağlantısı kuruldu");
                Some(Arc::new(d))
            }
            Err(e) => {
                let raw = e.to_string();
                log::warn!("🐳  MANUS-POOL: Docker yok, simülasyon modu → {}", translate_error(&raw));
                None
            }
        };
        
        let simulation = docker.is_none();
        
        Ok(Self {
            docker,
            containers: Arc::new(Mutex::new(HashMap::new())),
            default_policy: SandboxPolicy::sovereign(),
            simulation,
        })
    }
    
    /// Container al (veya oluştur)
    pub async fn acquire(&self, language: Language) -> ManusResult<String> {
        let mut containers = self.containers.lock().await;
        
        // Mevcut ready container ara
        for (id, info) in containers.iter_mut() {
            if info.language == language && info.status == ContainerStatus::Ready {
                info.status = ContainerStatus::Busy;
                info.last_used = Some(chrono::Utc::now());
                info.usage_count += 1;
                log::debug!("🐳  MANUS-POOL: Mevcut container alındı → {}", id);
                return Ok(id.clone());
            }
        }
        
        // Yeni container oluştur
        drop(containers);
        self.create_container(language).await
    }
    
    /// Yeni container oluştur
    pub async fn create_container(&self, language: Language) -> ManusResult<String> {
        if self.simulation {
            return self.create_simulated_container(language).await;
        }
        
        let docker = self.docker.as_ref()
            .ok_or_else(|| ManusError::DockerConnectionFailed("Docker not available".into()))?;
        
        let config = ContainerConfig::from_policy(&self.default_policy, language);
        let name = format!("manus_{}_{}", 
            language.extension().trim_start_matches('.'),
            uuid::Uuid::new_v4().to_string().split('-').next().expect("operation failed")
        );
        
        log::info!("🐳  MANUS-POOL: Container oluşturuluyor → {} ({:?})", name, language);
        
        // Host config
        let host_config = HostConfig {
            memory: Some((config.memory_mb as i64) * 1024 * 1024),
            cpu_quota: Some((config.cpu_quota * 100000.0) as i64),
            cpu_period: Some(100000),
            network_mode: if config.network_enabled {
                Some("bridge".into())
            } else {
                Some("none".into())
            },
            pids_limit: Some(64_i64),
            ..Default::default()
        };
        
        // Env vars
        let mut env_vec: Vec<String> = config.env_vars.iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect();
        env_vec.push("MANUS_SANDBOX=true".into());
        env_vec.push("PYTHONUNBUFFERED=1".into());
        
        // Container config
        let container_config = Config {
            image: Some(language.docker_image().into()),
            env: Some(env_vec),
            working_dir: Some("/workspace".into()),
            host_config: Some(host_config),
            cmd: Some(vec!["sleep".into(), "infinity".into()]),
            ..Default::default()
        };
        
        // Oluştur
        let container = docker.create_container(
            Some(CreateContainerOptions {
                name: name.clone(),
                platform: None,
            }),
            container_config,
        ).await.map_err(|e| {
            ManusError::ContainerCreateFailed(translate_error(&e.to_string()))
        })?;
        
        // Başlat
        docker.start_container(
            &container.id,
            None::<StartContainerOptions<String>>,
        ).await.map_err(|e| {
            ManusError::ContainerStartFailed(translate_error(&e.to_string()))
        })?;
        
        // Kaydet
        let info = ContainerInfo {
            id: container.id.clone(),
            name: name.clone(),
            language,
            status: ContainerStatus::Busy,
            created_at: chrono::Utc::now(),
            last_used: Some(chrono::Utc::now()),
            usage_count: 1,
        };
        
        let mut containers = self.containers.lock().await;
        containers.insert(container.id.clone(), info);
        
        log::info!("🐳  MANUS-POOL: Container hazır → {}", &container.id[..12.min(container.id.len())]);
        Ok(container.id)
    }
    
    /// Simülasyon container oluştur
    async fn create_simulated_container(&self, language: Language) -> ManusResult<String> {
        let id = format!("sim_{}", uuid::Uuid::new_v4());
        let info = ContainerInfo {
            id: id.clone(),
            name: format!("sim_{}", language.extension().trim_start_matches('.')),
            language,
            status: ContainerStatus::Busy,
            created_at: chrono::Utc::now(),
            last_used: Some(chrono::Utc::now()),
            usage_count: 1,
        };
        
        let mut containers = self.containers.lock().await;
        containers.insert(id.clone(), info);
        
        log::debug!("🐳  MANUS-POOL: Simülasyon container → {}", id);
        Ok(id)
    }
    
    /// Container bırak
    pub async fn release(&self, container_id: &str) -> ManusResult<()> {
        let mut containers = self.containers.lock().await;
        
        if let Some(info) = containers.get_mut(container_id) {
            info.status = ContainerStatus::Ready;
            log::debug!("🐳  MANUS-POOL: Container serbest → {}", &container_id[..12.min(container_id.len())]);
        }
        
        Ok(())
    }
    
    /// Container durdur
    pub async fn stop_container(&self, container_id: &str) -> ManusResult<()> {
        if self.simulation {
            return Ok(());
        }
        
        let docker = self.docker.as_ref()
            .ok_or_else(|| ManusError::DockerConnectionFailed("Docker not available".into()))?;
        
        docker.stop_container(container_id, Some(StopContainerOptions { t: 10 }))
            .await.map_err(|e| {
                ManusError::ContainerError(translate_error(&e.to_string()))
            })?;
        
        let mut containers = self.containers.lock().await;
        if let Some(info) = containers.get_mut(container_id) {
            info.status = ContainerStatus::Stopped;
        }
        
        log::info!("🐳  MANUS-POOL: Container durduruldu → {}", &container_id[..12.min(container_id.len())]);
        Ok(())
    }
    
    /// Container sil
    pub async fn destroy_container(&self, container_id: &str) -> ManusResult<()> {
        if self.simulation {
            let mut containers = self.containers.lock().await;
            containers.remove(container_id);
            return Ok(());
        }
        
        let docker = self.docker.as_ref()
            .ok_or_else(|| ManusError::DockerConnectionFailed("Docker not available".into()))?;
        
        docker.remove_container(container_id, Some(RemoveContainerOptions { 
            force: true, 
            ..Default::default() 
        })).await.map_err(|e| {
            ManusError::ContainerError(translate_error(&e.to_string()))
        })?;
        
        let mut containers = self.containers.lock().await;
        containers.remove(container_id);
        
        log::info!("🐳  MANUS-POOL: Container silindi → {}", &container_id[..12.min(container_id.len())]);
        Ok(())
    }
    
    /// Tüm container'ları temizle
    pub async fn cleanup_all(&self) -> ManusResult<usize> {
        let container_ids: Vec<String>;
        {
            let containers = self.containers.lock().await;
            container_ids = containers.keys().cloned().collect();
        }
        
        let count = container_ids.len();
        
        for id in container_ids {
            let _ = self.destroy_container(&id).await;
        }
        
        log::info!("🧹  MANUS-POOL: {} container temizlendi", count);
        Ok(count)
    }
    
    /// Havuz durumu
    pub fn status(&self) -> PoolStatus {
        // Sync implementation - use try_lock
        match self.containers.try_lock() {
            Ok(containers) => {
                let total = containers.len();
                let active = containers.values()
                    .filter(|c| c.status == ContainerStatus::Busy)
                    .count();
                let available = containers.values()
                    .filter(|c| c.status == ContainerStatus::Ready)
                    .count();
                
                PoolStatus { total, active, available }
            }
            Err(_) => PoolStatus::default()
        }
    }
    
    /// Container sayısı
    pub fn count(&self) -> usize {
        match self.containers.try_lock() {
            Ok(c) => c.len(),
            Err(_) => 0,
        }
    }
    
    /// Docker mevcut mu?
    pub fn is_docker_available(&self) -> bool {
        !self.simulation
    }
    
    /// Docker client'ı al (public)
    pub fn docker(&self) -> Option<Arc<Docker>> {
        self.docker.clone()
    }
    
    /// Simülasyon modu mu?
    pub fn is_simulation(&self) -> bool {
        self.simulation
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_container_config_default() {
        let config = ContainerConfig::default();
        assert_eq!(config.language, Language::Python);
        assert_eq!(config.memory_mb, 256);
    }

    #[test]
    fn test_container_config_from_policy() {
        let policy = SandboxPolicy::sovereign();
        let config = ContainerConfig::from_policy(&policy, Language::JavaScript);
        assert_eq!(config.language, Language::JavaScript);
        assert_eq!(config.memory_mb, policy.resources.memory_mb);
    }

    #[test]
    fn test_container_status() {
        let info = ContainerInfo {
            id: "test".into(),
            name: "test".into(),
            language: Language::Python,
            status: ContainerStatus::Ready,
            created_at: chrono::Utc::now(),
            last_used: None,
            usage_count: 0,
        };
        assert_eq!(info.status, ContainerStatus::Ready);
    }

    #[test]
    fn test_pool_status() {
        let status = PoolStatus::default();
        assert_eq!(status.total, 0);
        assert_eq!(status.active, 0);
    }
}
