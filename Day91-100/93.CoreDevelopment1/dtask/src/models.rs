use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Task {
    pub id: String,
    pub name: String,
    pub payload: Vec<u8>,
    pub status: TaskStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TaskStatus {
    Pending,
    Running,
    Completed,
    Failed,
}

// Raft Request (Command)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AppRequest {
    SubmitTask(Task),
    // Future: CancelTask, UpdateTask
}

// Raft Response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AppResponse {
    TaskSubmitted(String),
    None,
}
