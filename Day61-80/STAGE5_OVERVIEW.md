# 第五阶段：Web 开发实战 (Days 61-80) - 完整概览

## 阶段目标
掌握 Rust Web 开发全栈技能，从框架基础到完整项目实战。

## 学习路径

### Week 1: Web 框架基础 (Days 61-65)

#### Day 61: Rust Web 开发概览 ✅
- Rust Web 生态系统介绍
- 主流框架对比（Axum, Actix-web, Rocket, Warp）
- HTTP 基础概念
- 异步 Web 开发模式

#### Day 62: Axum/Actix-web 入门 ✅
- Axum 核心概念和架构
- Actix-web Actor 模型
- 提取器（Extractors）详解
- 响应类型和错误处理

#### Day 63: 路由与请求处理
**核心内容：**
- 嵌套路由和路由组
- 路径参数和查询参数
- 请求体解析（JSON, Form, Multipart）
- 自定义提取器

**示例代码：**
```rust
// 嵌套路由
let api_routes = Router::new()
    .route("/users", get(list_users).post(create_user))
    .route("/users/:id", get(get_user).put(update_user).delete(delete_user))
    .route("/posts", get(list_posts).post(create_post));

let app = Router::new()
    .nest("/api/v1", api_routes)
    .route("/health", get(health_check));
```

#### Day 64: 中间件开发
**核心内容：**
- Tower 中间件系统
- 日志中间件
- 认证中间件
- CORS 和安全头
- 自定义中间件

**示例代码：**
```rust
use tower_http::{trace::TraceLayer, cors::CorsLayer};

let app = Router::new()
    .route("/api/users", get(list_users))
    .layer(TraceLayer::new_for_http())
    .layer(CorsLayer::permissive());
```

#### Day 65: 请求状态管理
**核心内容：**
- 应用状态共享
- Arc + RwLock 模式
- 状态注入
- 连接池管理

**示例代码：**
```rust
#[derive(Clone)]
struct AppState {
    db: Arc<RwLock<Database>>,
    cache: Arc<RwLock<Cache>>,
}

async fn handler(State(state): State<AppState>) -> Result<Json<Data>> {
    let db = state.db.read().await;
    // 使用数据库
}
```

### Week 2: 数据库与业务逻辑 (Days 66-70)

#### Day 66: 数据库迁移 (Migrations)
**核心内容：**
- SQLx 迁移系统
- 迁移文件编写
- 版本控制
- 回滚策略

**示例：**
```sql
-- migrations/20240101_create_users.sql
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(50) UNIQUE NOT NULL,
    email VARCHAR(100) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
```

#### Day 67: CRUD 操作实战
**核心内容：**
- SQLx 查询构建
- 参数化查询
- 事务处理
- 错误处理

**示例代码：**
```rust
async fn create_user(pool: &PgPool, user: CreateUser) -> Result<User> {
    let user = sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (username, email, password_hash)
        VALUES ($1, $2, $3)
        RETURNING id, username, email, created_at
        "#,
        user.username,
        user.email,
        user.password_hash
    )
    .fetch_one(pool)
    .await?;
    
    Ok(user)
}
```

#### Day 68: 复杂查询与事务
**核心内容：**
- JOIN 查询
- 聚合函数
- 子查询
- 事务管理
- 乐观锁和悲观锁

**示例代码：**
```rust
async fn transfer_money(
    pool: &PgPool,
    from_id: i32,
    to_id: i32,
    amount: Decimal,
) -> Result<()> {
    let mut tx = pool.begin().await?;
    
    sqlx::query!(
        "UPDATE accounts SET balance = balance - $1 WHERE id = $2",
        amount,
        from_id
    )
    .execute(&mut *tx)
    .await?;
    
    sqlx::query!(
        "UPDATE accounts SET balance = balance + $1 WHERE id = $2",
        amount,
        to_id
    )
    .execute(&mut *tx)
    .await?;
    
    tx.commit().await?;
    Ok(())
}
```

#### Day 69: 用户认证 (JWT)
**核心内容：**
- JWT 原理
- 令牌生成和验证
- 密码哈希（bcrypt/argon2）
- 登录/注册流程

**示例代码：**
```rust
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

fn create_jwt(user_id: &str) -> Result<String> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .unwrap()
        .timestamp() as usize;
    
    let claims = Claims {
        sub: user_id.to_owned(),
        exp: expiration,
    };
    
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret("secret".as_ref()),
    )
}
```

#### Day 70: 权限控制 (RBAC)
**核心内容：**
- 角色基础访问控制
- 权限模型设计
- 中间件实现
- 资源级权限

**示例代码：**
```rust
#[derive(Debug, Clone)]
enum Role {
    Admin,
    User,
    Guest,
}

#[derive(Debug, Clone)]
enum Permission {
    ReadUser,
    WriteUser,
    DeleteUser,
}

async fn require_permission(
    Extension(user): Extension<User>,
    permission: Permission,
) -> Result<(), StatusCode> {
    if user.has_permission(&permission) {
        Ok(())
    } else {
        Err(StatusCode::FORBIDDEN)
    }
}
```

### Week 3: 前端交互与部署 (Days 71-75)

#### Day 71: 模板引擎 (Askama/Tera)
**核心内容：**
- Askama 模板语法
- 模板继承
- 过滤器和函数
- 服务端渲染

**示例：**
```rust
use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    title: String,
    users: Vec<User>,
}

async fn index() -> impl IntoResponse {
    let template = IndexTemplate {
        title: "User List".to_string(),
        users: get_users().await,
    };
    
    HtmlTemplate(template)
}
```

#### Day 72: WebAssembly (WASM) 简介
**核心内容：**
- Rust to WASM 编译
- wasm-bindgen 使用
- 与 JavaScript 交互
- Yew 框架简介

**示例代码：**
```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[wasm_bindgen]
pub fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}
```

#### Day 73: 前端集成实战
**核心内容：**
- 静态文件服务
- SPA 路由配置
- API 代理
- 前后端分离架构

#### Day 74: API 文档生成 (OpenAPI)
**核心内容：**
- utoipa 库使用
- Swagger UI 集成
- API 文档自动生成
- 文档测试

**示例代码：**
```rust
use utoipa::{OpenApi, ToSchema};

#[derive(ToSchema, Serialize, Deserialize)]
struct User {
    id: i32,
    username: String,
    email: String,
}

#[utoipa::path(
    get,
    path = "/api/users/{id}",
    responses(
        (status = 200, description = "User found", body = User),
        (status = 404, description = "User not found")
    )
)]
async fn get_user(Path(id): Path<i32>) -> Result<Json<User>> {
    // Implementation
}
```

#### Day 75: 容器化部署 (Docker)
**核心内容：**
- Dockerfile 编写
- 多阶段构建
- Docker Compose
- 生产环境配置

**Dockerfile 示例：**
```dockerfile
FROM rust:1.75 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libssl3 ca-certificates
COPY --from=builder /app/target/release/app /usr/local/bin/app
EXPOSE 8080
CMD ["app"]
```

### Week 4: 完整项目实战 - 博客系统 (Days 76-80)

#### Day 76: 需求分析与设计
**功能需求：**
- 用户注册/登录
- 文章 CRUD
- 评论系统
- 标签和分类
- 搜索功能
- Markdown 支持

**数据库设计：**
```sql
-- 用户表
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(50) UNIQUE NOT NULL,
    email VARCHAR(100) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    role VARCHAR(20) DEFAULT 'user',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- 文章表
CREATE TABLE posts (
    id SERIAL PRIMARY KEY,
    title VARCHAR(200) NOT NULL,
    slug VARCHAR(200) UNIQUE NOT NULL,
    content TEXT NOT NULL,
    author_id INTEGER REFERENCES users(id),
    published BOOLEAN DEFAULT false,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- 评论表
CREATE TABLE comments (
    id SERIAL PRIMARY KEY,
    post_id INTEGER REFERENCES posts(id) ON DELETE CASCADE,
    user_id INTEGER REFERENCES users(id),
    content TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- 标签表
CREATE TABLE tags (
    id SERIAL PRIMARY KEY,
    name VARCHAR(50) UNIQUE NOT NULL
);

-- 文章标签关联表
CREATE TABLE post_tags (
    post_id INTEGER REFERENCES posts(id) ON DELETE CASCADE,
    tag_id INTEGER REFERENCES tags(id) ON DELETE CASCADE,
    PRIMARY KEY (post_id, tag_id)
);
```

#### Day 77: 核心业务实现
**实现模块：**
- 用户服务（注册、登录、个人资料）
- 文章服务（CRUD、发布、草稿）
- 评论服务
- 标签服务

#### Day 78: API 接口开发
**API 端点：**
```
POST   /api/auth/register
POST   /api/auth/login
GET    /api/users/me
PUT    /api/users/me

GET    /api/posts
POST   /api/posts
GET    /api/posts/:id
PUT    /api/posts/:id
DELETE /api/posts/:id

GET    /api/posts/:id/comments
POST   /api/posts/:id/comments
DELETE /api/comments/:id

GET    /api/tags
POST   /api/tags
```

#### Day 79: 测试与优化
**测试内容：**
- 单元测试
- 集成测试
- API 测试
- 性能测试

**优化方向：**
- 数据库查询优化
- 缓存策略
- 连接池调优
- 响应压缩

#### Day 80: 项目总结与展示
**总结内容：**
- 架构回顾
- 技术栈总结
- 遇到的问题和解决方案
- 性能指标
- 未来改进方向

## 技术栈总结

### 后端框架
- Axum 0.7 - Web 框架
- Tower - 中间件
- SQLx - 数据库访问

### 数据库
- PostgreSQL - 主数据库
- Redis - 缓存（可选）

### 认证授权
- jsonwebtoken - JWT 处理
- argon2 - 密码哈希

### 序列化
- Serde - JSON 序列化
- Serde_json - JSON 处理

### 模板引擎
- Askama - 服务端渲染

### 文档
- utoipa - OpenAPI 文档生成

### 部署
- Docker - 容器化
- Docker Compose - 编排

## 学习资源

### 书籍
- Zero To Production In Rust
- Rust Web Development

### 文档
- [Axum 官方文档](https://docs.rs/axum)
- [SQLx 文档](https://docs.rs/sqlx)
- [Tower 文档](https://docs.rs/tower)

### 视频教程
- Let's Get Rusty - Web Development Series
- Jon Gjengset - Rust Streams

## 项目检查清单

- [ ] 用户认证系统完整
- [ ] CRUD 操作正常
- [ ] 错误处理完善
- [ ] API 文档完整
- [ ] 单元测试覆盖
- [ ] 集成测试通过
- [ ] Docker 部署成功
- [ ] 性能达标
- [ ] 安全审计通过
- [ ] 代码文档完整

## 下一阶段预告

第六阶段（Days 81-90）将深入系统编程和性能优化，包括：
- 操作系统底层交互
- 文件系统深入
- 性能分析和优化
- SIMD 和底层优化
- CI/CD 流水线

继续加油！🚀
