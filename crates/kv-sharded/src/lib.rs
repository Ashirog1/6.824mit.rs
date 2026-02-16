//! Sharded key-value service.
//!
//! Ported from MIT 6.824's shardkv package.
//!
//! # Overview
//!
//! This crate implements a horizontally sharded, fault-tolerant key-value store
//! with automatic shard rebalancing and migration.
//!
//! # Key Features
//!
//! - Horizontal sharding across replica groups
//! - Shard-aware client routing
//! - Dynamic shard migration during reconfiguration
//! - Fault tolerance via Raft within each shard group
//! - Duplicate detection
//!
//! # Key Types
//!
//! - `ShardKV` - Sharded KV server
//! - `Clerk` - Client with shard-aware routing
//! - Error types: `ErrNoKey`, `ErrWrongGroup`, `ErrWrongLeader`

// TODO: Implement ShardKV, Clerk, and error types
// Reference: go-version/src/shardkv/server.go, client.go, common.go
