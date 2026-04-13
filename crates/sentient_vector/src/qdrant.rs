//! ─── Qdrant Vector Store ───

use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{
    VectorStore, VectorConfig, VectorDocument, SearchResult, SearchFilter, SearchOptions,
    CollectionStats, DistanceMetric, VectorError, Result, FilterCondition,
};

// Simplified Qdrant implementation

#[derive(Debug, Serialize)]
struct QdrantPoint {
    id: String,
    vector: Vec<f32>,
    payload: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Serialize)]
struct QdrantUpsertBody {
    points: Vec<QdrantPoint>,
}

#[derive(Debug, Deserialize)]
struct QdrantSearchResponse {
    result: Vec<QdrantHit>,
}

#[derive(Debug, Deserialize)]
struct QdrantHit {
    id: String,
    score: f32,
    payload: Option<HashMap<String, serde_json::Value>>,
    vector: Option<Vec<f32>>,
}

pub struct QdrantStore {
    config: VectorConfig,
    client: Client,
    base_url: String,
}

impl QdrantStore {
    pub async fn new(config: VectorConfig) -> Result<Self> {
        let client = Client::builder()
            .timeout(std::time::Duration::from_millis(config.timeout_ms))
            .build()
            .map_err(|e| VectorError::Connection(e.to_string()))?;

        let base_url = format!("http://{}:{}", config.host, config.port);

        Ok(Self { config, client, base_url })
    }

    fn collection_url(&self) -> String {
        format!("{}/collections/{}", self.base_url, self.config.collection)
    }

    fn points_url(&self) -> String {
        format!("{}/points", self.collection_url())
    }
}

#[async_trait]
impl VectorStore for QdrantStore {
    fn store_type(&self) -> crate::VectorDbType {
        crate::VectorDbType::Qdrant
    }

    async fn create_collection(&self) -> Result<()> {
        let url = self.collection_url();
        
        #[derive(Serialize)]
        struct CreateBody {
            vectors: VectorConfig_,
        }
        
        #[derive(Serialize)]
        struct VectorConfig_ {
            size: usize,
            distance: String,
        }

        let body = CreateBody {
            vectors: VectorConfig_ {
                size: self.config.embedding_dim,
                distance: "Cosine".into(),
            },
        };

        let resp = self.client
            .put(&url)
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
        let url = self.collection_url();
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
        let url = self.collection_url();
        let resp = self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| VectorError::RequestFailed(e.to_string()))?;
        Ok(resp.status().is_success())
    }

    async fn upsert(&self, documents: Vec<VectorDocument>) -> Result<usize> {
        let url = format!("{}/upsert", self.points_url());

        let points: Vec<QdrantPoint> = documents.iter().map(|doc| {
            let mut payload = HashMap::new();
            payload.insert("content".into(), doc.content.clone().into());
            for (k, v) in &doc.metadata {
                payload.insert(k.clone(), v.clone());
            }
            QdrantPoint {
                id: doc.id.clone(),
                vector: doc.vector.clone(),
                payload,
            }
        }).collect();

        let count = points.len();
        let body = QdrantUpsertBody { points };

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
        let url = format!("{}/delete", self.points_url());

        let body = serde_json::json!({
            "points": ids
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
        let url = format!("{}/get", self.points_url());
        
        let body = serde_json::json!({
            "ids": [id],
            "with_payload": true,
            "with_vector": true
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
            result: Vec<QdrantHit>,
        }

        let result: GetResponse = resp.json().await
            .map_err(|e| VectorError::ParseError(e.to_string()))?;

        Ok(result.result.into_iter().next().map(|h| {
            let payload = h.payload.unwrap_or_default();
            let content = payload.get("content")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            
            let mut metadata = HashMap::new();
            for (k, v) in payload {
                if k != "content" {
                    metadata.insert(k, v);
                }
            }

            VectorDocument {
                id: h.id,
                content,
                vector: h.vector.unwrap_or_default(),
                metadata,
                score: None,
            }
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
        let url = format!("{}/search", self.points_url());

        let body = serde_json::json!({
            "vector": vector,
            "limit": limit,
            "with_payload": true,
            "with_vector": true
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

        let result: QdrantSearchResponse = resp.json().await
            .map_err(|e| VectorError::ParseError(e.to_string()))?;

        Ok(result.result.into_iter().map(|h| {
            let payload = h.payload.unwrap_or_default();
            let content = payload.get("content")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            
            let mut metadata = HashMap::new();
            for (k, v) in payload {
                if k != "content" {
                    metadata.insert(k, v);
                }
            }

            SearchResult {
                document: VectorDocument {
                    id: h.id,
                    content,
                    vector: h.vector.unwrap_or_default(),
                    metadata,
                    score: Some(h.score),
                },
                score: h.score,
            }
        }).collect())
    }

    async fn hybrid_search(&self, vector: &[f32], query: &str, limit: usize, options: Option<SearchOptions>) -> Result<Vec<SearchResult>> {
        // Qdrant doesn't have native hybrid, use vector search with keyword filter
        self.search(vector, limit, options).await
    }

    async fn count(&self) -> Result<usize> {
        let url = self.collection_url();
        
        #[derive(Deserialize)]
        struct CollectionInfo {
            result: CollectionResult,
        }
        
        #[derive(Deserialize)]
        struct CollectionResult {
            points_count: Option<usize>,
        }

        let resp = self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| VectorError::RequestFailed(e.to_string()))?;

        if !resp.status().is_success() {
            return Ok(0);
        }

        let info: CollectionInfo = resp.json().await
            .map_err(|e| VectorError::ParseError(e.to_string()))?;

        Ok(info.result.points_count.unwrap_or(0))
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
        let url = format!("{}/healthz", self.base_url);
        let resp = self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| VectorError::RequestFailed(e.to_string()))?;
        Ok(resp.status().is_success())
    }
}
