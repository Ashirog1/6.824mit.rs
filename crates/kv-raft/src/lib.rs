//! Raft-replicated key-value service.
//!
//! Ported from MIT 6.824's kvraft package.
//!
//! # Overview
//!
//! This crate implements a fault-tolerant key-value store using Raft for
//! replication and consensus.
//!
//! # Key Features
//!
//! - Get/Put/Append operations with linearizable semantics
//! - Fault tolerance via Raft replication
//! - Leader detection and retry logic
//! - Duplicate request detection
//!
//! # Key Types
//!
//! - `KVServer` - Server-side KV store backed by Raft
//! - `Clerk` - Client with leader detection and retry
//! - Error types: `ErrNoKey`, `ErrWrongLeader`

// TODO: Implement KVServer, Clerk, and error types
// Reference: go-version/src/kvraft/server.go, client.go, common.go
