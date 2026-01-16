# Day 90: Rust 编译优化与配置 (Compiler Optimization)

## 简介
本项目演示了如何通过配置 `Cargo.toml` 中的 Profile 来优化 Rust 二进制文件的大小和性能。

## 配置说明

我们定义了两个 Profile：
1.  **release** (默认): 关注性能，平衡编译时间。
2.  **min-size**: 关注二进制文件大小。

```toml
[profile.min-size]
inherits = "release"
opt-level = "z"     # 优化大小 (Optimize for size)
lto = true          # 链接时优化 (Link Time Optimization)
codegen-units = 1   # 减少并行代码生成单元 (Reduce parallelism for better optimization)
panic = "abort"     # Panic 时直接终止 (Abort on panic, removes unwinding symbols)
strip = true        # 剥离符号 (Strip symbols)
```

## 实验结果

在本项目中 (包含 `serde` 和 `serde_json`)，对比结果如下（示例）：

```bash
# 构建 Release 版本
cargo build --release

# 构建 Min-Size 版本
cargo build --profile min-size
```

文件大小对比 (approx):

-   `target/release/optimization-demo`: ~560K
-   `target/min-size/optimization-demo`: ~350K

**减少了约 37% 的体积！**

注意：
-   `strip = true` 会移除调试符号，使得调试变得困难。
-   `panic = "abort"` 会导致 panic 时无法捕获 (catch_unwind)，程序会直接退出。
-   `lto = true` 会显著增加编译时间。
