# Day 32: 序列化与反序列化 (Serialization with Serde)

在 Rust 中，处理序列化（将数据转换为可存储/传输的格式）和反序列化（将格式化数据转换回 Rust 对象）的事实标准是 `serde` 框架。

## 1. Serde 概览

Serde 是 **Ser**ialization 和 **De**serialization 的缩写。它是一个高效的通用序列化框架，不依赖于运行时反射，而是使用 Rust 的 Trait 系统和编译时宏派生。

## 2. 设置 Cargo.toml

你需要添加 `serde` 以及你想要支持的数据格式库（如 `serde_json`）。

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

*   `features = ["derive"]`: 允许我们使用 `#[derive(Serialize, Deserialize)]` 宏。

## 3. 基本用法

### 定义结构体

使用 `derive` 宏自动实现序列化 Trait。

```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: u8,
}
```

### 序列化 (To JSON)

```rust
let p = Person { name: "Alice".into(), age: 30 };
let json = serde_json::to_string(&p)?;
```

### 反序列化 (From JSON)

```rust
let data = r#"{"name": "Bob", "age": 25}"#;
let p: Person = serde_json::from_str(data)?;
```

## 4. 常见数据格式

Serde 支持多种格式，不仅仅是 JSON：
*   `serde_json`: JSON
*   `serde_yaml`: YAML
*   `toml`: TOML (Cargo 使用的格式)
*   `bincode`: 二进制格式 (高效)
