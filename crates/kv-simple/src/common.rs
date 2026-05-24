use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)] 
pub struct GetArgs {
    pub key: String,
    pub client_id: u64,
    pub seq: u64,
}

#[derive(Clone, Serialize, Deserialize)] 
pub struct GetReply{
    pub value: String,
}

#[derive(Clone, Serialize, Deserialize)] 
pub struct PutAppenedArgs{
    pub key: String,
    pub value: String,
    pub client_id: u64,
    pub seq: u64,
    pub ack_seq: u64,
}

#[derive(Clone, Serialize, Deserialize)] 
pub struct PutAppendReply {
    pub value: String,
}
