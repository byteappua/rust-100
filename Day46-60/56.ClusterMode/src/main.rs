use cluster_mode::server;
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    // Default configuration
    let addr = "127.0.0.1:6379";
    let aof_path = "appendonly.aof";

    info!("Starting server via main.rs wrapper");
    server::run(addr, aof_path).await
}
