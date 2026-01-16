#![allow(unexpected_cfgs)]
use openraft;

/// Type alias for Node ID.
pub type NodeId = u64;

/// Represents a node in the Raft cluster.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq, Default)]
pub struct Node {
    /// RPC address of the node.
    pub rpc_addr: String,
}

/// Request payload for the state machine.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct Request {
    pub payload: String,
}

/// Response from the state machine.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct Response {
    pub result: String,
}

openraft::declare_raft_types!(
    /// Configuration for DTask Raft types.
    pub TypeConfig: D = Request, R = Response, NodeId = NodeId, Node = Node,
    Entry = openraft::Entry<TypeConfig>, SnapshotData = std::io::Cursor<Vec<u8>>
);
