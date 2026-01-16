use std::time::Duration;
use std::thread;
use std::hint::black_box;

fn expensive_computation() {
    let mut sum = 0u64;
    for i in 0..10_000_000 {
        sum = sum.wrapping_add(i);
    }
    // Prevent compiler from optimizing away the loop using black_box
    black_box(sum);
}

fn io_operation() {
    thread::sleep(Duration::from_millis(100));
}

fn process_data() {
    for _ in 0..10 {
        expensive_computation();
    }
}

fn main() {
    println!("Starting performance test...");

    for _ in 0..5 {
        process_data();
        io_operation();
    }

    println!("Performance test completed");
}
