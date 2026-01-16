# Project Presentation: Distributed Task Scheduler (DTask)

## 1. Project Overview
DTask is a distributed task scheduling system built with Rust. It demonstrates a microservices architecture using gRPC for internal communication and a RESTful API for external interaction. The system is designed to handle task submission, persistent storage, and background processing.

## 2. Technical Stack
- **Language**: Rust 2021 Edition
- **Web Framework**: Axum 0.7
- **RPC Framework**: Tonic 0.12 (gRPC)
- **Database**: SQLite (via SQLx 0.7)
- **Async Runtime**: Tokio
- **Serialization**: Serde & Prost
- **Authentication**: JWT (jsonwebtoken)
- **Monitoring**: Prometheus Metrics & Tracing

## 3. Architecture

```mermaid
graph TD
    Client[Client (HTTP)] -->|REST API| API[API Gateway (Axum)]
    API -->|SQLx| DB[(SQLite DB)]
    API -->|Metrics| Prom[Prometheus]

    subgraph Core System
        Dispatcher[Task Dispatcher] -->|Poll| DB
        Scheduler[Task Scheduler (gRPC)] -->|Manage| DB
    end
```

## 4. Core Features
- **Task Submission**: Users can submit tasks via a secure REST API.
- **Task Processing**: A background dispatcher polls for pending tasks and executes them.
- **Persistence**: All tasks are stored in a SQLite database with status tracking (Pending, Running, Completed, Failed).
- **Authentication**: JWT-based protection for sensitive endpoints.
- **Observability**: Integrated Prometheus metrics and structured logging.

## 5. Performance Metrics
A load test was conducted with the following parameters:
- **Concurrent Users**: 50
- **Total Requests**: 1000
- **Endpoint**: `POST /api/tasks`

**Results:**
- **QPS (Requests Per Second)**: ~611
- **Latency**: P99 < 50ms (for successful requests)
- **Observations**: Under high concurrency, SQLite exhibited locking contention (`database is locked` / `attempt to write a readonly database` errors), indicating a bottleneck in the storage layer for write-heavy workloads.

## 6. Challenges & Solutions
- **Challenge**: Database Concurrency with SQLite.
  - *Symptom*: Frequent locking errors during load testing.
  - *Solution Attempted*: Configured connection pooling and busy timeouts.
  - *Future Fix*: Enable WAL (Write-Ahead Logging) mode or migrate to PostgreSQL for better concurrent write performance.
- **Challenge**: gRPC & HTTP Integration.
  - *Solution*: Running both servers concurrently within the same Tokio runtime using `tokio::spawn`.

## 7. Future Improvements
- **Storage**: Migrate to PostgreSQL to handle higher write concurrency.
- **Consensus**: Fully integrate OpenRaft to support a truly distributed cluster with leader election.
- **Resilience**: Implement retries and dead-letter queues for failed tasks.
- **Frontend**: Develop a React/WebAssembly dashboard for task monitoring.
