# Day 74: API 文档生成 (OpenAPI)

## 学习目标
- 理解 OpenAPI (Swagger) 规范的重要性
- 掌握 `utoipa` crate 的使用方法
- 学习如何自动生成 Rust Web API 文档
- 集成 Swagger UI 到 Axum 项目中

## 核心概念

### 1. OpenAPI 规范
OpenAPI Specification (OAS) 是一种用于 RESTful API 的标准描述格式。它允许人类和计算机在不访问源代码、文档或通过网络流量检查的情况下发现和理解服务的功能。

### 2. Utoipa
`utoipa` 是 Rust 生态中一个流行的库，用于从代码中自动生成 OpenAPI 文档。它通过派生宏（Derive Macros）和属性宏（Attribute Macros）来减少样板代码。
- `#[derive(ToSchema)]`: 为结构体生成 JSON Schema。
- `#[utoipa::path(...)]`: 为处理函数定义 API 路径、参数和响应。
- `#[derive(OpenApi)]`: 聚合所有路径和组件，生成最终的文档。

### 3. Swagger UI
Swagger UI 是一个基于 HTML/JS 的工具，它可以将 OpenAPI 规范可视化，并提供一个交互式的界面来测试 API。

## 代码示例

本节代码演示了如何使用 `utoipa` 和 `utoipa-swagger-ui` 与 `axum` 集成。

### 定义数据模型

```rust
use utoipa::ToSchema;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, ToSchema)]
struct User {
    id: i32,
    username: String,
}
```

### 定义 API 路径

```rust
use axum::Json;
use utoipa::OpenApi;

#[utoipa::path(
    get,
    path = "/users",
    responses(
        (status = 200, description = "List all users", body = [User])
    )
)]
async fn list_users() -> Json<Vec<User>> {
    // ...
}
```

### 集成 Swagger UI

```rust
use utoipa_swagger_ui::SwaggerUi;
use axum::Router;

// 定义 OpenAPI 结构
#[derive(OpenApi)]
#[openapi(paths(list_users), components(schemas(User)))]
struct ApiDoc;

// 在 Router 中挂载
let app = Router::new()
    .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()));
```

## 运行示例

进入 `openapi-demo` 目录并运行：

```bash
cd openapi-demo
cargo run
```

启动后，访问浏览器：
- **Swagger UI**: [http://localhost:3000/swagger-ui](http://localhost:3000/swagger-ui)
- **JSON Spec**: [http://localhost:3000/api-docs/openapi.json](http://localhost:3000/api-docs/openapi.json)

你可以在 Swagger UI 页面中直接测试 API，例如尝试 `POST /users` 创建用户，然后 `GET /users` 查看列表。

## 练习
1. 为 `User` 结构体添加更多字段（如 `age`, `role`），并更新文档。
2. 添加一个 `DELETE /users/{id}` 接口，并为其编写文档，注意处理 404 响应。
3. 尝试使用 `SecurityScheme` 为 API 添加 JWT 认证描述。

## 下一步
Day 75 将学习容器化部署 (Docker)。
