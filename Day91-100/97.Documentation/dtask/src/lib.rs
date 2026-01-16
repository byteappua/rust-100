//! DTask: A Distributed Task Scheduling System
//!
//! This library provides the core components for the DTask system, including:
//! - **API**: Web server handlers and routing (`api`).
//! - **Auth**: JWT authentication and verification (`auth`).
//! - **Dispatcher**: Task distribution and execution (`dispatcher`).
//! - **Model**: Data structures and database models (`model`).
//! - **Raft**: Consensus module for distributed state (`raft`).
//! - **Scheduler**: Task scheduling logic (`scheduler`).
//!
//! ## Example
//!
//! ```rust,no_run
//! use dtask::model::{Task, TaskStatus};
//!
//! let task = Task {
//!     id: "123".to_string(),
//!     payload: "test".to_string(),
//!     created_at: 0,
//!     status: TaskStatus::Pending,
//! };
//! ```

pub mod api;
pub mod auth;
pub mod dispatcher;
pub mod model;
pub mod raft;
pub mod scheduler;

/// Generated gRPC code from `dtask.proto`
pub mod dtask {
    tonic::include_proto!("dtask");
}
