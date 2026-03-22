use std::time::Duration;
use crate::{KeyValue, MapReduce, rpc::{Request, Task, read_msg, write_msg}};

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
    
}
