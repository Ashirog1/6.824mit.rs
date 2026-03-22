//! Word count MapReduce application.
//!
//! Reference: go-version/src/mrapps/wc.go

use crate::{KeyValue, MapReduce};

pub struct WordCount;

impl MapReduce for WordCount {
    fn map(&self, _filename: &str, contents: &str) -> Vec<KeyValue> {
        contents
            .split(|c: char| !c.is_alphabetic())
            .filter(|s| !s.is_empty())
            .map(|word| KeyValue::new(word.to_string(), "1".to_string()))
            .collect()
    }

    fn reduce(&self, _key: &str, values: &[String]) -> String {
        values.len().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_simple() {
        let wc = WordCount;
        let result = wc.map("test.txt", "hello world hello");
        assert_eq!(result.len(), 3);
        // Words should be: hello, world, hello
    }

    #[test]
    fn test_reduce_simple() {
        let wc = WordCount;
        let result = wc.reduce("hello", &["1".to_string(), "1".to_string()]);
        assert_eq!(result, "2");
    }
}
