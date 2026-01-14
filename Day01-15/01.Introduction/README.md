# Day 01: 初识 Rust

## 1. Rust 简介

Rust 是一门系统编程语言，专注于安全，尤其是并发安全。它支持函数式和命令式以及泛型等编程范式的多范式语言。Rust 在语法上和 C++ 类似，但是设计者通过**所有权**系统保证了内存安全，在编译期就能消除各种内存错误。

### 为什么选择 Rust?

*   **高性能**：Rust 速度惊人且内存利用率极高。由于没有运行时和垃圾回收，它能够胜任对性能要求特别高的服务。
*   **可靠性**：Rust 丰富的类型系统和所有权模型保证了内存安全和线程安全，让您在编译期就消除各种错误。
*   **生产力**：Rust 拥有出色的文档、友好的编译器和顶级的工具链（包管理器 Cargo）。

## 2. 环境搭建

### 安装 Rust

推荐使用 `rustup` 工具链管理器来安装 Rust。

**Linux / macOS:**

打开终端并运行：

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

**Windows:**

访问 [https://rustup.rs/](https://rustup.rs/) 下载 `rustup-init.exe` 并运行。

### 验证安装

安装完成后，关闭并重新打开终端，输入：

```bash
rustc --version
cargo --version
```

如果看到版本号，说明安装成功。

## 3. Hello World

按照编程传统，我们从 Hello World 开始。

### 使用 rustc 直接编译

创建一个名为 `main.rs` 的文件，输入以下代码：

```rust
fn main() {
    println!("Hello, world!");
}
```

在终端编译并运行：

```bash
rustc main.rs
./main  # Windows 上是 .\main.exe
```

### 使用 Cargo (推荐)

Cargo 是 Rust 的构建系统和包管理器。

1.  **创建新项目**：

```bash
cargo new hello_world
cd hello_world
```

2.  **项目结构**：

```text
hello_world
├── Cargo.toml
└── src
    └── main.rs
```

3.  **运行项目**：

```bash
cargo run
```

你应当能看到输出 `Hello, world!`。

## 4. 练习

1.  成功安装 Rust 环境。
2.  使用 Cargo 创建一个新项目，输出 "Hello Rust!"。
