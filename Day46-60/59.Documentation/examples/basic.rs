use mini_redis_doc::Client;
use bytes::Bytes;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. Establish connection
    let mut client = Client::connect("127.0.0.1:6379").await?;

    // 2. Set a value
    let key = "hello";
    let val = "world";
    println!(">>> SET {} = {}", key, val);
    client.set(key, Bytes::from(val)).await?;

    // 3. Get the value
    println!(">>> GET {}", key);
    let result = client.get(key).await?;
    println!("<<< Got: {:?}", result);

    assert_eq!(result, Some(Bytes::from(val)));

    Ok(())
}
