//! Shard configuration controller.
//!
//! Ported from MIT 6.824's shardctrler package.
//!
//! # Overview
//!
//! This crate manages shard configurations for a sharded key-value store,
//! handling replica group membership changes and shard rebalancing.
//!
//! # Constants
//!
//! - `NShards = 10` - Number of shards in the system
//!
//! # Key Features
//!
//! - Join: Add new replica groups
//! - Leave: Remove replica groups
//! - Move: Manually reassign a shard to a group
//! - Query: Retrieve configuration by number
//! - Automatic load balancing across groups
//!
//! # Key Types
//!
//! - `ShardCtrler` - Controller server
//! - `Clerk` - Client stub
//! - `Config` - Shard assignment configuration

pub const NSHARDS: usize = 10;

// TODO: Implement ShardCtrler, Clerk, Config, and operation types
// Reference: go-version/src/shardctrler/server.go, client.go, common.go
