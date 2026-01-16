use reqwest::Client;
use serde_json::json;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::Semaphore;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let base_url = "http://localhost:3000";

    // Login first
    let login_url = format!("{}/login", base_url);
    println!("Logging in to {}...", login_url);

    let login_res = client
        .post(&login_url)
        .json(&json!({
            "username": "admin",
            "password": "password"
        }))
        .send()
        .await?;

    if !login_res.status().is_success() {
        eprintln!("Login failed: {}", login_res.status());
        return Ok(());
    }

    let login_body: serde_json::Value = login_res.json().await?;
    let token = login_body["token"]
        .as_str()
        .ok_or("No token in response")?
        .to_string();

    println!("Got token. Starting load test...");

    let total_requests = 1000;
    let concurrency = 50;
    let semaphore = Arc::new(Semaphore::new(concurrency));
    let url = format!("{}/api/tasks", base_url);

    let start = Instant::now();
    let mut handles = vec![];

    for i in 0..total_requests {
        let permit = semaphore.clone().acquire_owned().await.unwrap();
        let client = client.clone();
        let url = url.clone();
        let token = token.clone();

        handles.push(tokio::spawn(async move {
            let _permit = permit;
            let res = client
                .post(&url)
                .header("Authorization", format!("Bearer {}", token))
                .json(&json!({ "payload": format!("task {}", i) }))
                .send()
                .await;

            match res {
                Ok(r) => {
                    if !r.status().is_success() {
                        eprintln!("Request failed: {}", r.status());
                    }
                }
                Err(e) => eprintln!("Request error: {}", e),
            }
        }));
    }

    for h in handles {
        h.await?;
    }

    let duration = start.elapsed();
    println!("Total requests: {}", total_requests);
    println!("Total time: {:?}", duration);
    println!("QPS: {:.2}", total_requests as f64 / duration.as_secs_f64());

    Ok(())
}
