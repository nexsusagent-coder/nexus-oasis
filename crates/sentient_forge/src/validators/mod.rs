//! ─── KOD VALIDATORLERI ───

use crate::ToolType;
use sentient_common::error::{SENTIENTError, SENTIENTResult};

/// Validation sonucu
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ValidationResult {
    /// Gecerli mi?
    pub valid: bool,
    /// Hatalar
    pub errors: Vec<String>,
    /// Uyarilar
    pub warnings: Vec<String>,
    /// Skor (0-100)
    pub score: u32,
}

impl ValidationResult {
    pub fn ok() -> Self {
        Self { valid: true, errors: vec![], warnings: vec![], score: 100 }
    }
    
    pub fn with_warnings(warnings: Vec<String>) -> Self {
        Self {
            valid: true,
            errors: vec![],
            warnings,
            score: 80,
        }
    }
    
    pub fn with_errors(errors: Vec<String>) -> Self {
        Self {
            valid: false,
            errors,
            warnings: vec![],
            score: 0,
        }
    }
}

/// Kod Validator
pub struct CodeValidator {
    python_validator: PythonValidator,
    json_validator: JsonValidator,
    yaml_validator: YamlValidator,
}

impl CodeValidator {
    pub fn new() -> Self {
        Self {
            python_validator: PythonValidator::new(),
            json_validator: JsonValidator::new(),
            yaml_validator: YamlValidator::new(),
        }
    }
    
    pub fn validate(&self, tool_type: &ToolType, code: &str) -> SENTIENTResult<ValidationResult> {
        match tool_type {
            ToolType::N8nWorkflow => self.json_validator.validate(code),
            ToolType::PythonScript => self.python_validator.validate(code),
            ToolType::GitHubAction => self.yaml_validator.validate(code),
            ToolType::DockerCompose => self.yaml_validator.validate(code),
            _ => Ok(ValidationResult::ok()),
        }
    }
}

impl Default for CodeValidator {
    fn default() -> Self {
        Self::new()
    }
}

/// Python Validator
pub struct PythonValidator {
    forbidden_patterns: Vec<&'static str>,
}

impl PythonValidator {
    pub fn new() -> Self {
        Self {
            forbidden_patterns: vec![
                "exec(",
                "eval(",
                "__import__",
                "compile(",
                "os.system",
                "subprocess.call",
            ],
        }
    }
    
    pub fn validate(&self, code: &str) -> SENTIENTResult<ValidationResult> {
        let mut errors = Vec::new();
        let mut warnings = Vec::new();
        
        // Yasak pattern'leri kontrol et
        for pattern in &self.forbidden_patterns {
            if code.contains(pattern) {
                errors.push(format!("Yasak pattern bulundu: {}", pattern));
            }
        }
        
        // Temel sentaks kontrolleri
        if code.contains("def ") {
            // Fonksiyon tanimi var
        } else {
            warnings.push("Fonksiyon tanimi bulunamadi".into());
        }
        
        // Import kontrolu
        if !code.contains("import") {
            warnings.push("Import deyimi bulunamadi".into());
        }
        
        if !errors.is_empty() {
            Ok(ValidationResult::with_errors(errors))
        } else if !warnings.is_empty() {
            Ok(ValidationResult::with_warnings(warnings))
        } else {
            Ok(ValidationResult::ok())
        }
    }
}

impl Default for PythonValidator {
    fn default() -> Self {
        Self::new()
    }
}

/// JSON Validator
pub struct JsonValidator;

impl JsonValidator {
    pub fn new() -> Self {
        Self
    }
    
    pub fn validate(&self, code: &str) -> SENTIENTResult<ValidationResult> {
        match serde_json::from_str::<serde_json::Value>(code) {
            Ok(value) => {
                // n8n workflow validasyonu
                if let Some(obj) = value.as_object() {
                    if obj.contains_key("name") && obj.contains_key("nodes") {
                        return Ok(ValidationResult::ok());
                    }
                }
                Ok(ValidationResult::with_warnings(vec!["Gecerli JSON ama n8n formatina uymuyor".into()]))
            }
            Err(e) => Ok(ValidationResult::with_errors(vec![format!("JSON parse hatasi: {}", e)]))
        }
    }
}

impl Default for JsonValidator {
    fn default() -> Self {
        Self::new()
    }
}

/// YAML Validator
pub struct YamlValidator {
    required_fields: Vec<&'static str>,
}

impl YamlValidator {
    pub fn new() -> Self {
        Self {
            required_fields: vec!["name:", "on:", "jobs:"],
        }
    }
    
    pub fn validate(&self, code: &str) -> SENTIENTResult<ValidationResult> {
        let mut warnings = Vec::new();
        
        // Temel YAML validasyonu
        for field in &self.required_fields {
            if !code.contains(field) {
                warnings.push(format!("Gerekli alan eksik: {}", field));
            }
        }
        
        // YAML parse denemesi
        match serde_yaml::from_str::<serde_json::Value>(code) {
            Ok(value) => {
                if warnings.is_empty() {
                    Ok(ValidationResult::ok())
                } else {
                    Ok(ValidationResult::with_warnings(warnings))
                }
            }
            Err(e) => Ok(ValidationResult::with_errors(vec![format!("YAML parse hatasi: {}", e)]))
        }
    }
}

impl Default for YamlValidator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_python_validator_ok() {
        let validator = PythonValidator::new();
        let code = "import requests\ndef main(): pass";
        let result = validator.validate(code).expect("operation failed");
        assert!(result.valid);
    }
    
    #[test]
    fn test_python_validator_forbidden() {
        let validator = PythonValidator::new();
        let code = "exec('hello')";
        let result = validator.validate(code).expect("operation failed");
        assert!(!result.valid);
    }
    
    #[test]
    fn test_json_validator_ok() {
        let validator = JsonValidator::new();
        let code = r#"{"name": "test", "nodes": []}"#;
        let result = validator.validate(code).expect("operation failed");
        assert!(result.valid);
    }
    
    #[test]
    fn test_json_validator_invalid() {
        let validator = JsonValidator::new();
        let code = "{invalid json}";
        let result = validator.validate(code).expect("operation failed");
        assert!(!result.valid);
    }
}
