use crate::dtask::{SubmitTaskRequest, SubmitTaskResponse, scheduler_server::Scheduler};
use crate::model::{Task, TaskStatus};
use tonic::{Request, Response, Status};
use std::sync::{Arc, Mutex};
use uuid::Uuid;
use chrono::Utc;

pub struct TaskScheduler {
    tasks: Arc<Mutex<Vec<Task>>>,
}

impl TaskScheduler {
    pub fn new(tasks: Arc<Mutex<Vec<Task>>>) -> Self {
        Self { tasks }
    }
}

#[tonic::async_trait]
impl Scheduler for TaskScheduler {
    async fn submit_task(
        &self,
        request: Request<SubmitTaskRequest>,
    ) -> Result<Response<SubmitTaskResponse>, Status> {
        let req = request.into_inner();
        let id = Uuid::new_v4().to_string();

        let task = Task {
            id: id.clone(),
            payload: req.payload,
            created_at: Utc::now().timestamp(),
            status: TaskStatus::Pending,
        };

        {
            let mut tasks = self.tasks.lock().unwrap();
            tasks.push(task);
        }

        tracing::info!("Task submitted: {}", id);

        Ok(Response::new(SubmitTaskResponse {
            id,
            success: true,
            message: "Task submitted successfully".to_string(),
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::TaskStatus;

    #[tokio::test]
    async fn test_submit_task() {
        let tasks = Arc::new(Mutex::new(Vec::new()));
        let scheduler = TaskScheduler::new(tasks.clone());

        let request = Request::new(SubmitTaskRequest {
            payload: "test".to_string(),
        });

        let response = scheduler.submit_task(request).await.unwrap();
        assert!(response.into_inner().success);

        let tasks = tasks.lock().unwrap();
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].payload, "test");
        assert_eq!(tasks[0].status, TaskStatus::Pending);
    }
}
