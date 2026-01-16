# Day 70: 权限控制 (RBAC)

## 学习目标
- 理解 RBAC 概念
- 设计权限模型
- 实现角色和权限系统
- 创建权限检查中间件

## RBAC 概念

RBAC (Role-Based Access Control) 是一种基于角色的访问控制模型。

```
用户 → 角色 → 权限 → 资源
```

### 核心概念

- **用户 (User)**: 系统的使用者
- **角色 (Role)**: 用户的职能分类
- **权限 (Permission)**: 对资源的操作许可
- **资源 (Resource)**: 被保护的对象

## 数据库设计

```sql
-- 用户表
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(50) UNIQUE NOT NULL,
    email VARCHAR(100) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- 角色表
CREATE TABLE roles (
    id SERIAL PRIMARY KEY,
    name VARCHAR(50) UNIQUE NOT NULL,
    description TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- 权限表
CREATE TABLE permissions (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) UNIQUE NOT NULL,
    resource VARCHAR(50) NOT NULL,
    action VARCHAR(50) NOT NULL,
    description TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- 用户角色关联表
CREATE TABLE user_roles (
    user_id INTEGER REFERENCES users(id) ON DELETE CASCADE,
    role_id INTEGER REFERENCES roles(id) ON DELETE CASCADE,
    PRIMARY KEY (user_id, role_id)
);

-- 角色权限关联表
CREATE TABLE role_permissions (
    role_id INTEGER REFERENCES roles(id) ON DELETE CASCADE,
    permission_id INTEGER REFERENCES permissions(id) ON DELETE CASCADE,
    PRIMARY KEY (role_id, permission_id)
);

-- 索引
CREATE INDEX idx_user_roles_user_id ON user_roles(user_id);
CREATE INDEX idx_user_roles_role_id ON user_roles(role_id);
CREATE INDEX idx_role_permissions_role_id ON role_permissions(role_id);
CREATE INDEX idx_role_permissions_permission_id ON role_permissions(permission_id);
```

## 数据模型

```rust
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Role {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Permission {
    pub id: i32,
    pub name: String,
    pub resource: String,
    pub action: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone)]
pub struct UserWithRoles {
    pub user: User,
    pub roles: Vec<Role>,
    pub permissions: Vec<Permission>,
}
```

## 权限枚举

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Resource {
    User,
    Post,
    Comment,
    Category,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Action {
    Create,
    Read,
    Update,
    Delete,
    List,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PermissionCheck {
    pub resource: Resource,
    pub action: Action,
}

impl PermissionCheck {
    pub fn new(resource: Resource, action: Action) -> Self {
        Self { resource, action }
    }
    
    pub fn to_string(&self) -> String {
        format!("{}:{:?}", self.resource_name(), self.action)
    }
    
    fn resource_name(&self) -> &str {
        match self.resource {
            Resource::User => "user",
            Resource::Post => "post",
            Resource::Comment => "comment",
            Resource::Category => "category",
        }
    }
}

// 便捷宏
macro_rules! permission {
    ($resource:ident, $action:ident) => {
        PermissionCheck::new(Resource::$resource, Action::$action)
    };
}
```

## 权限服务

```rust
use sqlx::PgPool;
use std::collections::HashSet;

pub struct PermissionService {
    pool: PgPool,
}

impl PermissionService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
    
    // 获取用户的所有角色
    pub async fn get_user_roles(&self, user_id: i32) -> Result<Vec<Role>> {
        let roles = sqlx::query_as!(
            Role,
            r#"
            SELECT r.id, r.name, r.description
            FROM roles r
            INNER JOIN user_roles ur ON r.id = ur.role_id
            WHERE ur.user_id = $1
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await?;
        
        Ok(roles)
    }
    
    // 获取用户的所有权限
    pub async fn get_user_permissions(&self, user_id: i32) -> Result<Vec<Permission>> {
        let permissions = sqlx::query_as!(
            Permission,
            r#"
            SELECT DISTINCT p.id, p.name, p.resource, p.action, p.description
            FROM permissions p
            INNER JOIN role_permissions rp ON p.id = rp.permission_id
            INNER JOIN user_roles ur ON rp.role_id = ur.role_id
            WHERE ur.user_id = $1
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await?;
        
        Ok(permissions)
    }
    
    // 检查用户是否有特定权限
    pub async fn has_permission(
        &self,
        user_id: i32,
        resource: &str,
        action: &str,
    ) -> Result<bool> {
        let result = sqlx::query!(
            r#"
            SELECT EXISTS(
                SELECT 1
                FROM permissions p
                INNER JOIN role_permissions rp ON p.id = rp.permission_id
                INNER JOIN user_roles ur ON rp.role_id = ur.role_id
                WHERE ur.user_id = $1
                  AND p.resource = $2
                  AND p.action = $3
            ) as "exists!"
            "#,
            user_id,
            resource,
            action
        )
        .fetch_one(&self.pool)
        .await?;
        
        Ok(result.exists)
    }
    
    // 分配角色给用户
    pub async fn assign_role(&self, user_id: i32, role_id: i32) -> Result<()> {
        sqlx::query!(
            "INSERT INTO user_roles (user_id, role_id) VALUES ($1, $2) ON CONFLICT DO NOTHING",
            user_id,
            role_id
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    // 移除用户角色
    pub async fn remove_role(&self, user_id: i32, role_id: i32) -> Result<()> {
        sqlx::query!(
            "DELETE FROM user_roles WHERE user_id = $1 AND role_id = $2",
            user_id,
            role_id
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    // 分配权限给角色
    pub async fn assign_permission(&self, role_id: i32, permission_id: i32) -> Result<()> {
        sqlx::query!(
            "INSERT INTO role_permissions (role_id, permission_id) VALUES ($1, $2) ON CONFLICT DO NOTHING",
            role_id,
            permission_id
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
}
```

## 权限中间件

```rust
use axum::{
    extract::{Request, State},
    middleware::Next,
    response::{Response, IntoResponse},
    http::StatusCode,
};

// 权限检查中间件
pub async fn require_permission(
    resource: String,
    action: String,
) -> impl Fn(Request, Next) -> impl Future<Output = Result<Response, StatusCode>> {
    move |request: Request, next: Next| {
        let resource = resource.clone();
        let action = action.clone();
        
        async move {
            // 从请求扩展中获取用户信息
            let user = request
                .extensions()
                .get::<AuthUser>()
                .ok_or(StatusCode::UNAUTHORIZED)?;
            
            // 检查权限
            if !user.has_permission(&resource, &action) {
                return Err(StatusCode::FORBIDDEN);
            }
            
            Ok(next.run(request).await)
        }
    }
}

// 角色检查中间件
pub async fn require_role(
    required_role: String,
) -> impl Fn(Request, Next) -> impl Future<Output = Result<Response, StatusCode>> {
    move |request: Request, next: Next| {
        let required_role = required_role.clone();
        
        async move {
            let user = request
                .extensions()
                .get::<AuthUser>()
                .ok_or(StatusCode::UNAUTHORIZED)?;
            
            if !user.has_role(&required_role) {
                return Err(StatusCode::FORBIDDEN);
            }
            
            Ok(next.run(request).await)
        }
    }
}
```

## 认证用户扩展

```rust
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct AuthUser {
    pub id: i32,
    pub username: String,
    pub roles: HashSet<String>,
    pub permissions: HashSet<String>,
}

impl AuthUser {
    pub fn has_role(&self, role: &str) -> bool {
        self.roles.contains(role)
    }
    
    pub fn has_permission(&self, resource: &str, action: &str) -> bool {
        let permission = format!("{}:{}", resource, action);
        self.permissions.contains(&permission)
    }
    
    pub fn has_any_role(&self, roles: &[&str]) -> bool {
        roles.iter().any(|role| self.has_role(role))
    }
    
    pub fn has_all_roles(&self, roles: &[&str]) -> bool {
        roles.iter().all(|role| self.has_role(role))
    }
}

// 从数据库加载用户权限
pub async fn load_auth_user(
    user_id: i32,
    service: &PermissionService,
) -> Result<AuthUser> {
    let user = service.get_user(user_id).await?;
    let roles = service.get_user_roles(user_id).await?;
    let permissions = service.get_user_permissions(user_id).await?;
    
    let role_names: HashSet<String> = roles
        .into_iter()
        .map(|r| r.name)
        .collect();
    
    let permission_names: HashSet<String> = permissions
        .into_iter()
        .map(|p| format!("{}:{}", p.resource, p.action))
        .collect();
    
    Ok(AuthUser {
        id: user.id,
        username: user.username,
        roles: role_names,
        permissions: permission_names,
    })
}
```

## 路由保护

```rust
use axum::{
    Router,
    routing::{get, post, put, delete},
    middleware,
};

// 用户管理路由（需要管理员权限）
fn admin_routes() -> Router {
    Router::new()
        .route("/users", get(list_users).post(create_user))
        .route("/users/:id", put(update_user).delete(delete_user))
        .route("/roles", get(list_roles).post(create_role))
        .layer(middleware::from_fn(require_role("admin")))
}

// 文章管理路由（需要编辑权限）
fn post_routes() -> Router {
    Router::new()
        .route("/posts", post(create_post))
        .route("/posts/:id", put(update_post).delete(delete_post))
        .layer(middleware::from_fn(|req, next| {
            require_permission("post".to_string(), "write".to_string())(req, next)
        }))
}

// 公开路由
fn public_routes() -> Router {
    Router::new()
        .route("/posts", get(list_posts))
        .route("/posts/:id", get(get_post))
}

fn app() -> Router {
    Router::new()
        .nest("/admin", admin_routes())
        .nest("/api", post_routes())
        .merge(public_routes())
}
```

## 处理器中的权限检查

```rust
use axum::{
    extract::{Path, State, Extension},
    Json,
    http::StatusCode,
};

// 更新文章（检查所有权）
async fn update_post(
    Path(post_id): Path<i32>,
    Extension(user): Extension<AuthUser>,
    State(service): State<PostService>,
    Json(payload): Json<UpdatePostRequest>,
) -> Result<Json<Post>, StatusCode> {
    let post = service
        .get_post(post_id)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;
    
    // 检查是否是文章作者或管理员
    if post.author_id != user.id && !user.has_role("admin") {
        return Err(StatusCode::FORBIDDEN);
    }
    
    let updated = service
        .update_post(post_id, payload)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(updated))
}

// 删除用户（仅管理员）
async fn delete_user(
    Path(user_id): Path<i32>,
    Extension(auth_user): Extension<AuthUser>,
    State(service): State<UserService>,
) -> Result<StatusCode, StatusCode> {
    // 检查管理员权限
    if !auth_user.has_role("admin") {
        return Err(StatusCode::FORBIDDEN);
    }
    
    // 不能删除自己
    if user_id == auth_user.id {
        return Err(StatusCode::BAD_REQUEST);
    }
    
    service
        .delete_user(user_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(StatusCode::NO_CONTENT)
}
```

## 权限初始化

```rust
pub async fn init_permissions(pool: &PgPool) -> Result<()> {
    // 创建角色
    let admin_role = sqlx::query!(
        "INSERT INTO roles (name, description) VALUES ($1, $2) RETURNING id",
        "admin",
        "Administrator with full access"
    )
    .fetch_one(pool)
    .await?;
    
    let editor_role = sqlx::query!(
        "INSERT INTO roles (name, description) VALUES ($1, $2) RETURNING id",
        "editor",
        "Can create and edit posts"
    )
    .fetch_one(pool)
    .await?;
    
    let user_role = sqlx::query!(
        "INSERT INTO roles (name, description) VALUES ($1, $2) RETURNING id",
        "user",
        "Regular user"
    )
    .fetch_one(pool)
    .await?;
    
    // 创建权限
    let permissions = vec![
        ("user:create", "user", "create"),
        ("user:read", "user", "read"),
        ("user:update", "user", "update"),
        ("user:delete", "user", "delete"),
        ("post:create", "post", "create"),
        ("post:read", "post", "read"),
        ("post:update", "post", "update"),
        ("post:delete", "post", "delete"),
    ];
    
    for (name, resource, action) in permissions {
        let perm = sqlx::query!(
            "INSERT INTO permissions (name, resource, action) VALUES ($1, $2, $3) RETURNING id",
            name,
            resource,
            action
        )
        .fetch_one(pool)
        .await?;
        
        // 管理员拥有所有权限
        sqlx::query!(
            "INSERT INTO role_permissions (role_id, permission_id) VALUES ($1, $2)",
            admin_role.id,
            perm.id
        )
        .execute(pool)
        .await?;
        
        // 编辑拥有文章相关权限
        if resource == "post" {
            sqlx::query!(
                "INSERT INTO role_permissions (role_id, permission_id) VALUES ($1, $2)",
                editor_role.id,
                perm.id
            )
            .execute(pool)
            .await?;
        }
        
        // 普通用户只有读权限
        if action == "read" {
            sqlx::query!(
                "INSERT INTO role_permissions (role_id, permission_id) VALUES ($1, $2)",
                user_role.id,
                perm.id
            )
            .execute(pool)
            .await?;
        }
    }
    
    Ok(())
}
```

## 完整示例

```rust
use axum::{
    Router,
    routing::{get, post},
    middleware,
    extract::{State, Extension},
    Json,
    http::StatusCode,
};
use sqlx::PgPool;

#[tokio::main]
async fn main() {
    // 连接数据库
    let pool = PgPool::connect("postgres://localhost/rbac_db")
        .await
        .unwrap();
    
    // 初始化权限
    init_permissions(&pool).await.unwrap();
    
    // 创建服务
    let permission_service = PermissionService::new(pool.clone());
    
    // 构建应用
    let app = Router::new()
        // 公开路由
        .route("/api/posts", get(list_posts))
        .route("/api/posts/:id", get(get_post))
        
        // 需要认证的路由
        .route("/api/posts", post(create_post))
        .layer(middleware::from_fn(auth_middleware))
        
        // 管理员路由
        .route("/api/admin/users", get(list_users))
        .layer(middleware::from_fn(require_role("admin")))
        
        .with_state(permission_service);
    
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    
    println!("🚀 Server running on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}
```

## 测试

```bash
# 以管理员身份登录
TOKEN=$(curl -X POST http://localhost:3000/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username":"admin","password":"admin123"}' \
  | jq -r '.token')

# 访问管理员路由
curl http://localhost:3000/api/admin/users \
  -H "Authorization: Bearer $TOKEN"

# 创建文章（需要编辑权限）
curl -X POST http://localhost:3000/api/posts \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"title":"New Post","content":"Content here"}'
```

## 练习

1. 实现资源级权限（如只能编辑自己的文章）
2. 添加权限继承机制
3. 实现动态权限加载
4. 创建权限管理 API
5. 添加权限缓存机制

## 下一步

Day 71 将学习模板引擎，实现服务端渲染。
