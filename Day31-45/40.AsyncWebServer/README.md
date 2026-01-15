# Day 40: 构建异步 Web 服务器 (Hyper)

在 Rust 生态中，**Hyper** 是一个快速且正确的 HTTP 实现（包括客户端和服务器）。它是许多高级 Web 框架（如 Axum, Warp, Reqwest）的基础。

今天，我们不使用高级框架，而是直接使用 Hyper 1.0 来构建一个底层的异步 Web 服务器，以理解其工作原理。

## 1. Hyper 1.0 架构

Hyper 1.0 经历了重大的 API 变革，旨在更加模块化和解耦。

*   **`hyper`**: 核心库，包含 HTTP 协议实现。
*   **`hyper-util`**: 提供与运行时（如 Tokio）集成的工具。
*   **`http-body-util`**: 处理请求和响应体的工具。

## 2. 核心组件

### Service Trait

Hyper 基于 `Service` 模式。一个 `Service` 只是一个异步函数，它接受一个 Request 并返回一个 Response。

```rust
async fn hello(req: Request<Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
    Ok(Response::new(Full::new(Bytes::from("Hello World"))))
}
```

### IO 适配器 (`TokioIo`)

Hyper 是运行时无关的。为了使用 Tokio 的 TCP 流，我们需要用 `hyper_util::rt::TokioIo` 包装它，使其满足 Hyper 的 IO Trait 要求。

### 连接处理 Loop

我们需要手动编写一个循环来接受 TCP 连接，并为每个连接生成一个新的 Tokio 任务来处理。

```rust
loop {
    let (stream, _) = listener.accept().await?;
    let io = TokioIo::new(stream);

    tokio::task::spawn(async move {
        // ... serve connection
    });
}
```

## 3. 运行与测试

启动服务器：
```bash
cargo run
```

测试 Endpoint：

*   **GET /**: 返回 "Hello, World!..."
*   **POST /echo**: 回显请求体内容。

```bash
curl http://127.0.0.1:3000
curl -X POST -d "Rust is awesome" http://127.0.0.1:3000/echo
```
