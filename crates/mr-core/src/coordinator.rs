use std::{
    sync::{Arc, Mutex}, time::{Duration, Instant}
};
use tokio::{net::UnixListener};
use crate::rpc::{Request, Task, TaskType, read_msg, write_msg};

#[derive(Debug, Clone, PartialEq)]
enum Phase { Idle, Running(Instant), Done }

struct State {
    files: Vec<String>,
    n_reduce: usize,
    maps: Vec<Phase>,
    reduces: Vec<Phase>,
}

impl State {
    fn new(files: Vec<String>, n_reduce: usize) -> Self {
        let n = files.len();
        Self { files, n_reduce: n_reduce, maps: vec![Phase::Idle; n], reduces: vec![Phase::Idle; n_reduce] }
    }
    fn assign(&mut self) -> Task {
        let n_map    = self.files.len();
        let now      = Instant::now();
        let timeout  = Duration::from_secs(10);
        let n_reduce = self.n_reduce;
        let stub     = |task_type| Task { task_type, task_id: 0, n_reduce, n_map: n_map, filename: String::new() };

        for p in self.maps.iter_mut().chain(self.reduces.iter_mut()) {
            if let Phase::Running(t) = *p {
                if now.duration_since(t) > timeout { *p = Phase::Idle; }
            }
        }
        
        for (id, p) in self.maps.iter_mut().enumerate() {
            if *p == Phase::Idle {
                *p = Phase::Running(now);
                return Task { task_type: TaskType::Map, task_id: id, n_reduce: self.n_reduce, n_map, filename: String::new() }
            }
        }
        
        if !self.maps.iter().all(|p| *p == Phase::Done) {
            return stub(TaskType::Wait);
        }
        
        for (id, p) in self.reduces.iter_mut().enumerate() {
            if *p == Phase::Idle {
                *p = Phase::Running(now);
                return Task { task_type: TaskType::Reduce, task_id: id, n_reduce: self.n_reduce, n_map, filename: String::new()}
            }
        }
        
        stub(TaskType::Done)
    }
    
    fn complete(&mut self, kind: TaskType, id: usize) {
        match kind {
            TaskType::Map => self.maps[id] = Phase::Done,
            TaskType::Reduce => self.reduces[id] = Phase::Done,
            _ => {}
        }
    }
    
    fn is_done(&self) -> bool {
        self.reduces.iter().all(|p| *p == Phase::Done)
    }
}

pub async fn run(files: Vec<String>, n_reduce: usize) -> anyhow::Result<()> {
    let sock = crate::rpc::sock_path();
    let _ = std::fs::remove_file(&sock);
    let listener = UnixListener::bind(&sock)?; 
    let state = Arc::new(Mutex::new(State::new(files, n_reduce)));
    
    loop {
        if state.lock().unwrap().is_done() {break; }
        let Ok(Ok((mut stream, _))) = 
            tokio::time::timeout(Duration::from_secs(1), listener.accept()).await
            else {
                continue;
            };
        
        let state = state.clone();
        tokio::spawn(async move {
            let req: Request = read_msg(&mut stream).await?;
            let task = {
                let mut s = state.lock().unwrap();
                if let Request::TaskDone { task_type, task_id} = req {s.complete(task_type, task_id);}
                s.assign()
            };
            write_msg(&mut stream, &task).await?;
            anyhow::Ok(())
        });
    }
    tokio::time::sleep(Duration::from_secs(1)).await;
    let _ = std::fs::remove_file(&sock);
    
    Ok(()) 
}