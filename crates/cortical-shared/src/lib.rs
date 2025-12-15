//! Cortical Shared - Common types and utilities for Cortical Compose
//!
//! This crate provides shared data models, error types, and HTTP clients
//! used across all brain region services.

pub mod clients;
pub mod config;
pub mod errors;
pub mod models;
pub mod policies;

pub use clients::*;
pub use config::*;
pub use errors::*;
pub use models::*;
pub use policies::*;
