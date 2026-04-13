//! ─── Weaviate Vector Store ───

use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{
    VectorStore, VectorConfig, VectorDocument, SearchResult, SearchOptions,
    CollectionStats, VectorError, Result,
};

pub struct WeaviateStore {
    config: VectorConfig,
    client: Client,
    base_url: String,
    class_name: String,
}

impl WeaviateStore {
    pub async fn new(config: VectorConfig) -> Result<Self> {
        let client = Client::builder()
            .timeout(std::time::Duration::from_millis(config.timeout_ms))
            .build()
            .map_err(|e| VectorError::Connection(e.to_string()))?;

        let base_url = format!("http://{}:{}", config.host, config.port);
        let class_name = config.collection.clone();

        Ok(Self { config, client, base_url, class_name })
    }
}

#[async_trait]
impl VectorStore for WeaviateStore {
    fn store_type(&self) -> crate::VectorDbType {
        crate::VectorDbType::Weaviate
    }

    async fn create_collection(&self) -> Result<()> {
        let url = format!("{}/v1/schema", self.base_url);

        let body = serde_json::json!({
            "class": self.class_name,
            "vectorizer": "none"
        });

        let resp = self.client
            .post(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| VectorError::RequestFailed(e.to_string()))?;

        if !resp.status().is_success() {
            let text = resp.text().await.unwrap_or_default();
            if !text.contains("already exists") {
                return Err(VectorError::ApiError(text));
            }
        }
        Ok(())
    }

    async fn delete_collection(&self) -> Result<()> {
        let url = format!("{}/v1/schema/{}", self.base_url, self.class_name);
        let resp = self.client
            .delete(&url)
            .send()
            .await
            .map_err(|e| VectorError::RequestFailed(e.to_string()))?;

        if !resp.status().is_success() {
            let text = resp.text().await.unwrap_or_default();
            return Err(VectorError::ApiError(text));
        }
        Ok(())
    }

    async fn collection_exists(&self) -> Result<bool> {
        let url = format!("{}/v1/schema/{}", self.base_url, self.class_name);
        let resp = self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| VectorError::RequestFailed(e.to_string()))?;
        Ok(resp.status().is_success())
    }

    async fn upsert(&self, documents: Vec<VectorDocument>) -> Result<usize> {
        let count = documents.len();

        for doc in documents {
            let url = format!("{}/v1/objects", self.base_url);

            let mut props = HashMap::new();
            props.insert("content".into(), doc.content.clone().into());
            for (k, v) in &doc.metadata {
                props.insert(k.clone(), v.clone());
            }

            let body = serde_json::json!({
                "class": self.class_name,
                "id": doc.id,
                "properties": props,
                "vector": doc.vector
            });

            let resp = self.client
                .post(&url)
                .json(&body)
                .send()
                .await
                .map_err(|e| VectorError::RequestFailed(e.to_string()))?;

            if !resp.status().is_success() {
                log::warn!("Weaviate upsert failed");
            }
        }

        Ok(count)
    }

    async fn delete(&self, ids: &[&str]) -> Result<usize> {
        let mut count = 0;
        for id in ids {
            let url = format!("{}/v1/objects/{}/{}", self.base_url, self.class_name, id);
            let resp = self.client
                .delete(&url)
                .send()
                .await
                .map_err(|e| VectorError::RequestFailed(e.to_string()))?;

            if resp.status().is_success() {
                count += 1;
            }
        }
        Ok(count)
    }

    async fn get(&self, id: &str) -> Result<Option<VectorDocument>> {
        let url = format!("{}/v1/objects/{}/{}?include=vector", self.base_url, self.class_name, id);

        let resp = self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| VectorError::RequestFailed(e.to_string()))?;

        if !resp.status().is_success() {
            return Ok(None);
        }

        #[derive(Deserialize)]
        struct ObjectResponse {
            id: String,
            vector: Option<Vec<f32>>,
            properties: Option<HashMap<String, serde_json::Value>>,
        }

        let result: ObjectResponse = resp.json().await
            .map_err(|e| VectorError::ParseError(e.to_string()))?;

        let props = result.properties.unwrap_or_default();
        let content = props.get("content")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        let mut metadata = HashMap::new();
        for (k, v) in props {
            if k != "content" {
                metadata.insert(k, v);
            }
        }

        Ok(Some(VectorDocument {
            id: result.id,
            content,
            vector: result.vector.unwrap_or_default(),
            metadata,
            score: None,
        }))
    }

    async fn get_batch(&self, ids: &[&str]) -> Result<Vec<VectorDocument>> {
        let mut results = Vec::new();
        for id in ids {
            if let Some(doc) = self.get(id).await? {
                results.push(doc);
            }
        }
        Ok(results)
    }

    async fn search(&self, vector: &[f32], limit: usize, _options: Option<SearchOptions>) -> Result<Vec<SearchResult>> {
        let url = format!("{}/v1/graphql", self.base_url);

        let vector_str = vector.iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(",");

        let query = format!(r#"{{
            Get {{
                {class} (
                    nearVector: {{ vector: [{vector_str}] }}
                    limit: {limit}
                ) {{
                    _additional {{ id certainty vector }}
                    content
                }}
            }}
        }}"#, class = self.class_name, vector_str = vector_str, limit = limit);

        let body = serde_json::json!({ "query": query });

        let resp = self.client
            .post(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| VectorError::RequestFailed(e.to_string()))?;

        if !resp.status().is_success() {
            return Ok(vec![]);
        }

        #[derive(Deserialize)]
        struct GraphQLResponse {
            data: GraphQLData,
        }

        #[derive(Deserialize)]
        struct GraphQLData {
            #[serde(rename = "Get")]
            get: serde_json::Map<String, serde_json::Value>,
        }

        let result: GraphQLResponse = resp.json().await
            .map_err(|e| VectorError::ParseError(e.to_string()))?;

        let results = result.data.get.get(&self.class_name)
            .and_then(|v| v.as_array())
            .map(|arr| arr.clone())
            .unwrap_or_default();

        Ok(results.into_iter().filter_map(|item| {
            let obj = item.as_object()?;
            let additional = obj.get("_additional")?.as_object()?;
            let id = additional.get("id")?.as_str()?.to_string();
            let score = additional.get("certainty")?.as_f64()? as f32;
            let vector = additional.get("vector")
                .and_then(|v| v.as_array())
                .map(|arr| arr.iter().filter_map(|v| v.as_f64().map(|f| f as f32)).collect())
                .unwrap_or_default();
            let content = obj.get("content")?.as_str()?.to_string();

            Some(SearchResult {
                document: VectorDocument { id, content, vector, metadata: HashMap::new(), score: Some(score) },
                score,
            })
        }).collect())
    }

    async fn hybrid_search(&self, vector: &[f32], _query: &str, limit: usize, options: Option<SearchOptions>) -> Result<Vec<SearchResult>> {
        self.search(vector, limit, options).await
    }

    async fn count(&self) -> Result<usize> {
        Ok(0) // Weaviate doesn't have a simple count API
    }

    async fn stats(&self) -> Result<CollectionStats> {
        Ok(CollectionStats {
            vector_count: 0,
            index_size_bytes: 0,
            dimension: self.config.embedding_dim,
            metric: self.config.metric,
            status: "green".into(),
        })
    }

    async fn health(&self) -> Result<bool> {
        let url = format!("{}/v1/.well-known/ready", self.base_url);
        let resp = self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| VectorError::RequestFailed(e.to_string()))?;
        Ok(resp.status().is_success())
    }
}
