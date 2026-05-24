use anyhow::Ok;
use serde::{Deserialize, Serialize};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::UnixStream;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum TaskType { Map, Reduce, Wait, Done }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub task_type: TaskType, 
    pub task_id: usize,
    pub n_reduce: usize,
    pub n_map: usize,
    pub filename: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Request {
    AssignTask, TaskDone { task_type : TaskType, task_id: usize }, 
}

pub fn sock_path() -> String {
    let user = std::env::var("USER").unwrap_or_else(|_| "nobody".to_string());
    format!("/var/tmp/5840-mr-{}", user)
}

pub async fn write_msg<T: Serialize>(s: &mut UnixStream, v: &T) -> anyhow::Result<()> {
    let b = bincode::serialize(v)?;
    s.write_u32_le(b.len() as u32).await?;
    s.write_all(&b).await?;
    Ok(())
}

pub async fn read_msg<T: for<'de> Deserialize<'de>>(s: &mut UnixStream) -> anyhow::Result<T> {
    let n = s.read_u32_le().await? as usize;
    let mut buf = vec![0u8; n];
    s.read_exact(&mut buf).await?;
    Ok(bincode::deserialize(&buf)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::net::UnixStream;

    #[tokio::test]
    async fn task_write_read_roundtrip() {
        let (mut w, mut r) = UnixStream::pair().unwrap();
        let sent = Task { task_type: TaskType::Map, task_id: 7, n_reduce: 10, n_map: 5, filename: String::new()};
        write_msg(&mut w, &sent).await.unwrap();
        let got: Task = read_msg(&mut r).await.unwrap();
        assert_eq!(got.task_type, sent.task_type);
        assert_eq!(got.task_id,   sent.task_id);
        assert_eq!(got.n_reduce,  sent.n_reduce);
        assert_eq!(got.n_map,     sent.n_map);
    }

    #[tokio::test]
    async fn all_task_types_roundtrip() {
        for kind in [TaskType::Map, TaskType::Reduce, TaskType::Wait, TaskType::Done] {
            let (mut w, mut r) = UnixStream::pair().unwrap();
            let t = Task { task_type: kind, task_id: 0, n_reduce: 4, n_map: 4, filename: String::new() };
            write_msg(&mut w, &t).await.unwrap();
            let got: Task = read_msg(&mut r).await.unwrap();
            assert_eq!(got.task_type, kind);
        }
    }
}
