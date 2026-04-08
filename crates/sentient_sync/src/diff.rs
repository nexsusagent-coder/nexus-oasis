//! Diff Analyzer - Değişiklik analizi

use crate::SyncError;
use git2::{Repository, DiffOptions, Delta, Oid};
use serde::{Deserialize, Serialize};

/// Diff sonucu
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffResult {
    /// Eklenen dosyalar
    pub added: Vec<String>,
    
    /// Silinen dosyalar
    pub deleted: Vec<String>,
    
    /// Değiştirilen dosyalar
    pub modified: Vec<String>,
    
    /// Yeniden adlandırılan dosyalar
    pub renamed: Vec<RenamedFile>,
    
    /// Toplam değişiklik
    pub total_changes: usize,
    
    /// Etkilenen satırlar
    pub lines_added: usize,
    pub lines_deleted: usize,
}

/// Yeniden adlandırılan dosya
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenamedFile {
    pub old_path: String,
    pub new_path: String,
}

/// Diff analyzer
pub struct DiffAnalyzer;

impl DiffAnalyzer {
    /// İki commit arasındaki farkı analiz et
    pub fn analyze(repo: &Repository, old_commit: &str, new_commit: &str) -> Result<DiffResult, SyncError> {
        let old_oid = Oid::from_str(old_commit)?;
        let new_oid = Oid::from_str(new_commit)?;
        
        let old_tree = repo.find_commit(old_oid)?.tree()?;
        let new_tree = repo.find_commit(new_oid)?.tree()?;
        
        let mut diff_options = DiffOptions::new();
        diff_options
            .ignore_submodules(true)
            .include_untracked(false);
        
        let diff = repo.diff_tree_to_tree(Some(&old_tree), Some(&new_tree), Some(&mut diff_options))?;
        
        let mut result = DiffResult {
            added: Vec::new(),
            deleted: Vec::new(),
            modified: Vec::new(),
            renamed: Vec::new(),
            total_changes: 0,
            lines_added: 0,
            lines_deleted: 0,
        };
        
        // Dosya değişikliklerini topla
        diff.foreach(
            &mut |delta, _| {
                match delta.status() {
                    Delta::Added => {
                        if let Some(path) = delta.new_file().path() {
                            result.added.push(path.display().to_string());
                        }
                    }
                    Delta::Deleted => {
                        if let Some(path) = delta.old_file().path() {
                            result.deleted.push(path.display().to_string());
                        }
                    }
                    Delta::Modified => {
                        if let Some(path) = delta.new_file().path() {
                            result.modified.push(path.display().to_string());
                        }
                    }
                    Delta::Renamed => {
                        result.renamed.push(RenamedFile {
                            old_path: delta.old_file().path()
                                .map(|p| p.display().to_string())
                                .unwrap_or_default(),
                            new_path: delta.new_file().path()
                                .map(|p| p.display().to_string())
                                .unwrap_or_default(),
                        });
                    }
                    _ => {}
                }
                result.total_changes += 1;
                true
            },
            None,
            None,
            None,
        )?;
        
        // Satır değişikliklerini hesapla
        let stats = diff.stats()?;
        result.lines_added = stats.insertions();
        result.lines_deleted = stats.deletions();
        
        Ok(result)
    }
    
    /// Önemli değişiklik var mı kontrol et
    pub fn has_important_changes(diff: &DiffResult) -> bool {
        // Kaynak kod dosyaları
        let code_extensions = [".rs", ".py", ".ts", ".js", ".go", ".java", ".cpp", ".c", ".h"];
        
        for path in diff.added.iter().chain(diff.modified.iter()) {
            if code_extensions.iter().any(|ext| path.ends_with(ext)) {
                return true;
            }
        }
        
        false
    }
    
    /// Güvenlik açısından kritik dosya değişimi var mı
    pub fn has_security_impact(diff: &DiffResult) -> bool {
        let security_patterns = [
            "auth", "login", "password", "token", "key", "secret",
            "permission", "role", "admin", "config"
        ];
        
        for path in diff.added.iter().chain(diff.modified.iter()).chain(diff.deleted.iter()) {
            let path_lower = path.to_lowercase();
            if security_patterns.iter().any(|p| path_lower.contains(p)) {
                return true;
            }
        }
        
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_diff_analysis() {
        // Test implementation
    }
}
