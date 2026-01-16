use sentinel_demo::{server, Client, SentinelService};
use tokio::sync::broadcast;
use tokio::time::{sleep, Duration};
use bytes::Bytes;
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Only show info logs to keep output clean
    // tracing_subscriber::fmt::init();

    // Channels to control server shutdowns
    let (shutdown_master_tx, shutdown_master_rx) = broadcast::channel(1);
    let (shutdown_replica_tx, shutdown_replica_rx) = broadcast::channel(1);

    // 1. Start Master (6379)
    tokio::spawn(async move {
        if let Err(e) = server::run("127.0.0.1:6379", "master.aof", shutdown_master_rx).await {
            eprintln!("Master server error: {:?}", e);
        }
    });

    // 2. Start Replica (6380)
    tokio::spawn(async move {
        if let Err(e) = server::run("127.0.0.1:6380", "replica.aof", shutdown_replica_rx).await {
            eprintln!("Replica server error: {:?}", e);
        }
    });

    sleep(Duration::from_millis(500)).await;
    println!(">>> Servers started: Master(6379), Replica(6380)");

    // 3. Start Sentinel
    let sentinel = Arc::new(Mutex::new(SentinelService::new(
        "127.0.0.1:6379".to_string(),
        vec!["127.0.0.1:6380".to_string()],
    )));

    let sentinel_clone = sentinel.clone();
    tokio::spawn(async move {
        SentinelService::start(sentinel_clone, Duration::from_millis(500)).await;
    });

    println!(">>> Sentinel started monitoring.");

    // 4. Client connects via Sentinel
    {
        let sentinel_lock = sentinel.lock().await;
        let master_addr = sentinel_lock.get_master_addr().await;
        println!(">>> Client querying Sentinel... Current Master: {}", master_addr);

        let mut client = Client::connect(&master_addr).await?;
        client.set("status", Bytes::from("ok")).await?;
        println!(">>> Client wrote 'status'='ok' to {}", master_addr);
    }

    // 5. Kill Master
    println!("\n>>> SIMULATING FAILURE: Shutting down Master(6379)...");
    shutdown_master_tx.send(())?;

    // Wait for failover detection (Sentinel interval is 500ms)
    sleep(Duration::from_secs(2)).await;

    // 6. Client reconnects via Sentinel
    {
        let sentinel_lock = sentinel.lock().await;
        let master_addr = sentinel_lock.get_master_addr().await;
        println!(">>> Client querying Sentinel... New Master: {}", master_addr);

        if master_addr == "127.0.0.1:6380" {
            println!(">>> SUCCESS: Failover occurred correctly.");
        } else {
            println!(">>> FAILURE: Failover did not occur. Master is {}", master_addr);
        }

        // Verify we can talk to new master
        match Client::connect(&master_addr).await {
            Ok(mut client) => {
                let pong = client.ping(None).await?;
                println!(">>> Ping new master: {:?}", pong);
            }
            Err(e) => println!(">>> Could not connect to new master: {:?}", e),
        }
    }

    // Cleanup
    let _ = shutdown_replica_tx.send(());
    let _ = std::fs::remove_file("master.aof");
    let _ = std::fs::remove_file("replica.aof");

    Ok(())
}
