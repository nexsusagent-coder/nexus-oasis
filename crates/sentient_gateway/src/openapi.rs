//! ═══════════════════════════════════════════════════════════════════════════════
//!  OpenAPI Documentation Generator
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! Generate OpenAPI/Swagger documentation:
//! - From route definitions
//! - Schema generation
//! - Interactive docs UI
//! - Export formats

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ═══════════════════════════════════════════════════════════════════════════════
//  OPENAPI TYPES
// ═══════════════════════════════════════════════════════════════════════════════

/// OpenAPI document
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenApiDoc {
    pub openapi: String,
    pub info: Info,
    pub servers: Vec<Server>,
    pub paths: HashMap<String, PathItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Components>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<Tag>,
}

/// API info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Info {
    pub title: String,
    pub version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<Contact>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<License>,
}

/// Contact info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contact {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// License
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct License {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// Server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Server {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Path item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub get: Option<Operation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post: Option<Operation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub put: Option<Operation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete: Option<Operation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch: Option<Operation>,
}

/// Operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Operation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    pub parameters: Vec<Parameter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_body: Option<RequestBody>,
    pub responses: HashMap<String, Response>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub security: Vec<HashMap<String, Vec<String>>>,
}

/// Parameter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parameter {
    pub name: String,
    #[serde(rename = "in")]
    pub location: ParameterLocation,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub required: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<Schema>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ParameterLocation {
    #[serde(rename = "query")]
    Query,
    #[serde(rename = "header")]
    Header,
    #[serde(rename = "path")]
    Path,
    #[serde(rename = "cookie")]
    Cookie,
}

/// Request body
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub content: HashMap<String, MediaType>,
    pub required: bool,
}

/// Media type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaType {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<Schema>,
}

/// Response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    pub description: String,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub content: HashMap<String, MediaType>,
}

/// Components
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Components {
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub schemas: HashMap<String, Schema>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub security_schemes: HashMap<String, SecurityScheme>,
}

/// Schema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Schema {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub schema_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<HashMap<String, Schema>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Box<Schema>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enum_values: Option<Vec<String>>,
    #[serde(rename = "$ref", skip_serializing_if = "Option::is_none")]
    pub ref_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub example: Option<serde_json::Value>,
}

/// Security scheme
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityScheme {
    #[serde(rename = "type")]
    pub scheme_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheme: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bearer_format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "in", skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}

/// Tag
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

// ═══════════════════════════════════════════════════════════════════════════════
//  DOCUMENT BUILDER
// ═══════════════════════════════════════════════════════════════════════════════

/// OpenAPI builder
pub struct OpenApiBuilder {
    doc: OpenApiDoc,
}

impl OpenApiBuilder {
    pub fn new(title: &str, version: &str) -> Self {
        Self {
            doc: OpenApiDoc {
                openapi: "3.0.3".to_string(),
                info: Info {
                    title: title.to_string(),
                    version: version.to_string(),
                    description: None,
                    contact: None,
                    license: None,
                },
                servers: vec![],
                paths: HashMap::new(),
                components: None,
                tags: vec![],
            },
        }
    }
    
    pub fn description(mut self, desc: &str) -> Self {
        self.doc.info.description = Some(desc.to_string());
        self
    }
    
    pub fn server(mut self, url: &str, description: Option<&str>) -> Self {
        self.doc.servers.push(Server {
            url: url.to_string(),
            description: description.map(|s| s.to_string()),
        });
        self
    }
    
    pub fn tag(mut self, name: &str, description: Option<&str>) -> Self {
        self.doc.tags.push(Tag {
            name: name.to_string(),
            description: description.map(|s| s.to_string()),
        });
        self
    }
    
    pub fn path(mut self, path: &str, item: PathItem) -> Self {
        self.doc.paths.insert(path.to_string(), item);
        self
    }
    
    pub fn get(mut self, path: &str, op: Operation) -> Self {
        let path_item = self.doc.paths.entry(path.to_string()).or_insert(PathItem {
            summary: None,
            description: None,
            get: None,
            post: None,
            put: None,
            delete: None,
            patch: None,
        });
        path_item.get = Some(op);
        self
    }
    
    pub fn post(mut self, path: &str, op: Operation) -> Self {
        let path_item = self.doc.paths.entry(path.to_string()).or_insert(PathItem {
            summary: None,
            description: None,
            get: None,
            post: None,
            put: None,
            delete: None,
            patch: None,
        });
        path_item.post = Some(op);
        self
    }
    
    pub fn schema(mut self, name: &str, schema: Schema) -> Self {
        self.doc.components
            .get_or_insert_with(|| Components {
                schemas: HashMap::new(),
                security_schemes: HashMap::new(),
            })
            .schemas
            .insert(name.to_string(), schema);
        self
    }
    
    pub fn security_scheme(mut self, name: &str, scheme: SecurityScheme) -> Self {
        self.doc.components
            .get_or_insert_with(|| Components {
                schemas: HashMap::new(),
                security_schemes: HashMap::new(),
            })
            .security_schemes
            .insert(name.to_string(), scheme);
        self
    }
    
    pub fn build(self) -> OpenApiDoc {
        self.doc
    }
    
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(&self.doc)
    }
    
    pub fn to_yaml(&self) -> Result<String, serde_yaml::Error> {
        serde_yaml::to_string(&self.doc)
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  OPERATION BUILDER
// ═══════════════════════════════════════════════════════════════════════════════

/// Operation builder helper
pub struct OperationBuilder {
    op: Operation,
}

impl OperationBuilder {
    pub fn new() -> Self {
        Self {
            op: Operation {
                summary: None,
                description: None,
                operation_id: None,
                tags: vec![],
                parameters: vec![],
                request_body: None,
                responses: HashMap::new(),
                security: vec![],
            },
        }
    }
    
    pub fn summary(mut self, s: &str) -> Self {
        self.op.summary = Some(s.to_string());
        self
    }
    
    pub fn description(mut self, d: &str) -> Self {
        self.op.description = Some(d.to_string());
        self
    }
    
    pub fn operation_id(mut self, id: &str) -> Self {
        self.op.operation_id = Some(id.to_string());
        self
    }
    
    pub fn tag(mut self, tag: &str) -> Self {
        self.op.tags.push(tag.to_string());
        self
    }
    
    pub fn parameter(mut self, name: &str, location: ParameterLocation, required: bool) -> Self {
        self.op.parameters.push(Parameter {
            name: name.to_string(),
            location,
            description: None,
            required,
            schema: None,
        });
        self
    }
    
    pub fn response(mut self, code: &str, description: &str) -> Self {
        self.op.responses.insert(code.to_string(), Response {
            description: description.to_string(),
            content: HashMap::new(),
        });
        self
    }
    
    pub fn json_response(mut self, code: &str, description: &str, schema: Schema) -> Self {
        let mut content = HashMap::new();
        content.insert("application/json".to_string(), MediaType {
            schema: Some(schema),
        });
        
        self.op.responses.insert(code.to_string(), Response {
            description: description.to_string(),
            content,
        });
        self
    }
    
    pub fn json_body(mut self, description: &str, schema: Schema, required: bool) -> Self {
        let mut content = HashMap::new();
        content.insert("application/json".to_string(), MediaType {
            schema: Some(schema),
        });
        
        self.op.request_body = Some(RequestBody {
            description: Some(description.to_string()),
            content,
            required,
        });
        self
    }
    
    pub fn security(mut self, name: &str, scopes: Vec<&str>) -> Self {
        let mut sec = HashMap::new();
        sec.insert(name.to_string(), scopes.iter().map(|s| s.to_string()).collect());
        self.op.security.push(sec);
        self
    }
    
    pub fn build(self) -> Operation {
        self.op
    }
}

impl Default for OperationBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  SCHEMA HELPERS
// ═══════════════════════════════════════════════════════════════════════════════

impl Schema {
    pub fn string() -> Self {
        Self {
            schema_type: Some("string".to_string()),
            format: None,
            description: None,
            properties: None,
            required: None,
            items: None,
            enum_values: None,
            ref_path: None,
            example: None,
        }
    }
    
    pub fn integer() -> Self {
        Self {
            schema_type: Some("integer".to_string()),
            format: None,
            description: None,
            properties: None,
            required: None,
            items: None,
            enum_values: None,
            ref_path: None,
            example: None,
        }
    }
    
    pub fn number() -> Self {
        Self {
            schema_type: Some("number".to_string()),
            format: None,
            description: None,
            properties: None,
            required: None,
            items: None,
            enum_values: None,
            ref_path: None,
            example: None,
        }
    }
    
    pub fn boolean() -> Self {
        Self {
            schema_type: Some("boolean".to_string()),
            format: None,
            description: None,
            properties: None,
            required: None,
            items: None,
            enum_values: None,
            ref_path: None,
            example: None,
        }
    }
    
    pub fn array(items: Schema) -> Self {
        Self {
            schema_type: Some("array".to_string()),
            format: None,
            description: None,
            properties: None,
            required: None,
            items: Some(Box::new(items)),
            enum_values: None,
            ref_path: None,
            example: None,
        }
    }
    
    pub fn object(properties: HashMap<String, Schema>) -> Self {
        Self {
            schema_type: Some("object".to_string()),
            format: None,
            description: None,
            properties: Some(properties),
            required: None,
            items: None,
            enum_values: None,
            ref_path: None,
            example: None,
        }
    }
    
    pub fn ref_schema(ref_path: &str) -> Self {
        Self {
            schema_type: None,
            format: None,
            description: None,
            properties: None,
            required: None,
            items: None,
            enum_values: None,
            ref_path: Some(ref_path.to_string()),
            example: None,
        }
    }
    
    pub fn description(mut self, d: &str) -> Self {
        self.description = Some(d.to_string());
        self
    }
    
    pub fn format(mut self, f: &str) -> Self {
        self.format = Some(f.to_string());
        self
    }
    
    pub fn example(mut self, e: serde_json::Value) -> Self {
        self.example = Some(e);
        self
    }
    
    pub fn required(mut self, fields: Vec<&str>) -> Self {
        self.required = Some(fields.iter().map(|s| s.to_string()).collect());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_openapi_builder() {
        let doc = OpenApiDoc {
            openapi: "3.0.0".to_string(),
            info: Info {
                title: "Test API".to_string(),
                version: "1.0.0".to_string(),
                description: Some("Test description".to_string()),
                contact: None,
                license: None,
            },
            servers: vec![Server {
                url: "http://localhost:8080".to_string(),
                description: None,
            }],
            paths: HashMap::new(),
            components: None,
            tags: vec![],
        };
        
        assert_eq!(doc.openapi, "3.0.0");
    }
    
    #[test]
    fn test_operation_builder() {
        let op = OperationBuilder::new()
            .summary("Get users")
            .operation_id("getUsers")
            .response("200", "Success")
            .build();
        
        assert_eq!(op.summary, Some("Get users".to_string()));
    }
    
    #[test]
    fn test_schema_builders() {
        let s = Schema::string().format("email").description("Email address");
        assert_eq!(s.schema_type, Some("string".to_string()));
        assert_eq!(s.format, Some("email".to_string()));
    }
}
