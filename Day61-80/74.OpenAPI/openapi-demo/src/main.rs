use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;

#[derive(Serialize, Deserialize, ToSchema, Clone)]
struct User {
    id: i32,
    username: String,
    email: String,
}

#[derive(Serialize, Deserialize, ToSchema, Clone)]
struct CreateUser {
    username: String,
    email: String,
}

#[derive(OpenApi)]
#[openapi(
    paths(
        list_users,
        create_user,
        get_user,
    ),
    components(
        schemas(User, CreateUser)
    ),
    tags(
        (name = "users", description = "User management API")
    )
)]
struct ApiDoc;

type UserDb = Arc<RwLock<Vec<User>>>;

#[tokio::main]
async fn main() {
    // Shared state
    let db = Arc::new(RwLock::new(vec![
        User {
            id: 1,
            username: "alice".into(),
            email: "alice@example.com".into(),
        },
        User {
            id: 2,
            username: "bob".into(),
            email: "bob@example.com".into(),
        },
    ]));

    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route("/users", get(list_users).post(create_user))
        .route("/users/:id", get(get_user))
        .with_state(db);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    println!("Swagger UI: http://localhost:3000/swagger-ui");
    axum::serve(listener, app).await.unwrap();
}

/// List all users
#[utoipa::path(
    get,
    path = "/users",
    responses(
        (status = 200, description = "List all users", body = [User])
    ),
    tag = "users"
)]
async fn list_users(State(db): State<UserDb>) -> Json<Vec<User>> {
    let users = db.read().unwrap();
    Json(users.clone())
}

/// Create a new user
#[utoipa::path(
    post,
    path = "/users",
    request_body = CreateUser,
    responses(
        (status = 201, description = "User created", body = User)
    ),
    tag = "users"
)]
async fn create_user(
    State(db): State<UserDb>,
    Json(payload): Json<CreateUser>,
) -> impl IntoResponse {
    let mut users = db.write().unwrap();
    let id = users.len() as i32 + 1;
    let user = User {
        id,
        username: payload.username,
        email: payload.email,
    };
    users.push(user.clone());
    (StatusCode::CREATED, Json(user))
}

/// Get a user by ID
#[utoipa::path(
    get,
    path = "/users/{id}",
    params(
        ("id" = i32, Path, description = "User database ID")
    ),
    responses(
        (status = 200, description = "User found", body = User),
        (status = 404, description = "User not found")
    ),
    tag = "users"
)]
async fn get_user(Path(id): Path<i32>, State(db): State<UserDb>) -> impl IntoResponse {
    let users = db.read().unwrap();
    if let Some(user) = users.iter().find(|u| u.id == id) {
        Ok(Json(user.clone()))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}
