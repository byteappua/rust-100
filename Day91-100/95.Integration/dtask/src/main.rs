use dtask::dtask::scheduler_server::SchedulerServer;
use dtask::scheduler::TaskScheduler;
use dtask::dispatcher::Dispatcher;
use dtask::model::Task;
use dtask::api;
use tonic::transport::Server;
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let grpc_addr = "[::1]:50051".parse()?;
    let api_addr = "0.0.0.0:3000";

    // Shared state
    let tasks = Arc::new(Mutex::new(Vec::<Task>::new()));

    let scheduler = TaskScheduler::new(tasks.clone());
    let dispatcher = Dispatcher::new(tasks.clone());

    // API Router
    let app = api::app(tasks.clone());

    tracing::info!("Starting Task Scheduler (gRPC) on {}", grpc_addr);
    tracing::info!("Starting Web API on {}", api_addr);

    // Spawn dispatcher
    tokio::spawn(async move {
        dispatcher.run().await;
    });

    // Spawn API
    tokio::spawn(async move {
        let listener = tokio::net::TcpListener::bind(api_addr).await.unwrap();
        axum::serve(listener, app).await.unwrap();
    });

    Server::builder()
        .add_service(SchedulerServer::new(scheduler))
        .serve(grpc_addr)
        .await?;

    Ok(())
}
