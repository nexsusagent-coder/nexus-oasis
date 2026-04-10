//! Silent Updater - Sessiz güncelleme motoru
//! 
//! Kullanıcıya hissettirmeden repoları günceller

use crate::{SyncConfig, SyncError, UpdateResult, tracker::TrackedRepo, sync_state::SyncState};
use git2::{Repository, FetchOptions, RemoteCallbacks, Oid, AnnotatedCommit};
use chrono::Utc;
use std::collections::HashMap;
use parking_lot::RwLock;
use std::sync::Arc;

/// Silent Updater
#[derive(Debug, Clone)]
pub struct SilentUpdater {
    config: SyncConfig,
    state: Arc<RwLock<SyncState>>,
    /// Aktif güncellemeler (repo_name -> start_time)
    active_updates: Arc<RwLock<HashMap<String, chrono::DateTime<Utc>>>>,
}

impl SilentUpdater {
    /// Yeni updater oluştur
    pub fn new(config: &SyncConfig) -> Result<Self, SyncError> {
        Ok(Self {
            config: config.clone(),
            state: Arc::new(RwLock::new(SyncState::default())),
            active_updates: Arc::new(RwLock::new(HashMap::new())),
        })
    }
    
    /// Repo'yu güncelle
    pub async fn update_repo(&self, repo: &TrackedRepo) -> Result<UpdateResult, SyncError> {
        let start_time = Utc::now();
        
        // Aktif güncellemelere ekle
        {
            let mut active = self.active_updates.write();
            active.insert(repo.name.clone(), start_time);
        }
        
        let result = self.do_update(repo).await;
        
        // Aktif güncellemelerden çıkar
        {
            let mut active = self.active_updates.write();
            active.remove(&repo.name);
        }
        
        result
    }
    
    /// Asıl güncelleme işlemi
    async fn do_update(&self, repo: &TrackedRepo) -> Result<UpdateResult, SyncError> {
        tracing::debug!("Updating repository: {}", repo.name);
        
        let git_repo = Repository::open(&repo.path)?;
        
        // Güvenlik kontrolleri
        self.run_security_checks(&git_repo, repo)?;
        
        // Mevcut commit'i kaydet
        let old_commit = self.get_current_commit(&git_repo)?;
        
        // Local değişiklik varsa stash'le
        let mut git_repo_mut = Repository::open(&repo.path)?;
        let stashed = self.stash_local_changes(&mut git_repo_mut)?;
        
        // Fetch and merge
        let result = self.fetch_and_merge(&git_repo, &repo.branch).await;
        
        // Stash'i geri yükle (varsa)
        if stashed {
            let mut git_repo_for_unstash = Repository::open(&repo.path)?;
            self.unstash_local_changes(&mut git_repo_for_unstash)?;
        }
        
        match result {
            Ok(new_commit) => {
                let files_changed = self.count_changed_files(&git_repo, &old_commit, &new_commit)?;
                
                // State'i güncelle
                {
                    let mut state = self.state.write();
                    state.record_update(&repo.name, &new_commit);
                }
                
                tracing::info!(
                    "Updated {}: {} -> {} ({} files changed)",
                    repo.name, old_commit, new_commit, files_changed
                );
                
                Ok(UpdateResult {
                    repo_name: repo.name.clone(),
                    old_commit,
                    new_commit,
                    files_changed,
                    timestamp: Utc::now(),
                })
            }
            Err(e) => {
                tracing::error!("Failed to update {}: {}", repo.name, e);
                Err(e)
            }
        }
    }
    
    /// Güvenlik kontrolleri
    fn run_security_checks(&self, repo: &Repository, tracked: &TrackedRepo) -> Result<(), SyncError> {
        let security = &self.config.security;
        
        // Branch kontrolü
        if !security.allowed_branches.contains(&tracked.branch) {
            return Err(SyncError::Config(format!(
                "Branch '{}' not allowed for updates",
                tracked.branch
            )));
        }
        
        // Hassas dosya kontrolü
        if security.scan_secrets {
            self.scan_for_secrets(repo)?;
        }
        
        Ok(())
    }
    
    /// Hassas dosya taraması
    fn scan_for_secrets(&self, repo: &Repository) -> Result<(), SyncError> {
        let patterns = [
            ".env", "credentials", "secrets", "api_key", "password",
            "private_key", "access_token", "auth_token"
        ];
        
        let index = repo.index()?;
        for entry in index.iter() {
            let path = entry.path.clone();
            let path_str = String::from_utf8_lossy(&path).to_lowercase();
            
            for pattern in &patterns {
                if path_str.contains(pattern) {
                    tracing::warn!(
                        "Potential sensitive file detected: {}",
                        String::from_utf8_lossy(&path)
                    );
                    // Sadece uyarı ver, güncellemeyi durdurma
                }
            }
        }
        
        Ok(())
    }
    
    /// Mevcut commit'i al
    fn get_current_commit(&self, repo: &Repository) -> Result<String, SyncError> {
        let head = repo.head()?;
        let commit = head.peel_to_commit()?;
        Ok(commit.id().to_string())
    }
    
    /// Local değişiklikleri stash'le
    fn stash_local_changes(&self, repo: &mut Repository) -> Result<bool, SyncError> {
        let has_changes = {
            let mut status_options = git2::StatusOptions::new();
            status_options.include_untracked(true);
            
            let statuses = repo.statuses(Some(&mut status_options))?;
            !statuses.is_empty()
        };
        
        if !has_changes {
            return Ok(false);
        }
        
        tracing::debug!("Stashing local changes");
        
        // Stash işlemi
        let sig = repo.signature()?;
        let message = "SENTIENT-SYNC: Auto-stash before update";
        
        repo.stash_save(&sig, message, None)?;
        
        Ok(true)
    }
    
    /// Stash'i geri yükle
    fn unstash_local_changes(&self, repo: &mut Repository) -> Result<(), SyncError> {
        // Stash apply
        repo.stash_apply(0, None)?;
        repo.stash_drop(0)?;
        
        Ok(())
    }
    
    /// Fetch ve merge yap
    async fn fetch_and_merge(&self, repo: &Repository, branch: &str) -> Result<String, SyncError> {
        // Fetch
        let mut remote = repo.find_remote("origin")?;
        
        let mut fetch_options = FetchOptions::new();
        let mut callbacks = RemoteCallbacks::new();
        
        callbacks.credentials(|_url, _username, _allowed_types| {
            git2::Cred::default()
        });
        
        callbacks.transfer_progress(|progress| {
            if progress.received_objects() > 0 {
                tracing::trace!(
                    "Fetching: {}/{} objects",
                    progress.indexed_objects(),
                    progress.total_objects()
                );
            }
            true
        });
        
        fetch_options.remote_callbacks(callbacks);
        
        remote.fetch(&[branch], Some(&mut fetch_options), None)?;
        
        // Merge
        let fetch_head = repo.find_reference("FETCH_HEAD")?;
        let fetch_commit = repo.reference_to_annotated_commit(&fetch_head)?;
        
        let analysis = repo.merge_analysis(&[&fetch_commit])?;
        
        if analysis.0.is_up_to_date() {
            return self.get_current_commit(repo);
        }
        
        if analysis.0.is_fast_forward() {
            // Fast-forward merge
            self.fast_forward_merge(repo, branch, &fetch_commit)?;
        } else if analysis.0.is_normal() {
            // Normal merge
            self.normal_merge(repo, &fetch_commit)?;
        }
        
        self.get_current_commit(repo)
    }
    
    /// Fast-forward merge
    fn fast_forward_merge(
        &self,
        repo: &Repository,
        branch: &str,
        fetch_commit: &AnnotatedCommit,
    ) -> Result<(), SyncError> {
        let mut reference = repo.find_reference(&format!("refs/heads/{}", branch))?;
        let target_id = fetch_commit.id();
        
        reference.set_target(target_id, "SENTIENT-SYNC: Fast-forward merge")?;
        
        // HEAD'i güncelle
        repo.set_head(&format!("refs/heads/{}", branch))?;
        
        // Working directory'i güncelle
        let commit = repo.find_commit(target_id)?;
        repo.reset(&commit.as_object(), git2::ResetType::Hard, None)?;
        
        Ok(())
    }
    
    /// Normal merge (conflict olabilir)
    fn normal_merge(
        &self,
        repo: &Repository,
        fetch_commit: &AnnotatedCommit,
    ) -> Result<(), SyncError> {
        match self.config.conflict_strategy {
            crate::config::ConflictStrategy::PreferTheirs => {
                // Upstream'ı tercih et
                repo.merge(&[fetch_commit], None, None)?;
                
                // Conflict varsa theirs'i kullan
                let index = repo.index()?;
                if index.has_conflicts() {
                    self.resolve_conflicts_theirs(repo)?;
                }
                
                // Commit
                let sig = repo.signature()?;
                let msg = "SENTIENT-SYNC: Auto-merge (prefer upstream)";
                let head = repo.head()?;
                let head_commit = repo.find_commit(head.target().expect("operation failed"))?;
                let tree = head_commit.tree()?;
                repo.commit(Some("HEAD"), &sig, &sig, msg, &tree, &[])?;
                
                // Cleanup
                repo.cleanup_state()?;
            }
            crate::config::ConflictStrategy::PreferOurs => {
                // Local'i koru, merge yapma
                return Ok(());
            }
            crate::config::ConflictStrategy::Skip => {
                // Atla
                return Err(SyncError::Conflict {
                    repo: "unknown".to_string(),
                    message: "Conflict detected, skipping".to_string(),
                });
            }
            crate::config::ConflictStrategy::Manual => {
                return Err(SyncError::Conflict {
                    repo: "unknown".to_string(),
                    message: "Manual resolution required".to_string(),
                });
            }
        }
        
        Ok(())
    }
    
    /// Conflict'leri upstream lehine çöz
    fn resolve_conflicts_theirs(&self, repo: &Repository) -> Result<(), SyncError> {
        let mut index = repo.index()?;
        
        // Conflict'leri topla
        let conflicts: Vec<_> = index.conflicts()?
            .filter_map(|c| c.ok())
            .collect();
        
        for conflict in conflicts {
            if let Some(their) = conflict.their {
                index.add(&their)?;
            }
        }
        
        index.write()?;
        Ok(())
    }
    
    /// Değişen dosya sayısını hesapla
    fn count_changed_files(
        &self,
        repo: &Repository,
        old_commit: &str,
        new_commit: &str,
    ) -> Result<usize, SyncError> {
        let old_oid = Oid::from_str(old_commit)?;
        let new_oid = Oid::from_str(new_commit)?;
        
        let old_tree = repo.find_commit(old_oid)?.tree()?;
        let new_tree = repo.find_commit(new_oid)?.tree()?;
        
        let diff = repo.diff_tree_to_tree(Some(&old_tree), Some(&new_tree), None)?;
        
        Ok(diff.stats()?.files_changed())
    }
    
    /// Aktif güncelleme var mı kontrol et
    pub fn is_updating(&self, repo_name: &str) -> bool {
        let active = self.active_updates.read();
        active.contains_key(repo_name)
    }
    
    /// Aktif güncelleme sayısı
    pub fn active_count(&self) -> usize {
        self.active_updates.read().len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_silent_update() {
        // Test implementation
    }
}
