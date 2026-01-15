use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};

// A CPU-intensive task simulation
fn expensive_calculation() -> u64 {
    std::thread::sleep(Duration::from_millis(500));
    42
}

#[tokio::main]
async fn main() {
    println!("--- Day 39: Tokio Deep Dive ---");

    // 1. Spawning Tasks (Lightweight Threads)
    let task_one = tokio::spawn(async {
        sleep(Duration::from_millis(100)).await;
        println!("Task 1 completed");
        "Result 1"
    });

    let task_two = tokio::spawn(async {
        println!("Task 2 completed immediately");
        "Result 2"
    });

    let (res1, res2) = tokio::join!(task_one, task_two);
    println!("Joined Results: {:?}, {:?}", res1.unwrap(), res2.unwrap());

    // 2. Channels (MPSC: Multi-Producer, Single-Consumer)
    let (tx, mut rx) = mpsc::channel(32);
    let tx2 = tx.clone();

    tokio::spawn(async move {
        tx.send("Msg from Sender 1").await.unwrap();
    });

    tokio::spawn(async move {
        tx2.send("Msg from Sender 2").await.unwrap();
    });

    // Receive messages
    // We expect 2 messages. In a real app we might loop while let Some(msg) = rx.recv().await
    println!("Received: {}", rx.recv().await.unwrap());
    println!("Received: {}", rx.recv().await.unwrap());

    // 3. Select! (Waiting for the first branch to complete)
    let branch1 = sleep(Duration::from_millis(200));
    let branch2 = sleep(Duration::from_millis(100));

    tokio::select! {
        _ = branch1 => println!("Branch 1 finished first"),
        _ = branch2 => println!("Branch 2 finished first"),
    }

    // 4. Blocking Tasks (Bridging Sync and Async)
    // NEVER run blocking code (std::thread::sleep, heavy CPU) directly in async fn!
    // Use spawn_blocking to offload it to a dedicated thread pool.
    println!("Starting blocking task...");
    let blocking_result = tokio::task::spawn_blocking(|| {
        expensive_calculation()
    }).await.unwrap();
    println!("Blocking task finished with: {}", blocking_result);
}
