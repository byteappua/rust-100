# Day 42: 日志与监控 (Tracing)

在传统的应用中，我们使用日志（Logging）来记录发生的事件。但在异步和并发系统中，传统的日志很难追踪一个请求的完整生命周期。

Rust 社区推出了 **Tracing**，一个用于应用程序级跟踪的框架。

## 1. 结构化日志 vs 文本日志

*   **Log**: 只是文本行。`INFO: User logged in`.
*   **Tracing**: 结构化事件。`{ level: "INFO", message: "User logged in", user_id: 42, span: "auth" }`.

## 2. 核心概念

### Span (跨度)
表示一段时间内的执行上下文。例如，一个 HTTP 请求的处理过程就是一个 Span。Span 可以嵌套。

### Event (事件)
表示某个时间点发生的事情。类似于传统的日志记录。

### Subscriber (订阅者)
负责收集 Span 和 Event 并将其记录下来（如打印到 stdout，发送到 Jaeger 等）。

## 3. 使用方法

### 初始化
```rust
tracing_subscriber::fmt::init();
```

### 记录事件
```rust
use tracing::{info, error};
info!("Something happened");
error!("Something bad happened");
```

### 使用 Span
手动创建：
```rust
let span = tracing::info_span!("my_span");
let _enter = span.enter();
// ... code ...
```

使用宏自动 instrument 函数：
```rust
#[tracing::instrument]
fn my_function(arg: i32) {
    // ...
}
```

## 运行

默认情况下 `tracing-subscriber` 可能会过滤掉部分日志。使用 `RUST_LOG` 环境变量控制级别。

```bash
RUST_LOG=info cargo run
```
