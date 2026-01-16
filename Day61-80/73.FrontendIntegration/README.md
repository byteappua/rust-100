# Day 73: 前端集成实战

本章节通过一个完整的示例演示如何在 Axum 项目中集成前端 SPA (Single Page Application) 应用。

## 项目结构

```
frontend-integration-demo/
├── Cargo.toml      # Rust 项目配置
├── assets/         # 前端静态资源 (模拟构建产物)
│   ├── app.js      # 简单的 SPA 路由逻辑
│   ├── index.html  # 入口 HTML
│   └── style.css   # 样式
└── src/
    └── main.rs     # 后端服务
```

## 核心功能

1.  **静态文件服务**: 使用 `tower_http::services::ServeDir` 服务 `assets` 目录。
2.  **SPA 路由 fallback**: 前端路由（如 `/users`, `/about`）在后端不存在对应 API 时，统一返回 `index.html`，由前端 JS 接管路由。
3.  **API 代理**: 提供 `/api/users` 等后端接口，供前端调用。

## 运行 demo

进入 `frontend-integration-demo` 目录并运行：

```bash
cd frontend-integration-demo
cargo run
```

然后访问 `http://localhost:3000`。

### 演示效果

1.  **Home**: 默认首页。
2.  **Users**: 点击导航栏的 "Users"，前端 JS 会发起 `fetch('/api/users')` 请求，并将返回的 JSON 数据渲染到页面上。这展示了前后端分离的数据交互。
3.  **About**: 纯前端页面切换。
4.  **SPA Routing**: 尝试直接访问 `http://localhost:3000/about`，后端会返回 `index.html`，前端 JS 初始化后会解析 URL 并显示 About 页面。

## 代码解析

### 后端 (Axum)

```rust
// 定义静态文件服务，如果文件找不到则返回 index.html (SPA 模式)
let assets_path = std::env::current_dir().unwrap().join("assets");
let spa_service = ServeDir::new(&assets_path)
    .fallback(ServeFile::new(assets_path.join("index.html")));

let app = Router::new()
    .nest("/api", api_routes) // 优先匹配 API
    .fallback_service(spa_service); // 其他所有请求交给静态文件服务
```

### 前端 (JS)

前端使用 HTML5 History API 实现无刷新跳转，并在页面加载时根据 URL 渲染不同组件。

## 下一步

Day 74 将学习 API 文档生成 (OpenAPI)，为前后端协作提供更好的文档支持。
