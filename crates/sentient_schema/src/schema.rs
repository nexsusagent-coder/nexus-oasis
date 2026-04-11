// ═══════════════════════════════════════════════════════════════════════════════
//  SENTIENT OS - JSON Schema Generation
// ═══════════════════════════════════════════════════════════════════════════════

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use schemars::schema_for;
use std::collections::HashMap;

/// Re-export schemars JsonSchema trait for derive
pub use schemars::JsonSchema;

/// Schema wrapper for structured outputs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Schema {
    /// Schema name
    pub name: String,
    /// Schema definition
    pub schema: Value,
    /// Description
    pub description: Option<String>,
}

impl Schema {
    /// Create from a type that implements JsonSchema
    pub fn from_type<T: schemars::JsonSchema>() -> Self {
        let schema = schema_for!(T);
        Self {
            name: T::schema_name(),
            schema: serde_json::to_value(schema).unwrap_or_default(),
            description: None,
        }
    }

    /// Create from raw JSON Value
    pub fn from_value(name: impl Into<String>, schema: Value) -> Self {
        Self {
            name: name.into(),
            schema,
            description: None,
        }
    }

    /// Add description
    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    /// Convert to OpenAI function format
    pub fn to_openai_function(&self) -> Value {
        json!({
            "name": self.name,
            "parameters": self.schema
        })
    }

    /// Convert to Anthropic tool format
    pub fn to_anthropic_tool(&self) -> Value {
        json!({
            "name": self.name,
            "input_schema": self.schema
        })
    }

    /// Convert to Ollama format (grammar)
    pub fn to_ollama_format(&self) -> Value {
        json!({
            "format": "json",
            "schema": self.schema
        })
    }
}

/// Schema builder for dynamic schemas
pub struct SchemaBuilder {
    name: String,
    schema_type: String,
    properties: HashMap<String, Value>,
    required: Vec<String>,
    description: Option<String>,
}

impl SchemaBuilder {
    /// Create a new object schema
    pub fn object(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            schema_type: "object".to_string(),
            properties: HashMap::new(),
            required: Vec::new(),
            description: None,
        }
    }

    /// Add string property
    pub fn string(mut self, name: impl Into<String>, desc: Option<&str>) -> Self {
        let prop = json!({
            "type": "string",
            "description": desc.unwrap_or("")
        });
        self.properties.insert(name.into(), prop);
        self
    }

    /// Add required string property
    pub fn required_string(mut self, name: impl Into<String>, desc: Option<&str>) -> Self {
        let name_str = name.into();
        self.properties.insert(
            name_str.clone(),
            json!({
                "type": "string",
                "description": desc.unwrap_or("")
            }),
        );
        self.required.push(name_str);
        self
    }

    /// Add integer property
    pub fn integer(mut self, name: impl Into<String>, desc: Option<&str>) -> Self {
        self.properties.insert(
            name.into(),
            json!({
                "type": "integer",
                "description": desc.unwrap_or("")
            }),
        );
        self
    }

    /// Add number property
    pub fn number(mut self, name: impl Into<String>, desc: Option<&str>) -> Self {
        self.properties.insert(
            name.into(),
            json!({
                "type": "number",
                "description": desc.unwrap_or("")
            }),
        );
        self
    }

    /// Add boolean property
    pub fn boolean(mut self, name: impl Into<String>, desc: Option<&str>) -> Self {
        self.properties.insert(
            name.into(),
            json!({
                "type": "boolean",
                "description": desc.unwrap_or("")
            }),
        );
        self
    }

    /// Add array property
    pub fn array(mut self, name: impl Into<String>, item_type: &str, desc: Option<&str>) -> Self {
        self.properties.insert(
            name.into(),
            json!({
                "type": "array",
                "items": {"type": item_type},
                "description": desc.unwrap_or("")
            }),
        );
        self
    }

    /// Add enum property
    pub fn enum_values(mut self, name: impl Into<String>, values: &[&str], desc: Option<&str>) -> Self {
        self.properties.insert(
            name.into(),
            json!({
                "type": "string",
                "enum": values,
                "description": desc.unwrap_or("")
            }),
        );
        self
    }

    /// Add nested object property
    pub fn object_property(mut self, name: impl Into<String>, schema: Schema) -> Self {
        self.properties.insert(name.into(), schema.schema);
        self
    }

    /// Set description
    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    /// Build the schema
    pub fn build(self) -> Schema {
        let schema = json!({
            "type": self.schema_type,
            "properties": self.properties,
            "required": self.required,
            "description": self.description.unwrap_or_default()
        });

        Schema {
            name: self.name,
            schema,
            description: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_schema_builder() {
        let schema = SchemaBuilder::object("Person")
            .required_string("name", Some("Person's name"))
            .integer("age", Some("Person's age"))
            .enum_values("gender", &["male", "female", "other"], None)
            .build();

        assert_eq!(schema.name, "Person");
        assert!(schema.schema["properties"]["name"].is_object());
    }

    #[test]
    fn test_openai_function_format() {
        let schema = SchemaBuilder::object("Test")
            .required_string("field", None)
            .build();

        let func = schema.to_openai_function();
        assert_eq!(func["name"], "Test");
    }
}
