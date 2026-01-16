use cluster_mode::{server, ClusterClient};
use bytes::Bytes;
use tokio::time::Duration;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing to see server logs, but maybe quiet it down for the demo output clarity?
    // tracing_subscriber::fmt::init();
    // Let's skip tracing init to keep output clean, or use a specific filter.
    // For now, let's keep it simple.

    // 1. Start 3 mini-redis instances on different ports
    let nodes = vec![
        "127.0.0.1:9001".to_string(),
        "127.0.0.1:9002".to_string(),
        "127.0.0.1:9003".to_string(),
    ];

    for (i, addr) in nodes.iter().enumerate() {
        let addr = addr.clone();
        // Use different AOF files to avoid conflict
        let aof_path = format!("node_{}.aof", i + 1);
        tokio::spawn(async move {
            // Suppress logs for servers to avoid cluttering demo output
            if let Err(e) = server::run(&addr, &aof_path).await {
                eprintln!("Server {} error: {:?}", addr, e);
            }
        });
    }

    // Give servers time to start
    tokio::time::sleep(Duration::from_millis(500)).await;

    println!(">>> Cluster started with nodes: {:?}", nodes);

    // 2. Initialize Cluster Client
    let mut client = ClusterClient::new(nodes.clone());

    // 3. Set some keys
    let keys = vec!["apple", "banana", "cherry", "date", "elderberry", "fig", "grape"];

    println!("\n>>> Writing keys...");
    for key in &keys {
        let val = Bytes::from(format!("value-{}", key));
        let target = client.get_target_node(key);
        println!("SET {} -> {} (Target: {})", key, String::from_utf8_lossy(&val), target);

        client.set(key, val).await?;
    }

    // 4. Read back keys
    println!("\n>>> Reading keys...");
    for key in &keys {
        let val = client.get(key).await?;
        let target = client.get_target_node(key);
        match val {
            Some(v) => println!("GET {} -> {:?} (From: {})", key, v, target),
            None => println!("GET {} -> None (From: {})", key, target),
        }
    }

    println!("\n>>> Demo completed successfully.");

    // Clean up AOF files
    for i in 1..=3 {
        let _ = std::fs::remove_file(format!("node_{}.aof", i));
    }

    Ok(())
}
