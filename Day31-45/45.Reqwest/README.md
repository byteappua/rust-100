# Day 45: HTTP 客户端 (Reqwest)

**`reqwest`** 是 Rust 生态中最流行的 HTTP 客户端库。它建立在 `hyper` 之上，提供了更高级、更易用的 API。

## 1. 特性

*   **异步与同步**: 默认提供异步 (Async) API，也通过 `blocking` 特性提供同步 API。
*   **JSON 支持**: 集成 `serde`，方便地处理 JSON 数据。
*   **功能丰富**: 支持 HTTPS, Proxy, Cookies, Connection Pooling 等。

## 2. 依赖配置

在 `Cargo.toml` 中添加依赖：

```toml
[dependencies]
reqwest = { version = "0.12", features = ["json"] }
tokio = { version = "1", features = ["full"] } # 异步运行时
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

## 3. 基本用法

### 发送 GET 请求

```rust
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;

    println!("{:#?}", resp);
    Ok(())
}
```

### 发送 POST 请求

```rust
let client = reqwest::Client::new();
let res = client.post("https://httpbin.org/post")
    .body("the exact body that is sent")
    .send()
    .await?;
```

### 处理 JSON 响应

我们可以定义结构体来自动反序列化响应数据：

```rust
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct IpResponse {
    origin: String,
}

// ...
let ip: IpResponse = reqwest::get("https://httpbin.org/ip").await?.json().await?;
```
