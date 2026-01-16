use axum::{
    routing::{get, post},
    Router,
    extract::{Path, Json},
    response::IntoResponse,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
struct User {
    id: u64,
    username: String,
    email: String,
}

#[derive(Deserialize)]
struct CreateUser {
    username: String,
    email: String,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/api/v1/users", get(list_users).post(create_user))
        .route("/api/v1/users/:id", get(get_user));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("🚀 Server running on http://127.0.0.1:3000");

    axum::serve(listener, app)
        .await
        .unwrap();
}

async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, "OK")
}

async fn list_users() -> Json<Vec<User>> {
    let users = vec![
        User { id: 1, username: "alice".into(), email: "alice@example.com".into() },
        User { id: 2, username: "bob".into(), email: "bob@example.com".into() },
    ];
    Json(users)
}

async fn create_user(Json(payload): Json<CreateUser>) -> impl IntoResponse {
    let user = User {
        id: 1337,
        username: payload.username,
        email: payload.email,
    };
    (StatusCode::CREATED, Json(user))
}

async fn get_user(Path(id): Path<u64>) -> impl IntoResponse {
    let user = User {
        id,
        username: format!("user_{}", id),
        email: format!("user_{}@example.com", id),
    };
    Json(user)
}
