//! ─── Skill Installer ───

use crate::{SkillsError, skill::SkillManifest};
use std::path::{Path, PathBuf};
use tokio::sync::mpsc;

/// Install progress
#[derive(Debug, Clone)]
pub enum InstallProgress {
    Downloading { progress: f32 },
    Extracting,
    Installing,
    Complete,
    Error(String),
}

/// Skill installer
pub struct Installer {
    registry_path: PathBuf,
}

impl Installer {
    pub fn new() -> Self {
        let registry_path = dirs::data_local_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("sentient")
            .join("skills");
        
        Self { registry_path }
    }
    
    /// Install skill
    pub async fn install(&self, skill_id: &str) -> Result<(), SkillsError> {
        let (tx, mut rx) = mpsc::channel(16);
        
        let skill_id = skill_id.to_string();
        let registry_path = self.registry_path.clone();
        
        tokio::spawn(async move {
            if let Err(e) = install_skill(&skill_id, &registry_path, tx.clone()).await {
                let _ = tx.send(InstallProgress::Error(e.to_string())).await;
            }
        });
        
        // Wait for completion
        while let Some(progress) = rx.recv().await {
            match progress {
                InstallProgress::Complete => return Ok(()),
                InstallProgress::Error(e) => return Err(SkillsError::Install(e)),
                _ => log::debug!("Install progress: {:?}", progress),
            }
        }
        
        Err(SkillsError::Install("Installation interrupted".into()))
    }
    
    /// Update skill
    pub async fn update(&self, skill_id: &str) -> Result<(), SkillsError> {
        // Remove and reinstall
        let skill_path = self.registry_path.join(skill_id);
        if skill_path.exists() {
            std::fs::remove_dir_all(&skill_path)?;
        }
        
        self.install(skill_id).await
    }
}

impl Default for Installer {
    fn default() -> Self {
        Self::new()
    }
}

async fn install_skill(
    skill_id: &str,
    registry_path: &Path,
    progress: mpsc::Sender<InstallProgress>,
) -> Result<(), SkillsError> {
    // Create skill directory
    let skill_path = registry_path.join(skill_id);
    std::fs::create_dir_all(&skill_path)?;
    
    progress.send(InstallProgress::Downloading { progress: 0.0 }).await.ok();
    
    // Download skill (simulated - would fetch from ClawHub/Git)
    let client = reqwest::Client::new();
    
    // Try ClawHub first
    let download_url = format!("https://api.clawhub.ai/v1/skills/{}/download", skill_id);
    
    let response = client
        .get(&download_url)
        .send()
        .await
        .map_err(|e| SkillsError::Network(e.to_string()))?;
    
    if response.status().is_success() {
        progress.send(InstallProgress::Downloading { progress: 0.5 }).await.ok();
        
        let bytes = response.bytes().await
            .map_err(|e| SkillsError::Network(e.to_string()))?;
        
        progress.send(InstallProgress::Extracting).await.ok();
        
        // Extract archive
        extract_archive(&bytes, &skill_path)?;
    } else {
        // Create a minimal skill structure
        create_minimal_skill(&skill_path, skill_id)?;
    }
    
    progress.send(InstallProgress::Installing).await.ok();
    
    // Validate manifest
    let manifest_path = skill_path.join("skill.yaml");
    if !manifest_path.exists() {
        // Create default manifest
        let manifest = SkillManifest {
            name: skill_id.to_string(),
            version: "1.0.0".into(),
            description: format!("Skill: {}", skill_id),
            author: "unknown".into(),
            main: "index.js".into(),
            dependencies: vec![],
            config: None,
        };
        
        let yaml = serde_yaml::to_string(&manifest)?;
        std::fs::write(manifest_path, yaml)?;
    }
    
    // Install dependencies (npm install)
    let package_json = skill_path.join("package.json");
    if package_json.exists() {
        // Run npm install
        let _ = tokio::process::Command::new("npm")
            .arg("install")
            .current_dir(&skill_path)
            .output()
            .await;
    }
    
    progress.send(InstallProgress::Complete).await.ok();
    
    Ok(())
}

/// Extract archive (zip, tar.gz)
fn extract_archive(bytes: &[u8], dest: &Path) -> Result<(), SkillsError> {
    // Try zip first
    if bytes.starts_with(b"PK") {
        let reader = std::io::Cursor::new(bytes);
        let mut archive = zip::ZipArchive::new(reader)
            .map_err(|e| SkillsError::Install(format!("Invalid zip: {}", e)))?;
        
        for i in 0..archive.len() {
            let mut file = archive.by_index(i).map_err(|e| SkillsError::Install(e.to_string()))?;
            let outpath = match file.enclosed_name() {
                Some(path) => dest.join(path),
                None => continue,
            };
            
            if file.name().ends_with('/') {
                std::fs::create_dir_all(&outpath)?;
            } else {
                if let Some(p) = outpath.parent() {
                    std::fs::create_dir_all(p)?;
                }
                let mut outfile = std::fs::File::create(&outpath)?;
                std::io::copy(&mut file, &mut outfile)?;
            }
        }
        
        return Ok(());
    }
    
    // Try tar.gz
    if bytes.starts_with(&[0x1f, 0x8b]) {
        let reader = std::io::Cursor::new(bytes);
        let gz_decoder = flate2::read::GzDecoder::new(reader);
        let mut archive = tar::Archive::new(gz_decoder);
        
        archive.unpack(dest)
            .map_err(|e| SkillsError::Install(format!("Invalid tar.gz: {}", e)))?;
        
        return Ok(());
    }
    
    Err(SkillsError::Install("Unknown archive format".into()))
}

/// Create minimal skill structure
fn create_minimal_skill(skill_path: &Path, skill_id: &str) -> Result<(), SkillsError> {
    // Create index.js
    let index_js = r#"
// SENTIENT Skill: ${skill_id}
module.exports = {
  name: "${skill_id}",
  version: "1.0.0",
  
  async execute(context, ...args) {
    return { success: true, message: "Skill executed" };
  }
};
"#.replace("${skill_id}", skill_id);
    
    std::fs::write(skill_path.join("index.js"), index_js)?;
    
    // Create package.json
    let package_json = serde_json::json!({
        "name": skill_id,
        "version": "1.0.0",
        "main": "index.js",
        "scripts": {
            "test": "echo \"No tests\""
        }
    });
    
    std::fs::write(
        skill_path.join("package.json"),
        serde_json::to_string_pretty(&package_json)?
    )?;
    
    Ok(())
}
