//! Cortical Shared - Common types and utilities for Cortical Compose
//!
//! This crate provides shared data models, error types, and HTTP clients
//! used across all brain region services.
//!
//! # Module Organization
//!
//! - **core** - Fundamental types: models, errors, config
//! - **comms** - Communication: HTTP clients, inter-brain protocol, crypto
//! - **policy** - Decision policies: pathways, help-seeking, domains

pub mod core;
pub mod comms;
pub mod policy;

// Re-export all public types for convenience
pub use core::*;
pub use comms::*;
pub use policy::*;
