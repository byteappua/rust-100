use mini_redis_tls::Client;
use bytes::Bytes;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Start the connection
    let mut client = Client::connect("127.0.0.1:6379").await?;

    // Set a key
    client.set("hello", Bytes::from("world")).await?;

    // Get the key
    let result = client.get("hello").await?;

    println!("got value from server: {:?}", result);
    assert_eq!(result, Some(Bytes::from("world")));

    // Publish
    let count = client.publish("foo", Bytes::from("bar")).await?;
    println!("Published to {} subscribers", count);

    Ok(())
}
