pub mod model;
pub mod scheduler;
pub mod dispatcher;
pub mod raft;

pub mod dtask {
    tonic::include_proto!("dtask");
}
