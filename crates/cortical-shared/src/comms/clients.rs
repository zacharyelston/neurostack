//! HTTP clients for inter-service communication.

use crate::{
    CorticalError, CorticalResult, Context, DecisionRequest, Decision, Experience,
    ExperienceSource, FeedbackRequest, FeaturePacket, HealthResponse, IngestRequest,
    QueryRequest, QueryResponse, StoreRequest, StoreResponse,
};
use reqwest::Client;
use std::time::Duration;

/// Base HTTP client for service communication
#[derive(Debug, Clone)]
pub struct ServiceClient {
    base_url: String,
    client: Client,
}

impl ServiceClient {
    pub fn new(base_url: impl Into<String>) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            base_url: base_url.into().trim_end_matches('/').to_string(),
            client,
        }
    }

    pub fn with_timeout(base_url: impl Into<String>, timeout: Duration) -> Self {
        let client = Client::builder()
            .timeout(timeout)
            .build()
            .expect("Failed to create HTTP client");

        Self {
            base_url: base_url.into().trim_end_matches('/').to_string(),
            client,
        }
    }

    pub async fn health(&self) -> CorticalResult<HealthResponse> {
        let response = self
            .client
            .get(format!("{}/health", self.base_url))
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(CorticalError::ServiceUnavailable(format!(
                "Health check failed with status: {}",
                response.status()
            )));
        }

        Ok(response.json().await?)
    }
}

/// Client for Thalamus service
#[derive(Debug, Clone)]
pub struct ThalamusClient {
    inner: ServiceClient,
}

impl ThalamusClient {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            inner: ServiceClient::new(base_url),
        }
    }

    pub async fn health(&self) -> CorticalResult<HealthResponse> {
        self.inner.health().await
    }

    pub async fn ingest(
        &self,
        text: impl Into<String>,
        source: ExperienceSource,
        context: Option<Context>,
    ) -> CorticalResult<FeaturePacket> {
        let request = IngestRequest {
            text: text.into(),
            source,
            context: context.unwrap_or_default(),
            actor: None,
        };

        let response = self
            .inner
            .client
            .post(format!("{}/ingest", self.inner.base_url))
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(CorticalError::ServiceUnavailable(format!(
                "Ingest failed with status {}: {}",
                status, body
            )));
        }

        Ok(response.json().await?)
    }
}

/// Client for memory-enabled region services (Hippocampus, Basal Ganglia, etc.)
#[derive(Debug, Clone)]
pub struct RegionClient {
    inner: ServiceClient,
    region_name: String,
}

impl RegionClient {
    pub fn new(base_url: impl Into<String>, region_name: impl Into<String>) -> Self {
        Self {
            inner: ServiceClient::new(base_url),
            region_name: region_name.into(),
        }
    }

    pub async fn health(&self) -> CorticalResult<HealthResponse> {
        self.inner.health().await
    }

    pub async fn store(
        &self,
        experience: Experience,
        metadata: Option<serde_json::Value>,
    ) -> CorticalResult<StoreResponse> {
        let request = StoreRequest {
            experience,
            metadata: metadata.unwrap_or(serde_json::Value::Null),
        };

        let response = self
            .inner
            .client
            .post(format!("{}/store", self.inner.base_url))
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(CorticalError::ServiceUnavailable(format!(
                "Store to {} failed with status {}: {}",
                self.region_name, status, body
            )));
        }

        Ok(response.json().await?)
    }

    pub async fn query(
        &self,
        embedding: Vec<f32>,
        limit: usize,
        filters: Option<serde_json::Value>,
    ) -> CorticalResult<QueryResponse> {
        let request = QueryRequest {
            embedding,
            limit,
            filters: filters.unwrap_or(serde_json::Value::Null),
            include_metadata: true,
        };

        let response = self
            .inner
            .client
            .post(format!("{}/query", self.inner.base_url))
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(CorticalError::ServiceUnavailable(format!(
                "Query to {} failed with status {}: {}",
                self.region_name, status, body
            )));
        }

        Ok(response.json().await?)
    }
}

/// Client for Prefrontal Cortex service
#[derive(Debug, Clone)]
pub struct PrefrontalCortexClient {
    inner: ServiceClient,
}

impl PrefrontalCortexClient {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            inner: ServiceClient::new(base_url),
        }
    }

    pub async fn health(&self) -> CorticalResult<HealthResponse> {
        self.inner.health().await
    }

    pub async fn decide(&self, request: DecisionRequest) -> CorticalResult<Decision> {
        let response = self
            .inner
            .client
            .post(format!("{}/decide", self.inner.base_url))
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(CorticalError::ServiceUnavailable(format!(
                "Decision failed with status {}: {}",
                status, body
            )));
        }

        Ok(response.json().await?)
    }

    pub async fn feedback(&self, request: FeedbackRequest) -> CorticalResult<()> {
        let response = self
            .inner
            .client
            .post(format!("{}/feedback", self.inner.base_url))
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(CorticalError::ServiceUnavailable(format!(
                "Feedback failed with status {}: {}",
                status, body
            )));
        }

        Ok(())
    }
}
