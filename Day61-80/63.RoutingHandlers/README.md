# Day 63: 路由与请求处理

## 学习目标
- 掌握嵌套路由
- 理解路径参数和查询参数
- 学习请求体解析
- 实现自定义提取器

## 嵌套路由

### Axum 嵌套路由

```rust
use axum::{
    routing::{get, post, put, delete},
    Router,
};

// API v1 路由
fn api_v1_routes() -> Router {
    Router::new()
        .route("/users", get(list_users).post(create_user))
        .route("/users/:id", get(get_user).put(update_user).delete(delete_user))
        .route("/posts", get(list_posts).post(create_post))
        .route("/posts/:id", get(get_post))
}

// API v2 路由
fn api_v2_routes() -> Router {
    Router::new()
        .route("/users", get(list_users_v2))
        .route("/posts", get(list_posts_v2))
}

// 主应用
fn app() -> Router {
    Router::new()
        .route("/health", get(health_check))
        .nest("/api/v1", api_v1_routes())
        .nest("/api/v2", api_v2_routes())
        .fallback(not_found)
}
```

### 路由组

```rust
use axum::Router;

// 用户相关路由
fn user_routes() -> Router {
    Router::new()
        .route("/", get(list_users).post(create_user))
        .route("/:id", get(get_user).put(update_user).delete(delete_user))
        .route("/:id/posts", get(user_posts))
        .route("/:id/profile", get(user_profile))
}

// 文章相关路由
fn post_routes() -> Router {
    Router::new()
        .route("/", get(list_posts).post(create_post))
        .route("/:id", get(get_post).put(update_post).delete(delete_post))
        .route("/:id/comments", get(post_comments).post(create_comment))
}

// 组合路由
fn app() -> Router {
    Router::new()
        .nest("/users", user_routes())
        .nest("/posts", post_routes())
}
```

## 路径参数

### 单个参数

```rust
use axum::{
    extract::Path,
    Json,
};

// GET /users/:id
async fn get_user(Path(id): Path<i32>) -> Json<User> {
    let user = find_user_by_id(id).await;
    Json(user)
}

// GET /posts/:slug
async fn get_post_by_slug(Path(slug): Path<String>) -> Json<Post> {
    let post = find_post_by_slug(&slug).await;
    Json(post)
}
```

### 多个参数

```rust
// GET /users/:user_id/posts/:post_id
async fn get_user_post(
    Path((user_id, post_id)): Path<(i32, i32)>
) -> Json<Post> {
    let post = find_user_post(user_id, post_id).await;
    Json(post)
}

// 使用结构体
#[derive(Deserialize)]
struct PostPath {
    user_id: i32,
    post_id: i32,
}

async fn get_user_post_v2(
    Path(params): Path<PostPath>
) -> Json<Post> {
    let post = find_user_post(params.user_id, params.post_id).await;
    Json(post)
}
```

## 查询参数

### 基本查询参数

```rust
use axum::extract::Query;
use serde::Deserialize;

#[derive(Deserialize)]
struct Pagination {
    page: Option<u32>,
    per_page: Option<u32>,
}

// GET /users?page=1&per_page=20
async fn list_users(Query(params): Query<Pagination>) -> Json<Vec<User>> {
    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(10);
    
    let users = fetch_users(page, per_page).await;
    Json(users)
}
```

### 复杂查询参数

```rust
#[derive(Deserialize)]
struct SearchParams {
    q: Option<String>,
    category: Option<String>,
    tags: Option<Vec<String>>,
    sort: Option<String>,
    order: Option<String>,
    #[serde(flatten)]
    pagination: Pagination,
}

// GET /posts?q=rust&category=tech&tags=web&tags=async&sort=date&order=desc&page=1
async fn search_posts(Query(params): Query<SearchParams>) -> Json<Vec<Post>> {
    let posts = search_posts_with_params(params).await;
    Json(posts)
}
```

## 请求体解析

### JSON 请求体

```rust
use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct CreateUserRequest {
    username: String,
    email: String,
    password: String,
}

#[derive(Serialize)]
struct UserResponse {
    id: i32,
    username: String,
    email: String,
}

async fn create_user(
    Json(payload): Json<CreateUserRequest>
) -> (StatusCode, Json<UserResponse>) {
    let user = User::create(payload).await;
    
    (
        StatusCode::CREATED,
        Json(UserResponse {
            id: user.id,
            username: user.username,
            email: user.email,
        })
    )
}
```

### 表单数据

```rust
use axum::Form;

#[derive(Deserialize)]
struct LoginForm {
    username: String,
    password: String,
}

async fn login(Form(form): Form<LoginForm>) -> Result<String, StatusCode> {
    if verify_credentials(&form.username, &form.password).await {
        Ok("Login successful".to_string())
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}
```

### Multipart 文件上传

```rust
use axum::extract::Multipart;

async fn upload_file(mut multipart: Multipart) -> Result<String, StatusCode> {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let data = field.bytes().await.unwrap();
        
        if name == "file" {
            // 保存文件
            tokio::fs::write(format!("uploads/{}", name), data)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        }
    }
    
    Ok("File uploaded successfully".to_string())
}
```

## 自定义提取器

### 简单提取器

```rust
use axum::{
    async_trait,
    extract::{FromRequestParts, rejection::JsonRejection},
    http::request::Parts,
};

// 自定义 JSON 提取器，带验证
struct ValidatedJson<T>(T);

#[async_trait]
impl<S, T> FromRequestParts<S> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);
    
    async fn from_request_parts(
        parts: &mut Parts,
        state: &S
    ) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request_parts(parts, state)
            .await
            .map_err(|err| {
                (StatusCode::BAD_REQUEST, format!("Invalid JSON: {}", err))
            })?;
        
        value.validate().map_err(|err| {
            (StatusCode::BAD_REQUEST, format!("Validation error: {}", err))
        })?;
        
        Ok(ValidatedJson(value))
    }
}

// 使用
async fn create_user(
    ValidatedJson(user): ValidatedJson<CreateUserRequest>
) -> Json<UserResponse> {
    // user 已经通过验证
    let created = User::create(user).await;
    Json(created)
}
```

### 认证提取器

```rust
use axum::http::HeaderMap;

struct AuthUser {
    id: i32,
    username: String,
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = StatusCode;
    
    async fn from_request_parts(
        parts: &mut Parts,
        _state: &S
    ) -> Result<Self, Self::Rejection> {
        let headers = &parts.headers;
        
        let token = headers
            .get("Authorization")
            .and_then(|h| h.to_str().ok())
            .and_then(|h| h.strip_prefix("Bearer "))
            .ok_or(StatusCode::UNAUTHORIZED)?;
        
        let claims = verify_jwt(token)
            .map_err(|_| StatusCode::UNAUTHORIZED)?;
        
        Ok(AuthUser {
            id: claims.user_id,
            username: claims.username,
        })
    }
}

// 使用
async fn get_profile(user: AuthUser) -> Json<Profile> {
    let profile = fetch_profile(user.id).await;
    Json(profile)
}
```

## 请求头处理

```rust
use axum::http::{HeaderMap, HeaderValue};

async fn handle_headers(headers: HeaderMap) -> String {
    let user_agent = headers
        .get("user-agent")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("unknown");
    
    let accept = headers
        .get("accept")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("*/*");
    
    format!("User-Agent: {}, Accept: {}", user_agent, accept)
}

// 设置响应头
async fn with_custom_headers() -> (HeaderMap, String) {
    let mut headers = HeaderMap::new();
    headers.insert("X-Custom-Header", HeaderValue::from_static("value"));
    headers.insert("X-Request-Id", HeaderValue::from_static("123"));
    
    (headers, "Response with custom headers".to_string())
}
```

## 完整示例

```rust
use axum::{
    extract::{Path, Query, State, Json},
    http::StatusCode,
    routing::{get, post, put, delete},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

// 数据模型
#[derive(Debug, Clone, Serialize, Deserialize)]
struct User {
    id: i32,
    username: String,
    email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Post {
    id: i32,
    user_id: i32,
    title: String,
    content: String,
}

// 请求/响应模型
#[derive(Deserialize)]
struct CreateUserRequest {
    username: String,
    email: String,
}

#[derive(Deserialize)]
struct UpdateUserRequest {
    username: Option<String>,
    email: Option<String>,
}

#[derive(Deserialize)]
struct ListQuery {
    page: Option<u32>,
    per_page: Option<u32>,
    sort: Option<String>,
}

// 应用状态
#[derive(Clone)]
struct AppState {
    users: Arc<RwLock<Vec<User>>>,
    posts: Arc<RwLock<Vec<Post>>>,
}

// 用户处理器
async fn list_users(
    State(state): State<AppState>,
    Query(params): Query<ListQuery>,
) -> Json<Vec<User>> {
    let users = state.users.read().await;
    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(10);
    
    let start = ((page - 1) * per_page) as usize;
    let end = (start + per_page as usize).min(users.len());
    
    Json(users[start..end].to_vec())
}

async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<User>, StatusCode> {
    let users = state.users.read().await;
    users
        .iter()
        .find(|u| u.id == id)
        .cloned()
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> (StatusCode, Json<User>) {
    let mut users = state.users.write().await;
    let id = users.len() as i32 + 1;
    
    let user = User {
        id,
        username: payload.username,
        email: payload.email,
    };
    
    users.push(user.clone());
    (StatusCode::CREATED, Json(user))
}

async fn update_user(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<Json<User>, StatusCode> {
    let mut users = state.users.write().await;
    
    let user = users
        .iter_mut()
        .find(|u| u.id == id)
        .ok_or(StatusCode::NOT_FOUND)?;
    
    if let Some(username) = payload.username {
        user.username = username;
    }
    if let Some(email) = payload.email {
        user.email = email;
    }
    
    Ok(Json(user.clone()))
}

async fn delete_user(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> StatusCode {
    let mut users = state.users.write().await;
    
    if let Some(pos) = users.iter().position(|u| u.id == id) {
        users.remove(pos);
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}

// 文章处理器
async fn user_posts(
    State(state): State<AppState>,
    Path(user_id): Path<i32>,
) -> Json<Vec<Post>> {
    let posts = state.posts.read().await;
    let user_posts: Vec<Post> = posts
        .iter()
        .filter(|p| p.user_id == user_id)
        .cloned()
        .collect();
    
    Json(user_posts)
}

// 路由配置
fn user_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(list_users).post(create_user))
        .route("/:id", get(get_user).put(update_user).delete(delete_user))
        .route("/:id/posts", get(user_posts))
}

#[tokio::main]
async fn main() {
    let state = AppState {
        users: Arc::new(RwLock::new(vec![
            User {
                id: 1,
                username: "alice".to_string(),
                email: "alice@example.com".to_string(),
            },
        ])),
        posts: Arc::new(RwLock::new(vec![])),
    };
    
    let app = Router::new()
        .nest("/api/users", user_routes())
        .with_state(state);
    
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    
    println!("🚀 Server running on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}
```

## 测试 API

```bash
# 列出用户
curl http://localhost:3000/api/users

# 分页查询
curl "http://localhost:3000/api/users?page=1&per_page=5"

# 获取单个用户
curl http://localhost:3000/api/users/1

# 创建用户
curl -X POST http://localhost:3000/api/users \
  -H "Content-Type: application/json" \
  -d '{"username":"bob","email":"bob@example.com"}'

# 更新用户
curl -X PUT http://localhost:3000/api/users/1 \
  -H "Content-Type: application/json" \
  -d '{"username":"alice_updated"}'

# 删除用户
curl -X DELETE http://localhost:3000/api/users/1

# 获取用户的文章
curl http://localhost:3000/api/users/1/posts
```

## 练习

1. 实现文章的 CRUD 操作
2. 添加搜索功能（支持多个查询参数）
3. 实现文件上传功能
4. 创建自定义提取器进行输入验证
5. 添加请求日志中间件

## 下一步

Day 64 将学习中间件开发，包括日志、认证、CORS 等。
