# Day 01: 初识 Rust

## 📝 学习目标
- 了解 Rust 语言的特性和优势
- 搭建 Rust 开发环境 (`rustup`, `cargo`, `rustc`)
- 掌握 Cargo 的基本命令 (`new`, `run`, `build`)
- 编写并运行第一个 Rust 程序

## 🎯 为什么要学这个
Rust 连续多年被 Stack Overflow 评为"最受喜爱的编程语言"。
- **内存安全**：没有 GC 也能保证内存安全，告别段错误 (Segmentation Fault)。
- **高性能**：媲美 C/C++ 的运行速度。
- **现代化工具链**：Cargo 提供了开箱即用的包管理、构建和测试体验。
- **广泛应用**：从操作系统 (Linux, Windows) 到 WebAssembly，从区块链到高性能网络服务，Rust 无处不在。

## 📖 核心概念

### 1. 核心组件
- **`rustc`**: Rust 编译器。通常由 Cargo 调用，很少直接手动使用。
- **`rustup`**: 工具链管理器。用于安装、更新 Rust 版本和管理组件。
- **`cargo`**: Rust 的构建系统和包管理器。你的日常好伙伴。

### 2. Hello World 剖析
```rust
fn main() {
    println!("Hello, world!");
}
```
- `fn main() {}`: 程序入口点。
- `println!`: 这是一个 **宏 (Macro)**，而不是函数（注意那个 `!`）。它用于打印文本到控制台。

### 3. Cargo 项目结构
当你运行 `cargo new my_project` 时，Cargo 会生成：
```text
my_project
├── Cargo.toml    # 配置文件 (依赖、元数据)
├── src
│   └── main.rs   # 源代码
└── .gitignore    # Git 忽略文件
```

## 💻 常用命令

```bash
# 安装 Rust (Linux/macOS)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 检查版本
rustc --version
cargo --version

# 创建新项目
cargo new hello_cargo

# 编译并运行 (Debug 模式)
cargo run

# 仅编译
cargo build

# 编译发布版本 (Release 模式，带优化)
cargo build --release

# 检查代码是否能编译 (速度快，不生成可执行文件)
cargo check
```

## 🏋️ 练习题

我们为你准备了从基础输出到 Cargo 配置的练习。

- **练习 1**: Hello, Rust!
- **练习 2**: 个人信息卡片
- **练习 3**: 简单计算器
- **练习 4**: ASCII 艺术
- **练习 5**: Cargo 项目结构

👉 **[点击这里查看练习题](./exercises/README.md)**

## 🤔 常见问题 (FAQ)

### Q1: 为什么第一次 `cargo run` 比较慢？
A: Cargo 需要下载依赖（如果有）并编译整个项目及其依赖。之后的运行会利用缓存，速度会快很多。

### Q2: `Cargo.lock` 是什么？
A: 它记录了你项目所有依赖库的确切版本号，确保你的项目在任何机器上构建结果一致。**不要**手动修改它。

### Q3: 为什么 `println!` 后面有个感叹号？
A: 在 Rust 中，以 `!` 结尾的调用是 **宏 (Macro)**。宏在编译期展开代码，比普通函数更灵活强大。后续课程会详细介绍。

## 💡 最佳实践
- **始终使用 Cargo**：即使是小脚本，也建议用 `cargo new` 创建项目，以便于依赖管理。
- **经常使用 `cargo check`**：在编写代码时，用它快速检查语法错误，比 `cargo build` 快得多。
- **格式化代码**：运行 `cargo fmt` 自动格式化代码，保持风格统一。

## 🔗 扩展阅读
- [Rust 程序设计语言 - 入门指南](https://doc.rust-lang.org/book/ch01-00-getting-started.html)
- [Cargo 手册](https://doc.rust-lang.org/cargo/)

## 📚 本节要点回顾
- 使用 `rustup` 管理环境。
- 使用 `cargo new` 创建项目。
- 使用 `cargo run` 运行项目。
- `println!` 是宏。

## ⏭️ 下一步
环境搭建好了，让我们开始学习 Rust 的基础语法。

下一节: [Day 02: 变量与数据类型](../02.VariablesAndTypes/README.md)
