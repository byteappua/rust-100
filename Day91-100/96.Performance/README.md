# Day 96: Performance Testing and Tuning

This directory contains the performance-optimized version of the `dtask` project.

## Changes
- **Database**: Migrated from in-memory `Vec<Task>` to `SQLite` using `sqlx`.
- **Schema**: Added `tasks` table and an index on `status` for optimization.
- **Connection Pool**: Configured `SqlitePool` with max connections.
- **Build Profile**: Added `release` profile with `LTO` and `strip` enabled.

## How to Run

1. **Start the Server**:
   ```bash
   cd dtask
   cargo run --release
   ```

2. **Run Load Test**:
   ```bash
   cd dtask
   cargo run --example load_test --release
   ```

## Optimization Checklist
- [x] Database Query Optimization (Index on `status` added in migration `20240101000001_add_index.sql`)
- [x] Connection Pool Tuning (`max_connections(50)`)
- [x] Compile Optimization (`lto = true`, `strip = true`)
- [x] Async I/O (Using `tokio` and `sqlx` async pool)

## Benchmarking Results
(Run the load test to see QPS)
