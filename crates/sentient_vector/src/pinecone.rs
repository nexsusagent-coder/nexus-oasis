//! ─── Pinecone Vector Store ───

use async_trait::async_trait;
use reqwest::Client;
use std::collections::HashMap;

use crate::{
    VectorStore, VectorConfig, VectorDocument, SearchResult, SearchOptions,
    CollectionStats, VectorError, Result,
};
use serde::Deserialize;

pub struct PineconeStore {
    config: VectorConfig,
    client: Client,
    base_url: String,
    api_key: String,
}

impl PineconeStore {
    pub async fn new(config: VectorConfig) -> Result<Self> {
        let api_key = config.api_key.clone().ok_or_else(|| 
            VectorError::InvalidInput("Pinecone requires API key".into()))?;

        let client = Client::builder()
            .timeout(std::time::Duration::from_millis(config.timeout_ms))
            .build()
            .map_err(|e| VectorError::Connection(e.to_string()))?;

        let base_url = format!("https://{}.svc.pinecone.io", config.collection);

        Ok(Self { config, client, base_url, api_key })
    }
}

#[async_trait]
impl VectorStore for PineconeStore {
    fn store_type(&self) -> crate::VectorDbType {
        crate::VectorDbType::Pinecone
    }

    async fn create_collection(&self) -> Result<()> {
        Ok(()) // Pinecone indexes created via console
    }

    async fn delete_collection(&self) -> Result<()> {
        Err(VectorError::NotImplemented("Delete via console".into()))
    }

    async fn collection_exists(&self) -> Result<bool> {
        self.health().await
    }

    async fn upsert(&self, documents: Vec<VectorDocument>) -> Result<usize> {
        let url = format!("{}/vectors/upsert", self.base_url);

        let vectors: Vec<serde_json::Value> = documents.iter().map(|doc| {
            let mut meta = HashMap::new();
            meta.insert("content".into(), doc.content.clone().into());
            for (k, v) in &doc.metadata {
                meta.insert(k.clone(), v.clone());
            }

            serde_json::json!({
                "id": doc.id,
                "values": doc.vector,
                "metadata": meta
            })
        }).collect();

        let count = vectors.len();
        let body = serde_json::json!({ "vectors": vectors });

        let resp = self.client
            .post(&url)
            .header("Api-Key", &self.api_key)
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
        let url = format!("{}/vectors/delete", self.base_url);

        let body = serde_json::json!({ "ids": ids });

        let resp = self.client
            .post(&url)
            .header("Api-Key", &self.api_key)
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
        let url = format!("{}/vectors/fetch", self.base_url);

        let resp = self.client
            .get(&url)
            .header("Api-Key", &self.api_key)
            .query(&[("ids", ids.join(","))])
            .send()
            .await
            .map_err(|e| VectorError::RequestFailed(e.to_string()))?;

        if !resp.status().is_success() {
            return Ok(vec![]);
        }

        #[derive(Deserialize)]
        struct FetchResponse {
            vectors: HashMap<String, VectorData>,
        }

        #[derive(Deserialize)]
        struct VectorData {
            values: Vec<f32>,
            metadata: Option<HashMap<String, serde_json::Value>>,
        }

        let result: FetchResponse = resp.json().await
            .map_err(|e| VectorError::ParseError(e.to_string()))?;

        Ok(result.vectors.into_iter().map(|(id, data)| {
            let metadata = data.metadata.unwrap_or_default();
            let content = metadata.get("content")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();

            let mut meta = HashMap::new();
            for (k, v) in metadata {
                if k != "content" {
                    meta.insert(k, v);
                }
            }

            VectorDocument { id, content, vector: data.values, metadata: meta, score: None }
        }).collect())
    }

    async fn search(&self, vector: &[f32], limit: usize, _options: Option<SearchOptions>) -> Result<Vec<SearchResult>> {
        let url = format!("{}/query", self.base_url);

        let body = serde_json::json!({
            "vector": vector,
            "top_k": limit,
            "include_values": true,
            "include_metadata": true
        });

        let resp = self.client
            .post(&url)
            .header("Api-Key", &self.api_key)
            .json(&body)
            .send()
            .await
            .map_err(|e| VectorError::RequestFailed(e.to_string()))?;

        if !resp.status().is_success() {
            let text = resp.text().await.unwrap_or_default();
            return Err(VectorError::ApiError(text));
        }

        #[derive(Deserialize)]
        struct QueryResponse {
            matches: Vec<Match>,
        }

        #[derive(Deserialize)]
        struct Match {
            id: String,
            score: f32,
            values: Option<Vec<f32>>,
            metadata: Option<HashMap<String, serde_json::Value>>,
        }

        let result: QueryResponse = resp.json().await
            .map_err(|e| VectorError::ParseError(e.to_string()))?;

        Ok(result.matches.into_iter().map(|m| {
            let metadata = m.metadata.unwrap_or_default();
            let content = metadata.get("content")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();

            let mut meta = HashMap::new();
            for (k, v) in metadata {
                if k != "content" {
                    meta.insert(k, v);
                }
            }

            SearchResult {
                document: VectorDocument {
                    id: m.id,
                    content,
                    vector: m.values.unwrap_or_default(),
                    metadata: meta,
                    score: Some(m.score),
                },
                score: m.score,
            }
        }).collect())
    }

    async fn hybrid_search(&self, vector: &[f32], _query: &str, limit: usize, options: Option<SearchOptions>) -> Result<Vec<SearchResult>> {
        self.search(vector, limit, options).await
    }

    async fn count(&self) -> Result<usize> {
        let url = format!("{}/describe-index-stats", self.base_url);

        let resp = self.client
            .get(&url)
            .header("Api-Key", &self.api_key)
            .send()
            .await
            .map_err(|e| VectorError::RequestFailed(e.to_string()))?;

        if !resp.status().is_success() {
            return Ok(0);
        }

        #[derive(Deserialize)]
        struct StatsResponse {
            total_vector_count: usize,
        }

        let result: StatsResponse = resp.json().await
            .map_err(|e| VectorError::ParseError(e.to_string()))?;

        Ok(result.total_vector_count)
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
        self.count().await.map(|_| true)
    }
}
