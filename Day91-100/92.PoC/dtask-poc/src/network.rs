use crate::TypeConfig;
use openraft::error::InstallSnapshotError;
use openraft::error::NetworkError;
use openraft::error::RPCError;
use openraft::error::RaftError;
use openraft::network::RaftNetwork;
use openraft::network::RaftNetworkFactory;
use openraft::network::RPCOption;
use openraft::raft::AppendEntriesRequest;
use openraft::raft::AppendEntriesResponse;
use openraft::raft::InstallSnapshotRequest;
use openraft::raft::InstallSnapshotResponse;
use openraft::raft::VoteRequest;
use openraft::raft::VoteResponse;
use openraft::BasicNode;

pub struct MyNetwork {}

impl RaftNetworkFactory<TypeConfig> for MyNetwork {
    type Network = MyNetworkConnection;

    async fn new_client(&mut self, _target: u64, _node: &BasicNode) -> Self::Network {
        MyNetworkConnection {}
    }
}

pub struct MyNetworkConnection {}

impl RaftNetwork<TypeConfig> for MyNetworkConnection {
    async fn append_entries(
        &mut self,
        _rpc: AppendEntriesRequest<TypeConfig>,
        _option: RPCOption,
    ) -> Result<AppendEntriesResponse<u64>, RPCError<u64, BasicNode, RaftError<u64>>> {
        Err(RPCError::Network(NetworkError::new(&std::io::Error::new(std::io::ErrorKind::Other, "Unimplemented"))))
    }

    async fn install_snapshot(
        &mut self,
        _rpc: InstallSnapshotRequest<TypeConfig>,
        _option: RPCOption,
    ) -> Result<InstallSnapshotResponse<u64>, RPCError<u64, BasicNode, RaftError<u64, InstallSnapshotError>>> {
        Err(RPCError::Network(NetworkError::new(&std::io::Error::new(std::io::ErrorKind::Other, "Unimplemented"))))
    }

    async fn vote(
        &mut self,
        _rpc: VoteRequest<u64>,
        _option: RPCOption,
    ) -> Result<VoteResponse<u64>, RPCError<u64, BasicNode, RaftError<u64>>> {
        Err(RPCError::Network(NetworkError::new(&std::io::Error::new(std::io::ErrorKind::Other, "Unimplemented"))))
    }
}
