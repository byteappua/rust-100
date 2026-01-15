# Day 33: 构建命令行工具 (Building a CLI Tool)

这是一个经典的 Rust 入门项目：实现 `grep` 命令的简化版 `minigrep`。

## 涉及的概念

1.  **接收命令行参数**: `std::env::args()`。
2.  **文件读取**: `std::fs::read_to_string()`。
3.  **错误处理**: 返回 `Result` 并使用 `Box<dyn Error>` 处理多种错误类型。
4.  **模块化**: 将逻辑拆分到 `lib.rs`，`main.rs` 只负责参数解析和错误退出。
5.  **环境变量**: 使用 `std::env::var()` 读取环境变量。
6.  **测试驱动开发 (TDD)**: 先写测试，再写实现。

## 如何运行

标准运行（区分大小写）：

```bash
cargo run -- query filename.txt
```

忽略大小写（设置环境变量）：

```bash
IGNORE_CASE=1 cargo run -- query filename.txt
```

## 代码结构

*   `src/main.rs`: 二进制入口，处理参数收集和错误报告。
*   `src/lib.rs`: 包含配置结构体、业务逻辑 (`run` 函数) 和搜索算法。
