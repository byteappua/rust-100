# Day 39: Tokio 运行时深入

Tokio 是 Rust 生态中最流行的异步运行时。不仅提供了 executor（执行器），还提供了基于 reactor 模式的非阻塞 I/O 驱动，以及定时器、文件系统操作等工具。

## 1. 任务 (Tasks) vs 线程 (Threads)

*   **OS 线程**：重量级，由操作系统调度，上下文切换开销大。
*   **Tokio 任务**：轻量级（Green Threads），由 Tokio 运行时在用户态调度。
*   **`tokio::spawn`**：生成一个新的异步任务。它需要一个 `Future`，并且该 `Future` 必须是 `Send` + `'static`（因为它可能在不同线程间移动）。

```rust
tokio::spawn(async {
    // 这是一个并发运行的任务
});
```

## 2. 消息传递 (Channels)

Tokio 提供了几种异步通信原语：

*   **`mpsc`** (Multi-Producer, Single-Consumer): 多生产者单消费者。最常用。
*   **`oneshot`**: 一次性通道，发送一个值。
*   **`broadcast`**: 多生产者多消费者（广播）。
*   **`watch`**: 单生产者多消费者，只保存最新值。

```rust
let (tx, mut rx) = mpsc::channel(32);
tx.send("message").await.unwrap();
let msg = rx.recv().await;
```

## 3. `tokio::select!`

允许同时等待多个异步操作，并在**第一个**操作完成时继续执行。其他的操作会被取消（Dropped）。

```rust
tokio::select! {
    val = op1() => { println!("op1 finished: {:?}", val)},
    val = op2() => { println!("op2 finished: {:?}", val)},
}
```

## 4. 阻塞操作 (`spawn_blocking`)

**永远不要在异步函数中执行阻塞操作！**
如果执行了阻塞操作（如大量的 CPU 计算、同步的文件 IO、`std::thread::sleep`），会阻塞整个 Tokio 调度器线程，导致其他任务无法执行。

解决方案：
使用 `tokio::task::spawn_blocking` 将阻塞任务放入专门的线程池中执行。

```rust
let res = tokio::task::spawn_blocking(|| {
    // 这里可以放心地执行阻塞代码
    std::thread::sleep(std::time::Duration::from_secs(1));
    "done"
}).await?;
```
