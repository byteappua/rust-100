# Day 94: 核心模块开发 (二)

本项目 (**DTask**) 的核心开发第二阶段。

## 开发目标

1.  **gRPC 服务实现**: 完善 `TaskScheduler` 的 gRPC 接口实现。
2.  **任务分发器**: 实现 `Dispatcher` 的后台轮询和执行逻辑。
3.  **状态共享**: 解决服务间的数据共享和并发访问问题。

## 完成内容

### 1. gRPC 调度器 (`TaskScheduler`)

-   基于 `tonic` 实现了 `dtask.proto` 中定义的 `Scheduler` 服务。
-   实现了 `submit_task` 接口，接收客户端请求并将任务存入共享状态。
-   使用了 `Arc<Mutex<Vec<Task>>>` 作为临时的共享存储（后续将对接 Raft）。

### 2. 任务分发器 (`Dispatcher`)

-   实现了后台轮询循环，定期检查待处理任务。
-   模拟了任务执行过程（简单的打印日志和状态更新）。
-   与 `TaskScheduler` 共享同一个任务列表，确保数据一致性。

### 3. 主程序入口 (`main.rs`)

-   在 `tokio` 运行时中同时启动 gRPC Server 和 Dispatcher 后台任务。
-   使用 `tokio::spawn` 运行 Dispatcher，避免阻塞 gRPC 服务。

## 运行方式

```bash
cd Day91-100/94.CoreDevelopment2/dtask

# 启动服务端
cargo run

# 在另一个终端运行测试客户端
cargo run --example client
```

## 下一步

Day 95 将集成 Web API (Axum) 和认证模块，对外提供 HTTP 接口。
