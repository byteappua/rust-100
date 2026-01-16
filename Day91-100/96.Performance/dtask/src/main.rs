use dtask::api;
use dtask::dispatcher::Dispatcher;
use dtask::dtask::scheduler_server::SchedulerServer;
use dtask::scheduler::TaskScheduler;
use sqlx::sqlite::SqlitePoolOptions;
use std::path::Path;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let grpc_addr = "[::1]:50051".parse()?;
    let api_addr = "0.0.0.0:3000";

    // Setup Database
    let db_path = "dtask.db";
    if !Path::new(db_path).exists() {
        std::fs::File::create(db_path)?;
    }
    let db_url = format!("sqlite:{}", db_path);

    let pool = SqlitePoolOptions::new()
        .max_connections(50)
        .connect(&db_url)
        .await?;

    // Run migrations
    sqlx::migrate!("./migrations").run(&pool).await?;

    let scheduler = TaskScheduler::new(pool.clone());
    let dispatcher = Dispatcher::new(pool.clone());

    // API Router
    let app = api::app(pool.clone());

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
