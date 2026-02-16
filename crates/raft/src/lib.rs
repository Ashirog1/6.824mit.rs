//! Raft consensus algorithm implementation.
//!
//! Ported from MIT 6.824's Raft implementation.
//!
//! # Overview
//!
//! This crate implements the Raft consensus algorithm for building
//! fault-tolerant replicated state machines.
//!
//! # Key Features
//!
//! - Leader election
//! - Log replication
//! - Safety guarantees
//! - Log compaction via snapshots
//! - Persistence
//!
//! # Key Types
//!
//! - `Raft` - Main Raft state machine
//! - `ApplyMsg` - Messages sent to the application via the apply channel
//! - `Persister` - Persistent state storage for Raft state and snapshots

// TODO: Implement Raft, ApplyMsg, Persister types
// Reference: go-version/src/raft/raft.go, persister.go
