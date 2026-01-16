use clap::Parser;
use dtask::api;
use dtask::dispatcher::Dispatcher;
use dtask::dtask::scheduler_server::SchedulerServer;
use dtask::network::DTaskNetwork;
use dtask::raft::DTaskRaft;
use dtask::scheduler::TaskScheduler;
use dtask::store::DTaskStore;
use openraft::Config;
use openraft::storage::Adaptor;
use sqlx::sqlite::SqlitePoolOptions;
use std::path::Path;
use std::sync::Arc;
use tonic::transport::Server;

#[derive(Parser, Clone, Debug)]
#[clap(author, version, about, long_about = None)]
struct Opt {
    #[clap(long, default_value = "1")]
    id: u64,

    #[clap(long, default_value = "0.0.0.0:3000")]
    http_addr: String,

    #[clap(long, default_value = "[::1]:50051")]
    grpc_addr: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    let opt = Opt::parse();

    let db_path = format!("dtask_{}.db", opt.id);
    let db_url = format!("sqlite:{}", db_path);

    if !Path::new(&db_path).exists() {
        std::fs::File::create(&db_path)?;
    }

    let pool = SqlitePoolOptions::new()
        .max_connections(50)
        .connect(&db_url)
        .await?;

    // Run migrations
    sqlx::migrate!("./migrations").run(&pool).await?;

    // Raft Setup
    let config = Config {
        heartbeat_interval: 500,
        election_timeout_min: 1500,
        election_timeout_max: 3000,
        ..Default::default()
    };

    let config = Arc::new(config);
    let store = DTaskStore::new(pool.clone());
    let (log_store, state_machine) = Adaptor::new(store);

    let network = DTaskNetwork::new();

    let raft = DTaskRaft::new(
        opt.id,
        config,
        network,
        log_store,
        state_machine,
    ).await?;

    let scheduler = TaskScheduler::new(raft.clone());
    let dispatcher = Dispatcher::new(pool.clone(), raft.clone());

    // API Router
    let app = api::app(pool.clone(), raft.clone());

    tracing::info!("Node ID: {}", opt.id);
    tracing::info!("Starting Web API on {}", opt.http_addr);
    tracing::info!("Starting Task Scheduler (gRPC) on {}", opt.grpc_addr);

    // Spawn dispatcher
    tokio::spawn(async move {
        dispatcher.run().await;
    });

    // Spawn API
    let api_addr = opt.http_addr.clone();
    tokio::spawn(async move {
        let listener = tokio::net::TcpListener::bind(api_addr).await.unwrap();
        axum::serve(listener, app).await.unwrap();
    });

    let grpc_addr = opt.grpc_addr.parse()?;
    Server::builder()
        .add_service(SchedulerServer::new(scheduler))
        .serve(grpc_addr)
        .await?;

    Ok(())
}
