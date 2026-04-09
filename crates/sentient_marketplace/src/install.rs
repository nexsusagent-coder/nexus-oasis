//! ─── Skill Installation ───

use std::path::PathBuf;
use crate::{MarketplaceError, MarketplaceSkill, SkillManifest};

/// Installation result
#[derive(Debug, Clone)]
pub struct InstallResult {
    pub id: String,
    pub version: String,
    pub path: String,
    pub installed_files: Vec<String>,
}

/// Skill installer
pub struct SkillInstaller {
    skills_dir: PathBuf,
}

impl SkillInstaller {
    pub fn new(skills_dir: PathBuf) -> Self {
        Self { skills_dir }
    }
    
    /// Install skill
    pub async fn install(&self, skill: &MarketplaceSkill, version: Option<&str>) -> Result<InstallResult, MarketplaceError> {
        let version = version.or(skill.latest_version.as_deref())
            .ok_or_else(|| MarketplaceError::VersionNotFound("No version specified".into()))?;
        
        // Find version info
        let version_info = skill.versions.iter()
            .find(|v| v.version == version)
            .ok_or_else(|| MarketplaceError::VersionNotFound(version.into()))?;
        
        // Check if already installed
        let install_path = self.skills_dir.join(&skill.id);
        if install_path.exists() {
            return Err(MarketplaceError::AlreadyInstalled(skill.id.clone()));
        }
        
        // Create directory
        std::fs::create_dir_all(&install_path)?;
        
        // Download
        log::info!("Downloading skill {} v{}", skill.id, version);
        let bytes = self.download(&version_info.download_url).await?;
        
        // Verify checksum
        self.verify_checksum(&bytes, &version_info.checksum)?;
        
        // Extract
        let installed_files = self.extract(&bytes, &install_path)?;
        
        log::info!("Installed skill {} v{}", skill.id, version);
        
        Ok(InstallResult {
            id: skill.id.clone(),
            version: version.to_string(),
            path: install_path.to_string_lossy().to_string(),
            installed_files,
        })
    }
    
    /// Update skill
    pub async fn update(&self, skill: &MarketplaceSkill) -> Result<InstallResult, MarketplaceError> {
        // Uninstall first
        self.uninstall(&skill.id).await.ok();
        
        // Install latest
        self.install(skill, None).await
    }
    
    /// Uninstall skill
    pub async fn uninstall(&self, id: &str) -> Result<(), MarketplaceError> {
        let install_path = self.skills_dir.join(id);
        
        if !install_path.exists() {
            return Err(MarketplaceError::NotInstalled(id.into()));
        }
        
        std::fs::remove_dir_all(&install_path)?;
        log::info!("Uninstalled skill {}", id);
        
        Ok(())
    }
    
    /// Load manifest from directory
    pub fn load_manifest(&self, path: &str) -> Result<SkillManifest, MarketplaceError> {
        SkillManifest::from_dir(path)
    }
    
    /// Download file
    async fn download(&self, url: &str) -> Result<Vec<u8>, MarketplaceError> {
        let response = reqwest::get(url).await
            .map_err(|e| MarketplaceError::Network(e.to_string()))?;
        
        if !response.status().is_success() {
            return Err(MarketplaceError::Network(format!("Download failed: {}", response.status())));
        }
        
        let bytes = response.bytes().await
            .map_err(|e| MarketplaceError::Network(e.to_string()))?;
        
        Ok(bytes.to_vec())
    }
    
    /// Verify checksum
    fn verify_checksum(&self, bytes: &[u8], expected: &str) -> Result<(), MarketplaceError> {
        use sha2::{Sha256, Digest};
        
        let mut hasher = Sha256::new();
        hasher.update(bytes);
        let hash = hasher.finalize();
        let checksum = format!("{:x}", hash);
        
        if checksum != expected {
            return Err(MarketplaceError::InstallFailed("Checksum mismatch".into()));
        }
        
        Ok(())
    }
    
    /// Extract archive
    fn extract(&self, bytes: &[u8], dest: &PathBuf) -> Result<Vec<String>, MarketplaceError> {
        let mut installed_files = Vec::new();
        
        // Try zip first
        if let Ok(mut archive) = zip::ZipArchive::new(std::io::Cursor::new(bytes)) {
            for i in 0..archive.len() {
                let mut file = archive.by_index(i).map_err(|e| 
                    MarketplaceError::InstallFailed(e.to_string()))?;
                
                let outpath = dest.join(file.name());
                
                if file.name().ends_with('/') {
                    std::fs::create_dir_all(&outpath)?;
                } else {
                    if let Some(p) = outpath.parent() {
                        std::fs::create_dir_all(p)?;
                    }
                    let mut outfile = std::fs::File::create(&outpath)?;
                    std::io::copy(&mut file, &mut outfile)?;
                    installed_files.push(file.name().to_string());
                }
            }
            return Ok(installed_files);
        }
        
        // Try tar.gz
        let gz_decoder = flate2::read::GzDecoder::new(std::io::Cursor::new(bytes));
        let mut archive = tar::Archive::new(gz_decoder);
        
        archive.unpack(dest)?;
        
        Ok(installed_files)
    }
}

impl Default for SkillInstaller {
    fn default() -> Self {
        let skills_dir = dirs::data_local_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("sentient")
            .join("skills");
        
        Self::new(skills_dir)
    }
}
