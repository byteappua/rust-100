use serde::{Deserialize, Serialize};
use sqlx::{Decode, Encode, FromRow, Sqlite, Type};
use utoipa::ToSchema;

/// Represents a task in the system.
///
/// A task contains an ID, a payload describing the work, a creation timestamp,
/// and its current execution status.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Task {
    /// Unique identifier for the task (UUID v4).
    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    pub id: String,
    /// The actual data or command payload for the task.
    #[schema(example = "Execute backup")]
    pub payload: String,
    /// Timestamp when the task was created (Unix timestamp).
    pub created_at: i64,
    /// Current status of the task.
    pub status: TaskStatus,
}

/// Enumeration of possible task statuses.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, ToSchema)]
#[repr(i32)]
pub enum TaskStatus {
    /// Task is waiting to be picked up.
    Pending = 0,
    /// Task is currently being executed.
    Running = 1,
    /// Task has finished successfully.
    Completed = 2,
    /// Task failed during execution.
    Failed = 3,
}

impl From<i32> for TaskStatus {
    fn from(i: i32) -> Self {
        match i {
            0 => TaskStatus::Pending,
            1 => TaskStatus::Running,
            2 => TaskStatus::Completed,
            3 => TaskStatus::Failed,
            _ => TaskStatus::Pending,
        }
    }
}

impl Type<Sqlite> for TaskStatus {
    fn type_info() -> sqlx::sqlite::SqliteTypeInfo {
        <i32 as Type<Sqlite>>::type_info()
    }
}

impl<'q> Encode<'q, Sqlite> for TaskStatus {
    fn encode_by_ref(
        &self,
        args: &mut Vec<sqlx::sqlite::SqliteArgumentValue<'q>>,
    ) -> sqlx::encode::IsNull {
        <i32 as Encode<'q, Sqlite>>::encode_by_ref(&(*self as i32), args)
    }
}

impl<'r> Decode<'r, Sqlite> for TaskStatus {
    fn decode(value: sqlx::sqlite::SqliteValueRef<'r>) -> Result<Self, sqlx::error::BoxDynError> {
        let v: i32 = <i32 as Decode<Sqlite>>::decode(value)?;
        Ok(TaskStatus::from(v))
    }
}
