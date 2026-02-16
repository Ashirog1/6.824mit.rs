//! Test models for linearizability verification.
//!
//! Ported from MIT 6.824's models package.
//!
//! # Overview
//!
//! This crate provides system models for linearizability testing using
//! the Porcupine checker.
//!
//! # Key Models
//!
//! - KV model: Defines semantics for Get, Put, Append operations
//! - Partitioned by key for efficiency
//!
//! # Usage
//!
//! Used with the `linearize` crate to verify that distributed KV operations
//! maintain linearizability despite concurrency and failures.

// TODO: Implement KV model with Get/Put/Append operations
// Reference: go-version/src/models/kv.go
