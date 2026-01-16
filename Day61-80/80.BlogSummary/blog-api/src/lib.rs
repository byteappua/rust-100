use axum::{
    routing::{delete, get, post, put},
    Router,
};
use sqlx::SqlitePool;

pub mod api;
pub mod app_state;
pub mod middleware;
pub mod models;
pub mod repository;
pub mod utils;

use api::{auth, post as post_api};
pub use app_state::AppState;
use middleware as auth_middleware;

pub async fn create_app(pool: SqlitePool) -> Router {
    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    let user_repo = repository::UserRepository::new(pool.clone());
    let post_repo = repository::PostRepository::new(pool.clone());

    let state = AppState {
        user_repository: user_repo,
        post_repository: post_repo,
    };

    let public_routes = Router::new()
        .route("/posts", get(post_api::list))
        .route("/posts/:id", get(post_api::get_post));

    let protected_routes = Router::new()
        .route("/posts", post(post_api::create))
        .route("/posts/:id", put(post_api::update).delete(post_api::delete))
        .layer(axum::middleware::from_fn(auth_middleware::auth));

    let api_routes = Router::new().merge(public_routes).merge(protected_routes);

    let auth_routes = Router::new()
        .route("/register", post(auth::register))
        .route("/login", post(auth::login));

    Router::new()
        .route("/", get(root))
        .nest("/api/auth", auth_routes)
        .nest("/api", api_routes)
        .with_state(state)
}

async fn root() -> &'static str {
    "Hello, Blog API!"
}
