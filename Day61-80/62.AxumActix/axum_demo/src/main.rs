use axum::{
    extract::{Path, Query, State, Json},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get},
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
    
    // Check bounds
    if start >= users.len() {
        return Json(vec![]);
    }

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
    #[allow(dead_code)]
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
