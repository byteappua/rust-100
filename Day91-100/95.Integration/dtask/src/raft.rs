use openraft;

pub type NodeId = u64;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq, Default)]
pub struct Node {
    pub rpc_addr: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct Request {
    pub payload: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct Response {
    pub result: String,
}

openraft::declare_raft_types!(
    pub TypeConfig: D = Request, R = Response, NodeId = NodeId, Node = Node,
    Entry = openraft::Entry<TypeConfig>, SnapshotData = std::io::Cursor<Vec<u8>>
);
