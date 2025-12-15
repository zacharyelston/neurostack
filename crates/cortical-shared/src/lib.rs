//! Cortical Shared - Common types and utilities for Cortical Compose
//!
//! This crate provides shared data models, error types, and HTTP clients
//! used across all brain region services.

pub mod assistance;
pub mod clients;
pub mod config;
pub mod domains;
pub mod errors;
pub mod interop;
pub mod models;
pub mod policies;

pub use assistance::*;
pub use clients::*;
pub use config::*;
pub use domains::*;
pub use errors::*;
pub use interop::*;
pub use models::*;
pub use policies::*;
