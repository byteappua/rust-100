use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    info!("Mini-Redis server is starting...");
    info!("Ready to accept connections (Simulated)");

    // Simulate some async work
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    info!("Mini-Redis server initialization complete.");

    Ok(())
}
