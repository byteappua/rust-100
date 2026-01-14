# Day 18: Cargo 进阶与 Crates.io

Cargo 不仅是构建工具，也是依赖管理和包管理器。

## 1. 发布配置 (Release Profiles)

Cargo 有两个主要的配置：
*   `dev` profile: `cargo build` 使用。默认优化程度低，包含 debug 信息。
*   `release` profile: `cargo build --release` 使用。默认优化程度高。

可以在 `Cargo.toml` 中自定义：

```toml
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
```

## 2. 文档注释

使用 `///` 编写文档注释，支持 Markdown。

```rust
/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
```

*   运行 `cargo doc --open` 生成并查看文档。
*   文档注释中的代码块会被 `cargo test` 作为测试运行（文档测试）。

## 3. 发布到 Crates.io

1.  注册账号并获取 API token。
2.  `cargo login <token>`。
3.  在 `Cargo.toml` 中添加元数据（license, description, etc.）。
4.  `cargo publish`。

注意：一旦发布，版本永久存在。可以使用 `cargo yank` 撤回（标记为不可用，但已存在的依赖不受影响）。

## 4. Cargo 工作空间 (Workspaces)

工作空间是一组共享同一个 `Cargo.lock` 和输出目录的包。

### 目录结构

```
workspace_demo/
├── Cargo.toml
├── add_one/
│   ├── Cargo.toml
│   └── src/main.rs
└── adder/
    ├── Cargo.toml
    └── src/main.rs
```

### 根 Cargo.toml

```toml
[workspace]

members = [
    "adder",
    "add_one",
]
```
