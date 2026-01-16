use axum::{
    routing::{get, post},
    extract::{State, Path},
    Json,
    Router,
    http::StatusCode,
};
use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;

// --- Mocking External Services for Demo ---
// In a real app, this would be sqlx::PgPool
#[derive(Clone)]
struct MockDbPool;

// In a real app, this would be redis::aio::ConnectionManager
#[derive(Clone)]
struct MockRedis;

// --- Application State ---
#[derive(Clone)]
struct AppState {
    // Shared mutable state (In-memory cache)
    cache: Arc<RwLock<HashMap<String, String>>>,
    // Shared immutable configuration
    app_name: String,
    // Connection pools (simulated)
    #[allow(dead_code)]
    db: MockDbPool,
    #[allow(dead_code)]
    redis: MockRedis,
}

// --- Handlers ---

// Read from in-memory cache
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

// Write to in-memory cache
async fn set_cache(
    State(state): State<AppState>,
    Path(key): Path<String>,
    body: String,
) -> StatusCode {
    let mut cache = state.cache.write().await;
    cache.insert(key, body);
    StatusCode::CREATED
}

// Info endpoint using immutable config
async fn info(State(state): State<AppState>) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "app": state.app_name,
        "status": "running"
    }))
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // Initialize State
    let state = AppState {
        cache: Arc::new(RwLock::new(HashMap::new())),
        app_name: "State Demo App".to_string(),
        db: MockDbPool,
        redis: MockRedis,
    };

    let app = Router::new()
        .route("/info", get(info))
        .route("/cache/:key", get(get_cache).post(set_cache))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("🚀 Server running on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}
