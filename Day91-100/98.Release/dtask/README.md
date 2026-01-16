# Distributed Task Scheduler (DTask)

DTask is a distributed task scheduling system built with Rust, Tokio, gRPC, and OpenRaft.

## Features
- **Distributed Scheduling**: Tasks are distributed across nodes.
- **gRPC API**: High-performance communication for task submission.
- **REST API**: Web interface for management (Axum).
- **Persistence**: SQLite storage with potential for Raft consensus.
- **Observability**: Prometheus metrics and Tracing.

## Getting Started

### Prerequisites
- Rust 1.75+
- Docker (optional)

### Running Locally
```bash
# Start the server
cargo run --release
```

The gRPC server listens on `0.0.0.0:50051`.
The Web API listens on `0.0.0.0:3000`.

### Running with Docker
```bash
docker build -t dtask .
docker run -p 3000:3000 -p 50051:50051 dtask
```

## API

- **POST /api/tasks**: Submit a task.
- **GET /api/tasks/:id**: Get task status.

## License
MIT
