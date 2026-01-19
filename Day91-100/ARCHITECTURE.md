# DTask System Architecture

This document provides a visual overview of the DTask distributed task scheduling system architecture, including high-level design, component interactions, and state transitions.

## 1. System Architecture (C4 Context)

The DTask system consists of a cluster of Scheduler nodes (managing state via Raft) and Worker nodes (executing tasks).

```mermaid
C4Context
    title "System Context Diagram for DTask"

    Person(user, "User/Client", "Submits tasks and queries status")

    Boundary(cluster, "DTask Cluster") {
        System(scheduler_leader, "Scheduler (Leader)", "Manages consensus, scheduling, and task dispatching")
        System(scheduler_follower, "Scheduler (Follower)", "Replicates logs, redirects writes to Leader")
        System(db, "PostgreSQL", "Persists task history and metadata")
    }

    System_Ext(worker, "Worker Node", "Executes assigned tasks")

    Rel(user, scheduler_leader, "Submits Task (gRPC/HTTP)")
    Rel(user, scheduler_follower, "Read Request")
    Rel(scheduler_follower, scheduler_leader, "Forward Write Request")

    Rel(scheduler_leader, scheduler_follower, "Raft Replication")
    Rel(scheduler_leader, db, "Persist State")

    Rel(scheduler_leader, worker, "Dispatch Task (gRPC)")
    Rel(worker, scheduler_leader, "Heartbeat / Report Result")
```

## 2. Component Relationship Diagram

This diagram shows how the internal Rust modules interact within a single Scheduler node.

```mermaid
classDiagram
    class Main {
        +start()
        +init_raft()
        +init_api()
    }

    class API {
        +submit_task()
        +get_status()
    }

    class RaftNode {
        +append_entries()
        +vote()
        +install_snapshot()
    }

    class Store {
        +save_log()
        +apply_log()
        +get_state_machine()
    }

    class Network {
        +send_append_entries()
        +send_vote()
    }

    class Scheduler {
        +schedule_loop()
        +assign_task()
    }

    class Dispatcher {
        +dispatch_loop()
        +send_to_worker()
    }

    Main --> API : Starts
    Main --> RaftNode : Initializes
    API --> RaftNode : Propose Proposal
    RaftNode --> Store : Persist Log/State
    RaftNode --> Network : Communicate with Peers
    RaftNode --|> Scheduler : Notifies on Commit
    Scheduler --> Dispatcher : Push Ready Task
    Dispatcher --> Worker : gRPC Call
```

## 3. Task Execution Sequence

The lifecycle of a task from submission to completion.

```mermaid
sequenceDiagram
    participant Client
    participant Leader as Scheduler (Leader)
    participant Follower as Scheduler (Follower)
    participant Store as Raft Store
    participant Worker

    Client->>Leader: Submit Task (HTTP/gRPC)
    activate Leader
    Leader->>Leader: Create Entry
    Leader->>Follower: AppendEntries (Log)
    activate Follower
    Follower->>Follower: Write to Log
    Follower-->>Leader: Success
    deactivate Follower

    Leader->>Store: Commit & Apply (Write to DB)
    activate Store
    Store-->>Leader: Applied
    deactivate Store

    Leader-->>Client: Task Accepted (ID)
    deactivate Leader

    loop Async Dispatch Loop
        Leader->>Leader: Check Pending Tasks
        Leader->>Worker: Dispatch Task
        activate Worker
        Worker-->>Leader: Ack
        Worker->>Worker: Execute Task
        Worker-->>Leader: Report Result (Success/Fail)
        deactivate Worker
        Leader->>Store: Update Task Status
    end
```

## 4. Raft State Machine

The states a DTask node can be in regarding the consensus protocol.

```mermaid
stateDiagram-v2
    [*] --> Follower

    state Follower {
        [*] --> Idle
        Idle --> ReceivingLog : AppendEntries
        ReceivingLog --> Idle
    }

    Follower --> Candidate : Election Timeout

    state Candidate {
        [*] --> RequestVote
        RequestVote --> Voting : Send Requests
    }

    Candidate --> Leader : Majority Votes Received
    Candidate --> Follower : Higher Term Discovered

    state Leader {
        [*] --> Heartbeat
        Heartbeat --> Replicating : New Client Request
        Replicating --> Heartbeat : Log Committed
    }

    Leader --> Follower : Higher Term Discovered
```

## 5. Entity Relationship (Database)

```mermaid
erDiagram
    TASKS {
        string id PK
        string name
        string type
        string status
        string cron_expr
        int max_retries
        timestamp created_at
        timestamp next_run_at
    }

    EXECUTION_LOGS {
        int id PK
        string task_id FK
        string worker_id
        string status
        text output
        text error
        timestamp started_at
        timestamp finished_at
    }

    TASKS ||--o{ EXECUTION_LOGS : "has history"
```
