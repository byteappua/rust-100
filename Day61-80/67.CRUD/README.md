# Day 67: CRUD 操作实战

## 学习目标
- 掌握 SQLx 查询构建
- 实现完整的 CRUD 操作
- 学习参数化查询
- 理解事务处理

## CRUD 概念

CRUD 代表：
- **C**reate - 创建
- **R**ead - 读取
- **U**pdate - 更新
- **D**elete - 删除

## SQLx 基础

### 查询宏

```rust
use sqlx::{PgPool, FromRow};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, FromRow)]
struct User {
    id: i32,
    username: String,
    email: String,
}

// 使用 query! 宏（编译时检查）
async fn get_user_by_id(pool: &PgPool, id: i32) -> Result<User, sqlx::Error> {
    let user = sqlx::query_as!(
        User,
        "SELECT id, username, email FROM users WHERE id = $1",
        id
    )
    .fetch_one(pool)
    .await?;
    
    Ok(user)
}

// 使用 query_as（运行时检查）
async fn get_user_by_username(pool: &PgPool, username: &str) -> Result<User, sqlx::Error> {
    let user = sqlx::query_as::<_, User>(
        "SELECT id, username, email FROM users WHERE username = $1"
    )
    .bind(username)
    .fetch_one(pool)
    .await?;
    
    Ok(user)
}
```

## Create - 创建操作

### 插入单条记录

```rust
#[derive(Deserialize)]
struct CreateUser {
    username: String,
    email: String,
    password: String,
}

async fn create_user(pool: &PgPool, user: CreateUser) -> Result<User, sqlx::Error> {
    let password_hash = hash_password(&user.password);
    
    let user = sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (username, email, password_hash)
        VALUES ($1, $2, $3)
        RETURNING id, username, email
        "#,
        user.username,
        user.email,
        password_hash
    )
    .fetch_one(pool)
    .await?;
    
    Ok(user)
}
```

### 批量插入

```rust
async fn create_users_batch(
    pool: &PgPool,
    users: Vec<CreateUser>
) -> Result<Vec<User>, sqlx::Error> {
    let mut tx = pool.begin().await?;
    let mut created_users = Vec::new();
    
    for user in users {
        let password_hash = hash_password(&user.password);
        
        let created = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (username, email, password_hash)
            VALUES ($1, $2, $3)
            RETURNING id, username, email
            "#,
            user.username,
            user.email,
            password_hash
        )
        .fetch_one(&mut *tx)
        .await?;
        
        created_users.push(created);
    }
    
    tx.commit().await?;
    Ok(created_users)
}
```

## Read - 读取操作

### 查询单条记录

```rust
async fn get_user(pool: &PgPool, id: i32) -> Result<Option<User>, sqlx::Error> {
    let user = sqlx::query_as!(
        User,
        "SELECT id, username, email FROM users WHERE id = $1",
        id
    )
    .fetch_optional(pool)
    .await?;
    
    Ok(user)
}
```

### 查询多条记录

```rust
async fn list_users(pool: &PgPool) -> Result<Vec<User>, sqlx::Error> {
    let users = sqlx::query_as!(
        User,
        "SELECT id, username, email FROM users ORDER BY id"
    )
    .fetch_all(pool)
    .await?;
    
    Ok(users)
}
```

### 分页查询

```rust
#[derive(Deserialize)]
struct Pagination {
    page: i64,
    per_page: i64,
}

async fn list_users_paginated(
    pool: &PgPool,
    pagination: Pagination
) -> Result<Vec<User>, sqlx::Error> {
    let offset = (pagination.page - 1) * pagination.per_page;
    
    let users = sqlx::query_as!(
        User,
        r#"
        SELECT id, username, email
        FROM users
        ORDER BY id
        LIMIT $1 OFFSET $2
        "#,
        pagination.per_page,
        offset
    )
    .fetch_all(pool)
    .await?;
    
    Ok(users)
}

async fn count_users(pool: &PgPool) -> Result<i64, sqlx::Error> {
    let count = sqlx::query_scalar!("SELECT COUNT(*) FROM users")
        .fetch_one(pool)
        .await?;
    
    Ok(count.unwrap_or(0))
}
```

### 条件查询

```rust
#[derive(Deserialize)]
struct UserFilter {
    username: Option<String>,
    email: Option<String>,
}

async fn search_users(
    pool: &PgPool,
    filter: UserFilter
) -> Result<Vec<User>, sqlx::Error> {
    let mut query = String::from("SELECT id, username, email FROM users WHERE 1=1");
    let mut bindings = Vec::new();
    
    if let Some(username) = &filter.username {
        query.push_str(&format!(" AND username ILIKE ${}", bindings.len() + 1));
        bindings.push(format!("%{}%", username));
    }
    
    if let Some(email) = &filter.email {
        query.push_str(&format!(" AND email ILIKE ${}", bindings.len() + 1));
        bindings.push(format!("%{}%", email));
    }
    
    let mut query_builder = sqlx::query_as::<_, User>(&query);
    
    for binding in bindings {
        query_builder = query_builder.bind(binding);
    }
    
    let users = query_builder.fetch_all(pool).await?;
    
    Ok(users)
}
```

## Update - 更新操作

### 更新单条记录

```rust
#[derive(Deserialize)]
struct UpdateUser {
    username: Option<String>,
    email: Option<String>,
}

async fn update_user(
    pool: &PgPool,
    id: i32,
    update: UpdateUser
) -> Result<User, sqlx::Error> {
    let user = sqlx::query_as!(
        User,
        r#"
        UPDATE users
        SET 
            username = COALESCE($1, username),
            email = COALESCE($2, email),
            updated_at = CURRENT_TIMESTAMP
        WHERE id = $3
        RETURNING id, username, email
        "#,
        update.username,
        update.email,
        id
    )
    .fetch_one(pool)
    .await?;
    
    Ok(user)
}
```

### 批量更新

```rust
async fn update_users_role(
    pool: &PgPool,
    user_ids: Vec<i32>,
    role: &str
) -> Result<u64, sqlx::Error> {
    let result = sqlx::query!(
        "UPDATE users SET role = $1 WHERE id = ANY($2)",
        role,
        &user_ids
    )
    .execute(pool)
    .await?;
    
    Ok(result.rows_affected())
}
```

## Delete - 删除操作

### 删除单条记录

```rust
async fn delete_user(pool: &PgPool, id: i32) -> Result<bool, sqlx::Error> {
    let result = sqlx::query!("DELETE FROM users WHERE id = $1", id)
        .execute(pool)
        .await?;
    
    Ok(result.rows_affected() > 0)
}
```

### 软删除

```rust
async fn soft_delete_user(pool: &PgPool, id: i32) -> Result<User, sqlx::Error> {
    let user = sqlx::query_as!(
        User,
        r#"
        UPDATE users
        SET deleted_at = CURRENT_TIMESTAMP
        WHERE id = $1 AND deleted_at IS NULL
        RETURNING id, username, email
        "#,
        id
    )
    .fetch_one(pool)
    .await?;
    
    Ok(user)
}
```

## 完整的 Repository 模式

```rust
use sqlx::PgPool;
use async_trait::async_trait;

#[async_trait]
trait UserRepository {
    async fn create(&self, user: CreateUser) -> Result<User, sqlx::Error>;
    async fn find_by_id(&self, id: i32) -> Result<Option<User>, sqlx::Error>;
    async fn find_all(&self) -> Result<Vec<User>, sqlx::Error>;
    async fn update(&self, id: i32, user: UpdateUser) -> Result<User, sqlx::Error>;
    async fn delete(&self, id: i32) -> Result<bool, sqlx::Error>;
}

struct PostgresUserRepository {
    pool: PgPool,
}

impl PostgresUserRepository {
    fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn create(&self, user: CreateUser) -> Result<User, sqlx::Error> {
        let password_hash = hash_password(&user.password);
        
        sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (username, email, password_hash)
            VALUES ($1, $2, $3)
            RETURNING id, username, email
            "#,
            user.username,
            user.email,
            password_hash
        )
        .fetch_one(&self.pool)
        .await
    }
    
    async fn find_by_id(&self, id: i32) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as!(
            User,
            "SELECT id, username, email FROM users WHERE id = $1",
            id
        )
        .fetch_optional(&self.pool)
        .await
    }
    
    async fn find_all(&self) -> Result<Vec<User>, sqlx::Error> {
        sqlx::query_as!(
            User,
            "SELECT id, username, email FROM users ORDER BY id"
        )
        .fetch_all(&self.pool)
        .await
    }
    
    async fn update(&self, id: i32, user: UpdateUser) -> Result<User, sqlx::Error> {
        sqlx::query_as!(
            User,
            r#"
            UPDATE users
            SET 
                username = COALESCE($1, username),
                email = COALESCE($2, email)
            WHERE id = $3
            RETURNING id, username, email
            "#,
            user.username,
            user.email,
            id
        )
        .fetch_one(&self.pool)
        .await
    }
    
    async fn delete(&self, id: i32) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!("DELETE FROM users WHERE id = $1", id)
            .execute(&self.pool)
            .await?;
        
        Ok(result.rows_affected() > 0)
    }
}
```

## 与 Axum 集成

```rust
use axum::{
    Router,
    routing::{get, post, put, delete},
    extract::{State, Path},
    Json,
    http::StatusCode,
};

#[derive(Clone)]
struct AppState {
    user_repo: Arc<PostgresUserRepository>,
}

async fn create_user_handler(
    State(state): State<AppState>,
    Json(user): Json<CreateUser>,
) -> Result<(StatusCode, Json<User>), StatusCode> {
    let user = state.user_repo
        .create(user)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok((StatusCode::CREATED, Json(user)))
}

async fn get_user_handler(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<User>, StatusCode> {
    let user = state.user_repo
        .find_by_id(id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;
    
    Ok(Json(user))
}

async fn list_users_handler(
    State(state): State<AppState>,
) -> Result<Json<Vec<User>>, StatusCode> {
    let users = state.user_repo
        .find_all()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(users))
}

async fn update_user_handler(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(user): Json<UpdateUser>,
) -> Result<Json<User>, StatusCode> {
    let user = state.user_repo
        .update(id, user)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(user))
}

async fn delete_user_handler(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> StatusCode {
    match state.user_repo.delete(id).await {
        Ok(true) => StatusCode::NO_CONTENT,
        Ok(false) => StatusCode::NOT_FOUND,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

fn app(state: AppState) -> Router {
    Router::new()
        .route("/users", get(list_users_handler).post(create_user_handler))
        .route("/users/:id", 
            get(get_user_handler)
            .put(update_user_handler)
            .delete(delete_user_handler)
        )
        .with_state(state)
}
```

## 练习

1. 实现文章的完整 CRUD 操作
2. 添加复杂的搜索和过滤功能
3. 实现软删除和恢复功能
4. 创建审计日志系统
5. 实现批量操作 API

## 下一步

Day 68 将学习复杂查询与事务，包括 JOIN、聚合和事务管理。
