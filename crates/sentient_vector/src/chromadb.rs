//! ─── ChromaDB Vector Store ───

use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{
    VectorStore, VectorConfig, VectorDocument, SearchResult, SearchOptions,
    CollectionStats, VectorError, Result,
};

#[derive(Debug, Serialize)]
struct ChromaAddBody {
    ids: Vec<String>,
    documents: Vec<String>,
    embeddings: Vec<Vec<f32>>,
    metadatas: Vec<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Deserialize)]
struct ChromaQueryResponse {
    ids: Vec<Vec<String>>,
    documents: Option<Vec<Vec<String>>>,
    embeddings: Option<Vec<Vec<Vec<f32>>>>,
    distances: Option<Vec<Vec<f32>>>,
    metadatas: Option<Vec<Vec<HashMap<String, serde_json::Value>>>>,
}

pub struct ChromaStore {
    config: VectorConfig,
    client: Client,
    base_url: String,
}

impl ChromaStore {
    pub async fn new(config: VectorConfig) -> Result<Self> {
        let client = Client::builder()
            .timeout(std::time::Duration::from_millis(config.timeout_ms))
            .build()
            .map_err(|e| VectorError::Connection(e.to_string()))?;

        let base_url = format!("http://{}:{}", config.host, config.port);

        Ok(Self { config, client, base_url })
    }

    fn api_url(&self, path: &str) -> String {
        format!("{}/api/v1{}", self.base_url, path)
    }
}

#[async_trait]
impl VectorStore for ChromaStore {
    fn store_type(&self) -> crate::VectorDbType {
        crate::VectorDbType::ChromaDB
    }

    async fn create_collection(&self) -> Result<()> {
        let url = self.api_url("/collections");

        #[derive(Serialize)]
        struct CreateBody {
            name: String,
        }

        let resp = self.client
            .post(&url)
            .json(&CreateBody { name: self.config.collection.clone() })
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
        let url = self.api_url(&format!("/collections/{}", self.config.collection));
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
        let url = self.api_url(&format!("/collections/{}", self.config.collection));
        let resp = self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| VectorError::RequestFailed(e.to_string()))?;
        Ok(resp.status().is_success())
    }

    async fn upsert(&self, documents: Vec<VectorDocument>) -> Result<usize> {
        let url = self.api_url(&format!("/collections/{}/add", self.config.collection));

        let ids: Vec<String> = documents.iter().map(|d| d.id.clone()).collect();
        let docs: Vec<String> = documents.iter().map(|d| d.content.clone()).collect();
        let embeddings: Vec<Vec<f32>> = documents.iter().map(|d| d.vector.clone()).collect();
        let metadatas: Vec<HashMap<String, serde_json::Value>> = documents.iter().map(|d| {
            let mut meta = HashMap::new();
            for (k, v) in &d.metadata {
                meta.insert(k.clone(), v.clone());
            }
            meta
        }).collect();

        let count = ids.len();
        let body = ChromaAddBody { ids, documents: docs, embeddings, metadatas };

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
        let url = self.api_url(&format!("/collections/{}/delete", self.config.collection));

        let body = serde_json::json!({ "ids": ids });

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
        let url = self.api_url(&format!("/collections/{}/get", self.config.collection));

        let body = serde_json::json!({
            "ids": [id],
            "include": ["documents", "embeddings", "metadatas"]
        });

        let resp = self.client
            .post(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| VectorError::RequestFailed(e.to_string()))?;

        if !resp.status().is_success() {
            return Ok(None);
        }

        #[derive(Deserialize)]
        struct GetResponse {
            ids: Vec<String>,
            documents: Option<Vec<String>>,
            embeddings: Option<Vec<Vec<f32>>>,
            metadatas: Option<Vec<HashMap<String, serde_json::Value>>>,
        }

        let result: GetResponse = resp.json().await
            .map_err(|e| VectorError::ParseError(e.to_string()))?;

        Ok(result.ids.into_iter().enumerate().next().map(|(i, id)| {
            let content = result.documents.and_then(|d| d.get(i).cloned()).unwrap_or_default();
            let vector = result.embeddings.and_then(|e| e.get(i).cloned()).unwrap_or_default();
            let metadata = result.metadatas.and_then(|m| m.get(i).cloned()).unwrap_or_default();

            VectorDocument { id, content, vector, metadata, score: None }
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
        let url = self.api_url(&format!("/collections/{}/query", self.config.collection));

        let body = serde_json::json!({
            "query_embeddings": [vector],
            "n_results": limit,
            "include": ["documents", "embeddings", "metadatas", "distances"]
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

        let result: ChromaQueryResponse = resp.json().await
            .map_err(|e| VectorError::ParseError(e.to_string()))?;

        let ids = result.ids.first().cloned().unwrap_or_default();
        let documents = result.documents.and_then(|d| d.first().cloned()).unwrap_or_default();
        let embeddings = result.embeddings.and_then(|e| e.first().cloned()).unwrap_or_default();
        let distances = result.distances.and_then(|d| d.first().cloned()).unwrap_or_default();
        let metadatas = result.metadatas.and_then(|m| m.first().cloned()).unwrap_or_default();

        let max_dist = distances.iter().cloned().fold(0.0_f32, f32::max).max(1.0);

        Ok(ids.into_iter().enumerate().map(|(i, id)| {
            let content = documents.get(i).cloned().unwrap_or_default();
            let vec = embeddings.get(i).cloned().unwrap_or_default();
            let dist = distances.get(i).copied().unwrap_or(0.0);
            let score = 1.0 - (dist / max_dist);
            let metadata = metadatas.get(i).cloned().unwrap_or_default();

            SearchResult {
                document: VectorDocument { id, content, vector: vec, metadata, score: Some(score) },
                score,
            }
        }).collect())
    }

    async fn hybrid_search(&self, vector: &[f32], _query: &str, limit: usize, options: Option<SearchOptions>) -> Result<Vec<SearchResult>> {
        self.search(vector, limit, options).await
    }

    async fn count(&self) -> Result<usize> {
        let url = self.api_url(&format!("/collections/{}/count", self.config.collection));

        let resp = self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| VectorError::RequestFailed(e.to_string()))?;

        if !resp.status().is_success() {
            return Ok(0);
        }

        #[derive(Deserialize)]
        struct CountResult { count: usize }

        let result: CountResult = resp.json().await
            .map_err(|e| VectorError::ParseError(e.to_string()))?;

        Ok(result.count)
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
        let url = self.api_url("/heartbeat");
        let resp = self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| VectorError::RequestFailed(e.to_string()))?;
        Ok(resp.status().is_success())
    }
}
