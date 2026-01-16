use crate::{
    utils::{jwt, password},
    AppState,
};
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(length(min = 3))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 6))]
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub token: String,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> impl IntoResponse {
    if let Err(e) = payload.validate() {
        return (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: e.to_string(),
            }),
        )
            .into_response();
    }

    let password_hash = match password::hash_password(&payload.password) {
        Ok(hash) => hash,
        Err(_) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to hash password").into_response()
        }
    };

    match state
        .user_repository
        .create(&payload.username, &payload.email, &password_hash)
        .await
    {
        Ok(user) => {
            let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "secret".to_string());
            match jwt::create_jwt(&user.username, &secret) {
                Ok(token) => (StatusCode::CREATED, Json(AuthResponse { token })).into_response(),
                Err(_) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to generate token",
                )
                    .into_response(),
            }
        }
        Err(e) => (
            StatusCode::CONFLICT,
            Json(ErrorResponse {
                error: e.to_string(),
            }),
        )
            .into_response(),
    }
}

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> impl IntoResponse {
    let user = match state.user_repository.find_by_email(&payload.email).await {
        Ok(Some(user)) => user,
        Ok(None) => return (StatusCode::UNAUTHORIZED, "Invalid credentials").into_response(),
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };

    let valid = match password::verify_password(&payload.password, &user.password_hash) {
        Ok(v) => v,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to verify password",
            )
                .into_response()
        }
    };

    if !valid {
        return (StatusCode::UNAUTHORIZED, "Invalid credentials").into_response();
    }

    let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "secret".to_string());
    match jwt::create_jwt(&user.username, &secret) {
        Ok(token) => (StatusCode::OK, Json(AuthResponse { token })).into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to generate token",
        )
            .into_response(),
    }
}
