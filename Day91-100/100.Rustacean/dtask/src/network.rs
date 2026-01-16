use async_trait::async_trait;
use openraft::error::{InstallSnapshotError, NetworkError, RPCError, RaftError};
use openraft::raft::{AppendEntriesRequest, AppendEntriesResponse, InstallSnapshotRequest, InstallSnapshotResponse, VoteRequest, VoteResponse};
use openraft::{RaftNetwork, RaftNetworkFactory};
use crate::raft::{Node, NodeId, TypeConfig};
use reqwest::Client;

pub struct DTaskNetwork {}

impl Default for DTaskNetwork {
    fn default() -> Self {
        Self::new()
    }
}

impl DTaskNetwork {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl RaftNetworkFactory<TypeConfig> for DTaskNetwork {
    type Network = DTaskNetworkConnection;

    async fn new_client(&mut self, _target: NodeId, node: &Node) -> Self::Network {
        DTaskNetworkConnection {
            target_node: node.clone(),
            client: Client::new(),
        }
    }
}

pub struct DTaskNetworkConnection {
    target_node: Node,
    client: Client,
}

impl DTaskNetworkConnection {
    async fn send_rpc<Req, Resp, Err>(&self, uri: &str, req: Req) -> Result<Resp, RPCError<NodeId, Node, Err>>
    where
        Req: serde::Serialize,
        Resp: serde::de::DeserializeOwned,
        Err: std::error::Error + serde::de::DeserializeOwned,
    {
        let url = format!("http://{}/raft/{}", self.target_node.rpc_addr, uri);
        let resp = self.client.post(&url).json(&req).send().await.map_err(|e| {
            RPCError::Network(NetworkError::new(&e))
        })?;

        if !resp.status().is_success() {
             let err_msg = format!("Error response: {}", resp.status());
             let io_err = std::io::Error::other(err_msg);
             return Err(RPCError::Network(NetworkError::new(&io_err)));
        }

        let res: Resp = resp.json().await.map_err(|e| {
            RPCError::Network(NetworkError::new(&e))
        })?;

        Ok(res)
    }
}

#[async_trait]
impl RaftNetwork<TypeConfig> for DTaskNetworkConnection {
    async fn send_append_entries(
        &mut self,
        rpc: AppendEntriesRequest<TypeConfig>,
    ) -> Result<AppendEntriesResponse<NodeId>, RPCError<NodeId, Node, RaftError<NodeId>>> {
        self.send_rpc("append", rpc).await
    }

    async fn send_install_snapshot(
        &mut self,
        rpc: InstallSnapshotRequest<TypeConfig>,
    ) -> Result<InstallSnapshotResponse<NodeId>, RPCError<NodeId, Node, RaftError<NodeId, InstallSnapshotError>>> {
        self.send_rpc("snapshot", rpc).await
    }

    async fn send_vote(
        &mut self,
        rpc: VoteRequest<NodeId>,
    ) -> Result<VoteResponse<NodeId>, RPCError<NodeId, Node, RaftError<NodeId>>> {
        self.send_rpc("vote", rpc).await
    }
}
