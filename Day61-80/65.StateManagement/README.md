# Day 65: 请求状态管理

## 学习目标
- 理解应用状态管理
- 掌握 Arc + RwLock 模式
- 学习连接池管理
- 实现状态注入

## 应用状态概念

在 Web 应用中，状态管理用于：
- 共享数据库连接池
- 缓存配置信息
- 共享应用级数据
- 管理全局资源

## Axum 状态管理

### 基本状态

```rust
use axum::{
    Router,
    routing::get,
    extract::State,
    Json,
};
use std::sync::Arc;

// 简单状态
#[derive(Clone)]
struct AppState {
    app_name: String,
    version: String,
}

async fn info(State(state): State<AppState>) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "app": state.app_name,
        "version": state.version
    }))
}

#[tokio::main]
async fn main() {
    let state = AppState {
        app_name: "My App".to_string(),
        version: "1.0.0".to_string(),
    };
    
    let app = Router::new()
        .route("/info", get(info))
        .with_state(state);
    
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    
    axum::serve(listener, app).await.unwrap();
}
```

### 共享可变状态

```rust
use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;

#[derive(Clone)]
struct AppState {
    // 使用 Arc + RwLock 实现共享可变状态
    cache: Arc<RwLock<HashMap<String, String>>>,
    counter: Arc<RwLock<u64>>,
}

// 读取缓存
async fn get_cache(
    State(state): State<AppState>,
    Path(key): Path<String>,
) -> Result<String, StatusCode> {
    let cache = state.cache.read().await;
    cache
        .get(&key)
        .cloned()
        .ok_or(StatusCode::NOT_FOUND)
}

// 写入缓存
async fn set_cache(
    State(state): State<AppState>,
    Path(key): Path<String>,
    body: String,
) -> StatusCode {
    let mut cache = state.cache.write().await;
    cache.insert(key, body);
    StatusCode::CREATED
}

// 增加计数器
async fn increment(State(state): State<AppState>) -> String {
    let mut counter = state.counter.write().await;
    *counter += 1;
    format!("Counter: {}", *counter)
}

fn app() -> Router {
    let state = AppState {
        cache: Arc::new(RwLock::new(HashMap::new())),
        counter: Arc::new(RwLock::new(0)),
    };
    
    Router::new()
        .route("/cache/:key", get(get_cache).post(set_cache))
        .route("/counter", get(increment))
        .with_state(state)
}
```

## 数据库连接池

### SQLx 连接池

```rust
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

#[derive(Clone)]
struct AppState {
    db: PgPool,
}

async fn create_state() -> AppState {
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool");
    
    AppState { db: pool }
}

// 使用数据库
async fn get_users(
    State(state): State<AppState>,
) -> Result<Json<Vec<User>>, StatusCode> {
    let users = sqlx::query_as!(
        User,
        "SELECT id, username, email FROM users"
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(users))
}

#[tokio::main]
async fn main() {
    let state = create_state().await;
    
    let app = Router::new()
        .route("/users", get(get_users))
        .with_state(state);
    
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    
    axum::serve(listener, app).await.unwrap();
}
```

### Redis 连接池

```rust
use redis::{Client, aio::ConnectionManager};

#[derive(Clone)]
struct AppState {
    redis: ConnectionManager,
}

async fn create_redis_state() -> AppState {
    let client = Client::open("redis://127.0.0.1/")
        .expect("Failed to create Redis client");
    
    let manager = ConnectionManager::new(client)
        .await
        .expect("Failed to create connection manager");
    
    AppState { redis: manager }
}

// 使用 Redis
async fn get_value(
    State(state): State<AppState>,
    Path(key): Path<String>,
) -> Result<String, StatusCode> {
    let mut conn = state.redis.clone();
    
    redis::cmd("GET")
        .arg(&key)
        .query_async(&mut conn)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn set_value(
    State(state): State<AppState>,
    Path(key): Path<String>,
    body: String,
) -> StatusCode {
    let mut conn = state.redis.clone();
    
    let _: () = redis::cmd("SET")
        .arg(&key)
        .arg(&body)
        .query_async(&mut conn)
        .await
        .unwrap();
    
    StatusCode::CREATED
}
```

## 复杂状态管理

### 多层状态结构

```rust
use std::sync::Arc;
use tokio::sync::RwLock;
use sqlx::PgPool;
use redis::aio::ConnectionManager;

// 配置
#[derive(Clone)]
struct Config {
    app_name: String,
    max_upload_size: usize,
    jwt_secret: String,
}

// 缓存层
#[derive(Clone)]
struct CacheLayer {
    redis: ConnectionManager,
    local_cache: Arc<RwLock<HashMap<String, String>>>,
}

// 数据库层
#[derive(Clone)]
struct DatabaseLayer {
    pg_pool: PgPool,
}

// 应用状态
#[derive(Clone)]
struct AppState {
    config: Arc<Config>,
    cache: CacheLayer,
    database: DatabaseLayer,
    metrics: Arc<RwLock<Metrics>>,
}

#[derive(Default)]
struct Metrics {
    request_count: u64,
    error_count: u64,
}

impl AppState {
    async fn new() -> Self {
        let config = Arc::new(Config {
            app_name: "My App".to_string(),
            max_upload_size: 10 * 1024 * 1024, // 10MB
            jwt_secret: std::env::var("JWT_SECRET").unwrap(),
        });
        
        let redis_client = redis::Client::open("redis://127.0.0.1/").unwrap();
        let redis = ConnectionManager::new(redis_client).await.unwrap();
        
        let cache = CacheLayer {
            redis,
            local_cache: Arc::new(RwLock::new(HashMap::new())),
        };
        
        let pg_pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&std::env::var("DATABASE_URL").unwrap())
            .await
            .unwrap();
        
        let database = DatabaseLayer { pg_pool };
        
        Self {
            config,
            cache,
            database,
            metrics: Arc::new(RwLock::new(Metrics::default())),
        }
    }
    
    async fn increment_request_count(&self) {
        let mut metrics = self.metrics.write().await;
        metrics.request_count += 1;
    }
}

// 使用状态
async fn handler(State(state): State<AppState>) -> String {
    state.increment_request_count().await;
    format!("App: {}", state.config.app_name)
}
```

## 状态初始化

### 延迟初始化

```rust
use std::sync::Arc;
use tokio::sync::OnceCell;

#[derive(Clone)]
struct AppState {
    expensive_resource: Arc<OnceCell<ExpensiveResource>>,
}

struct ExpensiveResource {
    data: Vec<u8>,
}

impl ExpensiveResource {
    async fn new() -> Self {
        // 模拟耗时初始化
        tokio::time::sleep(Duration::from_secs(2)).await;
        Self {
            data: vec![0; 1024 * 1024], // 1MB
        }
    }
}

async fn use_resource(State(state): State<AppState>) -> String {
    let resource = state
        .expensive_resource
        .get_or_init(|| async {
            ExpensiveResource::new().await
        })
        .await;
    
    format!("Resource size: {}", resource.data.len())
}
```

### 健康检查

```rust
async fn health_check(State(state): State<AppState>) -> Result<Json<HealthStatus>, StatusCode> {
    // 检查数据库连接
    let db_healthy = sqlx::query("SELECT 1")
        .fetch_one(&state.database.pg_pool)
        .await
        .is_ok();
    
    // 检查 Redis 连接
    let mut redis_conn = state.cache.redis.clone();
    let redis_healthy = redis::cmd("PING")
        .query_async::<_, String>(&mut redis_conn)
        .await
        .is_ok();
    
    let status = HealthStatus {
        status: if db_healthy && redis_healthy { "healthy" } else { "unhealthy" },
        database: db_healthy,
        redis: redis_healthy,
    };
    
    if db_healthy && redis_healthy {
        Ok(Json(status))
    } else {
        Err(StatusCode::SERVICE_UNAVAILABLE)
    }
}

#[derive(Serialize)]
struct HealthStatus {
    status: &'static str,
    database: bool,
    redis: bool,
}
```

## 状态访问模式

### 服务层模式

```rust
// 服务层封装状态访问
struct UserService {
    db: PgPool,
    cache: ConnectionManager,
}

impl UserService {
    fn new(state: &AppState) -> Self {
        Self {
            db: state.database.pg_pool.clone(),
            cache: state.cache.redis.clone(),
        }
    }
    
    async fn get_user(&self, id: i32) -> Result<User, Error> {
        // 先查缓存
        let cache_key = format!("user:{}", id);
        let mut conn = self.cache.clone();
        
        if let Ok(cached) = redis::cmd("GET")
            .arg(&cache_key)
            .query_async::<_, String>(&mut conn)
            .await
        {
            return Ok(serde_json::from_str(&cached)?);
        }
        
        // 查数据库
        let user = sqlx::query_as!(
            User,
            "SELECT * FROM users WHERE id = $1",
            id
        )
        .fetch_one(&self.db)
        .await?;
        
        // 写入缓存
        let _: () = redis::cmd("SETEX")
            .arg(&cache_key)
            .arg(3600) // 1小时过期
            .arg(serde_json::to_string(&user)?)
            .query_async(&mut conn)
            .await?;
        
        Ok(user)
    }
}

// 在处理器中使用
async fn get_user_handler(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<User>, StatusCode> {
    let service = UserService::new(&state);
    let user = service
        .get_user(id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(user))
}
```

## 完整示例

```rust
use axum::{
    Router,
    routing::{get, post},
    extract::{State, Path},
    Json,
    http::StatusCode,
};
use sqlx::postgres::PgPoolOptions;
use redis::aio::ConnectionManager;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;

// 配置
#[derive(Clone)]
struct Config {
    database_url: String,
    redis_url: String,
    jwt_secret: String,
}

// 应用状态
#[derive(Clone)]
struct AppState {
    config: Arc<Config>,
    db: sqlx::PgPool,
    redis: ConnectionManager,
    cache: Arc<RwLock<HashMap<String, String>>>,
}

impl AppState {
    async fn new(config: Config) -> Self {
        let db = PgPoolOptions::new()
            .max_connections(5)
            .connect(&config.database_url)
            .await
            .expect("Failed to connect to database");
        
        let redis_client = redis::Client::open(config.redis_url.as_str())
            .expect("Failed to create Redis client");
        let redis = ConnectionManager::new(redis_client)
            .await
            .expect("Failed to create Redis connection");
        
        Self {
            config: Arc::new(config),
            db,
            redis,
            cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

// 数据模型
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
struct User {
    id: i32,
    username: String,
    email: String,
}

// 处理器
async fn list_users(
    State(state): State<AppState>,
) -> Result<Json<Vec<User>>, StatusCode> {
    let users = sqlx::query_as!(
        User,
        "SELECT id, username, email FROM users"
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(users))
}

async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<User>, StatusCode> {
    let user = sqlx::query_as!(
        User,
        "SELECT id, username, email FROM users WHERE id = $1",
        id
    )
    .fetch_one(&state.db)
    .await
    .map_err(|_| StatusCode::NOT_FOUND)?;
    
    Ok(Json(user))
}

async fn health(State(state): State<AppState>) -> StatusCode {
    // 检查数据库
    if sqlx::query("SELECT 1")
        .fetch_one(&state.db)
        .await
        .is_err()
    {
        return StatusCode::SERVICE_UNAVAILABLE;
    }
    
    // 检查 Redis
    let mut conn = state.redis.clone();
    if redis::cmd("PING")
        .query_async::<_, String>(&mut conn)
        .await
        .is_err()
    {
        return StatusCode::SERVICE_UNAVAILABLE;
    }
    
    StatusCode::OK
}

#[tokio::main]
async fn main() {
    let config = Config {
        database_url: std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://localhost/mydb".to_string()),
        redis_url: std::env::var("REDIS_URL")
            .unwrap_or_else(|_| "redis://127.0.0.1/".to_string()),
        jwt_secret: std::env::var("JWT_SECRET")
            .unwrap_or_else(|_| "secret".to_string()),
    };
    
    let state = AppState::new(config).await;
    
    let app = Router::new()
        .route("/health", get(health))
        .route("/users", get(list_users))
        .route("/users/:id", get(get_user))
        .with_state(state);
    
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    
    println!("🚀 Server running on http://127.0.0.1:3000");
    
    axum::serve(listener, app).await.unwrap();
}
```

## 最佳实践

### 1. 使用 Arc 共享不可变数据

```rust
#[derive(Clone)]
struct AppState {
    config: Arc<Config>,  // 配置不需要修改
}
```

### 2. 使用 RwLock 共享可变数据

```rust
#[derive(Clone)]
struct AppState {
    cache: Arc<RwLock<HashMap<String, String>>>,  // 需要修改的缓存
}
```

### 3. 连接池直接克隆

```rust
#[derive(Clone)]
struct AppState {
    db: PgPool,  // PgPool 内部已经使用 Arc
}
```

### 4. 避免过度锁定

```rust
// ❌ 不好：长时间持有锁
async fn bad_handler(State(state): State<AppState>) {
    let mut cache = state.cache.write().await;
    // 执行耗时操作
    expensive_operation().await;
    cache.insert(key, value);
}

// ✅ 好：最小化锁持有时间
async fn good_handler(State(state): State<AppState>) {
    let value = expensive_operation().await;
    let mut cache = state.cache.write().await;
    cache.insert(key, value);
}
```

## 练习

1. 实现带缓存的用户服务
2. 添加请求计数器和统计
3. 实现配置热重载
4. 创建连接池监控端点
5. 实现分布式缓存

## 下一步

Day 66 将学习数据库迁移，使用 SQLx 管理数据库schema变更。
