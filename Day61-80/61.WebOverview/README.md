# Day 61: Rust Web 开发概览

## 学习目标
- 了解 Rust Web 生态系统
- 比较主流 Web 框架
- 理解异步 Web 开发模式
- 掌握 HTTP 基础概念

## Rust Web 生态系统

### 主流 Web 框架对比

| 框架 | 特点 | 适用场景 |
|------|------|----------|
| **Axum** | 基于 Tower，类型安全，ergonomic | 现代 API 服务 |
| **Actix-web** | 高性能，Actor 模型 | 高并发应用 |
| **Rocket** | 易用，宏驱动 | 快速原型开发 |
| **Warp** | 函数式，组合式 | 微服务 |
| **Tide** | 简洁，async-std | 学习和小项目 |

### 核心概念

```rust
// 1. 异步运行时
// Tokio - 最流行的异步运行时
// async-std - 标准库风格的异步运行时

// 2. HTTP 服务器
// Hyper - 底层 HTTP 实现
// Tower - 中间件抽象层

// 3. 序列化/反序列化
// Serde - JSON/其他格式处理
```

## Axum 快速入门

### 安装依赖

```toml
[dependencies]
axum = "0.7"
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

### Hello World 示例

```rust
use axum::{
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    // 创建路由
    let app = Router::new()
        .route("/", get(handler));

    // 启动服务器
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    
    println!("🚀 Server running on http://127.0.0.1:3000");
    
    axum::serve(listener, app)
        .await
        .unwrap();
}

async fn handler() -> &'static str {
    "Hello, Rust Web!"
}
```

## Actix-web 快速入门

### 安装依赖

```toml
[dependencies]
actix-web = "4"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

### Hello World 示例

```rust
use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, Actix!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 Server running on http://127.0.0.1:8080");
    
    HttpServer::new(|| {
        App::new()
            .service(hello)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
```

## HTTP 基础概念

### 请求方法

```rust
// GET - 获取资源
Router::new().route("/users", get(get_users))

// POST - 创建资源
Router::new().route("/users", post(create_user))

// PUT - 更新资源
Router::new().route("/users/:id", put(update_user))

// DELETE - 删除资源
Router::new().route("/users/:id", delete(delete_user))

// PATCH - 部分更新
Router::new().route("/users/:id", patch(patch_user))
```

### 状态码

```rust
use axum::http::StatusCode;

// 2xx 成功
StatusCode::OK                    // 200
StatusCode::CREATED               // 201
StatusCode::NO_CONTENT            // 204

// 4xx 客户端错误
StatusCode::BAD_REQUEST           // 400
StatusCode::UNAUTHORIZED          // 401
StatusCode::FORBIDDEN             // 403
StatusCode::NOT_FOUND             // 404

// 5xx 服务器错误
StatusCode::INTERNAL_SERVER_ERROR // 500
StatusCode::SERVICE_UNAVAILABLE   // 503
```

### 请求/响应结构

```rust
use axum::{
    extract::{Path, Query, Json},
    response::IntoResponse,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};

// 路径参数
async fn get_user(Path(id): Path<u32>) -> String {
    format!("User ID: {}", id)
}

// 查询参数
#[derive(Deserialize)]
struct Pagination {
    page: Option<u32>,
    limit: Option<u32>,
}

async fn list_users(Query(params): Query<Pagination>) -> String {
    format!("Page: {:?}, Limit: {:?}", params.page, params.limit)
}

// JSON 请求体
#[derive(Deserialize)]
struct CreateUser {
    name: String,
    email: String,
}

#[derive(Serialize)]
struct User {
    id: u32,
    name: String,
    email: String,
}

async fn create_user(
    Json(payload): Json<CreateUser>
) -> impl IntoResponse {
    let user = User {
        id: 1,
        name: payload.name,
        email: payload.email,
    };
    
    (StatusCode::CREATED, Json(user))
}
```

## 框架选择建议

### 选择 Axum 如果：
- ✅ 需要类型安全和编译时检查
- ✅ 喜欢组合式 API 设计
- ✅ 需要与 Tower 生态集成
- ✅ 构建现代 RESTful API

### 选择 Actix-web 如果：
- ✅ 需要极致性能
- ✅ 熟悉 Actor 模型
- ✅ 需要 WebSocket 支持
- ✅ 构建高并发应用

### 选择 Rocket 如果：
- ✅ 快速原型开发
- ✅ 喜欢宏驱动的 API
- ✅ 需要内置表单验证
- ✅ 学习 Rust Web 开发

## 性能对比

根据 TechEmpower Benchmark：

```
框架性能排名（请求/秒）：
1. Actix-web: ~700k
2. Axum: ~650k
3. Warp: ~600k
4. Rocket: ~400k
```

## 实践项目

创建一个简单的 API 服务器，支持：
- GET /health - 健康检查
- GET /api/v1/users - 获取用户列表
- POST /api/v1/users - 创建用户
- GET /api/v1/users/:id - 获取单个用户

## 学习资源

- [Axum 官方文档](https://docs.rs/axum)
- [Actix-web 官方文档](https://actix.rs)
- [Rust Web 开发实战](https://www.zero2prod.com)
- [Tower 中间件指南](https://docs.rs/tower)

## 下一步

Day 62 将深入学习 Axum 和 Actix-web 的核心特性，包括路由、提取器和响应类型。
