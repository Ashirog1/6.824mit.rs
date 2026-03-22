use std::{fs, io::Write};

use anyhow::{Context, Result};
use mr_core::{KeyValue, WordCount, MapReduce};

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        anyhow::bail!("Usage: {} <inputfiles...>", args[0]);
    }
    let app = WordCount;
    let mut intermediate: Vec<KeyValue> = Vec::new();
    for filename in &args[1..] {
        let contents = fs::read_to_string(filename)?;
        let kvs = app.map(filename, &contents);
        intermediate.extend(kvs);
    }
    intermediate.sort();
    let output_file = fs::File::create("mr-out-0").context("cannot create output file")?;
    let mut writer = std::io::BufWriter::new(output_file);
    
    for chunk in intermediate.chunk_by(|a, b| a.key == b.key) {
        let key = &chunk[0].key;
        let values: Vec<String> = chunk.iter().map(|kv| kv.value.clone()).collect();
        writeln!(writer, "{} {}", key, app.reduce(key, &values))?;
    }
    
    Ok(())
}