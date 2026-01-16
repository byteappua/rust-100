use crate::models::{Task, TaskStatus};
use crate::store::MemStorage;
use tokio::sync::mpsc;
use tracing::{info};

pub struct TaskScheduler {
    storage: MemStorage,
    dispatch_tx: mpsc::Sender<Task>,
}

impl TaskScheduler {
    pub fn new(storage: MemStorage, dispatch_tx: mpsc::Sender<Task>) -> Self {
        Self {
            storage,
            dispatch_tx,
        }
    }

    pub async fn run(&self) {
        info!("TaskScheduler started");
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

            let pending_tasks = self.storage.get_pending_tasks().await;
            if !pending_tasks.is_empty() {
                info!("Found {} pending tasks", pending_tasks.len());
                for task in pending_tasks {
                    // Send to dispatcher
                    if let Err(e) = self.dispatch_tx.send(task.clone()).await {
                        info!("Failed to dispatch task {}: {:?}", task.id, e);
                    } else {
                        info!("Dispatched task {}", task.id);
                        // In a real app, we would update status to Running here via Raft
                    }
                }
            }
        }
    }
}
