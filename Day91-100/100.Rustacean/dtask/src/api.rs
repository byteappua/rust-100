use crate::auth::{self, LoginRequest, LoginResponse};
use crate::model::Task;
use crate::raft::{DTaskRaft, Node, NodeId, Request as RaftRequest};
use axum::{
    extract::{Json, Path, Request, State},
    http::{header, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::{get, post},
    Router,
};
use lazy_static::lazy_static;
use openraft::raft::{AppendEntriesRequest, InstallSnapshotRequest, VoteRequest};
use prometheus::{register_counter, register_histogram, Counter, Encoder, Histogram, TextEncoder};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use std::collections::BTreeMap;

lazy_static! {
    static ref HTTP_REQUESTS_TOTAL: Counter =
        register_counter!("http_requests_total", "Total number of HTTP requests made.").unwrap();
    static ref HTTP_REQUEST_DURATION_SECONDS: Histogram = register_histogram!(
        "http_request_duration_seconds",
        "The HTTP request latencies in seconds."
    )
    .unwrap();
}

#[derive(Clone)]
pub struct AppState {
    pub pool: SqlitePool,
    pub raft: DTaskRaft,
}

pub fn app(pool: SqlitePool, raft: DTaskRaft) -> Router {
    let state = AppState { pool, raft };

    let api_routes = Router::new()
        .route("/tasks", post(submit_task))
        .route("/tasks/:id", get(get_task))
        .layer(middleware::from_fn(auth_middleware));

    let raft_routes = Router::new()
        .route("/vote", post(raft_vote))
        .route("/append", post(raft_append))
        .route("/snapshot", post(raft_snapshot));

    let cluster_routes = Router::new()
        .route("/join", post(cluster_join));

    Router::new()
        .route("/login", post(login))
        .route("/metrics", get(metrics))
        .nest("/api", api_routes)
        .nest("/raft", raft_routes)
        .nest("/cluster", cluster_routes)
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
    let auth_header = req
        .headers()
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
        let token =
            auth::create_jwt(&payload.username).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
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

// Handler for submitting tasks via Raft
async fn submit_task(
    State(state): State<AppState>,
    Json(payload): Json<CreateTaskRequest>,
) -> Result<Json<CreateTaskResponse>, StatusCode> {
    let raft_req = RaftRequest { payload: payload.payload };

    // Submit to Raft
    let response = state.raft.client_write(raft_req).await;

    match response {
        Ok(resp) => {
            Ok(Json(CreateTaskResponse {
                id: resp.data.result, // Assuming result contains ID
                success: true,
                message: "Task submitted via Raft".to_string(),
            }))
        },
        Err(e) => {
            tracing::error!("Raft write error: {:?}", e);
            // If forwarding is needed, we should handle it.
            // For now return 500 or proper error.
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn get_task(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Task>, StatusCode> {
    let task = sqlx::query_as::<_, Task>("SELECT * FROM tasks WHERE id = ?")
        .bind(id)
        .fetch_optional(&state.pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to fetch task: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

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
        buffer,
    )
}

// Raft Handlers

async fn raft_vote(
    State(state): State<AppState>,
    Json(req): Json<VoteRequest<NodeId>>,
) -> impl IntoResponse {
    let res = state.raft.vote(req).await;
    match res {
        Ok(v) => (StatusCode::OK, Json(v)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Vote error: {:?}", e)).into_response(),
    }
}

async fn raft_append(
    State(state): State<AppState>,
    Json(req): Json<AppendEntriesRequest<crate::raft::TypeConfig>>,
) -> impl IntoResponse {
    let res = state.raft.append_entries(req).await;
    match res {
        Ok(v) => (StatusCode::OK, Json(v)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Append error: {:?}", e)).into_response(),
    }
}

async fn raft_snapshot(
    State(state): State<AppState>,
    Json(req): Json<InstallSnapshotRequest<crate::raft::TypeConfig>>,
) -> impl IntoResponse {
    let res = state.raft.install_snapshot(req).await;
    match res {
        Ok(v) => (StatusCode::OK, Json(v)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Snapshot error: {:?}", e)).into_response(),
    }
}

#[derive(Deserialize)]
struct JoinRequest {
    node_id: u64,
    rpc_addr: String,
}

async fn cluster_join(
    State(state): State<AppState>,
    Json(req): Json<JoinRequest>,
) -> impl IntoResponse {
    let mut nodes = BTreeMap::new();
    nodes.insert(req.node_id, Node { rpc_addr: req.rpc_addr });

    let res = state.raft.add_learner(req.node_id, nodes.get(&req.node_id).unwrap().clone(), true).await;
    match res {
        Ok(_) => {
             // Promote to voter immediately for simplicity in this demo
             let res = state.raft.change_membership(nodes.keys().cloned(), true).await;
              match res {
                Ok(_) => (StatusCode::OK, "Joined and promoted").into_response(),
                Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Promotion error: {:?}", e)).into_response(),
             }
        },
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Join error: {:?}", e)).into_response(),
    }
}
