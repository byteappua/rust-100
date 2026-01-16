use std::sync::{Arc, Mutex};
use tokio::time::{sleep, Duration};
use crate::model::{Task, TaskStatus};

pub struct Dispatcher {
    tasks: Arc<Mutex<Vec<Task>>>,
}

impl Dispatcher {
    pub fn new(tasks: Arc<Mutex<Vec<Task>>>) -> Self {
        Self { tasks }
    }

    pub async fn run(&self) {
        tracing::info!("Dispatcher started");
        loop {
            let task_id = {
                let tasks = self.tasks.lock().unwrap();
                tasks.iter().find(|t| t.status == TaskStatus::Pending).map(|t| t.id.clone())
            };

            if let Some(id) = task_id {
                tracing::info!("Dispatching task: {}", id);

                // Update status to running
                {
                    let mut tasks = self.tasks.lock().unwrap();
                    if let Some(t) = tasks.iter_mut().find(|t| t.id == id) {
                        t.status = TaskStatus::Running;
                    }
                }

                // Simulate execution
                sleep(Duration::from_millis(500)).await;
                tracing::info!("Task {} completed", id);

                // Update status to completed
                {
                    let mut tasks = self.tasks.lock().unwrap();
                    if let Some(t) = tasks.iter_mut().find(|t| t.id == id) {
                        t.status = TaskStatus::Completed;
                    }
                }
            } else {
                sleep(Duration::from_millis(100)).await;
            }
        }
    }
}
