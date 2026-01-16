# Day 55: 性能基准测试

在软件开发中，性能优化离不开**基准测试 (Benchmarking)**。Rust 社区最流行的基准测试库是 **Criterion.rs**。

## 1. Criterion 简介

Criterion.rs 提供了统计驱动的基准测试，能够自动处理预热、多次迭代、统计分析，并生成详细的 HTML 报告。

## 2. 配置 Benchmarking

首先，在 `Cargo.toml` 中添加依赖，并配置 `[[bench]]`：

```toml
[dev-dependencies]
criterion = "0.5"

[[bench]]
name = "my_benchmark"
harness = false
```

## 3. 编写 Benchmark

我们将在 `benches/my_benchmark.rs` 中编写测试代码。为了简单起见，我们直接测试核心组件（如 RESP 解析或内存数据库操作），避免网络 I/O 的干扰，从而获得更纯粹的代码性能数据。

### 测试用例：Frame 解析

我们将测试 `Frame::parse` 在处理大量数据时的性能。

### 测试用例：DB 操作

我们将测试 `Db::set` 和 `Db::get` 在高并发下的吞吐量（虽然 Criterion 主要测试延迟，但我们可以通过迭代次数估算）。

## 4. 运行测试

使用 `cargo bench` 命令运行。

```bash
cargo bench
```

运行完成后，可以在 `target/criterion/report/index.html` 查看可视化的报告。
