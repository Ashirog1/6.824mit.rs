//! MapReduce framework core.
//!
//! Ported from MIT 6.824's mr package.
//!
//! # Overview
//!
//! This crate provides:
//! - Core types (`KeyValue`) and traits (`MapReduce`)
//! - Built-in MapReduce applications (word count, indexer, etc.)
//! - Future: Distributed coordinator and worker processes
//!
//! # Usage
//!
//! ```ignore
//! use mr_core::{MapReduce, WordCount, KeyValue};
//!
//! let app = WordCount;
//! let results = app.map("file.txt", "hello world");
//! ```

/// A key-value pair produced by map and consumed by reduce.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
pub struct KeyValue {
    pub key: String,
    pub value: String,
}

impl KeyValue {
    pub fn new(key: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            value: value.into(),
        }
    }
}

/// Trait for MapReduce applications.
///
/// Implement this trait to create custom MapReduce jobs.
pub trait MapReduce {
    /// Map function: processes input and produces key-value pairs.
    ///
    /// # Arguments
    /// * `filename` - Name of the input file (for context)
    /// * `contents` - Complete contents of the file
    ///
    /// # Returns
    /// Vector of key-value pairs to be shuffled and reduced
    fn map(&self, filename: &str, contents: &str) -> Vec<KeyValue>;

    /// Reduce function: aggregates values for a single key.
    ///
    /// # Arguments
    /// * `key` - The key to reduce
    /// * `values` - All values associated with this key
    ///
    /// # Returns
    /// The reduced result as a string
    fn reduce(&self, key: &str, values: &[String]) -> String;
}

/// Built-in MapReduce applications
pub mod apps;
pub mod rpc;
pub mod coordinator;
pub mod worker;

// Re-export common apps for convenience
pub use apps::WordCount;
