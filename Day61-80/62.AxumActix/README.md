# Day 62: Axum/Actix-web 入门

## 学习目标
- 深入理解 Axum 框架核心概念
- 掌握 Actix-web 基本用法
- 学习提取器（Extractors）
- 理解响应类型

## Axum 深入

### 项目设置

```toml
[package]
name = "axum_demo"
version = "0.1.0"
edition = "2021"

[dependencies]
axum = "0.7"
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tower = "0.4"
tower-http = { version = "0.5", features = ["trace", "cors"] }
tracing = "0.1"
tracing-subscriber = "0.3"
```

### 完整示例

```rust
use axum::{
    extract::{Path, Query, State, Json},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

// 应用状态
#[derive(Clone)]
struct AppState {
    users: Arc<RwLock<Vec<User>>>,
}

// 数据模型
#[derive(Debug, Clone, Serialize, Deserialize)]
struct User {
    id: u32,
    name: String,
    email: String,
}

#[derive(Deserialize)]
struct CreateUserRequest {
    name: String,
    email: String,
}

#[derive(Deserialize)]
struct ListQuery {
    page: Option<u32>,
    limit: Option<u32>,
}

// 路由处理器
async fn health_check() -> &'static str {
    "OK"
}

async fn list_users(
    State(state): State<AppState>,
    Query(params): Query<ListQuery>,
) -> Json<Vec<User>> {
    let users = state.users.read().await;
    let page = params.page.unwrap_or(1);
    let limit = params.limit.unwrap_or(10);
    
    let start = ((page - 1) * limit) as usize;
    let end = (start + limit as usize).min(users.len());
    
    Json(users[start..end].to_vec())
}

async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<u32>,
) -> Result<Json<User>, AppError> {
    let users = state.users.read().await;
    users
        .iter()
        .find(|u| u.id == id)
        .cloned()
        .map(Json)
        .ok_or(AppError::NotFound)
}

async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<User>), AppError> {
    let mut users = state.users.write().await;
    
    let id = users.len() as u32 + 1;
    let user = User {
        id,
        name: payload.name,
        email: payload.email,
    };
    
    users.push(user.clone());
    Ok((StatusCode::CREATED, Json(user)))
}

// 错误处理
enum AppError {
    NotFound,
    InternalError,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::NotFound => (StatusCode::NOT_FOUND, "Resource not found"),
            AppError::InternalError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error",
            ),
        };
        
        (status, message).into_response()
    }
}

#[tokio::main]
async fn main() {
    // 初始化日志
    tracing_subscriber::fmt::init();
    
    // 创建应用状态
    let state = AppState {
        users: Arc::new(RwLock::new(vec![
            User {
                id: 1,
                name: "Alice".to_string(),
                email: "alice@example.com".to_string(),
            },
        ])),
    };
    
    // 构建路由
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/api/users", get(list_users).post(create_user))
        .route("/api/users/:id", get(get_user))
        .with_state(state);
    
    // 启动服务器
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    
    println!("🚀 Server running on http://127.0.0.1:3000");
    
    axum::serve(listener, app).await.unwrap();
}
```

## Actix-web 深入

### 项目设置

```toml
[dependencies]
actix-web = "4"
actix-rt = "2"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tokio = { version = "1", features = ["sync"] }
```

### 完整示例

```rust
use actix_web::{
    get, post, web, App, HttpResponse, HttpServer, Responder, Result,
};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

// 应用状态
struct AppState {
    users: Mutex<Vec<User>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct User {
    id: u32,
    name: String,
    email: String,
}

#[derive(Deserialize)]
struct CreateUserRequest {
    name: String,
    email: String,
}

#[derive(Deserialize)]
struct ListQuery {
    page: Option<u32>,
    limit: Option<u32>,
}

// 路由处理器
#[get("/health")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

#[get("/api/users")]
async fn list_users(
    data: web::Data<AppState>,
    query: web::Query<ListQuery>,
) -> Result<impl Responder> {
    let users = data.users.lock().unwrap();
    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(10);
    
    let start = ((page - 1) * limit) as usize;
    let end = (start + limit as usize).min(users.len());
    
    Ok(web::Json(&users[start..end]))
}

#[get("/api/users/{id}")]
async fn get_user(
    data: web::Data<AppState>,
    path: web::Path<u32>,
) -> Result<impl Responder> {
    let users = data.users.lock().unwrap();
    let id = path.into_inner();
    
    users
        .iter()
        .find(|u| u.id == id)
        .map(|u| HttpResponse::Ok().json(u))
        .ok_or_else(|| actix_web::error::ErrorNotFound("User not found"))
}

#[post("/api/users")]
async fn create_user(
    data: web::Data<AppState>,
    payload: web::Json<CreateUserRequest>,
) -> Result<impl Responder> {
    let mut users = data.users.lock().unwrap();
    
    let id = users.len() as u32 + 1;
    let user = User {
        id,
        name: payload.name.clone(),
        email: payload.email.clone(),
    };
    
    users.push(user.clone());
    Ok(HttpResponse::Created().json(user))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 创建应用状态
    let app_state = web::Data::new(AppState {
        users: Mutex::new(vec![User {
            id: 1,
            name: "Alice".to_string(),
            email: "alice@example.com".to_string(),
        }]),
    });
    
    println!("🚀 Server running on http://127.0.0.1:8080");
    
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(health_check)
            .service(list_users)
            .service(get_user)
            .service(create_user)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
```

## 提取器对比

### Axum 提取器

```rust
// 路径参数
Path(id): Path<u32>

// 查询参数
Query(params): Query<MyQuery>

// JSON 请求体
Json(payload): Json<MyPayload>

// 表单数据
Form(data): Form<MyForm>

// 请求头
headers: HeaderMap

// 应用状态
State(state): State<AppState>

// 扩展数据
Extension(ext): Extension<MyExtension>
```

### Actix-web 提取器

```rust
// 路径参数
path: web::Path<u32>

// 查询参数
query: web::Query<MyQuery>

// JSON 请求体
payload: web::Json<MyPayload>

// 表单数据
form: web::Form<MyForm>

// 请求头
req: HttpRequest

// 应用状态
data: web::Data<AppState>
```

## 测试 API

```bash
# 健康检查
curl http://localhost:3000/health

# 获取用户列表
curl http://localhost:3000/api/users

# 分页查询
curl "http://localhost:3000/api/users?page=1&limit=5"

# 获取单个用户
curl http://localhost:3000/api/users/1

# 创建用户
curl -X POST http://localhost:3000/api/users \
  -H "Content-Type: application/json" \
  -d '{"name":"Bob","email":"bob@example.com"}'
```

## 练习

1. 添加 PUT 和 DELETE 端点
2. 实现输入验证
3. 添加日志中间件
4. 实现错误处理中间件
5. 添加 CORS 支持

## 下一步

Day 63 将学习高级路由和请求处理技术。
