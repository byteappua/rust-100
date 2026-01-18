/*
File Structure:
.
├── Cargo.toml (Workspace Root)
├── crates
│   ├── my-core
│   │   ├── Cargo.toml
│   │   └── src/lib.rs
│   ├── my-api
│   │   ├── Cargo.toml
│   │   └── src/main.rs
│   └── my-cli
│       ├── Cargo.toml
│       └── src/main.rs
*/

/*
// Root Cargo.toml
[workspace]
members = [
    "crates/my-core",
    "crates/my-api",
    "crates/my-cli",
]

resolver = "2"
*/

/*
// crates/my-api/Cargo.toml
[package]
name = "my-api"
version = "0.1.0"
edition = "2021"

[dependencies]
my-core = { path = "../my-core" }
axum = "0.7"
tokio = { version = "1", features = ["full"] }
*/

fn main() {
    println!("This is a design exercise. See the comments for the solution structure.");
}
