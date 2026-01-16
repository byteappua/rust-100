# Day 95: 外围模块与集成

本项目 (**DTask**) 的集成阶段，引入了 Web API 和监控体系。

## 开发目标

1.  **Web API**: 使用 `Axum` 提供 RESTful 接口，方便前端和第三方系统集成。
2.  **认证授权**: 实现基于 JWT 的用户认证。
3.  **监控指标**: 集成 Prometheus 监控指标。
4.  **服务并存**: 在同一个进程中同时运行 gRPC Server 和 HTTP Server。

## 完成内容

### 1. Web API 集成 (`api.rs`)

-   使用 `Axum` 0.7 构建了 HTTP 服务。
-   提供了 `/api/login` (获取 Token) 和 `/api/tasks` (提交/查询任务) 接口。
-   与 gRPC 服务共享底层的 `Arc<Mutex<Vec<Task>>>` 数据状态。

### 2. JWT 认证 (`auth.rs`)

-   使用 `jsonwebtoken` 库实现了 Token 的生成 (`create_jwt`) 和验证 (`verify_jwt`)。
-   实现了 `auth_middleware` 中间件，保护敏感 API 接口。

### 3. 可观测性 (`metrics`)

-   集成 `prometheus` crate，定义了 `http_requests_total` 等基础指标。
-   添加了 `/metrics` 端点，供 Prometheus Server 抓取数据。

### 4. 服务编排 (`main.rs`)

-   修改了 `main.rs`，使用 `tokio::select!` 或 `join!` 同时启动：
    -   gRPC Server (Port 50051)
    -   Axum HTTP Server (Port 3000)
    -   Dispatcher Loop

## 运行方式

```bash
cd Day91-100/95.Integration/dtask

# 启动服务
cargo run

# 测试 HTTP 接口
# 1. 登录获取 Token (模拟)
curl -X POST http://localhost:3000/api/login -H "Content-Type: application/json" -d '{"username":"admin", "password":"password"}'

# 2. 获取 Metrics
curl http://localhost:3000/metrics
```

## 下一步

Day 96 将进行性能测试与调优，引入 SQLite 持久化并优化并发性能。
