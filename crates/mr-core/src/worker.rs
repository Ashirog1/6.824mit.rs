use std::{time::Duration};

use crate::{KeyValue, MapReduce, rpc::{Request, Task, TaskType, read_msg, write_msg}};

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
    for chunk in kvs.chunk_by(|a, b| a.key == b.key) {
        let values: Vec<String> = chunk.iter().map(|kv| kv.value.clone()).collect();
        out.push_str(&format!("{} {}\n", chunk[0].key, app.reduce(&chunk[0].key, &values)));
    }
    let path = format!("mr-out-{}", task.task_id);
    let tmp = format!("{}.tmp.{}", path, std::process::id());
    tokio::fs::write(&tmp, out).await?;
    tokio::fs::rename(&tmp, &path).await?;
    
    // clean up
    for x in 0..task.n_map {
        let _ = tokio::fs::remove_file(format!("mr-{}-{}", x, task.task_id)).await;
    }
    Ok(())
}

pub async fn run(app: &dyn MapReduce) -> anyhow::Result<()> {
    let mut req = Request::AssignTask;
    loop {
        let task = match rpc(req).await{
            Ok(t) => t,
            Err(_) => break,
        };
        req = match task.task_type {
            TaskType::Map => {
                do_map(app, &task).await?;
                Request::TaskDone { task_type: TaskType::Map, task_id: task.task_id }
            }
            TaskType::Reduce => {
                do_reduce(app, &task).await?;
                Request::TaskDone { task_type: TaskType::Reduce, task_id: task.task_id }
            }
            TaskType::Wait => {
                tokio::time::sleep(Duration::from_millis(100)).await;
                Request::AssignTask
            }
            TaskType::Done => break,
        }
    }
    Ok(())
}
