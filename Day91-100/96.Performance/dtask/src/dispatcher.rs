use crate::model::{Task, TaskStatus};
use sqlx::SqlitePool;
use tokio::time::{sleep, Duration};

pub struct Dispatcher {
    pool: SqlitePool,
}

impl Dispatcher {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn run(&self) {
        tracing::info!("Dispatcher started");
        loop {
            let result: Option<Task> = sqlx::query_as(
                r#"
                UPDATE tasks
                SET status = ?
                WHERE id = (SELECT id FROM tasks WHERE status = ? LIMIT 1)
                RETURNING id, payload, created_at, status
                "#,
            )
            .bind(TaskStatus::Running)
            .bind(TaskStatus::Pending)
            .fetch_optional(&self.pool)
            .await
            .unwrap_or_else(|e| {
                tracing::error!("Failed to fetch task: {}", e);
                None
            });

            if let Some(task) = result {
                tracing::info!("Dispatching task: {}", task.id);

                // Simulate execution
                sleep(Duration::from_millis(500)).await;
                tracing::info!("Task {} completed", task.id);

                // Update status to completed
                let _ = sqlx::query("UPDATE tasks SET status = ? WHERE id = ?")
                    .bind(TaskStatus::Completed)
                    .bind(task.id)
                    .execute(&self.pool)
                    .await
                    .map_err(|e| tracing::error!("Failed to complete task: {}", e));
            } else {
                sleep(Duration::from_millis(100)).await;
            }
        }
    }
}
