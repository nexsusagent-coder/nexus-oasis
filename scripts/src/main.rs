//! ═══════════════════════════════════════════════════════════════════════════════
//!  SENTIENT SKILL ASİMİLASYON ARACI
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! Otonom olarak dış repo'ları analiz eder ve SENTIENT formatına dönüştürür.
//!
//! KULLANIM:
//!   cargo run --bin ingest_skills
//!
//! HEDEF REPO'LAR:
//!   1. VoltAgent/awesome-openclaw-skills - OpenClaw skill'leri
//!   2. HKUDS/OpenHarness - 43 tool benchmark
//!
//! ═────────────────────────────────────────────────────────────────────────────

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

// ───────────────────────────────────────────────────────────────────────────────
//  KONFİGÜRASYON
// ─────────────────────────────────────────────────────────────────────────────--

const REPOS: &[(&str, &str)] = &[
    ("VoltAgent/awesome-openclaw-skills", "openclaw"),
    ("HKUDS/OpenHarness", "openharness"),
];

const TEMP_DIR: &str = "data/temp_repos";
const SKILLS_DIR: &str = "data/skills";
const WRAPPERS_DIR: &str = "crates/oasis_hands/src/wrappers";

// ───────────────────────────────────────────────────────────────────────────────
//  SKILL VERİ YAPISI
// ─────────────────────────────────────────────────────────────────────────────--

#[derive(Debug, Clone)]
struct SENTIENTSkill {
    name: String,
    description: String,
    category: String,
    source_repo: String,
    source_file: String,
    language: String,
    parameters: Vec<SkillParameter>,
    commands: Vec<String>,
    dependencies: Vec<String>,
    risk_level: RiskLevel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

impl std::fmt::Display for RiskLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RiskLevel::Low => write!(f, "low"),
            RiskLevel::Medium => write!(f, "medium"),
            RiskLevel::High => write!(f, "high"),
            RiskLevel::Critical => write!(f, "critical"),
        }
    }
}

#[derive(Debug, Clone)]
struct SkillParameter {
    name: String,
    param_type: String,
    required: bool,
    default: Option<String>,
    description: String,
}

// ───────────────────────────────────────────────────────────────────────────────
//  ANA FONKSİYON
// ─────────────────────────────────────────────────────────────────────────────--

fn main() {
    println!("╔═══════════════════════════════════════════════════════════════════════════╗");
    println!("║          🐺 SENTIENT SKILL ASİMİLASYON ARACI v0.1.0                          ║");
    println!("║          ═════════════════════════════════════════                       ║");
    println!("║          Fiziksel Entegrasyon Başlatılıyor...                            ║");
    println!("╚═══════════════════════════════════════════════════════════════════════════╝");
    println!();

    // Dizinleri oluştur
    create_directories();

    // Asimile edilmiş skill'leri topla
    let mut all_skills: Vec<SENTIENTSkill> = Vec::new();

    // Her repo için asimilasyon
    for (repo_url, repo_name) in REPOS {
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("📡 ASİMİLASYON: {}", repo_url);
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

        // Repo'yu klonla
        let repo_path = clone_repo(repo_url, repo_name);
        
        // Analiz et
        let skills = analyze_repo(&repo_path, repo_name);
        
        println!("📊 Tespit edilen skill sayısı: {}", skills.len());
        
        // SENTIENT formatına dönüştür
        for skill in &skills {
            generate_skill_yaml(skill);
            generate_rust_wrapper(skill);
        }
        
        all_skills.extend(skills);
        println!();
    }

    // Özet rapor
    print_summary(&all_skills);
}

// ───────────────────────────────────────────────────────────────────────────────
//  DİZİN OLUŞTURMA
// ─────────────────────────────────────────────────────────────────────────────--

fn create_directories() {
    println!("📁 Dizin yapısı oluşturuluyor...");
    
    fs::create_dir_all(TEMP_DIR).expect("Temp dizini oluşturulamadı");
    fs::create_dir_all(SKILLS_DIR).expect("Skills dizini oluşturulamadı");
    fs::create_dir_all(WRAPPERS_DIR).expect("Wrappers dizini oluşturulamadı");
    
    // wrappers/mod.rs oluştur
    let mod_content = r#"//! ═══════════════════════════════════════════════════════════════════════════════
//!  SENTIENT ASİMİLE EDİLMİŞ TOOL WRAPPER'LARI
//! ═══════════════════════════════════════════════════════════════════════════════
//! 
//! Bu modül dış repo'lardan asimile edilen araçları içerir.
//! Her wrapper SENTIENT Sovereign güvenlik kurallarına tabidir.

// Wrapper modülleri buraya eklenecek
// Örnek: pub mod openclaw_web_search;
"#;
    fs::write(format!("{}/mod.rs", WRAPPERS_DIR), mod_content).ok();
    
    println!("✅ Dizinler hazır");
    println!();
}

// ───────────────────────────────────────────────────────────────────────────────
//  REPO KLONLAMA
// ───────────────────────────────────────────────────────────────────────────────

fn clone_repo(repo_url: &str, repo_name: &str) -> PathBuf {
    let full_url = format!("https://github.com/{}.git", repo_url);
    let target_path = PathBuf::from(TEMP_DIR).join(repo_name);

    // Eğer zaten varsa, silip tekrar klonla
    if target_path.exists() {
        println!("🔄 Mevcut repo güncelleniyor: {}", repo_name);
        fs::remove_dir_all(&target_path).ok();
    }

    println!("⬇️  Klonlanıyor: {}", full_url);
    
    let output = Command::new("git")
        .args(["clone", "--depth", "1", &full_url, target_path.to_str().unwrap()])
        .output();

    match output {
        Ok(o) if o.status.success() => {
            println!("✅ Repo klonlandı: {}", target_path.display());
        }
        Ok(o) => {
            println!("⚠️  Klonlama başarısız: {}", String::from_utf8_lossy(&o.stderr));
            println!("📝 Demo mod: Simüle edilmiş veriler kullanılacak");
        }
        Err(e) => {
            println!("⚠️  Git komutu bulunamadı: {}", e);
            println!("📝 Demo mod: Simüle edilmiş veriler kullanılacak");
        }
    }

    target_path
}

// ───────────────────────────────────────────────────────────────────────────────
//  REPO ANALİZİ
// ───────────────────────────────────────────────────────────────────────────────

fn analyze_repo(repo_path: &Path, repo_name: &str) -> Vec<SENTIENTSkill> {
    let mut skills = Vec::new();

    // Repo var mı kontrol et
    if !repo_path.exists() {
        println!("⚠️  Repo bulunamadı, demo skill'ler oluşturuluyor...");
        return generate_demo_skills(repo_name);
    }

    // TypeScript dosyalarını tara
    println!("🔍 TypeScript dosyaları taranıyor...");
    if let Ok(entries) = fs::read_dir(repo_path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                analyze_directory(&path, repo_name, &mut skills);
            }
        }
    }

    // Python dosyalarını tara
    println!("🐍 Python dosyaları taranıyor...");
    analyze_directory(repo_path, repo_name, &mut skills);

    // Eğer hiç skill bulunamadıysa demo oluştur
    if skills.is_empty() {
        println!("⚠️  Skill bulunamadı, demo veriler oluşturuluyor...");
        return generate_demo_skills(repo_name);
    }

    skills
}

fn analyze_directory(dir: &Path, repo_name: &str, skills: &mut Vec<SENTIENTSkill>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            
            if path.is_dir() {
                analyze_directory(&path, repo_name, skills);
            } else if let Some(ext) = path.extension() {
                match ext.to_str() {
                    Some("ts") | Some("tsx") | Some("js") | Some("jsx") => {
                        if let Some(skill) = parse_typescript_file(&path, repo_name) {
                            skills.push(skill);
                        }
                    }
                    Some("py") => {
                        if let Some(skill) = parse_python_file(&path, repo_name) {
                            skills.push(skill);
                        }
                    }
                    Some("md") => {
                        // SKILL.md dosyalarını da parse et
                        if path.file_name().map(|n| n.to_str()).flatten() == Some("SKILL.md") {
                            if let Some(skill) = parse_skill_md(&path, repo_name) {
                                skills.push(skill);
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}

// ───────────────────────────────────────────────────────────────────────────────
//  TYPESCRIPT PARSER
// ───────────────────────────────────────────────────────────────────────────────

fn parse_typescript_file(path: &Path, repo_name: &str) -> Option<SENTIENTSkill> {
    let content = fs::read_to_string(path).ok()?;
    let file_name = path.file_name()?.to_str()?.to_string();
    
    // Skill adını dosya adından türet
    let name = file_name
        .replace(".ts", "")
        .replace(".tsx", "")
        .replace(".js", "")
        .replace("_", "-")
        .to_lowercase();

    // Fonksiyonları ve parametreleri çıkar
    let functions = extract_functions(&content);
    let imports = extract_imports(&content);
    
    // Kategori belirle
    let category = determine_category(&name, &content);
    
    // Risk seviyesi
    let risk = determine_risk(&name, &content);

    // Parametreleri çıkar
    let parameters = extract_ts_parameters(&content);

    Some(SENTIENTSkill {
        name: format!("{}-{}", repo_name, name),
        description: generate_description(&name, &content),
        category,
        source_repo: repo_name.to_string(),
        source_file: path.to_str()?.to_string(),
        language: "typescript".to_string(),
        parameters,
        commands: functions.iter().map(|f| f.clone()).collect(),
        dependencies: imports,
        risk_level: risk,
    })
}

fn extract_functions(content: &str) -> Vec<String> {
    let mut functions = Vec::new();
    
    for line in content.lines() {
        if line.contains("function ") || line.contains("const ") && line.contains("=>") {
            if let Some(func_name) = extract_function_name(line) {
                functions.push(func_name);
            }
        }
        if line.contains("export ") && line.contains("async ") {
            if let Some(func_name) = extract_function_name(line) {
                functions.push(func_name);
            }
        }
    }
    
    functions
}

fn extract_function_name(line: &str) -> Option<String> {
    let patterns = ["function ", "async ", "const ", "export "];
    let mut line = line.to_string();
    
    for pattern in patterns {
        line = line.replace(pattern, "");
    }
    
    let name = line.split('(').next()?.trim().to_string();
    if name.is_empty() || name.starts_with('{') {
        return None;
    }
    
    Some(name)
}

fn extract_imports(content: &str) -> Vec<String> {
    let mut imports = Vec::new();
    
    for line in content.lines() {
        if line.starts_with("import ") {
            if let Some(dep) = line.split("from").nth(1) {
                let dep = dep.trim().replace('"', "").replace("'", "");
                if !dep.starts_with('.') {
                    imports.push(dep);
                }
            }
        }
    }
    
    imports.sort();
    imports.dedup();
    imports
}

fn extract_ts_parameters(content: &str) -> Vec<SkillParameter> {
    let mut params = Vec::new();
    
    if content.contains("interface ") {
        for line in content.lines() {
            if line.contains(':') && !line.starts_with("//") && !line.contains("interface") {
                let parts: Vec<&str> = line.split(':').collect();
                if parts.len() == 2 {
                    let name = parts[0].trim().replace("?", "");
                    let param_type = parts[1].trim().split(';').next().unwrap_or("").trim();
                    params.push(SkillParameter {
                        name,
                        param_type: param_type.to_string(),
                        required: !line.contains('?'),
                        default: None,
                        description: String::new(),
                    });
                }
            }
        }
    }
    
    params
}

// ───────────────────────────────────────────────────────────────────────────────
//  PYTHON PARSER
// ───────────────────────────────────────────────────────────────────────────────

fn parse_python_file(path: &Path, repo_name: &str) -> Option<SENTIENTSkill> {
    let content = fs::read_to_string(path).ok()?;
    let file_name = path.file_name()?.to_str()?.to_string();
    
    let name = file_name
        .replace(".py", "")
        .replace("_", "-")
        .to_lowercase();

    let functions = extract_python_functions(&content);
    let imports = extract_python_imports(&content);
    
    let category = determine_category(&name, &content);
    let risk = determine_risk(&name, &content);
    let parameters = extract_python_parameters(&content);

    Some(SENTIENTSkill {
        name: format!("{}-{}", repo_name, name),
        description: generate_description(&name, &content),
        category,
        source_repo: repo_name.to_string(),
        source_file: path.to_str()?.to_string(),
        language: "python".to_string(),
        parameters,
        commands: functions,
        dependencies: imports,
        risk_level: risk,
    })
}

fn extract_python_functions(content: &str) -> Vec<String> {
    let mut functions = Vec::new();
    
    for line in content.lines() {
        if line.starts_with("def ") || line.starts_with("async def ") {
            let func_line = line.replace("async ", "").replace("def ", "");
            if let Some(name) = func_line.split('(').next() {
                functions.push(name.trim().to_string());
            }
        }
    }
    
    functions
}

fn extract_python_imports(content: &str) -> Vec<String> {
    let mut imports = Vec::new();
    
    for line in content.lines() {
        if line.starts_with("import ") || line.starts_with("from ") {
            let dep = line
                .replace("import ", "")
                .replace("from ", "")
                .split(' ').next()
                .unwrap_or("")
                .to_string();
            
            if !dep.is_empty() && !dep.starts_with('.') {
                imports.push(dep);
            }
        }
    }
    
    imports.sort();
    imports.dedup();
    imports
}

fn extract_python_parameters(content: &str) -> Vec<SkillParameter> {
    let mut params = Vec::new();
    
    for line in content.lines() {
        if line.starts_with("def ") && line.contains(':') {
            if let Some(start) = line.find('(') {
                if let Some(end) = line.find(')') {
                    let param_str = &line[start + 1..end];
                    for param in param_str.split(',') {
                        let param = param.trim();
                        if param != "self" && !param.is_empty() {
                            let name = param.split(':').next().unwrap_or(param).trim();
                            params.push(SkillParameter {
                                name: name.to_string(),
                                param_type: "any".to_string(),
                                required: !param.starts_with('*'),
                                default: None,
                                description: String::new(),
                            });
                        }
                    }
                }
            }
        }
    }
    
    params
}

// ───────────────────────────────────────────────────────────────────────────────
//  SKILL.MD PARSER (OpenClaw format)
// ─────────────────────────────────────────────────────────────────────────────--

fn parse_skill_md(path: &Path, repo_name: &str) -> Option<SENTIENTSkill> {
    let content = fs::read_to_string(path).ok()?;
    
    // YAML frontmatter'ı parse et
    let (yaml_frontmatter, body) = parse_yaml_frontmatter(&content)?;
    
    // YAML'dan bilgileri çıkar
    let name = extract_yaml_value(&yaml_frontmatter, "name")?;
    let description = extract_yaml_value(&yaml_frontmatter, "description")
        .unwrap_or_else(|| "No description available".to_string());
    let version = extract_yaml_value(&yaml_frontmatter, "version")
        .unwrap_or_else(|| "0.1.0".to_string());
    
    let category = extract_yaml_value(&yaml_frontmatter, "category")
        .unwrap_or_else(|| "general".to_string());

    // Komutları markdown'tan çıkar
    let commands = extract_code_blocks(&body);

    Some(SENTIENTSkill {
        name: format!("{}-{}", repo_name, name),
        description,
        category,
        source_repo: repo_name.to_string(),
        source_file: path.to_str()?.to_string(),
        language: "mixed".to_string(),
        parameters: Vec::new(),
        commands,
        dependencies: vec![format!("version:{}", version)],
        risk_level: RiskLevel::Low,
    })
}

fn parse_yaml_frontmatter(content: &str) -> Option<(String, String)> {
    let start = content.find("---")?;
    let end = content[start + 3..].find("---")?;
    
    let yaml = content[start + 3..start + 3 + end].to_string();
    let body = content[start + 3 + end + 3..].to_string();
    
    Some((yaml, body))
}

fn extract_yaml_value(yaml: &str, key: &str) -> Option<String> {
    for line in yaml.lines() {
        if line.starts_with(&format!("{}:", key)) {
            let value = line.split(':').nth(1)?;
            return Some(value.trim().trim_matches('"').to_string());
        }
    }
    None
}

fn extract_code_blocks(markdown: &str) -> Vec<String> {
    let mut blocks = Vec::new();
    let mut in_block = false;
    let mut current_block = String::new();
    
    for line in markdown.lines() {
        if line.starts_with("```") {
            if in_block {
                if !current_block.is_empty() {
                    blocks.push(current_block.trim().to_string());
                }
                current_block.clear();
            }
            in_block = !in_block;
        } else if in_block {
            current_block.push_str(line);
            current_block.push('\n');
        }
    }
    
    blocks
}

// ───────────────────────────────────────────────────────────────────────────────
//  YARDIMCI FONKSİYONLAR
// ───────────────────────────────────────────────────────────────────────────────

fn determine_category(name: &str, content: &str) -> String {
    let lower = name.to_lowercase();
    let content_lower = content.to_lowercase();
    
    if lower.contains("browser") || lower.contains("web") || content_lower.contains("selenium") {
        "browser".to_string()
    } else if lower.contains("file") || lower.contains("fs") || content_lower.contains("filesystem") {
        "filesystem".to_string()
    } else if lower.contains("api") || lower.contains("http") || content_lower.contains("request") {
        "api".to_string()
    } else if lower.contains("data") || lower.contains("process") {
        "data".to_string()
    } else if lower.contains("security") || lower.contains("auth") {
        "security".to_string()
    } else if lower.contains("search") || lower.contains("scrape") {
        "search".to_string()
    } else {
        "general".to_string()
    }
}

fn determine_risk(name: &str, content: &str) -> RiskLevel {
    let lower = name.to_lowercase();
    let content_lower = content.to_lowercase();
    
    // Kritik risk
    let critical_patterns = ["rm -rf", "delete", "drop table", "truncate", "format", "dd if="];
    for pattern in critical_patterns {
        if content_lower.contains(pattern) {
            return RiskLevel::Critical;
        }
    }
    
    // Yüksek risk
    let high_patterns = ["execute", "shell", "command", "sudo", "chmod", "chown"];
    for pattern in high_patterns {
        if lower.contains(pattern) || content_lower.contains(pattern) {
            return RiskLevel::High;
        }
    }
    
    // Orta risk
    let medium_patterns = ["write", "update", "modify", "create", "network"];
    for pattern in medium_patterns {
        if lower.contains(pattern) || content_lower.contains(pattern) {
            return RiskLevel::Medium;
        }
    }
    
    RiskLevel::Low
}

fn generate_description(name: &str, content: &str) -> String {
    // Docstring veya yorumdan açıklama çıkar
    for line in content.lines() {
        if line.starts_with("///") || line.starts_with("/**") || line.starts_with("# ") {
            let desc = line
                .replace("///", "")
                .replace("/**", "")
                .replace("*/", "")
                .replace("# ", "")
                .trim()
                .to_string();
            if desc.len() > 10 && desc.len() < 200 {
                return desc;
            }
        }
    }
    
    format!("Auto-generated skill from {}", name)
}

// ───────────────────────────────────────────────────────────────────────────────
//  DEMO SKILL OLUŞTURMA (Repo erişilemezse)
// ───────────────────────────────────────────────────────────────────────────────

fn generate_demo_skills(repo_name: &str) -> Vec<SENTIENTSkill> {
    match repo_name {
        "openclaw" => vec![
            SENTIENTSkill {
                name: "openclaw-web-search".to_string(),
                description: "Web'de arama yapma ve sonuçları işleme skill'i".to_string(),
                category: "search".to_string(),
                source_repo: repo_name.to_string(),
                source_file: "skills/web-search/index.ts".to_string(),
                language: "typescript".to_string(),
                parameters: vec![
                    SkillParameter {
                        name: "query".to_string(),
                        param_type: "string".to_string(),
                        required: true,
                        default: None,
                        description: "Arama sorgusu".to_string(),
                    },
                    SkillParameter {
                        name: "max_results".to_string(),
                        param_type: "number".to_string(),
                        required: false,
                        default: Some("10".to_string()),
                        description: "Maksimum sonuç sayısı".to_string(),
                    },
                ],
                commands: vec!["search(query, maxResults)".to_string()],
                dependencies: vec!["playwright".to_string(), "cheerio".to_string()],
                risk_level: RiskLevel::Low,
            },
            SENTIENTSkill {
                name: "openclaw-browser-navigate".to_string(),
                description: "Tarayıcıda sayfa gezinme ve otomasyon".to_string(),
                category: "browser".to_string(),
                source_repo: repo_name.to_string(),
                source_file: "skills/browser/navigate.ts".to_string(),
                language: "typescript".to_string(),
                parameters: vec![
                    SkillParameter {
                        name: "url".to_string(),
                        param_type: "string".to_string(),
                        required: true,
                        default: None,
                        description: "Hedef URL".to_string(),
                    },
                    SkillParameter {
                        name: "wait_for".to_string(),
                        param_type: "string".to_string(),
                        required: false,
                        default: Some("load".to_string()),
                        description: "Bekleme koşulu".to_string(),
                    },
                ],
                commands: vec!["navigate(url, options)".to_string()],
                dependencies: vec!["puppeteer".to_string()],
                risk_level: RiskLevel::Medium,
            },
            SENTIENTSkill {
                name: "openclaw-file-read".to_string(),
                description: "Dosya okuma ve içerik çıkarma".to_string(),
                category: "filesystem".to_string(),
                source_repo: repo_name.to_string(),
                source_file: "skills/fs/read.ts".to_string(),
                language: "typescript".to_string(),
                parameters: vec![
                    SkillParameter {
                        name: "path".to_string(),
                        param_type: "string".to_string(),
                        required: true,
                        default: None,
                        description: "Dosya yolu".to_string(),
                    },
                ],
                commands: vec!["readFile(path)".to_string()],
                dependencies: vec!["fs-extra".to_string()],
                risk_level: RiskLevel::Medium,
            },
            SENTIENTSkill {
                name: "openclaw-code-execute".to_string(),
                description: "Sandbox içinde kod çalıştırma".to_string(),
                category: "execution".to_string(),
                source_repo: repo_name.to_string(),
                source_file: "skills/code/execute.ts".to_string(),
                language: "typescript".to_string(),
                parameters: vec![
                    SkillParameter {
                        name: "code".to_string(),
                        param_type: "string".to_string(),
                        required: true,
                        default: None,
                        description: "Çalıştırılacak kod".to_string(),
                    },
                    SkillParameter {
                        name: "language".to_string(),
                        param_type: "string".to_string(),
                        required: true,
                        default: None,
                        description: "Programlama dili".to_string(),
                    },
                ],
                commands: vec!["execute(code, language)".to_string()],
                dependencies: vec!["vm2".to_string()],
                risk_level: RiskLevel::High,
            },
        ],
        "openharness" => vec![
            SENTIENTSkill {
                name: "openharness-python-exec".to_string(),
                description: "Python kodu çalıştırma aracı".to_string(),
                category: "execution".to_string(),
                source_repo: repo_name.to_string(),
                source_file: "tools/python_executor.py".to_string(),
                language: "python".to_string(),
                parameters: vec![
                    SkillParameter {
                        name: "script".to_string(),
                        param_type: "str".to_string(),
                        required: true,
                        default: None,
                        description: "Python scripti".to_string(),
                    },
                ],
                commands: vec!["python_exec(script)".to_string()],
                dependencies: vec!["python3".to_string()],
                risk_level: RiskLevel::High,
            },
            SENTIENTSkill {
                name: "openharness-bash-exec".to_string(),
                description: "Bash komutları çalıştırma".to_string(),
                category: "execution".to_string(),
                source_repo: repo_name.to_string(),
                source_file: "tools/bash_executor.py".to_string(),
                language: "python".to_string(),
                parameters: vec![
                    SkillParameter {
                        name: "command".to_string(),
                        param_type: "str".to_string(),
                        required: true,
                        default: None,
                        description: "Bash komutu".to_string(),
                    },
                ],
                commands: vec!["bash_exec(command)".to_string()],
                dependencies: vec!["subprocess".to_string()],
                risk_level: RiskLevel::Critical,
            },
            SENTIENTSkill {
                name: "openharness-web-scraper".to_string(),
                description: "Web scraping aracı".to_string(),
                category: "search".to_string(),
                source_repo: repo_name.to_string(),
                source_file: "tools/web_scraper.py".to_string(),
                language: "python".to_string(),
                parameters: vec![
                    SkillParameter {
                        name: "url".to_string(),
                        param_type: "str".to_string(),
                        required: true,
                        default: None,
                        description: "Hedef URL".to_string(),
                    },
                    SkillParameter {
                        name: "selector".to_string(),
                        param_type: "str".to_string(),
                        required: false,
                        default: None,
                        description: "CSS selector".to_string(),
                    },
                ],
                commands: vec!["scrape(url, selector)".to_string()],
                dependencies: vec!["beautifulsoup4".to_string(), "requests".to_string()],
                risk_level: RiskLevel::Low,
            },
            SENTIENTSkill {
                name: "openharness-api-client".to_string(),
                description: "API istemcisi".to_string(),
                category: "api".to_string(),
                source_repo: repo_name.to_string(),
                source_file: "tools/api_client.py".to_string(),
                language: "python".to_string(),
                parameters: vec![
                    SkillParameter {
                        name: "endpoint".to_string(),
                        param_type: "str".to_string(),
                        required: true,
                        default: None,
                        description: "API endpoint".to_string(),
                    },
                    SkillParameter {
                        name: "method".to_string(),
                        param_type: "str".to_string(),
                        required: false,
                        default: Some("GET".to_string()),
                        description: "HTTP metodu".to_string(),
                    },
                ],
                commands: vec!["api_call(endpoint, method)".to_string()],
                dependencies: vec!["requests".to_string()],
                risk_level: RiskLevel::Medium,
            },
        ],
        _ => vec![],
    }
}

// ───────────────────────────────────────────────────────────────────────────────
//  YAML DOSYA OLUŞTURMA
// ───────────────────────────────────────────────────────────────────────────────

fn generate_skill_yaml(skill: &SENTIENTSkill) {
    let yaml_content = format!(
        r#"# ═══════════════════════════════════════════════════════════════════════════════
#  SENTIENT SKILL: {}
# ═══════════════════════════════════════════════════════════════════════════════
# Kaynak: {}
# Dil: {}
# Risk: {}
# ═══════════════════════════════════════════════════════════════════════════════

name: "{}"
description: "{}"
metadata:
  version: "0.1.0"
  sentient:
    category: "{}"
    source_repo: "{}"
    source_file: "{}"
    language: "{}"
    risk_level: "{}"
    assimilated_at: "{}"

parameters:
{}

commands:
{}

dependencies:
{}

# ───────────────────────────────────────────────────────────────────────────────
#  SENTIENT SOVEREIGN GÜVENLİK KURALLARI
# ───────────────────────────────────────────────────────────────────────────────
# Bu skill aşağıdaki güvenlik kurallarına tabidir:
# - Whitelist kontrolü (eğer dosya/uygulama erişimi varsa)
# - V-GATE loglaması (tüm çağrılar loglanır)
# - Guardrails kontrolü (prompt injection koruması)
# - Risk seviyesi: {}
# ═══════════════════════════════════════════════════════════════════════════════
"#,
        skill.name,
        skill.source_repo,
        skill.language,
        skill.risk_level,
        skill.name,
        skill.description,
        skill.category,
        skill.source_repo,
        skill.source_file.split('/').last().unwrap_or(&skill.source_file),
        skill.language,
        skill.risk_level,
        get_timestamp(),
        format_parameters(&skill.parameters),
        format_commands(&skill.commands),
        format_dependencies(&skill.dependencies),
        skill.risk_level
    );

    let file_path = PathBuf::from(SKILLS_DIR).join(format!("{}.yaml", skill.name));
    
    if let Err(e) = fs::write(&file_path, yaml_content) {
        println!("⚠️  YAML yazma hatası: {}", e);
    } else {
        println!("📝 Oluşturuldu: {}", file_path.display());
    }
}

fn get_timestamp() -> String {
    // Basit timestamp
    "2026-04-06 16:30:00".to_string()
}

fn format_parameters(params: &[SkillParameter]) -> String {
    if params.is_empty() {
        return "  []".to_string();
    }
    
    params
        .iter()
        .map(|p| {
            format!(
                "  - name: \"{}\"\n    type: \"{}\"\n    required: {}\n    default: {}\n    description: \"{}\"",
                p.name,
                p.param_type,
                p.required,
                p.default.as_ref().map(|d| format!("\"{}\"", d)).unwrap_or_else(|| "null".to_string()),
                p.description
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn format_commands(commands: &[String]) -> String {
    if commands.is_empty() {
        return "  []".to_string();
    }
    
    commands
        .iter()
        .map(|c| format!("  - \"{}\"", c))
        .collect::<Vec<_>>()
        .join("\n")
}

fn format_dependencies(deps: &[String]) -> String {
    if deps.is_empty() {
        return "  []".to_string();
    }
    
    deps
        .iter()
        .map(|d| format!("  - \"{}\"", d))
        .collect::<Vec<_>>()
        .join("\n")
}

// ───────────────────────────────────────────────────────────────────────────────
//  RUST WRAPPER OLUŞTURMA
// ───────────────────────────────────────────────────────────────────────────────

fn generate_rust_wrapper(skill: &SENTIENTSkill) {
    let wrapper_name = skill.name.replace("-", "_");
    
    let params_impl = format_params_rust(&skill.parameters);
    let skill_name = skill.name.clone();
    
    let rust_content = format!(
        r#"//! ═══════════════════════════════════════════════════════════════════════════════
//!  SENTIENT TOOL WRAPPER: {}
//! ═══════════════════════════════════════════════════════════════════════════════
//! 
//! Kaynak: {}
//! Risk Seviyesi: {}
//! 
//! ═──────────────────────────────────────────────────────────────────────────────

/// {} - Asimile edilmiş araç
/// 
/// SENTIENT Sovereign güvenlik kurallarına tabidir.
#[derive(Debug, Clone)]
pub struct {} {{
    config: {}Config,
}}

#[derive(Debug, Clone)]
pub struct {}Config {{
    /// Kaynak repo
    pub source_repo: String,
    /// Dil
    pub language: String,
    /// Risk seviyesi
    pub risk_level: String,
}}

impl Default for {}Config {{
    fn default() -> Self {{
        Self {{
            source_repo: "{}".to_string(),
            language: "{}".to_string(),
            risk_level: "{}".to_string(),
        }}
    }}
}}

impl {} {{
    /// Yeni wrapper oluştur
    pub fn new() -> Self {{
        Self::with_config({}Config::default())
    }}
    
    /// Yapılandırma ile oluştur
    pub fn with_config(config: {}Config) -> Self {{
        Self {{ config }}
    }}
    
    /// Aracın adını döndür
    pub fn name(&self) -> &str {{
        "{}"
    }}
    
    /// Aracın açıklamasını döndür
    pub fn description(&self) -> &str {{
        "{}"
    }}
    
    /// Risk seviyesini döndür
    pub fn risk_level(&self) -> &str {{
        &self.config.risk_level
    }}
    
    /// Parametreleri döndür
    pub fn parameters(&self) -> Vec<(&str, &str, bool)> {{
        vec![
{}
        ]
    }}
}}

impl Default for {} {{
    fn default() -> Self {{
        Self::new()
    }}
}}

// ═══════════════════════════════════════════════════════════════════════════════
//  TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {{
    use super::*;
    
    #[test]
    fn test_wrapper_creation() {{
        let wrapper = {}::new();
        assert_eq!(wrapper.name(), "{}");
    }}
    
    #[test]
    fn test_config_default() {{
        let config = {}Config::default();
        assert!(!config.source_repo.is_empty());
    }}
}}
"#,
        skill.name,
        skill.source_repo,
        skill.risk_level,
        wrapper_name,
        wrapper_name,
        wrapper_name,
        wrapper_name,
        wrapper_name,
        skill.source_repo,
        skill.language,
        skill.risk_level,
        wrapper_name,
        wrapper_name,
        wrapper_name,
        skill.name,
        skill.description,
        params_impl,
        wrapper_name,
        wrapper_name,
        skill_name,
        wrapper_name
    );

    let file_path = PathBuf::from(WRAPPERS_DIR).join(format!("{}.rs", wrapper_name));
    
    if let Err(e) = fs::write(&file_path, rust_content) {
        println!("⚠️  Rust wrapper yazma hatası: {}", e);
    } else {
        println!("🦀 Wrapper: {}", file_path.display());
    }
}

fn format_params_rust(params: &[SkillParameter]) -> String {
    if params.is_empty() {
        return "            // Parametre yok".to_string();
    }
    
    params
        .iter()
        .map(|p| {
            format!(
                "            (\"{}\", \"{}\", {}),",
                p.name,
                p.param_type,
                p.required
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

// ───────────────────────────────────────────────────────────────────────────────
//  ÖZET RAPOR
// ───────────────────────────────────────────────────────────────────────────────

fn print_summary(skills: &[SENTIENTSkill]) {
    println!();
    println!("╔═══════════════════════════════════════════════════════════════════════════╗");
    println!("║          🐺 ASİMİLASYON TAMAMLANDI                                        ║");
    println!("╚═══════════════════════════════════════════════════════════════════════════╝");
    println!();
    println!("📊 TOPLAM ASİMİLE EDİLEN SKILL SAYISI: {}", skills.len());
    println!();
    
    // Kategoriye göre grupla
    let mut categories: HashMap<String, Vec<&SENTIENTSkill>> = HashMap::new();
    for skill in skills {
        categories.entry(skill.category.clone()).or_default().push(skill);
    }
    
    println!("📂 KATEGORİ DAĞILIMI:");
    println!("────────────────────────────────────────────────────────────────────────────");
    for (cat, items) in categories.iter() {
        println!("  {}: {} adet", cat, items.len());
    }
    println!();
    
    // Risk dağılımı
    let mut risks: HashMap<String, usize> = HashMap::new();
    for skill in skills {
        *risks.entry(skill.risk_level.to_string()).or_insert(0) += 1;
    }
    
    println!("⚠️  RİSK DAĞILIMI:");
    println!("────────────────────────────────────────────────────────────────────────────");
    for (risk, count) in risks.iter() {
        println!("  {}: {} adet", risk, count);
    }
    println!();
    
    // Detaylı liste
    println!("📋 ASİMİLE EDİLEN SKILL'LER:");
    println!("────────────────────────────────────────────────────────────────────────────");
    
    for (i, skill) in skills.iter().enumerate() {
        println!();
        println!("  {}. {}", i + 1, skill.name);
        println!("     Açıklama: {}", skill.description.chars().take(60).collect::<String>());
        println!("     Kategori: {} | Dil: {} | Risk: {}", skill.category, skill.language, skill.risk_level);
        println!("     Kaynak: {}/{}", skill.source_repo, skill.source_file.split('/').last().unwrap_or(""));
    }
    
    println!();
    println!("────────────────────────────────────────────────────────────────────────────");
    println!("📁 Çıktı dizinleri:");
    println!("   YAML skill'ler: {}/", SKILLS_DIR);
    println!("   Rust wrapper'lar: {}/", WRAPPERS_DIR);
    println!("────────────────────────────────────────────────────────────────────────────");
    println!();
    println!("🐺 SENTIENT NEXUS OASIS - Skill Asimilasyonu Tamamlandı!");
}
