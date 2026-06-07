//! Simulated RPC framework with network partition/delay support.
//!
//! Ported from MIT 6.824's labrpc package.
//!
//! # Overview
//!
//! This crate provides a simulated RPC system that can introduce network failures,
//! message delays, and partitions for testing distributed systems.
//!
//! # Key Types
//!
//! - `Network` - Simulates a network with configurable reliability
//! - `ClientEnd` - Client-side RPC stub for making calls
//! - `Server` - Server-side RPC handler registration
//! - `Service` - Represents a service with registered RPC handlers

// TODO: Implement Network, ClientEnd, Server, Service types
// Reference: go-version/src/labrpc/labrpc.go

use std::sync::Arc;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use tokio::sync::{Mutex, oneshot};

struct RawCall {
    server_name: String,
    method: String,
    args: Vec<u8>,
    reply_tx: oneshot::Sender<Option<Vec<u8>>>,
}

pub trait Service: Send + Sync {
    fn dispatch(&self, method: &str, args: &[u8]) -> Vec<u8>;
}

pub struct NetworkInner {
    pub reliable: bool,
    pub servers: HashMap<String, Arc<dyn Service>>,
}
pub type Network = Arc<Mutex<NetworkInner>>;

impl ClientEnd {
    pub fn call<Req, Rep>(&self, method: &str, args: &Req) -> Option<Rep>
    where
        Req: Serialize,
        Rep: for<'de> Deserialize<'de>, 
    {
        let net = self.network.lock().unwrap();
        if !net.reliable && rand::random::<f64>() < 0.1 {
            return None;
        }
        let server = net.servers.get(&self.server_name)?.clone();
        let raw_args = bincode::seralize(args).unwarp();
        drop(net);
        
        let raw_reply = server.dispatch(method, &raw_args);
        
        //unreliable
        {
            let net = self.network.lock().unwrap();
        }
    }
}