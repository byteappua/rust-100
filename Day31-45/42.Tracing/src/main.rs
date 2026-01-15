use tracing::{info, warn, error, instrument};

#[instrument]
fn expensive_work(input: i32) {
    info!("Starting expensive work with input: {}", input);
    // Simulate work
    std::thread::sleep(std::time::Duration::from_millis(100));
    if input < 0 {
        warn!("Input is negative, this might be odd");
    }
    info!("Expensive work finished");
}

fn main() {
    // 1. Initialize the subscriber
    // This will print structured logs to stdout
    tracing_subscriber::fmt::init();

    info!("--- Day 42: Tracing Demo ---");

    info!("Application started");

    let user_id = 42;
    // Spans represent a period of time
    let span = tracing::info_span!("user_session", user_id);
    let _enter = span.enter();

    info!("Processing user request");

    expensive_work(100);
    expensive_work(-5);

    error!("Something simulated went wrong!");

    // Exiting the scope of `_enter` closes the span
}
