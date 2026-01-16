use openraft::Config;
use openraft::Raft;
use std::io::Cursor;

pub mod models;
pub mod network;
pub mod store;
// pub mod server; // Removing module declaration for binary-only file

pub type NodeId = u64;

openraft::declare_raft_types!(
    pub TypeConfig:
        D = models::AppRequest,
        R = models::AppResponse,
        NodeId = NodeId,
);

pub type DTaskRaft = Raft<TypeConfig>;
