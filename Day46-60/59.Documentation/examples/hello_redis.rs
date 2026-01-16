//! # Hello Redis Example
//!
//! This example demonstrates a complete walkthrough of using the Mini-Redis client.
//! It assumes a server is running on localhost:6379.

use mini_redis_doc::Client;
use bytes::Bytes;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Attempt to connect to the server
    println!("Connecting to Mini-Redis server...");
    let mut client = match Client::connect("127.0.0.1:6379").await {
        Ok(client) => client,
        Err(e) => {
            eprintln!("Failed to connect: {}. Is the server running?", e);
            return Ok(());
        }
    };
    println!("Connected!");

    // Perform database operations
    println!("\n--- performing database operations ---");

    let key = "foo";
    let value = "bar";

    // Set
    client.set(key, Bytes::from(value)).await?;
    println!("SET {} -> {}", key, value);

    // Get
    let result = client.get(key).await?;
    println!("GET {} -> {:?}", key, result);

    // Publish (if supported by server version)
    // Note: To verify Pub/Sub, you would need another client subscribing.
    println!("\n--- performing pub/sub ---");
    let channel = "notifications";
    let message = "hello subscribers";
    let count = client.publish(channel, Bytes::from(message)).await?;
    println!("PUBLISH {} -> {} (received by {} clients)", channel, message, count);

    // Sleep briefly to simulate work
    sleep(Duration::from_millis(100)).await;

    println!("\nDone.");
    Ok(())
}
