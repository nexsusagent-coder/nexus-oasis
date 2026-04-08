//! Repository Tracker - Repoları keşfet ve takip et

use crate::{SyncConfig, SyncError};
use git2::{Repository, FetchOptions, RemoteCallbacks};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use walkdir::WalkDir;

/// Repository tracker
#[derive(Debug)]
pub struct RepoTracker {
    config: SyncConfig,
    repos: HashMap<String, TrackedRepo>,
}

/// Takip edilen repo bilgisi
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackedRepo {
    /// Repo adı
    pub name: String,
    
    /// Yerel yol
    pub path: PathBuf,
    
    /// Remote URL
    pub remote_url: Option<String>,
    
    /// Mevcut branch
    pub branch: String,
    
    /// Son bilinen commit
    pub current_commit: String,
    
    /// Son güncelleme zamanı
    pub last_sync: Option<DateTime<Utc>>,
    
    /// Son kontrol zamanı
    pub last_check: Option<DateTime<Utc>>,
    
    /// Repo durumu
    pub status: RepoStatus,
    
    /// Entegrasyon kategorisi
    pub category: String,
    
    /// Local değişiklik var mı?
    pub has_local_changes: bool,
}

/// Repo durumu
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RepoStatus {
    /// Güncel
    UpToDate,
    /// Güncelleme mevcut
    UpdateAvailable,
    /// Güncelleniyor
    Updating,
    /// Hata
    Error(String),
    /// Conflict var
    Conflict,
    /// İgnore edildi
    Ignored,
}

impl RepoTracker {
    /// Yeni tracker oluştur
    pub fn new(config: &SyncConfig) -> Result<Self, SyncError> {
        Ok(Self {
            config: config.clone(),
            repos: HashMap::new(),
        })
    }
    
    /// Tüm repoları keşfet
    pub async fn discover_repos(&self) -> Result<Vec<TrackedRepo>, SyncError> {
        let mut repos = Vec::new();
        let integrations_path = &self.config.integrations_path;
        
        if !integrations_path.exists() {
            return Ok(repos);
        }
        
        // Her kategori altında repoları tara
        for entry in WalkDir::new(integrations_path)
            .min_depth(2)
            .max_depth(3)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            
            // .git dizini var mı kontrol et
            let git_dir = path.join(".git");
            if git_dir.exists() && git_dir.is_dir() {
                if let Ok(repo_info) = self.analyze_repo(path).await {
                    repos.push(repo_info);
                }
            }
        }
        
        tracing::info!("Discovered {} repositories", repos.len());
        Ok(repos)
    }
    
    /// Repo analiz et
    async fn analyze_repo(&self, path: &Path) -> Result<TrackedRepo, SyncError> {
        let repo = Repository::open(path)?;
        
        // Repo adı
        let name = path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();
        
        // Kategori (parent directory)
        let category = path.parent()
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();
        
        // Remote URL
        let remote_url = repo.find_remote("origin")
            .ok()
            .and_then(|r| r.url().map(|s| s.to_string()));
        
        // Branch
        let head = repo.head()?;
        let branch = head.shorthand().unwrap_or("main").to_string();
        
        // Current commit
        let commit = repo.revparse_single("HEAD")?;
        let current_commit = commit.id().to_string();
        
        // Local değişiklik kontrolü
        let has_local_changes = self.check_local_changes(&repo)?;
        
        // Son sync zamanı (şimdilik None)
        let last_sync = None;
        let last_check = Some(Utc::now());
        
        Ok(TrackedRepo {
            name,
            path: path.to_path_buf(),
            remote_url,
            branch,
            current_commit,
            last_sync,
            last_check,
            status: RepoStatus::UpToDate,
            category,
            has_local_changes,
        })
    }
    
    /// Local değişiklik kontrolü
    fn check_local_changes(&self, repo: &Repository) -> Result<bool, SyncError> {
        let mut status_options = git2::StatusOptions::new();
        status_options.include_untracked(true);
        
        let statuses = repo.statuses(Some(&mut status_options))?;
        
        Ok(!statuses.is_empty())
    }
    
    /// Remote'dan güncelleme kontrolü
    pub async fn check_for_updates(&self, repo: &mut TrackedRepo) -> Result<bool, SyncError> {
        let git_repo = Repository::open(&repo.path)?;
        
        // Remote'ı al
        let mut remote = git_repo.find_remote("origin")
            .map_err(|e| SyncError::Git(e))?;
        
        // Fetch options
        let mut fetch_options = FetchOptions::new();
        let mut callbacks = RemoteCallbacks::new();
        
        // Credentials (opsiyonel - public repo için gerekmez)
        callbacks.credentials(|_url, _username_from_url, _allowed_types| {
            git2::Cred::default()
        });
        
        fetch_options.remote_callbacks(callbacks);
        
        // Fetch
        remote.fetch(&[&repo.branch], Some(&mut fetch_options), None)?;
        
        // Remote commit'i kontrol et
        let remote_branch = format!("origin/{}", repo.branch);
        let remote_commit = git_repo.revparse_single(&remote_branch)?;
        let remote_id = remote_commit.id();
        
        // Local commit
        let local_commit = git_repo.revparse_single("HEAD")?;
        let local_id = local_commit.id();
        
        // Karşılaştır
        if remote_id != local_id {
            repo.status = RepoStatus::UpdateAvailable;
            return Ok(true);
        }
        
        repo.status = RepoStatus::UpToDate;
        Ok(false)
    }
    
    /// Tüm repoları kontrol et
    pub async fn check_all(&mut self) -> Result<HashMap<String, bool>, SyncError> {
        let repos = self.discover_repos().await?;
        let mut results = HashMap::new();
        
        for mut repo in repos {
            let repo_name = repo.name.clone();
            match self.check_for_updates(&mut repo).await {
                Ok(has_update) => {
                    results.insert(repo_name.clone(), has_update);
                    self.repos.insert(repo_name, repo);
                }
                Err(e) => {
                    tracing::warn!("Failed to check repo: {}", e);
                }
            }
        }
        
        Ok(results)
    }
    
    /// Repo sayısını döndür
    pub fn repo_count(&self) -> usize {
        self.repos.len()
    }
    
    /// Kategori bazlı repo sayısı
    pub fn count_by_category(&self) -> HashMap<String, usize> {
        let mut counts = HashMap::new();
        for repo in self.repos.values() {
            *counts.entry(repo.category.clone()).or_insert(0) += 1;
        }
        counts
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_discover_repos() {
        // Test implementation
    }
}
