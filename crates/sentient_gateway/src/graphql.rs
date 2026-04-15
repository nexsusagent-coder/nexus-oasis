//! ═══════════════════════════════════════════════════════════════════════════════
//!  GraphQL API Support
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! GraphQL API for flexible queries:
//! - Query/Mutation/Subscription
//! - Schema definition
//! - Resolver implementation
//! - Real-time subscriptions

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ═══════════════════════════════════════════════════════════════════════════════
//  GRAPHQL TYPES
// ═══════════════════════════════════════════════════════════════════════════════

/// GraphQL type kind
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TypeKind {
    SCALAR,
    OBJECT,
    INTERFACE,
    UNION,
    ENUM,
    INPUT_OBJECT,
    LIST,
    NON_NULL,
}

/// GraphQL field definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldDef {
    pub name: String,
    pub description: Option<String>,
    pub args: Vec<InputValueDef>,
    pub type_: String,
    pub is_deprecated: bool,
    pub deprecation_reason: Option<String>,
}

/// Input value definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputValueDef {
    pub name: String,
    pub description: Option<String>,
    pub type_: String,
    pub default_value: Option<String>,
}

/// GraphQL type definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeDef {
    pub kind: TypeKind,
    pub name: String,
    pub description: Option<String>,
    pub fields: Vec<FieldDef>,
    pub interfaces: Vec<String>,
    pub possible_types: Vec<String>,
    pub enum_values: Vec<EnumValueDef>,
    pub input_fields: Vec<InputValueDef>,
}

/// Enum value definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnumValueDef {
    pub name: String,
    pub description: Option<String>,
    pub is_deprecated: bool,
    pub deprecation_reason: Option<String>,
}

// ═══════════════════════════════════════════════════════════════════════════════
//  GRAPHQL REQUEST/RESPONSE
// ═══════════════════════════════════════════════════════════════════════════════

/// GraphQL request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphQLRequest {
    pub query: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_name: Option<String>,
    #[serde(default)]
    pub variables: HashMap<String, serde_json::Value>,
}

/// GraphQL response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphQLResponse {
    pub data: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<GraphQLError>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<HashMap<String, serde_json::Value>>,
}

/// GraphQL error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphQLError {
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locations: Option<Vec<SourceLocation>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<Vec<PathSegment>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<HashMap<String, serde_json::Value>>,
}

/// Source location
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceLocation {
    pub line: u32,
    pub column: u32,
}

/// Path segment
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PathSegment {
    String(String),
    Int(i32),
}

// ═══════════════════════════════════════════════════════════════════════════════
//  GRAPHQL SCHEMA BUILDER
// ═══════════════════════════════════════════════════════════════════════════════

/// GraphQL schema
#[derive(Debug, Clone, Default)]
pub struct GraphQLSchema {
    pub types: HashMap<String, TypeDef>,
    pub query_type: String,
    pub mutation_type: Option<String>,
    pub subscription_type: Option<String>,
    pub directives: Vec<DirectiveDef>,
}

/// Directive definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectiveDef {
    pub name: String,
    pub description: Option<String>,
    pub locations: Vec<DirectiveLocation>,
    pub args: Vec<InputValueDef>,
}

/// Directive location
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DirectiveLocation {
    QUERY,
    MUTATION,
    SUBSCRIPTION,
    FIELD,
    FRAGMENT_DEFINITION,
    FRAGMENT_SPREAD,
    INLINE_FRAGMENT,
    VARIABLE_DEFINITION,
}

/// Schema builder
pub struct SchemaBuilder {
    schema: GraphQLSchema,
}

impl SchemaBuilder {
    pub fn new() -> Self {
        let mut schema = GraphQLSchema::default();
        
        // Add built-in scalars
        Self::add_scalar(&mut schema, "String", "UTF-8 character sequence");
        Self::add_scalar(&mut schema, "Int", "Signed 32-bit integer");
        Self::add_scalar(&mut schema, "Float", "Signed double-precision floating-point value");
        Self::add_scalar(&mut schema, "Boolean", "true or false");
        Self::add_scalar(&mut schema, "ID", "Unique identifier");
        
        Self { schema }
    }
    
    fn add_scalar(schema: &mut GraphQLSchema, name: &str, description: &str) {
        schema.types.insert(name.to_string(), TypeDef {
            kind: TypeKind::SCALAR,
            name: name.to_string(),
            description: Some(description.to_string()),
            fields: vec![],
            interfaces: vec![],
            possible_types: vec![],
            enum_values: vec![],
            input_fields: vec![],
        });
    }
    
    /// Add a query type
    pub fn query_type(mut self, name: &str) -> Self {
        self.schema.query_type = name.to_string();
        self.schema.types.insert(name.to_string(), TypeDef {
            kind: TypeKind::OBJECT,
            name: name.to_string(),
            description: Some("Root query type".to_string()),
            fields: vec![],
            interfaces: vec![],
            possible_types: vec![],
            enum_values: vec![],
            input_fields: vec![],
        });
        self
    }
    
    /// Add a mutation type
    pub fn mutation_type(mut self, name: &str) -> Self {
        self.schema.mutation_type = Some(name.to_string());
        self.schema.types.insert(name.to_string(), TypeDef {
            kind: TypeKind::OBJECT,
            name: name.to_string(),
            description: Some("Root mutation type".to_string()),
            fields: vec![],
            interfaces: vec![],
            possible_types: vec![],
            enum_values: vec![],
            input_fields: vec![],
        });
        self
    }
    
    /// Add a subscription type
    pub fn subscription_type(mut self, name: &str) -> Self {
        self.schema.subscription_type = Some(name.to_string());
        self.schema.types.insert(name.to_string(), TypeDef {
            kind: TypeKind::OBJECT,
            name: name.to_string(),
            description: Some("Root subscription type".to_string()),
            fields: vec![],
            interfaces: vec![],
            possible_types: vec![],
            enum_values: vec![],
            input_fields: vec![],
        });
        self
    }
    
    /// Add a field to a type
    pub fn field(mut self, type_name: &str, field: FieldDef) -> Self {
        if let Some(t) = self.schema.types.get_mut(type_name) {
            t.fields.push(field);
        }
        self
    }
    
    /// Add an object type
    pub fn object_type(mut self, name: &str, description: Option<&str>) -> Self {
        self.schema.types.insert(name.to_string(), TypeDef {
            kind: TypeKind::OBJECT,
            name: name.to_string(),
            description: description.map(|s| s.to_string()),
            fields: vec![],
            interfaces: vec![],
            possible_types: vec![],
            enum_values: vec![],
            input_fields: vec![],
        });
        self
    }
    
    /// Add an enum type
    pub fn enum_type(mut self, name: &str, values: Vec<&str>, description: Option<&str>) -> Self {
        self.schema.types.insert(name.to_string(), TypeDef {
            kind: TypeKind::ENUM,
            name: name.to_string(),
            description: description.map(|s| s.to_string()),
            fields: vec![],
            interfaces: vec![],
            possible_types: vec![],
            enum_values: values.iter().map(|v| EnumValueDef {
                name: v.to_string(),
                description: None,
                is_deprecated: false,
                deprecation_reason: None,
            }).collect(),
            input_fields: vec![],
        });
        self
    }
    
    /// Add an input type
    pub fn input_type(mut self, name: &str, fields: Vec<InputValueDef>, description: Option<&str>) -> Self {
        self.schema.types.insert(name.to_string(), TypeDef {
            kind: TypeKind::INPUT_OBJECT,
            name: name.to_string(),
            description: description.map(|s| s.to_string()),
            fields: vec![],
            interfaces: vec![],
            possible_types: vec![],
            enum_values: vec![],
            input_fields: fields,
        });
        self
    }
    
    /// Build the schema
    pub fn build(self) -> GraphQLSchema {
        self.schema
    }
}

impl Default for SchemaBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  QUERY EXECUTOR
// ═══════════════════════════════════════════════════════════════════════════════

/// Query executor error
#[derive(Debug, thiserror::Error)]
pub enum GraphQLErrorKind {
    #[error("Parse error: {0}")]
    ParseError(String),
    
    #[error("Validation error: {0}")]
    ValidationError(String),
    
    #[error("Execution error: {0}")]
    ExecutionError(String),
    
    #[error("Unknown type: {0}")]
    UnknownType(String),
    
    #[error("Unknown field: {0}.{1}")]
    UnknownField(String, String),
}

/// Resolver function type
pub type ResolverFn = Box<dyn Fn(&HashMap<String, serde_json::Value>) -> Result<serde_json::Value, String> + Send + Sync>;

/// Query executor
pub struct GraphQLExecutor {
    schema: GraphQLSchema,
    resolvers: HashMap<String, ResolverFn>,
}

impl GraphQLExecutor {
    pub fn new(schema: GraphQLSchema) -> Self {
        Self {
            schema,
            resolvers: HashMap::new(),
        }
    }
    
    /// Register a resolver for a field
    pub fn register_resolver<F>(&mut self, type_name: &str, field_name: &str, resolver: F)
    where
        F: Fn(&HashMap<String, serde_json::Value>) -> Result<serde_json::Value, String> + Send + Sync + 'static,
    {
        let key = format!("{}.{}", type_name, field_name);
        self.resolvers.insert(key, Box::new(resolver));
    }
    
    /// Execute a query
    pub fn execute(&self, request: GraphQLRequest) -> GraphQLResponse {
        // Parse query (simplified - in production use graphql-parser)
        let query = request.query.trim();
        
        // Extract operation type and selection set
        let operation_type = if query.starts_with("mutation") {
            "mutation"
        } else if query.starts_with("subscription") {
            "subscription"
        } else {
            "query"
        };
        
        // Extract fields (very simplified)
        let fields = self.extract_fields(query);
        
        // Execute resolvers
        let mut data = serde_json::Map::new();
        let mut errors = Vec::new();
        
        let root_type = match operation_type {
            "query" => &self.schema.query_type,
            "mutation" => self.schema.mutation_type.as_ref().unwrap_or(&self.schema.query_type),
            _ => &self.schema.query_type,
        };
        
        for field in fields {
            let key = format!("{}.{}", root_type, &field);
            
            if let Some(resolver) = self.resolvers.get(&key) {
                match resolver(&request.variables) {
                    Ok(value) => {
                        data.insert(field, value);
                    }
                    Err(e) => {
                        errors.push(GraphQLError {
                            message: e,
                            locations: None,
                            path: Some(vec![PathSegment::String(field)]),
                            extensions: None,
                        });
                    }
                }
            } else {
                // Return null for unimplemented fields
                data.insert(field, serde_json::Value::Null);
            }
        }
        
        GraphQLResponse {
            data: Some(serde_json::Value::Object(data)),
            errors: if errors.is_empty() { None } else { Some(errors) },
            extensions: None,
        }
    }
    
    fn extract_fields(&self, query: &str) -> Vec<String> {
        // Very simplified field extraction
        let query = query.trim_start_matches("query")
            .trim_start_matches("mutation")
            .trim_start_matches("subscription")
            .trim();
        
        // Find content between first { and last }
        let start = query.find('{');
        let end = query.rfind('}');
        
        if let (Some(s), Some(e)) = (start, end) {
            let content = &query[s + 1..e];
            content.split(',')
                .map(|f| f.trim().split_whitespace().next().unwrap_or("").to_string())
                .filter(|f| !f.is_empty())
                .collect()
        } else {
            vec![]
        }
    }
    
    /// Introspection query
    pub fn introspect(&self) -> GraphQLResponse {
        let types: Vec<serde_json::Value> = self.schema.types.values()
            .map(|t| serde_json::to_value(t).unwrap_or(serde_json::Value::Null))
            .collect();
        
        GraphQLResponse {
            data: Some(serde_json::json!({
                "__schema": {
                    "types": types,
                    "queryType": { "name": self.schema.query_type },
                    "mutationType": self.schema.mutation_type.as_ref().map(|n| serde_json::json!({"name": n})),
                    "subscriptionType": self.schema.subscription_type.as_ref().map(|n| serde_json::json!({"name": n})),
                }
            })),
            errors: None,
            extensions: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_schema_builder() {
        let schema = SchemaBuilder::new()
            .query_type("Query")
            .mutation_type("Mutation")
            .object_type("User", Some("A user"))
            .enum_type("Status", vec!["ACTIVE", "INACTIVE"], None)
            .build();
        
        assert!(schema.types.contains_key("Query"));
        assert!(schema.types.contains_key("User"));
        assert!(schema.types.contains_key("Status"));
    }
    
    #[test]
    fn test_query_execution() {
        let schema = SchemaBuilder::new()
            .query_type("Query")
            .build();
        
        let mut executor = GraphQLExecutor::new(schema);
        
        executor.register_resolver("Query", "hello", |_args| {
            Ok(serde_json::json!("Hello, World!"))
        });
        
        let request = GraphQLRequest {
            query: "query { hello }".to_string(),
            operation_name: None,
            variables: HashMap::new(),
        };
        
        let response = executor.execute(request);
        
        assert!(response.data.is_some());
        assert!(response.data.unwrap()["hello"] == "Hello, World!");
    }
}
