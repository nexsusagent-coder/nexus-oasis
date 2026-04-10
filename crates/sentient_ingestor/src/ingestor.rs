//! ═══════════════════════════════════════════════════════════════════════════════
//!  MASS INGESTOR - 5400+ Skill Assimilation Engine
//! ═══════════════════════════════════════════════════════════════════════════════

use crate::error::{IngestorError, IngestorResult};
use crate::parser::{SkillParser, ParsedSkill};
use crate::categories::SkillCategory;
use crate::db::SkillDatabase;
use std::path::{Path, PathBuf};
use std::sync::{Arc, atomic::{AtomicU64, Ordering}};
use std::time::Instant;
use rayon::prelude::*;
use tracing::{info, warn, debug};

/// Ingest istatistikleri
#[derive(Debug, Clone, Default)]
pub struct IngestStats {
    pub total_files: u64,
    pub total_skills: u64,
    pub new_skills: u64,
    pub updated_skills: u64,
    pub skipped_skills: u64,
    pub errors: u64,
    pub duration_secs: f64,
    pub categories: Vec<(String, u64)>,
}

/// Mass Ingestor
pub struct MassIngestor {
    parser: SkillParser,
    db: Arc<SkillDatabase>,
    output_dir: PathBuf,
    stats: IngestStats,
}

impl MassIngestor {
    /// Yeni MassIngestor oluştur
    pub fn new<P: AsRef<Path>>(db_path: P, output_dir: P) -> IngestorResult<Self> {
        let db = SkillDatabase::new(&db_path)?;
        let parser = SkillParser::new()?;
        
        Ok(Self {
            parser,
            db: Arc::new(db),
            output_dir: output_dir.as_ref().to_path_buf(),
            stats: IngestStats::default(),
        })
    }
    
    /// Kategori dosyalarını ingest et
    pub fn ingest_categories(&mut self, categories_dir: &Path) -> IngestorResult<IngestStats> {
        let start = Instant::now();
        info!("🚀 Mass Ingestion başlatılıyor: {:?}", categories_dir);
        
        let category_files = self.find_category_files(categories_dir)?;
        self.stats.total_files = category_files.len() as u64;
        
        info!("📂 {} kategori dosyası bulundu", category_files.len());
        
        for file_path in &category_files {
            if let Err(e) = self.ingest_category_file(file_path) {
                warn!("⚠️ Dosya işleme hatası {:?}: {}", file_path, e);
                self.stats.errors += 1;
            }
        }
        
        self.write_yaml_files()?;
        self.stats.duration_secs = start.elapsed().as_secs_f64();
        self.stats.categories = self.db.category_stats()?
            .into_iter()
            .map(|(cat, count)| (cat, count as u64))
            .collect();
        
        info!(
            "✅ Ingestion tamamlandı: {} skill ({} yeni, {} güncellendi, {} hata) {:.2}s",
            self.stats.total_skills,
            self.stats.new_skills,
            self.stats.updated_skills,
            self.stats.errors,
            self.stats.duration_secs
        );
        
        Ok(IngestStats::clone(&self.stats))
    }
    
    /// README.md dosyasını ingest et
    pub fn ingest_readme(&mut self, readme_path: &Path) -> IngestorResult<IngestStats> {
        let start = Instant::now();
        info!("📖 README.md ingest ediliyor: {:?}", readme_path);
        
        let content = std::fs::read_to_string(readme_path)?;
        let mut current_category = SkillCategory::Unknown;
        
        for line in content.lines() {
            if line.starts_with("### ") || line.starts_with("## ") {
                current_category = SkillCategory::from_filename(line);
                debug!("Kategori: {:?}", current_category);
                continue;
            }
            
            if let Ok(Some(skill)) = self.parser.parse_markdown_line(line, &current_category) {
                self.stats.total_skills += 1;
                
                let unified = self.parser.to_unified(&skill);
                match self.db.upsert_skill(&unified) {
                    Ok(true) => self.stats.new_skills += 1,
                    Ok(false) => self.stats.updated_skills += 1,
                    Err(e) => {
                        warn!("DB hatası: {}", e);
                        self.stats.errors += 1;
                    }
                }
            }
        }
        
        self.stats.duration_secs = start.elapsed().as_secs_f64();
        
        info!(
            "✅ README ingestion: {} skill ({} yeni, {} güncellendi)",
            self.stats.total_skills,
            self.stats.new_skills,
            self.stats.updated_skills
        );
        
        Ok(IngestStats::clone(&self.stats))
    }
    
    /// ECC (Everything Claude Code) skill'lerini ingest et
    pub fn ingest_ecc_skills(&mut self, ecc_dir: &Path) -> IngestorResult<IngestStats> {
        let start = Instant::now();
        info!("🔵 ECC skill'leri ingest ediliyor: {:?}", ecc_dir);
        
        let skills_dir = ecc_dir.join("skills");
        if !skills_dir.exists() {
            return Err(IngestorError::FileReadError(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "ECC skills dizini bulunamadı"
            )));
        }
        
        let mut ecc_count = 0u64;
        
        for entry in std::fs::read_dir(&skills_dir)? {
            let entry = entry?;
            let skill_dir = entry.path();
            
            if skill_dir.is_dir() {
                let skill_file = skill_dir.join("SKILL.md");
                if skill_file.exists() {
                    if let Ok(content) = std::fs::read_to_string(&skill_file) {
                        if let Some(skill) = self.parse_ecc_skill(&content, &skill_dir) {
                            self.stats.total_skills += 1;
                            ecc_count += 1;
                            
                            let unified = self.parser.to_unified(&skill);
                            match self.db.upsert_skill(&unified) {
                                Ok(true) => self.stats.new_skills += 1,
                                Ok(false) => self.stats.updated_skills += 1,
                                Err(_) => self.stats.errors += 1,
                            }
                        }
                    }
                }
            }
        }
        
        self.write_yaml_files()?;
        self.stats.duration_secs += start.elapsed().as_secs_f64();
        
        info!("✅ ECC ingestion: {} skill", ecc_count);
        Ok(IngestStats::clone(&self.stats))
    }
    
    /// Gstack skill'lerini ingest et
    pub fn ingest_gstack_skills(&mut self, gstack_dir: &Path) -> IngestorResult<IngestStats> {
        let start = Instant::now();
        info!("🟢 Gstack skill'leri ingest ediliyor: {:?}", gstack_dir);
        
        let skill_file = gstack_dir.join("SKILL.md");
        let mut gstack_count = 0u64;
        
        // Ana SKILL.md
        if skill_file.exists() {
            if let Ok(content) = std::fs::read_to_string(&skill_file) {
                if let Some(skill) = self.parse_ecc_skill(&content, gstack_dir) {
                    self.stats.total_skills += 1;
                    gstack_count += 1;
                    
                    let unified = self.parser.to_unified(&skill);
                    match self.db.upsert_skill(&unified) {
                        Ok(true) => self.stats.new_skills += 1,
                        Ok(false) => self.stats.updated_skills += 1,
                        Err(_) => self.stats.errors += 1,
                    }
                }
            }
        }
        
        // Alt dizinlerdeki skill'ler
        for entry in std::fs::read_dir(gstack_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_dir() {
                let sub_skill = path.join("SKILL.md");
                if sub_skill.exists() {
                    if let Ok(content) = std::fs::read_to_string(&sub_skill) {
                        if let Some(skill) = self.parse_ecc_skill(&content, &path) {
                            self.stats.total_skills += 1;
                            gstack_count += 1;
                            
                            let unified = self.parser.to_unified(&skill);
                            match self.db.upsert_skill(&unified) {
                                Ok(true) => self.stats.new_skills += 1,
                                Ok(false) => self.stats.updated_skills += 1,
                                Err(_) => self.stats.errors += 1,
                            }
                        }
                    }
                }
            }
        }
        
        self.write_yaml_files()?;
        self.stats.duration_secs += start.elapsed().as_secs_f64();
        
        info!("✅ Gstack ingestion: {} skill", gstack_count);
        Ok(IngestStats::clone(&self.stats))
    }
    
    /// ECC skill formatını parse et
    fn parse_ecc_skill(&self, content: &str, dir: &Path) -> Option<ParsedSkill> {
        let name = dir.file_name()?.to_str()?.to_string();
        
        // YAML frontmatter'dan description çıkar
        let description = if content.starts_with("---") {
            let end = content[3..].find("---")?;
            let frontmatter = &content[3..end+3];
            frontmatter.lines()
                .find(|l| l.starts_with("description:"))
                .map(|l| l.strip_prefix("description:").unwrap_or(l).trim().to_string())
                .unwrap_or_else(|| format!("ECC Skill: {}", name))
        } else {
            format!("ECC Skill: {}", name)
        };
        
        Some(ParsedSkill {
            name: name.clone(),
            slug: name.to_lowercase().replace(" ", "-"),
            description,
            category: SkillCategory::CodingAgentsIdes,
            url: None,
            github_url: None,
            author: Some("everything-claude-code".to_string()),
            tags: vec!["ecc".to_string(), "claude-code".to_string()],
        })
    }
    
    /// Kategori dosyalarını bul
    fn find_category_files(&self, dir: &Path) -> IngestorResult<Vec<PathBuf>> {
        let mut files = Vec::new();
        
        if dir.is_dir() {
            for entry in std::fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                
                if path.extension().map(|e| e == "md").unwrap_or(false) {
                    files.push(path);
                }
            }
        }
        
        files.sort();
        Ok(files)
    }
    
    /// Tek kategori dosyasını ingest et
    fn ingest_category_file(&mut self, file_path: &Path) -> IngestorResult<()> {
        let filename = file_path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown");
        
        let category = SkillCategory::from_filename(filename);
        info!("📄 İşleniyor: {} (kategori: {:?})", filename, category);
        
        let content = std::fs::read_to_string(file_path)?;
        let skills = self.parser.parse_markdown_file(&content, &category)?;
        
        for skill in skills {
            self.stats.total_skills += 1;
            
            let unified = self.parser.to_unified(&skill);
            match self.db.upsert_skill(&unified) {
                Ok(true) => self.stats.new_skills += 1,
                Ok(false) => self.stats.updated_skills += 1,
                Err(e) => {
                    warn!("DB hatası skill '{}' için: {}", skill.name, e);
                    self.stats.errors += 1;
                }
            }
        }
        
        Ok(())
    }
    
    /// YAML dosyalarını yaz
    fn write_yaml_files(&self) -> IngestorResult<()> {
        std::fs::create_dir_all(&self.output_dir)?;
        
        let stats = self.db.category_stats()?;
        
        for (category, _count) in &stats {
            let records = self.db.get_by_category(category)?;
            let category_dir = self.output_dir.join(category);
            std::fs::create_dir_all(&category_dir)?;
            
            for record in records {
                let yaml_path = category_dir.join(format!("{}.yaml", record.slug));
                std::fs::write(&yaml_path, &record.yaml_content)?;
            }
        }
        
        info!("💾 YAML dosyaları yazıldı: {:?}", self.output_dir);
        Ok(())
    }
    
    /// Skill ara
    pub fn search(&self, query: &str) -> IngestorResult<Vec<crate::db::SkillRecord>> {
        self.db.search(query)
    }
    
    /// Kategoriye göre getir
    pub fn get_by_category(&self, category: &str) -> IngestorResult<Vec<crate::db::SkillRecord>> {
        self.db.get_by_category(category)
    }
    
    /// Toplam skill sayısı
    pub fn total_skills(&self) -> IngestorResult<i64> {
        self.db.count()
    }
}

/// Paralel ingestion (rayon ile)
pub fn parallel_ingest(
    files: &[PathBuf],
    _output_dir: &Path,
    db: Arc<SkillDatabase>,
) -> IngestStats {
    let total = AtomicU64::new(0);
    let new = AtomicU64::new(0);
    let updated = AtomicU64::new(0);
    let errors = AtomicU64::new(0);
    
    let parser = SkillParser::new().expect("Parser oluşturulamadı");
    
    files.par_iter().for_each(|file_path| {
        let filename = file_path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown");
        
        let category = SkillCategory::from_filename(filename);
        
        if let Ok(content) = std::fs::read_to_string(file_path) {
            if let Ok(skills) = parser.parse_markdown_file(&content, &category) {
                for skill in skills {
                    total.fetch_add(1, Ordering::Relaxed);
                    
                    let unified = parser.to_unified(&skill);
                    match db.upsert_skill(&unified) {
                        Ok(true) => { new.fetch_add(1, Ordering::Relaxed); }
                        Ok(false) => { updated.fetch_add(1, Ordering::Relaxed); }
                        Err(_) => { errors.fetch_add(1, Ordering::Relaxed); }
                    }
                }
            }
        }
    });
    
    IngestStats {
        total_files: files.len() as u64,
        total_skills: total.load(Ordering::Relaxed),
        new_skills: new.load(Ordering::Relaxed),
        updated_skills: updated.load(Ordering::Relaxed),
        errors: errors.load(Ordering::Relaxed),
        ..Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    
    #[test]
    fn test_mass_ingestor_creation() {
        let dir = tempdir().expect("operation failed");
        let db_path = dir.path().join("test.db");
        let output_dir = dir.path().join("skills");
        
        let ingestor = MassIngestor::new(&db_path, &output_dir);
        assert!(ingestor.is_ok());
    }
    
    #[test]
    fn test_ingest_empty_dir() {
        let dir = tempdir().expect("operation failed");
        let db_path = dir.path().join("test.db");
        let output_dir = dir.path().join("skills");
        let categories_dir = dir.path().join("categories");
        
        std::fs::create_dir_all(&categories_dir).expect("operation failed");
        
        let mut ingestor = MassIngestor::new(&db_path, &output_dir).expect("operation failed");
        let stats = ingestor.ingest_categories(&categories_dir).expect("operation failed");
        
        assert_eq!(stats.total_files, 0);
        assert_eq!(stats.total_skills, 0);
    }
}
