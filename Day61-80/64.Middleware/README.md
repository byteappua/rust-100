# Day 64: 中间件开发

## 学习目标
- 理解中间件概念和工作原理
- 掌握 Tower 中间件系统
- 实现常用中间件
- 创建自定义中间件

## 中间件概念

中间件是在请求到达处理器之前或响应返回客户端之前执行的代码层。

```
请求 → 中间件1 → 中间件2 → 处理器 → 中间件2 → 中间件1 → 响应
```

## Tower 中间件

### 基本使用

```rust
use axum::{
    Router,
    middleware,
};
use tower::ServiceBuilder;
use tower_http::{
    trace::TraceLayer,
    cors::CorsLayer,
    compression::CompressionLayer,
};

let app = Router::new()
    .route("/api/users", get(list_users))
    .layer(
        ServiceBuilder::new()
            .layer(TraceLayer::new_for_http())
            .layer(CorsLayer::permissive())
            .layer(CompressionLayer::new())
    );
```

## 内置中间件

### 1. 日志追踪 (Tracing)

```rust
use tower_http::trace::{TraceLayer, DefaultMakeSpan, DefaultOnResponse};
use tracing::Level;

let trace_layer = TraceLayer::new_for_http()
    .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
    .on_response(DefaultOnResponse::new().level(Level::INFO));

let app = Router::new()
    .route("/", get(handler))
    .layer(trace_layer);
```

### 2. CORS

```rust
use tower_http::cors::{CorsLayer, Any};
use http::Method;

let cors = CorsLayer::new()
    .allow_origin(Any)
    .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
    .allow_headers(Any);

let app = Router::new()
    .route("/api/users", get(list_users))
    .layer(cors);
```

### 3. 压缩

```rust
use tower_http::compression::CompressionLayer;

let app = Router::new()
    .route("/", get(handler))
    .layer(CompressionLayer::new());
```

### 4. 超时

```rust
use tower::timeout::TimeoutLayer;
use std::time::Duration;

let app = Router::new()
    .route("/", get(handler))
    .layer(TimeoutLayer::new(Duration::from_secs(10)));
```

### 5. 限流

```rust
use tower::limit::RateLimitLayer;
use std::time::Duration;

let app = Router::new()
    .route("/api/users", get(list_users))
    .layer(RateLimitLayer::new(100, Duration::from_secs(60)));
```

## 自定义中间件

### 方法 1: 使用 `from_fn`

```rust
use axum::{
    middleware::{self, Next},
    extract::Request,
    response::Response,
};

async fn my_middleware(
    request: Request,
    next: Next,
) -> Response {
    // 请求前处理
    println!("Request: {} {}", request.method(), request.uri());
    
    // 调用下一个中间件或处理器
    let response = next.run(request).await;
    
    // 响应后处理
    println!("Response status: {}", response.status());
    
    response
}

let app = Router::new()
    .route("/", get(handler))
    .layer(middleware::from_fn(my_middleware));
```

### 方法 2: 实现 `Layer` trait

```rust
use tower::{Layer, Service};
use std::task::{Context, Poll};
use pin_project::pin_project;

#[derive(Clone)]
struct MyMiddlewareLayer;

impl<S> Layer<S> for MyMiddlewareLayer {
    type Service = MyMiddleware<S>;
    
    fn layer(&self, inner: S) -> Self::Service {
        MyMiddleware { inner }
    }
}

#[derive(Clone)]
struct MyMiddleware<S> {
    inner: S,
}

impl<S> Service<Request> for MyMiddleware<S>
where
    S: Service<Request, Response = Response> + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;
    
    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }
    
    fn call(&mut self, request: Request) -> Self::Future {
        let future = self.inner.call(request);
        
        Box::pin(async move {
            let response = future.await?;
            // 处理响应
            Ok(response)
        })
    }
}
```

## 实用中间件示例

### 1. 请求 ID 中间件

```rust
use axum::{
    middleware::Next,
    extract::Request,
    response::Response,
    http::HeaderValue,
};
use uuid::Uuid;

async fn request_id_middleware(
    mut request: Request,
    next: Next,
) -> Response {
    let request_id = Uuid::new_v4().to_string();
    
    // 添加到请求扩展
    request.extensions_mut().insert(request_id.clone());
    
    let mut response = next.run(request).await;
    
    // 添加到响应头
    response.headers_mut().insert(
        "X-Request-ID",
        HeaderValue::from_str(&request_id).unwrap(),
    );
    
    response
}
```

### 2. 认证中间件

```rust
use axum::{
    middleware::Next,
    extract::Request,
    response::{Response, IntoResponse},
    http::StatusCode,
};

async fn auth_middleware(
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth_header = request
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok());
    
    match auth_header {
        Some(token) if token.starts_with("Bearer ") => {
            let token = &token[7..];
            
            match verify_token(token).await {
                Ok(user) => {
                    let mut request = request;
                    request.extensions_mut().insert(user);
                    Ok(next.run(request).await)
                }
                Err(_) => Err(StatusCode::UNAUTHORIZED),
            }
        }
        _ => Err(StatusCode::UNAUTHORIZED),
    }
}

// 使用
let protected_routes = Router::new()
    .route("/profile", get(get_profile))
    .layer(middleware::from_fn(auth_middleware));
```

### 3. 日志中间件

```rust
use axum::{
    middleware::Next,
    extract::Request,
    response::Response,
};
use std::time::Instant;

async fn logging_middleware(
    request: Request,
    next: Next,
) -> Response {
    let method = request.method().clone();
    let uri = request.uri().clone();
    let start = Instant::now();
    
    let response = next.run(request).await;
    
    let duration = start.elapsed();
    let status = response.status();
    
    tracing::info!(
        method = %method,
        uri = %uri,
        status = %status,
        duration = ?duration,
        "Request completed"
    );
    
    response
}
```

### 4. 错误处理中间件

```rust
use axum::{
    middleware::Next,
    extract::Request,
    response::{Response, IntoResponse},
    http::StatusCode,
    Json,
};
use serde_json::json;

async fn error_handler_middleware(
    request: Request,
    next: Next,
) -> Response {
    let response = next.run(request).await;
    
    if response.status().is_server_error() {
        let error_response = Json(json!({
            "error": "Internal server error",
            "status": 500
        }));
        
        (StatusCode::INTERNAL_SERVER_ERROR, error_response).into_response()
    } else {
        response
    }
}
```

### 5. 限流中间件

```rust
use axum::{
    middleware::Next,
    extract::Request,
    response::Response,
    http::StatusCode,
};
use std::sync::Arc;
use tokio::sync::Mutex;
use std::collections::HashMap;
use std::time::{Duration, Instant};

#[derive(Clone)]
struct RateLimiter {
    requests: Arc<Mutex<HashMap<String, Vec<Instant>>>>,
    max_requests: usize,
    window: Duration,
}

impl RateLimiter {
    fn new(max_requests: usize, window: Duration) -> Self {
        Self {
            requests: Arc::new(Mutex::new(HashMap::new())),
            max_requests,
            window,
        }
    }
    
    async fn check(&self, key: &str) -> bool {
        let mut requests = self.requests.lock().await;
        let now = Instant::now();
        
        let entry = requests.entry(key.to_string()).or_insert_with(Vec::new);
        
        // 移除过期的请求
        entry.retain(|&time| now.duration_since(time) < self.window);
        
        if entry.len() < self.max_requests {
            entry.push(now);
            true
        } else {
            false
        }
    }
}

async fn rate_limit_middleware(
    request: Request,
    next: Next,
    limiter: Arc<RateLimiter>,
) -> Result<Response, StatusCode> {
    let ip = request
        .headers()
        .get("X-Forwarded-For")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("unknown");
    
    if limiter.check(ip).await {
        Ok(next.run(request).await)
    } else {
        Err(StatusCode::TOO_MANY_REQUESTS)
    }
}
```

### 6. 缓存中间件

```rust
use axum::{
    middleware::Next,
    extract::Request,
    response::Response,
    body::Body,
};
use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;

#[derive(Clone)]
struct CacheMiddleware {
    cache: Arc<RwLock<HashMap<String, Vec<u8>>>>,
}

impl CacheMiddleware {
    fn new() -> Self {
        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

async fn cache_middleware(
    request: Request,
    next: Next,
    cache: Arc<RwLock<HashMap<String, Vec<u8>>>>,
) -> Response {
    let key = format!("{} {}", request.method(), request.uri());
    
    // 检查缓存
    if request.method() == "GET" {
        let cache_read = cache.read().await;
        if let Some(cached) = cache_read.get(&key) {
            return Response::builder()
                .header("X-Cache", "HIT")
                .body(Body::from(cached.clone()))
                .unwrap();
        }
    }
    
    let response = next.run(request).await;
    
    // 缓存 GET 请求的成功响应
    if response.status().is_success() {
        // 注意：实际实现需要处理响应体
        // 这里简化了
    }
    
    response
}
```

## 中间件组合

```rust
use tower::ServiceBuilder;
use std::time::Duration;

let app = Router::new()
    .route("/api/users", get(list_users))
    .layer(
        ServiceBuilder::new()
            // 最外层：请求 ID
            .layer(middleware::from_fn(request_id_middleware))
            // 日志
            .layer(TraceLayer::new_for_http())
            // CORS
            .layer(CorsLayer::permissive())
            // 压缩
            .layer(CompressionLayer::new())
            // 超时
            .layer(TimeoutLayer::new(Duration::from_secs(30)))
            // 限流
            .layer(middleware::from_fn(rate_limit_middleware))
    );
```

## 条件中间件

```rust
use axum::Router;

fn app() -> Router {
    let mut app = Router::new()
        .route("/api/users", get(list_users));
    
    // 仅在生产环境启用某些中间件
    if cfg!(not(debug_assertions)) {
        app = app.layer(CompressionLayer::new());
    }
    
    // 根据配置启用认证
    if std::env::var("ENABLE_AUTH").is_ok() {
        app = app.layer(middleware::from_fn(auth_middleware));
    }
    
    app
}
```

## 完整示例

```rust
use axum::{
    Router,
    routing::get,
    middleware::{self, Next},
    extract::Request,
    response::{Response, IntoResponse},
    http::{StatusCode, HeaderValue},
    Json,
};
use tower::ServiceBuilder;
use tower_http::{
    trace::TraceLayer,
    cors::CorsLayer,
    compression::CompressionLayer,
};
use std::time::Instant;
use uuid::Uuid;
use serde_json::json;

// 请求 ID 中间件
async fn request_id_middleware(
    mut request: Request,
    next: Next,
) -> Response {
    let request_id = Uuid::new_v4().to_string();
    request.extensions_mut().insert(request_id.clone());
    
    let mut response = next.run(request).await;
    response.headers_mut().insert(
        "X-Request-ID",
        HeaderValue::from_str(&request_id).unwrap(),
    );
    
    response
}

// 日志中间件
async fn logging_middleware(
    request: Request,
    next: Next,
) -> Response {
    let method = request.method().clone();
    let uri = request.uri().clone();
    let start = Instant::now();
    
    let response = next.run(request).await;
    
    let duration = start.elapsed();
    let status = response.status();
    
    println!(
        "{} {} - {} - {:?}",
        method, uri, status, duration
    );
    
    response
}

// 错误处理中间件
async fn error_handler(
    request: Request,
    next: Next,
) -> Response {
    let response = next.run(request).await;
    
    if response.status().is_server_error() {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": "Internal server error"
            }))
        ).into_response()
    } else {
        response
    }
}

// 处理器
async fn handler() -> &'static str {
    "Hello, World!"
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    
    let app = Router::new()
        .route("/", get(handler))
        .layer(
            ServiceBuilder::new()
                .layer(middleware::from_fn(request_id_middleware))
                .layer(middleware::from_fn(logging_middleware))
                .layer(middleware::from_fn(error_handler))
                .layer(TraceLayer::new_for_http())
                .layer(CorsLayer::permissive())
                .layer(CompressionLayer::new())
        );
    
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    
    println!("🚀 Server running on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}
```

## 练习

1. 实现 API 密钥认证中间件
2. 创建请求/响应日志中间件
3. 实现基于 IP 的限流中间件
4. 添加请求体大小限制中间件
5. 创建性能监控中间件

## 下一步

Day 65 将学习请求状态管理，包括应用状态、连接池等。
