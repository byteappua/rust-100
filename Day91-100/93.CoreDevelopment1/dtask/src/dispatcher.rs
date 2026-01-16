use crate::models::Task;
use tokio::sync::mpsc;
use tracing::info;

pub struct Dispatcher {
    rx: mpsc::Receiver<Task>,
}

impl Dispatcher {
    pub fn new(rx: mpsc::Receiver<Task>) -> Self {
        Self { rx }
    }

    pub async fn run(mut self) {
        info!("Dispatcher started");
        while let Some(task) = self.rx.recv().await {
            info!("Executing task: {} ({})", task.name, task.id);
            // Simulate execution
            tokio::spawn(async move {
                tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                info!("Task completed: {}", task.id);
            });
        }
    }
}
