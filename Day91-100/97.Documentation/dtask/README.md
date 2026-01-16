# DTask - Distributed Task Scheduling System

## Introduction
DTask is a distributed task scheduling system built with Rust, designed for high performance, reliability, and scalability. It leverages the actor model for concurrency, Raft for consensus, and gRPC for communication.

## Features
- **Distributed Scheduling**: Tasks are distributed across nodes using a scheduler.
- **High Availability**: Uses Raft for consensus to ensure system resilience.
- **RESTful API**: Manage tasks and view metrics via a Web API.
- **gRPC Interface**: Efficient service-to-service communication.
- **Authentication**: JWT-based authentication for secure access.
- **Metrics**: Prometheus integration for monitoring.
- **Interactive Documentation**: Swagger UI for API exploration.

## Tech Stack
- **Language**: Rust
- **Web Framework**: Axum
- **RPC**: Tonic (gRPC)
- **Database**: SQLite (SQLx)
- **Consensus**: OpenRaft
- **Runtime**: Tokio

## Quick Start

### Prerequisites
- Rust 1.75+
- Protobuf Compiler (`protoc`) (Optional, handled by build script in most cases)

### Installation

1. Navigate to the project directory:
   ```bash
   cd Day91-100/97.Documentation/dtask
   ```

2. Run the project:
   ```bash
   cargo run
   ```

### API Usage

The server starts on `127.0.0.1:3000` by default.

#### Swagger UI
Visit `http://127.0.0.1:3000/swagger-ui` to explore the API.

#### Authentication
Login to get a JWT token (Default credentials: admin/password):
```bash
curl -X POST http://127.0.0.1:3000/login \
  -H "Content-Type: application/json" \
  -d '{"username": "admin", "password": "password"}'
```

#### Submit a Task
```bash
curl -X POST http://127.0.0.1:3000/api/tasks \
  -H "Authorization: Bearer <YOUR_TOKEN>" \
  -H "Content-Type: application/json" \
  -d '{"payload": "Do something"}'
```

#### Get Task Status
```bash
curl -X GET http://127.0.0.1:3000/api/tasks/<TASK_ID> \
  -H "Authorization: Bearer <YOUR_TOKEN>"
```

#### Metrics
```bash
curl http://127.0.0.1:3000/metrics
```

## Documentation

Generate and view internal code documentation:
```bash
cargo doc --open
```

## Testing

Run unit and integration tests:
```bash
cargo test
```

## License
MIT
