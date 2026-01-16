use serde::{Deserialize, Serialize};
use sqlx::{Decode, Encode, FromRow, Sqlite, Type};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Task {
    pub id: String,
    pub payload: String,
    pub created_at: i64,
    pub status: TaskStatus,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[repr(i32)]
pub enum TaskStatus {
    Pending = 0,
    Running = 1,
    Completed = 2,
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
