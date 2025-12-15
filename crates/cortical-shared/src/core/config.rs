//! Configuration types for Cortical Compose services.

use serde::Deserialize;

/// Base configuration shared by all services
#[derive(Debug, Clone, Deserialize)]
pub struct BaseConfig {
    #[serde(default = "default_service_name")]
    pub service_name: String,
    #[serde(default = "default_version")]
    pub version: String,
    #[serde(default = "default_host")]
    pub host: String,
    #[serde(default = "default_port")]
    pub port: u16,
    #[serde(default)]
    pub debug: bool,
    #[serde(default = "default_log_level")]
    pub log_level: String,
}

fn default_service_name() -> String {
    "cortical-service".to_string()
}

fn default_version() -> String {
    "0.1.0".to_string()
}

fn default_host() -> String {
    "0.0.0.0".to_string()
}

fn default_port() -> u16 {
    8000
}

fn default_log_level() -> String {
    "info".to_string()
}

impl Default for BaseConfig {
    fn default() -> Self {
        Self {
            service_name: default_service_name(),
            version: default_version(),
            host: default_host(),
            port: default_port(),
            debug: false,
            log_level: default_log_level(),
        }
    }
}

/// Qdrant vector store configuration
#[derive(Debug, Clone, Deserialize)]
pub struct QdrantConfig {
    #[serde(default = "default_qdrant_url")]
    pub url: String,
    #[serde(default = "default_collection_prefix")]
    pub collection_prefix: String,
    #[serde(default = "default_vector_size")]
    pub vector_size: usize,
}

fn default_qdrant_url() -> String {
    "http://localhost:6333".to_string()
}

fn default_collection_prefix() -> String {
    "cortical".to_string()
}

fn default_vector_size() -> usize {
    384 // all-MiniLM-L6-v2 dimension
}

impl Default for QdrantConfig {
    fn default() -> Self {
        Self {
            url: default_qdrant_url(),
            collection_prefix: default_collection_prefix(),
            vector_size: default_vector_size(),
        }
    }
}

/// PostgreSQL configuration
#[derive(Debug, Clone, Deserialize)]
pub struct PostgresConfig {
    #[serde(default = "default_database_url")]
    pub url: String,
    #[serde(default = "default_max_connections")]
    pub max_connections: u32,
}

fn default_database_url() -> String {
    "postgres://cortical:cortical_secret@localhost:5432/cortical_db".to_string()
}

fn default_max_connections() -> u32 {
    10
}

impl Default for PostgresConfig {
    fn default() -> Self {
        Self {
            url: default_database_url(),
            max_connections: default_max_connections(),
        }
    }
}

/// Configuration for Thalamus service
#[derive(Debug, Clone, Deserialize)]
pub struct ThalamusConfig {
    #[serde(flatten)]
    pub base: BaseConfig,
    #[serde(default = "default_embedding_model")]
    pub embedding_model: String,
    #[serde(default = "default_max_text_length")]
    pub max_text_length: usize,
}

fn default_embedding_model() -> String {
    "all-MiniLM-L6-v2".to_string()
}

fn default_max_text_length() -> usize {
    8192
}

impl Default for ThalamusConfig {
    fn default() -> Self {
        Self {
            base: BaseConfig {
                service_name: "thalamus".to_string(),
                ..Default::default()
            },
            embedding_model: default_embedding_model(),
            max_text_length: default_max_text_length(),
        }
    }
}

/// Configuration for Prefrontal Cortex service
#[derive(Debug, Clone, Deserialize)]
pub struct PrefrontalCortexConfig {
    #[serde(flatten)]
    pub base: BaseConfig,
    #[serde(default = "default_thalamus_url")]
    pub thalamus_url: String,
    #[serde(default = "default_hippocampus_url")]
    pub hippocampus_url: String,
    #[serde(default = "default_basal_ganglia_url")]
    pub basal_ganglia_url: String,
    #[serde(default = "default_cerebellum_url")]
    pub cerebellum_url: String,
    #[serde(default = "default_amygdala_url")]
    pub amygdala_url: String,
    #[serde(default = "default_confidence_threshold")]
    pub default_confidence_threshold: f32,
    #[serde(default = "default_max_similar_results")]
    pub max_similar_results: usize,
}

fn default_thalamus_url() -> String {
    "http://localhost:8001".to_string()
}

fn default_hippocampus_url() -> String {
    "http://localhost:8002".to_string()
}

fn default_basal_ganglia_url() -> String {
    "http://localhost:8003".to_string()
}

fn default_cerebellum_url() -> String {
    "http://localhost:8004".to_string()
}

fn default_amygdala_url() -> String {
    "http://localhost:8005".to_string()
}

fn default_confidence_threshold() -> f32 {
    0.5
}

fn default_max_similar_results() -> usize {
    10
}

impl Default for PrefrontalCortexConfig {
    fn default() -> Self {
        Self {
            base: BaseConfig {
                service_name: "prefrontal_cortex".to_string(),
                ..Default::default()
            },
            thalamus_url: default_thalamus_url(),
            hippocampus_url: default_hippocampus_url(),
            basal_ganglia_url: default_basal_ganglia_url(),
            cerebellum_url: default_cerebellum_url(),
            amygdala_url: default_amygdala_url(),
            default_confidence_threshold: default_confidence_threshold(),
            max_similar_results: default_max_similar_results(),
        }
    }
}

/// Configuration for memory-enabled region services
#[derive(Debug, Clone, Deserialize)]
pub struct RegionConfig {
    #[serde(flatten)]
    pub base: BaseConfig,
    #[serde(default)]
    pub qdrant: QdrantConfig,
    #[serde(default)]
    pub postgres: PostgresConfig,
}

impl Default for RegionConfig {
    fn default() -> Self {
        Self {
            base: BaseConfig::default(),
            qdrant: QdrantConfig::default(),
            postgres: PostgresConfig::default(),
        }
    }
}
