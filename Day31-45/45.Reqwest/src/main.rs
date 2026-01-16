use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
struct IpResponse {
    origin: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct PostResponse {
    // data field contains the request body data
    data: String,
    // json field contains the parsed JSON if content-type was application/json
    json: Option<serde_json::Value>,
    url: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("1. Sending GET request to https://httpbin.org/ip");
    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<IpResponse>()
        .await?;

    println!("   > Success! IP: {}", resp.origin);

    println!("\n2. Sending POST request to https://httpbin.org/post");
    let client = reqwest::Client::new();
    let payload = HashMap::from([
        ("lang", "rust"),
        ("day", "45"),
    ]);

    let resp = client.post("https://httpbin.org/post")
        .json(&payload)
        .send()
        .await?
        .json::<PostResponse>()
        .await?;

    println!("   > Success! URL: {}", resp.url);
    if let Some(json) = resp.json {
        println!("   > Returned JSON: {}", json);
    }

    Ok(())
}
