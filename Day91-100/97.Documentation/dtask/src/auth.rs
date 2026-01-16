use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use utoipa::ToSchema;

/// Secret key for signing JWTs.
/// WARNING: In a real application, this should be loaded from environment variables.
pub const SECRET: &str = "secret_key";

/// JWT Claims structure.
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    /// Subject (username).
    pub sub: String,
    /// Expiration time.
    pub exp: usize,
}

/// Request body for login endpoint.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct LoginRequest {
    /// Username for authentication.
    #[schema(example = "admin")]
    pub username: String,
    /// Password for authentication.
    #[schema(example = "password")]
    pub password: String,
}

/// Response body for successful login.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct LoginResponse {
    /// The generated JWT token.
    pub token: String,
}

/// Creates a new JWT for the given username.
///
/// The token is valid for 1 hour.
pub fn create_jwt(username: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize
        + 3600; // 1 hour

    let claims = Claims {
        sub: username.to_owned(),
        exp: expiration,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(SECRET.as_ref()),
    )
}

/// Verifies a JWT and returns the claims if valid.
pub fn verify_jwt(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let validation = Validation::new(Algorithm::HS256);
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(SECRET.as_ref()),
        &validation,
    )?;
    Ok(token_data.claims)
}
