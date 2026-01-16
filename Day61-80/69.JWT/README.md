# Day 69: 用户认证 (JWT)

## 学习目标
- 理解 JWT 工作原理
- 实现用户注册和登录
- 掌握密码安全存储
- 实现认证中间件

## JWT 简介

JWT (JSON Web Token) 是一种开放标准 (RFC 7519)，用于在各方之间安全地传输信息。

### JWT 结构

```
Header.Payload.Signature
```

**Header (头部)**
```json
{
  "alg": "HS256",
  "typ": "JWT"
}
```

**Payload (载荷)**
```json
{
  "sub": "user_id",
  "name": "John Doe",
  "exp": 1516239022
}
```

**Signature (签名)**
```
HMACSHA256(
  base64UrlEncode(header) + "." +
  base64UrlEncode(payload),
  secret
)
```

## 项目设置

### Cargo.toml

```toml
[package]
name = "jwt_auth"
version = "0.1.0"
edition = "2021"

[dependencies]
axum = "0.7"
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
jsonwebtoken = "9"
bcrypt = "0.15"
sqlx = { version = "0.7", features = ["runtime-tokio-rustls", "postgres"] }
chrono = { version = "0.4", features = ["serde"] }
tower = "0.4"
tower-http = { version = "0.5", features = ["trace"] }
tracing = "0.1"
tracing-subscriber = "0.3"
```

## 完整实现

### 1. 数据模型

```rust
// src/models.rs
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserResponse,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: i32,
    pub username: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,  // user_id
    pub username: String,
    pub exp: usize,   // 过期时间
}
```

### 2. JWT 工具函数

```rust
// src/jwt.rs
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use chrono::{Utc, Duration};
use crate::models::Claims;

const SECRET: &str = "your-secret-key"; // 生产环境应从环境变量读取

pub fn create_jwt(user_id: i32, username: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;
    
    let claims = Claims {
        sub: user_id.to_string(),
        username: username.to_string(),
        exp: expiration,
    };
    
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(SECRET.as_ref()),
    )
}

pub fn verify_jwt(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(SECRET.as_ref()),
        &Validation::default(),
    )
    .map(|data| data.claims)
}
```

### 3. 密码处理

```rust
// src/password.rs
use bcrypt::{hash, verify, DEFAULT_COST};

pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    hash(password, DEFAULT_COST)
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, bcrypt::BcryptError> {
    verify(password, hash)
}
```

### 4. 认证服务

```rust
// src/auth.rs
use sqlx::PgPool;
use crate::models::*;
use crate::jwt::create_jwt;
use crate::password::{hash_password, verify_password};

pub struct AuthService {
    pool: PgPool,
}

impl AuthService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
    
    pub async fn register(&self, req: RegisterRequest) -> Result<AuthResponse, String> {
        // 检查用户名是否已存在
        let existing = sqlx::query!(
            "SELECT id FROM users WHERE username = $1 OR email = $2",
            req.username,
            req.email
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())?;
        
        if existing.is_some() {
            return Err("Username or email already exists".to_string());
        }
        
        // 哈希密码
        let password_hash = hash_password(&req.password)
            .map_err(|e| e.to_string())?;
        
        // 创建用户
        let user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (username, email, password_hash)
            VALUES ($1, $2, $3)
            RETURNING id, username, email, password_hash, created_at
            "#,
            req.username,
            req.email,
            password_hash
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())?;
        
        // 生成 JWT
        let token = create_jwt(user.id, &user.username)
            .map_err(|e| e.to_string())?;
        
        Ok(AuthResponse {
            token,
            user: UserResponse {
                id: user.id,
                username: user.username,
                email: user.email,
            },
        })
    }
    
    pub async fn login(&self, req: LoginRequest) -> Result<AuthResponse, String> {
        // 查找用户
        let user = sqlx::query_as!(
            User,
            "SELECT * FROM users WHERE username = $1",
            req.username
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())?
        .ok_or("Invalid credentials")?;
        
        // 验证密码
        let valid = verify_password(&req.password, &user.password_hash)
            .map_err(|e| e.to_string())?;
        
        if !valid {
            return Err("Invalid credentials".to_string());
        }
        
        // 生成 JWT
        let token = create_jwt(user.id, &user.username)
            .map_err(|e| e.to_string())?;
        
        Ok(AuthResponse {
            token,
            user: UserResponse {
                id: user.id,
                username: user.username,
                email: user.email,
            },
        })
    }
}
```

### 5. 认证中间件

```rust
// src/middleware.rs
use axum::{
    extract::Request,
    http::{StatusCode, HeaderMap},
    middleware::Next,
    response::Response,
};
use crate::jwt::verify_jwt;
use crate::models::Claims;

pub async fn auth_middleware(
    headers: HeaderMap,
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth_header = headers
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;
    
    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(StatusCode::UNAUTHORIZED)?;
    
    let claims = verify_jwt(token)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;
    
    // 将 claims 添加到请求扩展中
    req.extensions_mut().insert(claims);
    
    Ok(next.run(req).await)
}
```

### 6. API 处理器

```rust
// src/handlers.rs
use axum::{
    extract::{State, Extension},
    http::StatusCode,
    Json,
};
use crate::auth::AuthService;
use crate::models::*;

pub async fn register(
    State(service): State<AuthService>,
    Json(req): Json<RegisterRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, String)> {
    service
        .register(req)
        .await
        .map(Json)
        .map_err(|e| (StatusCode::BAD_REQUEST, e))
}

pub async fn login(
    State(service): State<AuthService>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, String)> {
    service
        .login(req)
        .await
        .map(Json)
        .map_err(|e| (StatusCode::UNAUTHORIZED, e))
}

pub async fn me(
    Extension(claims): Extension<Claims>,
) -> Json<Claims> {
    Json(claims)
}

pub async fn protected_route(
    Extension(claims): Extension<Claims>,
) -> String {
    format!("Hello, {}! This is a protected route.", claims.username)
}
```

### 7. 主程序

```rust
// src/main.rs
use axum::{
    routing::{get, post},
    Router,
    middleware,
};
use sqlx::postgres::PgPoolOptions;
use tower_http::trace::TraceLayer;

mod models;
mod jwt;
mod password;
mod auth;
mod handlers;
mod middleware as mw;

#[tokio::main]
async fn main() {
    // 初始化日志
    tracing_subscriber::fmt::init();
    
    // 连接数据库
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://localhost/auth_db")
        .await
        .expect("Failed to connect to database");
    
    // 创建服务
    let auth_service = auth::AuthService::new(pool);
    
    // 公开路由
    let public_routes = Router::new()
        .route("/register", post(handlers::register))
        .route("/login", post(handlers::login))
        .with_state(auth_service);
    
    // 受保护路由
    let protected_routes = Router::new()
        .route("/me", get(handlers::me))
        .route("/protected", get(handlers::protected_route))
        .layer(middleware::from_fn(mw::auth_middleware));
    
    // 组合路由
    let app = Router::new()
        .nest("/api/auth", public_routes)
        .nest("/api", protected_routes)
        .layer(TraceLayer::new_for_http());
    
    // 启动服务器
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    
    println!("🚀 Server running on http://127.0.0.1:3000");
    
    axum::serve(listener, app).await.unwrap();
}
```

### 8. 数据库迁移

```sql
-- migrations/001_create_users.sql
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(50) UNIQUE NOT NULL,
    email VARCHAR(100) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_users_username ON users(username);
CREATE INDEX idx_users_email ON users(email);
```

## 测试 API

### 注册用户

```bash
curl -X POST http://localhost:3000/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "username": "alice",
    "email": "alice@example.com",
    "password": "password123"
  }'
```

### 登录

```bash
curl -X POST http://localhost:3000/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "username": "alice",
    "password": "password123"
  }'
```

### 访问受保护路由

```bash
TOKEN="your-jwt-token"

curl http://localhost:3000/api/me \
  -H "Authorization: Bearer $TOKEN"

curl http://localhost:3000/api/protected \
  -H "Authorization: Bearer $TOKEN"
```

## 安全最佳实践

### 1. 密码安全
- ✅ 使用 bcrypt/argon2 哈希
- ✅ 设置最小密码长度
- ✅ 要求密码复杂度
- ✅ 防止暴力破解

### 2. JWT 安全
- ✅ 使用强密钥
- ✅ 设置合理过期时间
- ✅ 使用 HTTPS 传输
- ✅ 实现刷新令牌机制

### 3. API 安全
- ✅ 限流保护
- ✅ CORS 配置
- ✅ 输入验证
- ✅ SQL 注入防护

## 进阶功能

### 刷新令牌

```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshToken {
    pub token: String,
    pub user_id: i32,
    pub expires_at: chrono::DateTime<chrono::Utc>,
}

pub async fn refresh_token(
    State(service): State<AuthService>,
    Json(req): Json<RefreshTokenRequest>,
) -> Result<Json<AuthResponse>, StatusCode> {
    // 验证刷新令牌
    // 生成新的访问令牌
    todo!()
}
```

### 密码重置

```rust
pub async fn forgot_password(
    State(service): State<AuthService>,
    Json(req): Json<ForgotPasswordRequest>,
) -> Result<StatusCode, StatusCode> {
    // 生成重置令牌
    // 发送重置邮件
    todo!()
}

pub async fn reset_password(
    State(service): State<AuthService>,
    Json(req): Json<ResetPasswordRequest>,
) -> Result<StatusCode, StatusCode> {
    // 验证重置令牌
    // 更新密码
    todo!()
}
```

## 练习

1. 实现刷新令牌机制
2. 添加邮箱验证功能
3. 实现密码重置流程
4. 添加 OAuth2 登录
5. 实现多因素认证 (MFA)

## 下一步

Day 70 将学习基于角色的访问控制 (RBAC)，实现更细粒度的权限管理。
