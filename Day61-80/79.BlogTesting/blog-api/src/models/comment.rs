use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Comment {
    pub id: i64,
    pub post_id: Option<i64>,
    pub user_id: Option<i64>,
    pub content: String,
    pub created_at: DateTime<Utc>,
}
