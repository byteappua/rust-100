# Technical Review & Retrospective: 100 Days of Rust

## Journey Summary
This project marks the culmination of a 100-day journey into the Rust ecosystem. From understanding the basics of ownership and borrowing to building a distributed system with gRPC and async runtimes, the learning curve was steep but rewarding.

## Lessons Learned

### 1. The Power of the Type System
Rust's type system, especially the borrow checker, enforces correctness at compile time. While initially frustrating, it prevents entire classes of bugs (data races, null pointer dereferences) that plague other systems languages.

### 2. Async Programming with Tokio
Transitioning from synchronous to asynchronous programming required a shift in mental model. Understanding futures, pinning, and the role of the executor (Tokio) was crucial for building high-performance network services. The `async/await` syntax makes it readable, but the underlying mechanics (Send/Sync bounds, static lifetimes) require deep understanding.

### 3. Error Handling
The `Result<T, E>` pattern and the `?` operator encourage robust error handling. Libraries like `anyhow` and `thiserror` simplify managing error hierarchies in larger applications.

### 4. Ecosystem Maturity
The Rust ecosystem for web development (Axum), database access (SQLx), and serialization (Serde) is incredibly mature and developer-friendly. `Cargo` remains the gold standard for package management and build tooling.

## What Went Well
- **Code Quality**: The codebase is strongly typed, documented, and formatted. `clippy` was instrumental in maintaining style and catching common mistakes.
- **Architecture**: Separating the API, Scheduler, and Dispatcher allowed for independent development and testing.
- **Tooling**: Using `cargo-watch` and `sqlx-cli` streamlined the development workflow.

## Areas for Improvement
- **Testing**: While unit tests cover the basics, more integration tests simulating real-world distributed scenarios (e.g., node failure) would be beneficial.
- **Configuration Management**: Hardcoded values (ports, secrets) should be moved to a robust configuration management system (e.g., `config` crate or env vars).
- **Database Choice**: For a "distributed" system, relying on a single SQLite instance is a bottleneck. Using a distributed KV store or PostgreSQL would better align with the project goals.

## Final Thoughts
Rust is an excellent choice for systems programming and backend development. The strictness of the compiler pays off in the long run with maintainable and reliable software. I look forward to continuing the journey by exploring more advanced topics like eBPF, embedded Rust, and WASM.
