use dtask::dtask::scheduler_server::SchedulerServer;
use dtask::scheduler::TaskScheduler;
use dtask::dispatcher::Dispatcher;
use dtask::model::Task;
use tonic::transport::Server;
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let addr = "[::1]:50051".parse()?;

    // Shared state
    let tasks = Arc::new(Mutex::new(Vec::<Task>::new()));

    let scheduler = TaskScheduler::new(tasks.clone());
    let dispatcher = Dispatcher::new(tasks.clone());

    tracing::info!("Starting Task Scheduler on {}", addr);

    // Spawn dispatcher
    tokio::spawn(async move {
        dispatcher.run().await;
    });

    Server::builder()
        .add_service(SchedulerServer::new(scheduler))
        .serve(addr)
        .await?;

    Ok(())
}
