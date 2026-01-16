# Day 46: 项目初始化与配置

从今天开始，我们将进入为期 15 天的 **异步网络项目实战** 阶段。我们将从零开始实现一个兼容 Redis 协议的轻量级 KV 存储服务器 —— **Mini-Redis**。

这个项目将串联我们之前学到的众多知识点：
*   **异步编程**: 使用 Tokio 运行时。
*   **网络编程**: TCP Listener 与 Stream 处理。
*   **数据结构**: HashMap 存储数据。
*   **并发控制**: Mutex/RwLock 保护共享状态。
*   **协议解析**: 字节流操作。
*   **错误处理与日志**: anyhow, thiserror, tracing。

## 1. 项目目标

实现一个支持 RESP (Redis Serialization Protocol) 的服务器，支持以下命令：
*   `GET key`
*   `SET key value`
*   `PUBLISH channel message`
*   `SUBSCRIBE channel`

## 2. 初始化项目

我们创建一个新的二进制项目：

```bash
cargo init --bin mini-redis
```

## 3. 依赖配置

编辑 `Cargo.toml`，添加必要的依赖：

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
tracing = "0.1"
tracing-subscriber = "0.3"
anyhow = "1.0"
bytes = "1.5"
thiserror = "1.0"
serde = { version = "1.0", features = ["derive"] }
```

*   `tokio`: 异步运行时核心。
*   `tracing`: 结构化日志。
*   `bytes`: 高效处理网络字节流。
*   `anyhow`: 灵活的错误处理。

## 4. 基础骨架

在 `src/main.rs` 中初始化日志和 Tokio 运行时：

```rust
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 初始化日志
    tracing_subscriber::fmt::init();

    info!("Mini-Redis server is starting...");

    // TODO: 监听端口、处理连接

    Ok(())
}
```
