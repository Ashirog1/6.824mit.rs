//! Serialization codec with validation.
//!
//! Ported from MIT 6.824's labgob package.
//!
//! # Overview
//!
//! This crate provides a wrapper around serialization (bincode) that enforces
//! safety checks, such as ensuring all fields are public (in Rust: `pub`).
//!
//! # Purpose
//!
//! In Go, labgob checks that RPC message fields are capitalized (exported).
//! In Rust, we check that struct fields are `pub` to ensure proper serialization.

// TODO: Implement encoder/decoder with validation
// Reference: go-version/src/labgob/labgob.go
