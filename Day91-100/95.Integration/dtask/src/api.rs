use axum::{
    routing::{get, post},
    Router,
    extract::{State, Json, Path, Request},
    middleware::{self, Next},
    response::{Response, IntoResponse},
    http::{StatusCode, header},
};
use std::sync::{Arc, Mutex};
use crate::model::{Task, TaskStatus};
use crate::auth::{self, LoginRequest, LoginResponse};
use prometheus::{Encoder, TextEncoder, Counter, Histogram, register_counter, register_histogram};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

lazy_static! {
    static ref HTTP_REQUESTS_TOTAL: Counter = register_counter!(
        "http_requests_total",
        "Total number of HTTP requests made."
    ).unwrap();
    static ref HTTP_REQUEST_DURATION_SECONDS: Histogram = register_histogram!(
        "http_request_duration_seconds",
        "The HTTP request latencies in seconds."
    ).unwrap();
}

#[derive(Clone)]
pub struct AppState {
    pub tasks: Arc<Mutex<Vec<Task>>>,
}

pub fn app(tasks: Arc<Mutex<Vec<Task>>>) -> Router {
    let state = AppState { tasks };

    let api_routes = Router::new()
        .route("/tasks", post(submit_task))
        .route("/tasks/:id", get(get_task))
        .layer(middleware::from_fn(auth_middleware));

    Router::new()
        .route("/login", post(login))
        .route("/metrics", get(metrics))
        .nest("/api", api_routes)
        .with_state(state)
        .layer(middleware::from_fn(track_metrics))
}

async fn track_metrics(req: Request, next: Next) -> Response {
    let start = std::time::Instant::now();
    let response = next.run(req).await;
    let latency = start.elapsed().as_secs_f64();

    HTTP_REQUESTS_TOTAL.inc();
    HTTP_REQUEST_DURATION_SECONDS.observe(latency);

    response
}

async fn auth_middleware(req: Request, next: Next) -> Result<Response, StatusCode> {
    let auth_header = req.headers()
        .get(header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    let auth_header = if let Some(auth_header) = auth_header {
        auth_header
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    if let Some(token) = auth_header.strip_prefix("Bearer ") {
        if auth::verify_jwt(token).is_ok() {
            Ok(next.run(req).await)
        } else {
            Err(StatusCode::UNAUTHORIZED)
        }
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

async fn login(Json(payload): Json<LoginRequest>) -> Result<Json<LoginResponse>, StatusCode> {
    if payload.username == "admin" && payload.password == "password" {
        let token = auth::create_jwt(&payload.username).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        Ok(Json(LoginResponse { token }))
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

#[derive(Deserialize)]
pub struct CreateTaskRequest {
    pub payload: String,
}

#[derive(Serialize)]
pub struct CreateTaskResponse {
    pub id: String,
    pub success: bool,
    pub message: String,
}

async fn submit_task(
    State(state): State<AppState>,
    Json(payload): Json<CreateTaskRequest>,
) -> Json<CreateTaskResponse> {
    let id = uuid::Uuid::new_v4().to_string();
    let task = Task {
        id: id.clone(),
        payload: payload.payload,
        created_at: chrono::Utc::now().timestamp(),
        status: TaskStatus::Pending,
    };

    {
        let mut tasks = state.tasks.lock().unwrap();
        tasks.push(task);
    }

    Json(CreateTaskResponse {
        id,
        success: true,
        message: "Task submitted via API".to_string(),
    })
}

async fn get_task(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Task>, StatusCode> {
    let tasks = state.tasks.lock().unwrap();
    let task = tasks.iter().find(|t| t.id == id).cloned();

    if let Some(task) = task {
        Ok(Json(task))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

async fn metrics() -> impl IntoResponse {
    let encoder = TextEncoder::new();
    let metric_families = prometheus::gather();
    let mut buffer = vec![];
    encoder.encode(&metric_families, &mut buffer).unwrap();

    (
        [(header::CONTENT_TYPE, encoder.format_type().to_string())],
        buffer
    )
}
