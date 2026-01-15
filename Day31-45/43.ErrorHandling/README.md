# Day 43: 错误处理最佳实践

Rust 的错误处理非常强大，但也需要根据场景选择合适的工具。今天我们介绍两个事实标准的库：**`thiserror`** 和 **`anyhow`**。

## 1. 库 vs 应用 (Library vs Application)

在 Rust 中，错误处理通常分为两种场景：

1.  **库 (Library)**: 你在编写供他人使用的库。
    *   **需求**: 用户需要能够匹配 (`match`) 特定的错误类型，以便进行恢复处理。
    *   **工具**: `thiserror`。它帮助你轻松派生 `Error` trait，而无需手写繁琐的样板代码。
2.  **应用 (Application)**: 你在编写最终的二进制程序 (CLI, Web Server)。
    *   **需求**: 你通常不关心具体的错误类型，只关心发生了错误，并希望以人类可读的方式报告出来（包含上下文堆栈）。
    *   **工具**: `anyhow`。它提供了一个类似 `Box<dyn Error>` 的包装器，支持添加上下文 (`.context()`)。

## 2. 使用 `thiserror` (定义错误)

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyError {
    #[error("io error")]
    Io(#[from] std::io::Error), // 自动实现 From<io::Error>

    #[error("invalid header (expected {expected:?}, found {found:?})")]
    InvalidHeader { expected: String, found: String },
}
```

## 3. 使用 `anyhow` (处理错误)

`anyhow::Result<T>` 是 `Result<T, anyhow::Error>` 的别名。

```rust
use anyhow::{Context, Result};

fn main() -> Result<()> {
    let content = std::fs::read_to_string("config.toml")
        .context("Failed to read config file")?; // 添加上下文

    Ok(())
}
```

输出的错误信息会包含上下文链，方便调试：
```
Error: Failed to read config file

Caused by:
    No such file or directory (os error 2)
```
