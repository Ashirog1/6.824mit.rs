// crates/mr-core/tests/correctness.rs

use mr_core::{KeyValue, MapReduce, WordCount};
use std::{path::PathBuf, time::Duration};

fn run_sequential(files: &[PathBuf]) -> Vec<String> {
    let app = WordCount;
    let mut kvs: Vec<KeyValue> = files.iter()
        .flat_map(|f| {
            let contents = std::fs::read_to_string(f).unwrap();
            app.map(f.to_str().unwrap(), &contents)
        })
        .collect();
    kvs.sort();
    kvs.chunk_by(|a, b| a.key == b.key)
        .map(|chunk| {
            let values: Vec<String> = chunk.iter().map(|kv| kv.value.clone()).collect();
            format!("{} {}", chunk[0].key, app.reduce(&chunk[0].key, &values))
        })
        .collect()
}

// --- helper 2: run distributed in an isolated temp dir ---
async fn run_distributed(files: &[PathBuf], n_reduce: usize) -> Vec<String> {
    // Each test run gets its own directory — no file conflicts
    let dir = std::env::temp_dir().join(format!("mr-test-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let original = std::env::current_dir().unwrap();
    std::env::set_current_dir(&dir).unwrap();       // intermediate + output files land
    let file_strs: Vec<String> = files.iter()
        .map(|f| f.to_str().unwrap().to_string())
        .collect();

    // Start coordinator, give it a moment to bind the socket
    let coord = tokio::spawn(mr_core::coordinator::run(file_strs, n_reduce));
    tokio::time::sleep(Duration::from_millis(50)).await;

    // Run a single worker to completion
    mr_core::worker::run(&WordCount).await.unwrap();
    coord.await.unwrap().unwrap();

    // Collect all output lines
    let mut lines: Vec<String> = (0..n_reduce)
        .filter_map(|i| std::fs::read_to_string(format!("mr-out-{}", i)).ok())
        .flat_map(|s| s.lines().filter(|l|
!l.is_empty()).map(String::from).collect::<Vec<_>>())
        .collect();
    lines.sort();

    // Restore directory and clean up
    std::env::set_current_dir(original).unwrap();
    std::fs::remove_dir_all(&dir).unwrap();
    lines
}

#[tokio::test]
async fn wc_distributed_matches_sequential() {
    // Use absolute path — current dir during test run is unpredictable
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../crates/resources");
    let files = vec![root.join("pg-grimm.txt")];

    let expected = run_sequential(&files);
    let actual   = run_distributed(&files, 10).await;

    assert_eq!(actual, expected);
}
