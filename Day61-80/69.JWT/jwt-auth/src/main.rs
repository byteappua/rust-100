use axum::{
    routing::{get, post},
    extract::{Json, Request},
    http::{StatusCode, HeaderMap},
    response::{Response, IntoResponse},
    middleware::{self, Next},
    Router,
    Extension,
};
use serde::{Deserialize, Serialize};
use tower_http::trace::TraceLayer;
use crate::jwt::{create_jwt, verify_jwt, Claims};

mod jwt;

#[derive(Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct LoginResponse {
    token: String,
}

async fn login(Json(payload): Json<LoginRequest>) -> Result<Json<LoginResponse>, StatusCode> {
    // Demo: Only accept "admin" / "password"
    if payload.username == "admin" && payload.password == "password" {
        let token = create_jwt(&payload.username).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        Ok(Json(LoginResponse { token }))
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

async fn protected(Extension(claims): Extension<Claims>) -> String {
    format!("Welcome to the protected area, {}!", claims.sub)
}

async fn auth_middleware(
    headers: HeaderMap,
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth_header = headers
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    if !auth_header.starts_with("Bearer ") {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let token = &auth_header[7..];
    match verify_jwt(token) {
        Ok(claims) => {
            req.extensions_mut().insert(claims);
            Ok(next.run(req).await)
        }
        Err(_) => Err(StatusCode::UNAUTHORIZED),
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/login", post(login))
        .route("/protected", get(protected).layer(middleware::from_fn(auth_middleware)))
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("🚀 Server running on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}
