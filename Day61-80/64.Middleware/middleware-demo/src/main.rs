use axum::{
    routing::get,
    middleware::{self, Next},
    extract::Request,
    response::{Response, IntoResponse},
    http::{StatusCode, HeaderValue},
    Json,
    Router,
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

// 自定义日志中间件
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

    // Using println for demo purposes, but tracing is better
    println!(
        "[CustomLog] {} {} - {} - {:?}",
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
                "error": "Internal server error",
                "status": 500
            }))
        ).into_response()
    } else {
        response
    }
}

// 处理器
async fn handler() -> &'static str {
    "Hello, Middleware!"
}

async fn error_simulation() -> StatusCode {
    StatusCode::INTERNAL_SERVER_ERROR
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(handler))
        .route("/error", get(error_simulation))
        .layer(
            ServiceBuilder::new()
                // Outer layers wrap inner layers
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
