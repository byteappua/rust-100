use axum::{
    extract::{Path, Query, State, Json},
    http::StatusCode,
    routing::{get},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

// 数据模型
#[derive(Debug, Clone, Serialize, Deserialize)]
struct User {
    id: i32,
    username: String,
    email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Post {
    id: i32,
    user_id: i32,
    title: String,
    content: String,
}

// 请求/响应模型
#[derive(Deserialize)]
struct CreateUserRequest {
    username: String,
    email: String,
}

#[derive(Deserialize)]
struct UpdateUserRequest {
    username: Option<String>,
    email: Option<String>,
}

#[derive(Deserialize)]
struct ListQuery {
    page: Option<u32>,
    per_page: Option<u32>,
}

// 应用状态
#[derive(Clone)]
struct AppState {
    users: Arc<RwLock<Vec<User>>>,
    posts: Arc<RwLock<Vec<Post>>>,
}

// 用户处理器
async fn list_users(
    State(state): State<AppState>,
    Query(params): Query<ListQuery>,
) -> Json<Vec<User>> {
    let users = state.users.read().await;
    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(10);

    let start = ((page - 1) * per_page) as usize;
    let end = (start + per_page as usize).min(users.len());

    // Bounds check
    if start >= users.len() {
        return Json(vec![]);
    }

    Json(users[start..end].to_vec())
}

async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<User>, StatusCode> {
    let users = state.users.read().await;
    users
        .iter()
        .find(|u| u.id == id)
        .cloned()
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> (StatusCode, Json<User>) {
    let mut users = state.users.write().await;
    let id = users.iter().map(|u| u.id).max().unwrap_or(0) + 1;

    let user = User {
        id,
        username: payload.username,
        email: payload.email,
    };

    users.push(user.clone());
    (StatusCode::CREATED, Json(user))
}

async fn update_user(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<Json<User>, StatusCode> {
    let mut users = state.users.write().await;

    let user = users
        .iter_mut()
        .find(|u| u.id == id)
        .ok_or(StatusCode::NOT_FOUND)?;

    if let Some(username) = payload.username {
        user.username = username;
    }
    if let Some(email) = payload.email {
        user.email = email;
    }

    Ok(Json(user.clone()))
}

async fn delete_user(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> StatusCode {
    let mut users = state.users.write().await;

    if let Some(pos) = users.iter().position(|u| u.id == id) {
        users.remove(pos);
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}

// 文章处理器
async fn user_posts(
    State(state): State<AppState>,
    Path(user_id): Path<i32>,
) -> Json<Vec<Post>> {
    let posts = state.posts.read().await;
    let user_posts: Vec<Post> = posts
        .iter()
        .filter(|p| p.user_id == user_id)
        .cloned()
        .collect();

    Json(user_posts)
}

// 路由配置
fn user_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(list_users).post(create_user))
        .route("/:id", get(get_user).put(update_user).delete(delete_user))
        .route("/:id/posts", get(user_posts))
}

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    let state = AppState {
        users: Arc::new(RwLock::new(vec![
            User {
                id: 1,
                username: "alice".to_string(),
                email: "alice@example.com".to_string(),
            },
        ])),
        posts: Arc::new(RwLock::new(vec![
            Post {
                id: 1,
                user_id: 1,
                title: "First Post".into(),
                content: "Hello World".into(),
            }
        ])),
    };

    let app = Router::new()
        .nest("/api/users", user_routes())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("🚀 Server running on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}
