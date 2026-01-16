use anyhow::{Context, Result};
use std::fs;
use thiserror::Error;

// 1. Library Error: Define a structured error type using `thiserror`
// This is best for libraries where callers need to match on specific error cases.
#[derive(Error, Debug)]
pub enum DataStoreError {
    #[error("data store disconnected")]
    Disconnect(#[from] std::io::Error),
    #[error("the data for key `{0}` is not available")]
    Redaction(String),
    #[error("invalid header (expected {expected:?}, found {found:?})")]
    InvalidHeader { expected: String, found: String },
    #[error("unknown data store error")]
    Unknown,
}

// A function that returns our library error
fn read_data_from_store(key: &str) -> std::result::Result<String, DataStoreError> {
    if key == "secret" {
        return Err(DataStoreError::Redaction(key.to_string()));
    }
    if key == "io_fail" {
        // Simulate IO error
        return Err(DataStoreError::Disconnect(std::io::Error::new(
            std::io::ErrorKind::BrokenPipe,
            "simulated",
        )));
    }
    Ok(format!("Data for {}", key))
}

// 2. Application Error: Use `anyhow` for the top-level application
// This is best for binaries where we just want to report errors with context.
fn process_data() -> Result<()> {
    let content = fs::read_to_string("non_existent_file.txt")
        .context("Failed to read configuration file")?; // Add context to standard errors

    println!("Config: {}", content);
    Ok(())
}

fn fetch_user_data(user_id: &str) -> Result<()> {
    let data = read_data_from_store(user_id)
        .context(format!("Failed to fetch data for user {}", user_id))?; // Convert generic error to anyhow

    println!("User Data: {}", data);
    Ok(())
}

fn main() {
    println!("--- Day 43: Error Handling Best Practices ---");

    // Example 1: Anyhow with IO error
    if let Err(e) = process_data() {
        println!("\n[Error Report 1]");
        println!("{:?}", e); // Debug print shows the chain of context
    }

    // Example 2: Anyhow with Custom Error
    if let Err(e) = fetch_user_data("secret") {
        println!("\n[Error Report 2]");
        println!("{:?}", e);
    }

    // Example 3: Success case
    if let Ok(_) = fetch_user_data("public_info") {
         println!("\n[Success]");
    }
}
