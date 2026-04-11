// ═══════════════════════════════════════════════════════════════════════════════
//  SENTIENT OS - Function Calling
// ═══════════════════════════════════════════════════════════════════════════════

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Function definition for LLM tool use
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionDef {
    /// Function name
    pub name: String,
    /// Function description
    pub description: String,
    /// Function parameters
    pub parameters: Vec<Parameter>,
}

impl FunctionDef {
    /// Create a new function definition
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            parameters: Vec::new(),
        }
    }

    /// Add a parameter
    pub fn param(mut self, param: Parameter) -> Self {
        self.parameters.push(param);
        self
    }

    /// Add a string parameter
    pub fn string_param(
        self,
        name: impl Into<String>,
        description: impl Into<String>,
        required: bool,
    ) -> Self {
        self.param(Parameter::string(name, description, required))
    }

    /// Add an integer parameter
    pub fn integer_param(
        self,
        name: impl Into<String>,
        description: impl Into<String>,
        required: bool,
    ) -> Self {
        self.param(Parameter::integer(name, description, required))
    }

    /// Add an enum parameter
    pub fn enum_param(
        self,
        name: impl Into<String>,
        description: impl Into<String>,
        values: &[&str],
        required: bool,
    ) -> Self {
        self.param(Parameter::enum_values(name, description, values, required))
    }

    /// Convert to OpenAI function format
    pub fn to_openai(&self) -> Value {
        let mut properties = serde_json::Map::new();
        let mut required = Vec::new();

        for param in &self.parameters {
            properties.insert(param.name.clone(), param.to_schema());
            if param.required {
                required.push(param.name.clone());
            }
        }

        serde_json::json!({
            "name": self.name,
            "description": self.description,
            "parameters": {
                "type": "object",
                "properties": properties,
                "required": required
            }
        })
    }

    /// Convert to Anthropic tool format
    pub fn to_anthropic(&self) -> Value {
        let mut properties = serde_json::Map::new();
        let mut required = Vec::new();

        for param in &self.parameters {
            properties.insert(param.name.clone(), param.to_schema());
            if param.required {
                required.push(param.name.clone());
            }
        }

        serde_json::json!({
            "name": self.name,
            "description": self.description,
            "input_schema": {
                "type": "object",
                "properties": properties,
                "required": required
            }
        })
    }
}

/// Function parameter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parameter {
    /// Parameter name
    pub name: String,
    /// Parameter type
    pub param_type: ParameterType,
    /// Description
    pub description: String,
    /// Is required
    pub required: bool,
    /// Default value
    pub default: Option<Value>,
    /// Enum values (if applicable)
    pub enum_values: Vec<String>,
}

impl Parameter {
    /// Create a new parameter
    pub fn new(name: impl Into<String>, param_type: ParameterType) -> Self {
        Self {
            name: name.into(),
            param_type,
            description: String::new(),
            required: true,
            default: None,
            enum_values: Vec::new(),
        }
    }

    /// Create a string parameter
    pub fn string(name: impl Into<String>, description: impl Into<String>, required: bool) -> Self {
        Self {
            name: name.into(),
            param_type: ParameterType::String,
            description: description.into(),
            required,
            default: None,
            enum_values: Vec::new(),
        }
    }

    /// Create an integer parameter
    pub fn integer(name: impl Into<String>, description: impl Into<String>, required: bool) -> Self {
        Self {
            name: name.into(),
            param_type: ParameterType::Integer,
            description: description.into(),
            required,
            default: None,
            enum_values: Vec::new(),
        }
    }

    /// Create an enum parameter
    pub fn enum_values(
        name: impl Into<String>,
        description: impl Into<String>,
        values: &[&str],
        required: bool,
    ) -> Self {
        Self {
            name: name.into(),
            param_type: ParameterType::String,
            description: description.into(),
            required,
            default: None,
            enum_values: values.iter().map(|s| s.to_string()).collect(),
        }
    }

    /// Set default value
    pub fn with_default(mut self, value: Value) -> Self {
        self.default = Some(value);
        self
    }

    /// Convert to JSON Schema
    pub fn to_schema(&self) -> Value {
        let mut schema = serde_json::json!({
            "type": self.param_type.to_string(),
            "description": self.description
        });

        if !self.enum_values.is_empty() {
            schema["enum"] = serde_json::json!(self.enum_values);
        }

        if let Some(default) = &self.default {
            schema["default"] = default.clone();
        }

        schema
    }
}

/// Parameter types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ParameterType {
    String,
    Integer,
    Number,
    Boolean,
    Array(Box<ParameterType>),
    Object,
}

impl std::fmt::Display for ParameterType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParameterType::String => write!(f, "string"),
            ParameterType::Integer => write!(f, "integer"),
            ParameterType::Number => write!(f, "number"),
            ParameterType::Boolean => write!(f, "boolean"),
            ParameterType::Array(inner) => write!(f, "array({})", inner),
            ParameterType::Object => write!(f, "object"),
        }
    }
}

/// Function call result from LLM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionCall {
    /// Function name
    pub name: String,
    /// Arguments as JSON
    pub arguments: Value,
}

impl FunctionCall {
    /// Get argument as type
    pub fn get<T: for<'de> Deserialize<'de>>(&self, key: &str) -> Option<T> {
        self.arguments.get(key).and_then(|v| serde_json::from_value(v.clone()).ok())
    }

    /// Get all arguments as a struct
    pub fn parse<T: for<'de> Deserialize<'de>>(self) -> Result<T, serde_json::Error> {
        serde_json::from_value(self.arguments)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_def() {
        let func = FunctionDef::new("get_weather", "Get weather for a location")
            .string_param("location", "City name", true)
            .enum_param("unit", "Temperature unit", &["celsius", "fahrenheit"], false);

        let openai = func.to_openai();
        assert_eq!(openai["name"], "get_weather");
    }

    #[test]
    fn test_function_call() {
        let call = FunctionCall {
            name: "test".to_string(),
            arguments: serde_json::json!({"foo": "bar"}),
        };

        assert_eq!(call.get::<String>("foo"), Some("bar".to_string()));
    }
}
