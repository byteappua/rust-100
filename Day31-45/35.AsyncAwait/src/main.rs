use std::time::Duration;
use tokio::time::sleep;

// 1. Async Function Definition
// An async function returns a Future.
async fn say_hello() {
    println!("Hello from async!");
    sleep(Duration::from_millis(100)).await; // 2. .await
    println!("Hello again!");
}

async fn get_number() -> u32 {
    sleep(Duration::from_millis(50)).await;
    42
}

// 3. Tokio Runtime
#[tokio::main]
async fn main() {
    println!("Starting async main...");

    // Call an async function and await it
    say_hello().await;

    // Concurrent Execution (join!)
    let f1 = say_hello();
    let f2 = get_number();

    // join! waits for both to complete
    let (_, number) = tokio::join!(f1, f2);

    println!("Got number: {}", number);
}
