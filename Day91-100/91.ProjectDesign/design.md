# Distributed Task Scheduling System (DTask) - Design Document

## 1. Introduction

**Project Name:** DTask (Distributed Task Scheduler)

DTask is a high-performance, high-availability distributed task scheduling system designed to handle large-scale background job processing. It guarantees reliable task execution, supports cron-like scheduling, and provides horizontal scalability for both scheduling and execution.

## 2. Requirements Analysis

### 2.1 Functional Requirements

1.  **Task Submission**:
    *   Support immediate task execution.
    *   Support scheduled tasks (delay/cron).
    *   Support task priorities.
    *   Task payload support (JSON/Binary).
2.  **Task Execution**:
    *   Distributed execution across multiple worker nodes.
    *   Retry mechanism for failed tasks with configurable policies (exponential backoff).
    *   Timeouts for task execution.
3.  **Cluster Management**:
    *   Dynamic worker registration and deregistration.
    *   Health checking (Heartbeats) for workers.
    *   Leader election for scheduler nodes (High Availability).
4.  **Observability & Control**:
    *   Query task status (Pending, Running, Completed, Failed).
    *   Cancel running or pending tasks.
    *   View execution logs.

### 2.2 Non-Functional Requirements

1.  **Availability**: The system must survive the failure of scheduler nodes (via Raft consensus).
2.  **Scalability**:
    *   Scheduler layer should handle thousands of concurrent submissions.
    *   Worker layer should be horizontally scalable to increase throughput.
3.  **Reliability**: "At least once" execution guarantee. Tasks must not be lost if a worker crashes.
4.  **Performance**: Low latency for task dispatching.
5.  **Persistence**: All task states must be persisted to disk (PostgreSQL) to survive full system restarts.

## 3. Architecture Design

### 3.1 High-Level Architecture

```
                                  +-------------+
                                  |    Client   |
                                  +------+------+
                                         | gRPC
                                         v
+----------------------------------------+-----------------------------------------+
|                                  DTask Cluster                                   |
|                                                                                  |
|   +-------------------+          +-------------------+          +-------------------+   |
|   | Scheduler (L)     |<-------->| Scheduler (F)     |<-------->| Scheduler (F)     |   |
|   | (Raft Leader)     |   Raft   | (Raft Follower)   |   Raft   | (Raft Follower)   |   |
|   +--------+----------+          +-------------------+          +-------------------+   |
|            |                                                                     |
|            |                                                                     |
+------------|---------------------------------------------------------------------|---+
             | gRPC (Task Dispatch / Heartbeat)                                    |
             v                                                                     v
    +--------+--------+                                                   +--------+--------+
    |  Worker Node 1  |                                                   |  Worker Node N  |
    +--------+--------+                                                   +--------+--------+
             |                                                                     |
             v                                                                     v
    +--------+--------+                                                   +--------+--------+
    | Task Executor   |                                                   | Task Executor   |
    +-----------------+                                                   +-----------------+

Legend: L=Leader, F=Follower
```

### 3.2 Components

1.  **Scheduler Node (Master)**:
    *   Responsible for receiving task submissions from clients.
    *   Manages the state of the task queue.
    *   Uses **Raft** consensus to replicate task data and ensure a single leader manages scheduling.
    *   Dispatches tasks to available workers based on load balancing strategies.
    *   Persists state to PostgreSQL (via Raft log or state machine snapshotting).

2.  **Worker Node**:
    *   Stateless execution units.
    *   Registers with the Scheduler Leader on startup.
    *   Maintains a long-lived gRPC connection (or pulls) for tasks.
    *   Executes the actual logic (e.g., shell command, HTTP request, Docker container).
    *   Reports execution results back to the Scheduler.

3.  **Storage Layer**:
    *   **PostgreSQL**: Stores persistent data: User accounts, Task History, Execution Logs, Defined Jobs.
    *   **Redis** (Optional/Optimization): Used as a high-speed queue or for pub/sub real-time updates to UI, or potentially for distributed locking if Raft is overkill for some parts (though we aim to use Raft).

## 4. Technology Stack Selection

*   **Language**: [Rust](https://www.rust-lang.org/) (Performance, Safety).
*   **Async Runtime**: [Tokio](https://tokio.rs/) (Industry standard for async Rust).
*   **Communication (RPC)**: [Tonic](https://github.com/hyperium/tonic) (gRPC implementation based on Prost/Tokio).
*   **Consensus**: [OpenRaft](https://github.com/datafuselabs/openraft) (Async Raft implementation for Rust).
*   **Database**: [SQLx](https://github.com/launchbadge/sqlx) (Async SQL client) + **PostgreSQL**.
*   **Serialization**: [Protobuf](https://developers.google.com/protocol-buffers) (via Prost) for network, [Serde](https://serde.rs/) for internal/JSON.
*   **Logging/Tracing**: [Tracing](https://github.com/tokio-rs/tracing).
*   **CLI**: [Clap](https://github.com/clap-rs/clap).

## 5. Data Model Design (Draft)

### 5.1 Database Schema (PostgreSQL)

**Table: tasks**
```sql
CREATE TABLE tasks (
    id VARCHAR(36) PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    task_type VARCHAR(50) NOT NULL,
    payload BYTEA,
    cron_expr VARCHAR(100), -- Null for one-off
    status VARCHAR(20) NOT NULL, -- PENDING, RUNNING, COMPLETED, FAILED, RETRYING
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    next_run_at TIMESTAMP WITH TIME ZONE,
    max_retries INT DEFAULT 3,
    current_retry INT DEFAULT 0
);
```

**Table: execution_logs**
```sql
CREATE TABLE execution_logs (
    id SERIAL PRIMARY KEY,
    task_id VARCHAR(36) REFERENCES tasks(id),
    worker_id VARCHAR(36),
    started_at TIMESTAMP WITH TIME ZONE,
    finished_at TIMESTAMP WITH TIME ZONE,
    status VARCHAR(20),
    output TEXT,
    error_message TEXT
);
```

### 5.2 Protobuf Definitions

```protobuf
syntax = "proto3";
package dtask;

service Scheduler {
  rpc SubmitTask (SubmitTaskRequest) returns (SubmitTaskResponse);
  rpc CancelTask (CancelTaskRequest) returns (CancelTaskResponse);
  rpc GetTaskStatus (GetTaskStatusRequest) returns (GetTaskStatusResponse);

  // Worker Interface
  rpc RegisterWorker (RegisterWorkerRequest) returns (RegisterWorkerResponse);
  rpc Heartbeat (HeartbeatRequest) returns (HeartbeatResponse);
  rpc PollTask (PollTaskRequest) returns (PollTaskResponse);
  rpc ReportResult (ReportResultRequest) returns (ReportResultResponse);
}

message Task {
  string id = 1;
  string name = 2;
  string type = 3;
  bytes payload = 4;
  string cron_expr = 5;
}

// ... request/response messages
```

## 6. Implementation Plan (Days 92-100)

*   **Day 92 (PoC)**: Verify gRPC connection between Client-Server-Worker. Test basic Raft node setup.
*   **Day 93 (Core 1)**: Implement Raft consensus for the Scheduler. State machine implementation.
*   **Day 94 (Core 2)**: Implement Task persistence (SQLx) and Scheduler logic (dispatch loop).
*   **Day 95 (Integration)**: Worker implementation. Task execution logic.
*   **Day 96 (Performance)**: Benchmarking. Optimizing database queries and Raft log compaction.
*   **Day 97-98 (Docs & Release)**: API docs, Dockerfile, CI/CD pipeline.
*   **Day 99-100 (Final)**: Demo, Presentation, Cleanup.
