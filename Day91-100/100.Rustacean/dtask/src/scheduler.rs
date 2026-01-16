use crate::dtask::{scheduler_server::Scheduler, SubmitTaskRequest, SubmitTaskResponse};
use crate::raft::{DTaskRaft, Request as RaftRequest};
use tonic::{Request, Response, Status};

pub struct TaskScheduler {
    raft: DTaskRaft,
}

impl TaskScheduler {
    pub fn new(raft: DTaskRaft) -> Self {
        Self { raft }
    }
}

#[tonic::async_trait]
impl Scheduler for TaskScheduler {
    async fn submit_task(
        &self,
        request: Request<SubmitTaskRequest>,
    ) -> Result<Response<SubmitTaskResponse>, Status> {
        let req = request.into_inner();
        let raft_req = RaftRequest { payload: req.payload };

        let result = self.raft.client_write(raft_req).await;

        match result {
            Ok(resp) => {
                 Ok(Response::new(SubmitTaskResponse {
                    id: resp.data.result,
                    success: true,
                    message: "Task submitted via Raft".to_string(),
                }))
            },
            Err(e) => {
                tracing::error!("Raft write error: {:?}", e);
                Err(Status::internal(format!("Raft error: {:?}", e)))
            }
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use sqlx::sqlite::SqlitePoolOptions;
//
//     #[tokio::test]
//     async fn test_submit_task() {
//         // Test disabled as it requires full Raft setup
//     }
// }
