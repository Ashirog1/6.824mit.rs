//! Simple unreplicated key-value service.
//!
//! Ported from MIT 6.824's kvsrv package.
//!
//! # Overview
//!
//! This crate implements a simple, unreplicated key-value store as a baseline
//! for understanding more complex replicated systems.
//!
//! # Key Features
//!
//! - Get/Put/Append operations
//! - RPC-based client-server architecture
//! - No fault tolerance (single server)
//!
//! # Key Types
//!
//! - `KVServer` - Server-side key-value store
//! - `Clerk` - Client stub for making RPC calls
//! - RPC message types: `GetArgs`, `GetReply`, `PutAppendArgs`, `PutAppendReply`

// TODO: Implement KVServer, Clerk, and RPC types
// Reference: go-version/src/kvsrv/server.go, client.go, common.go
