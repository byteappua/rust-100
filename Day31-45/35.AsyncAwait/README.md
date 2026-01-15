# Day 35: 异步编程入门 (Async/Await)

Rust 的异步编程允许你在单个线程上并发运行多个任务，非常适合 I/O 密集型操作（如网络服务）。

## 1. async 与 .await

*   `async fn`: 标记一个函数为异步函数。它不直接返回结果，而是返回一个 `Future`。
*   `.await`: 挂起当前任务，直到 `Future` 完成。这不会阻塞线程，而是允许执行其他任务。

```rust
async fn do_something() {
    // ...
}

async fn main_func() {
    do_something().await;
}
```

## 2. 运行时 (Runtime)

Rust 标准库没有内置异步运行时。你需要选择一个社区库，**Tokio** 是最流行的选择。

在 `Cargo.toml` 中添加：
```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

使用 `#[tokio::main]` 宏将 `main` 函数变为异步入口。

## 3. Future Trait

`Future` 是异步编程的核心抽象。它表示一个未来某个时刻会产生值的对象。

```rust
pub trait Future {
    type Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
```

通常我们不需要手动实现 `Future`，`async/await` 语法会自动为我们将代码块转换为状态机并实现 `Future`。

## 4. 并发执行

使用 `tokio::join!` 等宏可以同时等待多个 `Future` 完成。

```rust
let f1 = task_one();
let f2 = task_two();
tokio::join!(f1, f2);
```
