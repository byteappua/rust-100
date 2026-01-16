use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use serde::Serialize;
use tower_http::{
    cors::CorsLayer,
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "frontend_integration_demo=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Define API routes
    let api_routes = Router::new()
        .route("/health", get(health_check))
        .route("/users", get(list_users));

    // Define static file service
    // Serve assets from the "assets" directory
    // In a real project, this might be "dist" or "build"
    let assets_path = std::env::current_dir().unwrap().join("assets");

    // ServeDir::new will serve files relative to assets_path.
    // If a file is not found, we fallback to index.html (SPA Routing).
    let spa_service = ServeDir::new(&assets_path)
        .fallback(ServeFile::new(assets_path.join("index.html")));

    // Build the application router
    let app = Router::new()
        .nest("/api", api_routes) // Mount API routes at /api
        .fallback_service(spa_service) // Handle all other routes with static file service
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive());

    // Run the server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    tracing::info!("serving assets from {}", assets_path.display());
    axum::serve(listener, app).await.unwrap();
}

async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, "OK")
}

#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
    role: String,
}

async fn list_users() -> Json<Vec<User>> {
    let users = vec![
        User {
            id: 1,
            username: "Alice".to_string(),
            role: "Admin".to_string(),
        },
        User {
            id: 2,
            username: "Bob".to_string(),
            role: "User".to_string(),
        },
        User {
            id: 3,
            username: "Charlie".to_string(),
            role: "Guest".to_string(),
        },
    ];
    Json(users)
}
