use sentinel_demo::server;
use tracing::info;
use tokio::sync::broadcast;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    // Default configuration
    let addr = "127.0.0.1:6379";
    let aof_path = "appendonly.aof";

    info!("Starting server via main.rs wrapper");

    let (tx, rx) = broadcast::channel(1);

    // Spawn a task to listen for Ctrl+C
    tokio::spawn(async move {
        if let Ok(()) = tokio::signal::ctrl_c().await {
            info!("Ctrl+C received, shutting down");
            let _ = tx.send(());
        }
    });

    server::run(addr, aof_path, rx).await
}
