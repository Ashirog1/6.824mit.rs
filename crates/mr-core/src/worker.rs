use std::{any, fmt::format};

use crate::{KeyValue, MapReduce, rpc::{Request, Task, read_msg, write_msg}};
use serde_json::{ser, to_string};

fn ihash(key: &str) -> usize {
    let mut h: u32 = 2166136261;
    for b in key.bytes() { h ^= b as u32; h = h.wrapping_mul(16777619); }
    (h & 0x7fffffff) as usize
}

async fn rpc(req: Request) -> anyhow::Result<Task> {
    let mut s = tokio::net::UnixStream::connect(crate::rpc::sock_path()).await?;
    write_msg(&mut s, &req).await?;
    read_msg(&mut s).await
}

async fn do_map(app: &dyn MapReduce, task: &Task) -> anyhow::Result<()> {
    let contents = tokio::fs::read_to_string(&task.filename).await?;
    let kvs = app.map(&task.filename, &contents);
    
    let mut buckets: Vec<Vec<&KeyValue>> = vec![vec![]; task.n_reduce];
    for kv in &kvs { buckets[ihash(&kv.key)%task.n_reduce].push(kv); }
    
    for (y, bucket) in buckets.iter().enumerate() {
        let path = format!("mr-{}-{}", task.task_id, y);
        let tmp = format!("{}.tmp.{}", path, std::process::id());
        tokio::fs::write(&tmp, serde_json::to_string(bucket)?).await?;
        tokio::fs::rename(&tmp, &path).await?;
    }
    Ok(())
}

async fn do_reduce(app: &dyn MapReduce, task: &Task) -> anyhow::Result<()> {
    let mut kvs: Vec<KeyValue> = Vec::new();
    for x in 0..task.n_map {
        let path = format!("mr-{}-{}", x, task.task_id);
        if let Ok(data) = tokio::fs::read_to_string(&path).await {
            kvs.extend(serde_json::from_str::<Vec<KeyValue>>(&data)?);
        }
    }
    kvs.sort();
    
    let mut out = String::new();
}
