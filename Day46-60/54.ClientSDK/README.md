# Day 54: 客户端 SDK 实现

到目前为止，我们已经构建了一个功能相对完善的 Mini-Redis 服务器。现在，为了让开发者更方便地使用它，我们需要提供一个 **Client SDK**。

## 1. 目标

封装底层的 TCP 连接、Frame 编解码，提供符合 Rust 习惯的高级 API。

```rust
// 目标 API
let mut client = Client::connect("127.0.0.1:6379").await?;
client.set("hello", "world".into()).await?;
let val = client.get("hello").await?;
```

## 2. 结构设计

*   `Client`: 核心结构体，持有一个 `Connection`。
*   `Subscriber`: 专门用于 Pub/Sub 模式的结构体。

## 3. 实现

我们将重用之前定义的 `Connection` 和 `Frame`，并暴露给外部（在 `lib.rs` 中）。

在 `src/client.rs` 中：
```rust
pub struct Client {
    connection: Connection,
}

impl Client {
    pub async fn connect(addr: &str) -> Result<Client> { ... }
    pub async fn get(&mut self, key: &str) -> Result<Option<Bytes>> { ... }
    pub async fn set(&mut self, key: &str, value: Bytes) -> Result<()> { ... }
    // ...
}
```

## 4. 示例

提供 `examples/hello_world.rs` 来展示用法。
