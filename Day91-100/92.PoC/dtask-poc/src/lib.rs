use openraft::Config;
use openraft::Raft;
use serde::{Deserialize, Serialize};
use std::io::Cursor;

pub mod store;
pub mod network;

pub type NodeId = u64;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Request {
    Set { key: String, value: String },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Response {
    pub value: Option<String>,
}

openraft::declare_raft_types!(
    pub TypeConfig:
        D = Request,
        R = Response,
        NodeId = NodeId,
);

pub type MyRaft = Raft<TypeConfig>;
