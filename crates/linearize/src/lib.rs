//! Linearizability checker (Porcupine port).
//!
//! Ported from the Porcupine linearizability checker.
//!
//! # Overview
//!
//! This crate verifies that the history of operations on a distributed system
//! is linearizable - that is, operations appear to execute atomically and
//! in some sequential order consistent with their real-time ordering.
//!
//! # Key Types
//!
//! - `Checker` - Main linearizability verification engine
//! - `Model` - Abstract model defining operation semantics
//! - `Operation` - Represents a single operation with call/return times
//! - `Event` - Call or return event with timestamp

// TODO: Implement Checker, Model, Operation, Event types
// Reference: go-version/src/porcupine/checker.go, model.go, porcupine.go
