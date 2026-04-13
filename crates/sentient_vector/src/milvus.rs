//! ─── Milvus Vector Store ───

use async_trait::async_trait;
use reqwest::Client;
use std::collections::HashMap;

use crate::{
    VectorStore, VectorConfig, VectorDocument, SearchResult, SearchOptions,
    CollectionStats, VectorError, Result,
};
use serde::Deserialize;

pub struct MilvusStore {
    config: VectorConfig,
    client: Client,
    base_url: String,
}

impl MilvusStore {
    pub async fn new(config: VectorConfig) -> Result<Self> {
        let client = Client::builder()
            .timeout(std::time::Duration::from_millis(config.timeout_ms))
            .build()
            .map_err(|e| VectorError::Connection(e.to_string()))?;

        let base_url = format!("http://{}:{}", config.host, config.port);

        Ok(Self { config, client, base_url })
    }
}

#[async_trait]
impl VectorStore for MilvusStore {
    fn store_type(&self) -> crate::VectorDbType {
        crate::VectorDbType::Milvus
    }

    async fn create_collection(&self) -> Result<()> {
        let url = format!("{}/v2/vectordb/collections/create", self.base_url);

        let body = serde_json::json!({
            "collectionName": self.config.collection,
            "dimension": self.config.embedding_dim,
            "metricType": "COSINE"
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
        let url = format!("{}/v2/vectordb/collections/drop", self.base_url);

        let body = serde_json::json!({
            "collectionName": self.config.collection
        });

        let resp = self.client
            .post(&url)
            .json(&body)
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
        let url = format!("{}/v2/vectordb/collections/describe", self.base_url);

        let body = serde_json::json!({
            "collectionName": self.config.collection
        });

        let resp = self.client
            .post(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| VectorError::RequestFailed(e.to_string()))?;

        Ok(resp.status().is_success())
    }

    async fn upsert(&self, documents: Vec<VectorDocument>) -> Result<usize> {
        let url = format!("{}/v2/vectordb/entities/insert", self.base_url);

        let data: Vec<serde_json::Value> = documents.iter().map(|doc| {
            let mut obj = serde_json::Map::new();
            obj.insert("id".into(), doc.id.clone().into());
            obj.insert("vector".into(), doc.vector.clone().into());
            obj.insert("content".into(), doc.content.clone().into());
            for (k, v) in &doc.metadata {
                obj.insert(k.clone(), v.clone());
            }
            obj.into()
        }).collect();

        let count = data.len();

        let body = serde_json::json!({
            "collectionName": self.config.collection,
            "data": data
        });

        let resp = self.client
            .post(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| VectorError::RequestFailed(e.to_string()))?;

        if !resp.status().is_success() {
            let text = resp.text().await.unwrap_or_default();
            return Err(VectorError::ApiError(text));
        }

        Ok(count)
    }

    async fn delete(&self, ids: &[&str]) -> Result<usize> {
        let url = format!("{}/v2/vectordb/entities/delete", self.base_url);

        let filter = format!("id in [{}]", ids.iter().map(|id| format!("'{}'", id)).collect::<Vec<_>>().join(","));

        let body = serde_json::json!({
            "collectionName": self.config.collection,
            "filter": filter
        });

        let resp = self.client
            .post(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| VectorError::RequestFailed(e.to_string()))?;

        if !resp.status().is_success() {
            let text = resp.text().await.unwrap_or_default();
            return Err(VectorError::ApiError(text));
        }

        Ok(ids.len())
    }

    async fn get(&self, id: &str) -> Result<Option<VectorDocument>> {
        let results = self.get_batch(&[id]).await?;
        Ok(results.into_iter().next())
    }

    async fn get_batch(&self, ids: &[&str]) -> Result<Vec<VectorDocument>> {
        let url = format!("{}/v2/vectordb/entities/query", self.base_url);

        let filter = format!("id in [{}]", ids.iter().map(|id| format!("'{}'", id)).collect::<Vec<_>>().join(","));

        let body = serde_json::json!({
            "collectionName": self.config.collection,
            "filter": filter,
            "outputFields": ["*"]
        });

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
        struct QueryResponse {
            data: Vec<serde_json::Value>,
        }

        let result: QueryResponse = resp.json().await
            .map_err(|e| VectorError::ParseError(e.to_string()))?;

        Ok(result.data.into_iter().filter_map(|item| {
            let obj = item.as_object()?;
            let id = obj.get("id")?.as_str()?.to_string();
            let content = obj.get("content")?.as_str()?.to_string();
            let vector = obj.get("vector")
                .and_then(|v| v.as_array())
                .map(|arr| arr.iter().filter_map(|v| v.as_f64().map(|f| f as f32)).collect())
                .unwrap_or_default();

            let mut metadata = HashMap::new();
            for (k, v) in obj {
                if !matches!(k.as_str(), "id" | "content" | "vector") {
                    metadata.insert(k.clone(), v.clone());
                }
            }

            Some(VectorDocument { id, content, vector, metadata, score: None })
        }).collect())
    }

    async fn search(&self, vector: &[f32], limit: usize, _options: Option<SearchOptions>) -> Result<Vec<SearchResult>> {
        let url = format!("{}/v2/vectordb/entities/search", self.base_url);

        let body = serde_json::json!({
            "collectionName": self.config.collection,
            "data": [vector],
            "annsField": "vector",
            "limit": limit,
            "outputFields": ["*"]
        });

        let resp = self.client
            .post(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| VectorError::RequestFailed(e.to_string()))?;

        if !resp.status().is_success() {
            let text = resp.text().await.unwrap_or_default();
            return Err(VectorError::ApiError(text));
        }

        #[derive(Deserialize)]
        struct SearchResponse {
            data: Vec<SearchHit>,
        }

        #[derive(Deserialize)]
        struct SearchHit {
            distance: f32,
            #[serde(flatten)]
            fields: serde_json::Value,
        }

        let result: SearchResponse = resp.json().await
            .map_err(|e| VectorError::ParseError(e.to_string()))?;

        Ok(result.data.into_iter().map(|hit| {
            let binding = serde_json::Map::new();
            let fields = hit.fields.as_object().unwrap_or(&binding);
            let id = fields.get("id").and_then(|v| v.as_str()).unwrap_or("").to_string();
            let content = fields.get("content").and_then(|v| v.as_str()).unwrap_or("").to_string();
            let vec = fields.get("vector")
                .and_then(|v| v.as_array())
                .map(|arr| arr.iter().filter_map(|v| v.as_f64().map(|f| f as f32)).collect())
                .unwrap_or_default();

            let mut metadata = HashMap::new();
            for (k, v) in fields {
                if !matches!(k.as_str(), "id" | "content" | "vector" | "distance") {
                    metadata.insert(k.clone(), v.clone());
                }
            }

            SearchResult {
                document: VectorDocument { id, content, vector: vec, metadata, score: Some(hit.distance) },
                score: hit.distance,
            }
        }).collect())
    }

    async fn hybrid_search(&self, vector: &[f32], _query: &str, limit: usize, options: Option<SearchOptions>) -> Result<Vec<SearchResult>> {
        self.search(vector, limit, options).await
    }

    async fn count(&self) -> Result<usize> {
        let url = format!("{}/v2/vectordb/collections/describe", self.base_url);

        let body = serde_json::json!({
            "collectionName": self.config.collection
        });

        let resp = self.client
            .post(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| VectorError::RequestFailed(e.to_string()))?;

        if !resp.status().is_success() {
            return Ok(0);
        }

        #[derive(Deserialize)]
        struct DescribeResponse {
            data: CollectionData,
        }

        #[derive(Deserialize)]
        struct CollectionData {
            rowCount: Option<usize>,
        }

        let result: DescribeResponse = resp.json().await
            .map_err(|e| VectorError::ParseError(e.to_string()))?;

        Ok(result.data.rowCount.unwrap_or(0))
    }

    async fn stats(&self) -> Result<CollectionStats> {
        let count = self.count().await?;
        Ok(CollectionStats {
            vector_count: count,
            index_size_bytes: 0,
            dimension: self.config.embedding_dim,
            metric: self.config.metric,
            status: "green".into(),
        })
    }

    async fn health(&self) -> Result<bool> {
        let url = format!("{}/v2/vectordb/collections/list", self.base_url);
        let resp = self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| VectorError::RequestFailed(e.to_string()))?;
        Ok(resp.status().is_success())
    }
}
