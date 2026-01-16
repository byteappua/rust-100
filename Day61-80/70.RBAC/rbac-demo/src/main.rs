use axum::{
    routing::get,
    extract::{Request, Extension},
    middleware::{self, Next},
    response::{Response, IntoResponse},
    http::StatusCode,
    Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use tower_http::trace::TraceLayer;

// --- Mocking User Identity ---
// In a real app, this comes from JWT or Session
#[derive(Debug, Clone, Serialize, Deserialize)]
struct AuthUser {
    username: String,
    roles: HashSet<String>,
}

impl AuthUser {
    fn has_role(&self, role: &str) -> bool {
        self.roles.contains(role)
    }
}

// --- Middleware: Mock Authentication ---
async fn mock_auth_middleware(
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let username_header = req.headers()
        .get("x-user")
        .and_then(|h| h.to_str().ok());

    let user = match username_header {
        Some("admin") => AuthUser {
            username: "admin".into(),
            roles: vec!["admin".into(), "user".into()].into_iter().collect(),
        },
        Some("user") => AuthUser {
            username: "user".into(),
            roles: vec!["user".into()].into_iter().collect(),
        },
        _ => return Err(StatusCode::UNAUTHORIZED),
    };

    req.extensions_mut().insert(user);
    Ok(next.run(req).await)
}

// --- Middleware: Role Guard ---
// Note: Middleware in axum usually takes (State, Request, Next) or (Request, Next).
// To pass arguments like 'role', we can use a closure with `from_fn` or `from_fn_with_state` if state is the role.
// However, `from_fn_with_state` expects the state to be the Router's state, or compatible.
// Here we are creating a specific middleware function for each role check using a closure that captures the role string.

async fn check_role(
    req: Request,
    next: Next,
    required_role: String,
) -> Result<Response, StatusCode> {
    let user = req.extensions().get::<AuthUser>().ok_or(StatusCode::UNAUTHORIZED)?;

    if user.has_role(&required_role) {
        Ok(next.run(req).await)
    } else {
        Err(StatusCode::FORBIDDEN)
    }
}

// --- Handlers ---
async fn admin_dashboard() -> &'static str {
    "Welcome Admin!"
}

async fn user_dashboard() -> &'static str {
    "Welcome User!"
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // Routes requiring "admin" role
    let admin_routes = Router::new()
        .route("/dashboard", get(admin_dashboard))
        .layer(middleware::from_fn(|req, next| check_role(req, next, "admin".to_string())));

    // Routes requiring "user" role
    let user_routes = Router::new()
        .route("/dashboard", get(user_dashboard))
        .layer(middleware::from_fn(|req, next| check_role(req, next, "user".to_string())));

    let app = Router::new()
        .nest("/admin", admin_routes)
        .nest("/user", user_routes)
        .layer(middleware::from_fn(mock_auth_middleware))
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("🚀 Server running on http://127.0.0.1:3000");
    println!("   Try: curl -H 'x-user: admin' http://127.0.0.1:3000/admin/dashboard");
    println!("   Try: curl -H 'x-user: user' http://127.0.0.1:3000/admin/dashboard (Should 403)");

    axum::serve(listener, app).await.unwrap();
}
