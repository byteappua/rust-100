# Day 59: 完善文档与示例

Rust 社区非常重视文档，良好的文档不仅是代码的一部分，更是生态系统的一等公民。`cargo doc` 工具使得生成高质量文档变得非常简单。

## 1. 文档注释

在 Rust 中，我们使用 `///` 来编写文档注释（Doc Comments）。这些注释支持 Markdown 格式。

-   **模块文档**: 使用 `//!` 在文件的开头编写。
-   **函数/结构体文档**: 使用 `///` 在定义上方编写。

### 常用章节

-   `# Examples`: 示例代码，这是文档中最有价值的部分。`cargo test` 会自动运行这些示例（Doctests），确保文档与代码保持同步。
-   `# Panics`: 说明函数在什么情况下会 Panic。
-   `# Errors`: 说明函数返回 `Result` 时可能包含的错误类型。
-   `# Safety`: 如果函数是 `unsafe` 的，必须说明调用者需要保证的安全条件。

## 2. 本节目标

在本节中，我们对 Mini-Redis 项目进行了文档化增强：

1.  **Library Crate 文档**: 在 `src/lib.rs` 中添加了 crate 级别的说明。
2.  **Public API 文档**: 为 `Client` 等核心结构体添加了详细的文档注释。
3.  **Doctests**: 在文档中加入了可运行的代码示例。
4.  **示例程序**: 在 `examples/` 目录下添加了 `basic.rs` 和 `hello_redis.rs`，展示完整用法。

## 3. 生成文档

运行以下命令生成并查看文档：

```bash
cargo doc --open
```

生成的 HTML 文档位于 `target/doc/mini_redis_doc/index.html`。

## 4. 运行示例

我们提供了两个示例程序：

**Basic Usage:**
```bash
cargo run --example basic
```

**Walkthrough:**
```bash
cargo run --example hello_redis
```

> 注意：运行示例前请确保已启动 Mini-Redis 服务器（可使用之前章节的 Server，或本目录下 `cargo run --bin mini-redis-server` 如果有 binary target 的话。本节主要关注 Library 文档，未包含 Server binary 的显式配置，可复用 Day 54/55 的 Server）。

## 5. 小贴士

-   使用 `cargo test` 来运行文档测试，确保示例代码是正确的。
-   文档应该简洁明了，重点突出。
-   尽量为每个公开的 API 添加文档。
