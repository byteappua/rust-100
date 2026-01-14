use std::thread;
use std::time::Duration;

fn main() {
    // 1. Basic Thread Spawning
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // Wait for the spawned thread to finish
    handle.join().unwrap();

    println!("---------------------------------");

    // 2. Using move Closures with Threads
    let v = vec![1, 2, 3];

    // We must use 'move' to transfer ownership of 'v' into the closure
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}
