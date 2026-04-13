//! ─── Elasticsearch Vector Store ───

use async_trait::async_trait;
use reqwest::Client;
use std::collections::HashMap;

use crate::{
    VectorStore, VectorConfig, VectorDocument, SearchResult, SearchOptions,
    CollectionStats, VectorError, Result,
};
use serde::Deserialize;

pub struct ElasticStore {
    config: VectorConfig,
    client: Client,
    base_url: String,
    index_name: String,
}

impl ElasticStore {
    pub async fn new(config: VectorConfig) -> Result<Self> {
        let client = Client::builder()
            .timeout(std::time::Duration::from_millis(config.timeout_ms))
            .build()
            .map_err(|e| VectorError::Connection(e.to_string()))?;

        let base_url = format!("http://{}:{}", config.host, config.port);
        let index_name = config.collection.clone();

        Ok(Self { config, client, base_url, index_name })
    }

    fn index_url(&self) -> String {
        format!("{}/{}", self.base_url, self.index_name)
    }

    fn search_url(&self) -> String {
        format!("{}/_search", self.index_url())
    }
}

#[async_trait]
impl VectorStore for ElasticStore {
    fn store_type(&self) -> crate::VectorDbType {
        crate::VectorDbType::Elasticsearch
    }

    async fn create_collection(&self) -> Result<()> {
        let url = self.index_url();

        let body = serde_json::json!({
            "settings": {
                "number_of_shards": 1,
                "number_of_replicas": 0
            },
            "mappings": {
                "properties": {
                    "content": { "type": "text" },
                    "vector": {
                        "type": "dense_vector",
                        "dims": self.config.embedding_dim,
                        "similarity": "cosine",
                        "index": true
                    }
                }
            }
        });

        let resp = self.client
            .put(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| VectorError::RequestFailed(e.to_string()))?;

        if !resp.status().is_success() {
            let text = resp.text().await.unwrap_or_default();
            if !text.contains("resource_already_exists") {
                return Err(VectorError::ApiError(text));
            }
        }
        Ok(())
    }

    async fn delete_collection(&self) -> Result<()> {
        let url = self.index_url();
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
        let url = self.index_url();
        let resp = self.client
            .head(&url)
            .send()
            .await
            .map_err(|e| VectorError::RequestFailed(e.to_string()))?;
        Ok(resp.status().is_success())
    }

    async fn upsert(&self, documents: Vec<VectorDocument>) -> Result<usize> {
        let url = format!("{}/_bulk", self.base_url);
        let count = documents.len();

        let mut body = String::new();
        for doc in documents {
            let action = serde_json::to_string(&serde_json::json!({
                "index": { "_index": self.index_name, "_id": doc.id }
            })).unwrap();

            let mut source = serde_json::Map::new();
            source.insert("content".into(), doc.content.clone().into());
            source.insert("vector".into(), doc.vector.clone().into());
            for (k, v) in &doc.metadata {
                source.insert(k.clone(), v.clone());
            }
            let source_json = serde_json::to_string(&source).unwrap();

            body.push_str(&action);
            body.push('\n');
            body.push_str(&source_json);
            body.push('\n');
        }

        let resp = self.client
            .post(&url)
            .header("Content-Type", "application/x-ndjson")
            .body(body)
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
        let url = format!("{}/_delete_by_query", self.index_url());

        let body = serde_json::json!({
            "query": {
                "terms": { "_id": ids }
            }
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
        let url = format!("{}/_doc/{}", self.index_url(), id);

        let resp = self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| VectorError::RequestFailed(e.to_string()))?;

        if !resp.status().is_success() {
            return Ok(None);
        }

        #[derive(Deserialize)]
        struct DocResponse {
            found: bool,
            _source: Option<DocSource>,
        }

        #[derive(Deserialize)]
        struct DocSource {
            content: String,
            vector: Vec<f32>,
            #[serde(flatten)]
            metadata: HashMap<String, serde_json::Value>,
        }

        let result: DocResponse = resp.json().await
            .map_err(|e| VectorError::ParseError(e.to_string()))?;

        Ok(result._source.map(|s| {
            let mut metadata = HashMap::new();
            for (k, v) in s.metadata {
                if !matches!(k.as_str(), "content" | "vector") {
                    metadata.insert(k, v);
                }
            }
            VectorDocument {
                id: id.to_string(),
                content: s.content,
                vector: s.vector,
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
        let url = self.search_url();

        let body = serde_json::json!({
            "knn": {
                "field": "vector",
                "query_vector": vector,
                "k": limit,
                "num_candidates": limit * 10
            }
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
            hits: HitsOuter,
        }

        #[derive(Deserialize)]
        struct HitsOuter {
            hits: Vec<Hit>,
        }

        #[derive(Deserialize)]
        struct Hit {
            _id: String,
            _score: Option<f32>,
            _source: Option<DocSource>,
        }

        #[derive(Deserialize)]
        struct DocSource {
            content: String,
            vector: Vec<f32>,
            #[serde(flatten)]
            metadata: HashMap<String, serde_json::Value>,
        }

        let result: SearchResponse = resp.json().await
            .map_err(|e| VectorError::ParseError(e.to_string()))?;

        Ok(result.hits.hits.into_iter().filter_map(|hit| {
            let source = hit._source?;
            let score = hit._score?;

            let mut metadata = HashMap::new();
            for (k, v) in source.metadata {
                if !matches!(k.as_str(), "content" | "vector") {
                    metadata.insert(k, v);
                }
            }

            Some(SearchResult {
                document: VectorDocument {
                    id: hit._id,
                    content: source.content,
                    vector: source.vector,
                    metadata,
                    score: Some(score),
                },
                score,
            })
        }).collect())
    }

    async fn hybrid_search(&self, vector: &[f32], query: &str, limit: usize, _options: Option<SearchOptions>) -> Result<Vec<SearchResult>> {
        let url = self.search_url();

        let body = serde_json::json!({
            "query": {
                "match": { "content": query }
            },
            "knn": {
                "field": "vector",
                "query_vector": vector,
                "k": limit,
                "num_candidates": limit * 10
            },
            "rank": {
                "rrf": {
                    "window_size": limit * 2,
                    "rank_constant": 60
                }
            }
        });

        let resp = self.client
            .post(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| VectorError::RequestFailed(e.to_string()))?;

        if !resp.status().is_success() {
            return self.search(vector, limit, None).await;
        }

        #[derive(Deserialize)]
        struct SearchResponse {
            hits: HitsOuter,
        }

        #[derive(Deserialize)]
        struct HitsOuter {
            hits: Vec<Hit>,
        }

        #[derive(Deserialize)]
        struct Hit {
            _id: String,
            _score: Option<f32>,
            _source: Option<DocSource>,
        }

        #[derive(Deserialize)]
        struct DocSource {
            content: String,
            vector: Vec<f32>,
            #[serde(flatten)]
            metadata: HashMap<String, serde_json::Value>,
        }

        let result: SearchResponse = resp.json().await
            .map_err(|e| VectorError::ParseError(e.to_string()))?;

        Ok(result.hits.hits.into_iter().filter_map(|hit| {
            let source = hit._source?;
            let score = hit._score.unwrap_or(0.0);

            let mut metadata = HashMap::new();
            for (k, v) in source.metadata {
                if !matches!(k.as_str(), "content" | "vector") {
                    metadata.insert(k, v);
                }
            }

            Some(SearchResult {
                document: VectorDocument {
                    id: hit._id,
                    content: source.content,
                    vector: source.vector,
                    metadata,
                    score: Some(score),
                },
                score,
            })
        }).collect())
    }

    async fn count(&self) -> Result<usize> {
        let url = format!("{}/_count", self.index_url());

        let resp = self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| VectorError::RequestFailed(e.to_string()))?;

        if !resp.status().is_success() {
            return Ok(0);
        }

        #[derive(Deserialize)]
        struct CountResponse {
            count: usize,
        }

        let result: CountResponse = resp.json().await
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
        let url = format!("{}/_cluster/health", self.base_url);
        let resp = self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| VectorError::RequestFailed(e.to_string()))?;
        Ok(resp.status().is_success())
    }
}
