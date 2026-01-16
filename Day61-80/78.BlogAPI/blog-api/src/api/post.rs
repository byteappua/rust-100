use crate::{middleware::AuthUser, AppState};
use axum::{
    extract::{Extension, Path, State},
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreatePostRequest {
    pub title: String,
    pub slug: String,
    pub content: String,
}

#[derive(Deserialize)]
pub struct UpdatePostRequest {
    pub title: String,
    pub slug: String,
    pub content: String,
    pub published: bool,
}

pub async fn list(State(state): State<AppState>) -> impl IntoResponse {
    match state.post_repository.find_all().await {
        Ok(posts) => (StatusCode::OK, Json(posts)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn get_post(State(state): State<AppState>, Path(id): Path<i64>) -> impl IntoResponse {
    match state.post_repository.find_by_id(id).await {
        Ok(Some(post)) => (StatusCode::OK, Json(post)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Post not found").into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn create(
    State(state): State<AppState>,
    Extension(user): Extension<AuthUser>,
    Json(payload): Json<CreatePostRequest>,
) -> impl IntoResponse {
    let user_record = match state.user_repository.find_by_username(&user.username).await {
        Ok(Some(u)) => u,
        Ok(None) => return (StatusCode::UNAUTHORIZED, "User not found").into_response(),
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };

    match state
        .post_repository
        .create(
            &payload.title,
            &payload.slug,
            &payload.content,
            user_record.id,
        )
        .await
    {
        Ok(post) => (StatusCode::CREATED, Json(post)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn update(
    State(state): State<AppState>,
    Extension(_user): Extension<AuthUser>, // In real app, check if user owns post
    Path(id): Path<i64>,
    Json(payload): Json<UpdatePostRequest>,
) -> impl IntoResponse {
    match state
        .post_repository
        .update(
            id,
            &payload.title,
            &payload.slug,
            &payload.content,
            payload.published,
        )
        .await
    {
        Ok(Some(post)) => (StatusCode::OK, Json(post)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Post not found").into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn delete(
    State(state): State<AppState>,
    Extension(_user): Extension<AuthUser>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    match state.post_repository.delete(id).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}
