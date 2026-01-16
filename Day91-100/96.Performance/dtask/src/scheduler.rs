use crate::dtask::{scheduler_server::Scheduler, SubmitTaskRequest, SubmitTaskResponse};
use crate::model::{Task, TaskStatus};
use chrono::Utc;
use sqlx::SqlitePool;
use tonic::{Request, Response, Status};
use uuid::Uuid;

pub struct TaskScheduler {
    pool: SqlitePool,
}

impl TaskScheduler {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
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

        sqlx::query("INSERT INTO tasks (id, payload, created_at, status) VALUES (?, ?, ?, ?)")
            .bind(&task.id)
            .bind(&task.payload)
            .bind(task.created_at)
            .bind(task.status)
            .execute(&self.pool)
            .await
            .map_err(|e| {
                tracing::error!("Failed to insert task: {}", e);
                Status::internal("Failed to insert task")
            })?;

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
    use sqlx::sqlite::SqlitePoolOptions;

    #[tokio::test]
    async fn test_submit_task() {
        let pool = SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .unwrap();

        sqlx::query("CREATE TABLE tasks (id TEXT PRIMARY KEY, payload TEXT NOT NULL, created_at INTEGER NOT NULL, status INTEGER NOT NULL)")
            .execute(&pool)
            .await
            .unwrap();

        let scheduler = TaskScheduler::new(pool.clone());

        let request = Request::new(SubmitTaskRequest {
            payload: "test".to_string(),
        });

        let response = scheduler.submit_task(request).await.unwrap();
        assert!(response.into_inner().success);

        let count: (i64,) = sqlx::query_as("SELECT count(*) FROM tasks")
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(count.0, 1);
    }
}
