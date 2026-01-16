use crate::utils::jwt;
use axum::{
    extract::Request,
    http::{header::AUTHORIZATION, StatusCode},
    middleware::Next,
    response::Response,
};

#[derive(Clone, Debug)]
pub struct AuthUser {
    pub username: String,
}

pub async fn auth(mut req: Request, next: Next) -> Result<Response, StatusCode> {
    let auth_header = req
        .headers()
        .get(AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    let auth_header = if let Some(auth_header) = auth_header {
        auth_header
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    if !auth_header.starts_with("Bearer ") {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let token = &auth_header[7..];
    let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "secret".to_string());

    match jwt::decode_jwt(token, &secret) {
        Ok(claims) => {
            req.extensions_mut().insert(AuthUser {
                username: claims.sub,
            });
            Ok(next.run(req).await)
        }
        Err(_) => Err(StatusCode::UNAUTHORIZED),
    }
}
