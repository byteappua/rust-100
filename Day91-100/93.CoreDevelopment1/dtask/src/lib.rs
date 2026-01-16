use openraft::Config;
use openraft::Raft;
use std::io::Cursor;

pub mod models;
pub mod network;
pub mod store;
pub mod scheduler;
pub mod dispatcher;

pub type NodeId = u64;

openraft::declare_raft_types!(
    pub TypeConfig:
        D = models::AppRequest,
        R = models::AppResponse,
        NodeId = NodeId,
        Node = openraft::BasicNode,
        Entry = openraft::Entry<TypeConfig>,
        SnapshotData = Cursor<Vec<u8>>
);

pub type DTaskRaft = Raft<TypeConfig>;
