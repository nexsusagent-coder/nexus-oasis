// ═══════════════════════════════════════════════════════════════════════════════
//  SENTIENT OS - Structured Extraction (Instructor-style)
// ═══════════════════════════════════════════════════════════════════════════════

use serde::{Deserialize, Serialize};
use regex::Regex;

/// Extraction configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractionConfig {
    /// Maximum retries on validation failure
    pub max_retries: u32,
    /// Temperature for generation
    pub temperature: f32,
    /// Model to use
    pub model: Option<String>,
    /// Timeout in seconds
    pub timeout: u64,
    /// Include validation feedback in retry
    pub validation_feedback: bool,
}

impl Default for ExtractionConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            temperature: 0.0,
            model: None,
            timeout: 30,
            validation_feedback: true,
        }
    }
}

/// Structured extractor for reliable LLM outputs
pub struct StructuredExtractor {
    config: ExtractionConfig,
}

impl StructuredExtractor {
    pub fn new(config: ExtractionConfig) -> Self {
        Self { config }
    }

    /// Extract JSON from LLM response
    pub fn extract_json(&self, response: &str) -> Option<serde_json::Value> {
        // Try direct parse first
        if let Ok(value) = serde_json::from_str(response) {
            return Some(value);
        }

        // Try to find JSON in markdown code blocks
        let json_block = Regex::new(r"```(?:json)?\s*([\s\S]*?)```").unwrap();
        if let Some(caps) = json_block.captures(response) {
            if let Ok(value) = serde_json::from_str(&caps[1]) {
                return Some(value);
            }
        }

        // Try to find JSON object/array patterns
        let json_pattern = Regex::new(r"\{[\s\S]*\}|\[[\s\S]*\]").unwrap();
        if let Some(m) = json_pattern.find(response) {
            if let Ok(value) = serde_json::from_str(m.as_str()) {
                return Some(value);
            }
        }

        None
    }

    /// Build prompt for extraction
    pub fn build_extraction_prompt(schema_name: &str, schema: &serde_json::Value, user_prompt: &str) -> String {
        format!(
            r#"You are a precise data extraction assistant. Your task is to extract structured data.

Schema: {}
{}

IMPORTANT:
- Respond ONLY with valid JSON that matches the schema
- Do not include any explanation or markdown
- Do not wrap in code blocks
- Ensure all required fields are present

User request: {}"#,
            schema_name,
            serde_json::to_string_pretty(schema).unwrap_or_default(),
            user_prompt
        )
    }

    /// Build retry prompt with validation error
    pub fn build_retry_prompt(
        schema_name: &str,
        schema: &serde_json::Value,
        user_prompt: &str,
        previous_response: &str,
        error: &str,
    ) -> String {
        format!(
            r#"Your previous response had validation errors.

Schema: {}
{}

Previous response:
{}

Error: {}

Please correct the response and provide valid JSON that matches the schema.
User request: {}"#,
            schema_name,
            serde_json::to_string_pretty(schema).unwrap_or_default(),
            previous_response,
            error,
            user_prompt
        )
    }

    /// Get config
    pub fn config(&self) -> &ExtractionConfig {
        &self.config
    }
}

impl Default for StructuredExtractor {
    fn default() -> Self {
        Self::new(ExtractionConfig::default())
    }
}

/// Validation result
#[derive(Debug)]
pub enum ValidationResult {
    Valid,
    Invalid(Vec<String>),
}

impl ValidationResult {
    pub fn is_valid(&self) -> bool {
        matches!(self, ValidationResult::Valid)
    }

    pub fn errors(&self) -> Option<&[String]> {
        match self {
            ValidationResult::Invalid(errors) => Some(errors),
            _ => None,
        }
    }
}

/// Schema validator
pub struct SchemaValidator;

impl SchemaValidator {
    /// Validate a value against a schema
    pub fn validate(value: &serde_json::Value, schema: &serde_json::Value) -> ValidationResult {
        let mut errors = Vec::new();

        // Check required fields
        if let Some(required) = schema.get("required").and_then(|r| r.as_array()) {
            if let Some(properties) = value.as_object() {
                for field in required {
                    if let Some(field_name) = field.as_str() {
                        if !properties.contains_key(field_name) {
                            errors.push(format!("Missing required field: {}", field_name));
                        }
                    }
                }
            }
        }

        // Check type constraints
        if let Some(props) = schema.get("properties").and_then(|p| p.as_object()) {
            if let Some(value_obj) = value.as_object() {
                for (key, prop_schema) in props {
                    if let Some(val) = value_obj.get(key) {
                        if let Some(type_error) = Self::check_type(val, prop_schema) {
                            errors.push(format!("Field '{}': {}", key, type_error));
                        }
                    }
                }
            }
        }

        if errors.is_empty() {
            ValidationResult::Valid
        } else {
            ValidationResult::Invalid(errors)
        }
    }

    fn check_type(value: &serde_json::Value, schema: &serde_json::Value) -> Option<String> {
        let expected_type = schema.get("type")?.as_str()?;

        let valid = match (expected_type, value) {
            ("string", serde_json::Value::String(_)) => true,
            ("integer", serde_json::Value::Number(n)) => n.is_i64(),
            ("number", serde_json::Value::Number(_)) => true,
            ("boolean", serde_json::Value::Bool(_)) => true,
            ("array", serde_json::Value::Array(_)) => true,
            ("object", serde_json::Value::Object(_)) => true,
            _ => false,
        };

        if valid {
            None
        } else {
            Some(format!("Expected type {}, got {:?}", expected_type, value))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_extract_json() {
        let extractor = StructuredExtractor::default();

        // Direct JSON
        let json = extractor.extract_json(r#"{"name": "test"}"#);
        assert!(json.is_some());

        // Markdown code block
        let json = extractor.extract_json(r#"```json
        {"name": "test"}
        ```"#);
        assert!(json.is_some());
    }

    #[test]
    fn test_validator() {
        let schema = json!({
            "type": "object",
            "required": ["name"],
            "properties": {
                "name": {"type": "string"},
                "age": {"type": "integer"}
            }
        });

        // Valid
        let valid = json!({"name": "John", "age": 30});
        assert!(SchemaValidator::validate(&valid, &schema).is_valid());

        // Missing required
        let invalid = json!({"age": 30});
        assert!(!SchemaValidator::validate(&invalid, &schema).is_valid());
    }
}
