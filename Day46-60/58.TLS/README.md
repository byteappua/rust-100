# Day 58: 安全性 (TLS)

在生产环境中，数据传输的安全性至关重要。Redis 支持 TLS/SSL 加密，以防止数据在传输过程中被窃听或篡改。

## 1. TLS 基础

**TLS (Transport Layer Security)** 协议位于 TCP 和应用层之间，为应用程序提供加密通信。

核心组件：
-   **证书 (Certificate)**: 用于验证服务器（或客户端）的身份。
-   **私钥 (Private Key)**: 用于解密数据和签名。
-   **CA (Certificate Authority)**: 证书颁发机构，用于签署证书，建立信任链。

## 2. 本节目标

在本节中，我们为 Mini-Redis 添加 TLS 支持：

1.  **Server 端**: 使用 `tokio-rustls` 包装 `TcpStream`，并在握手成功后处理加密流。
2.  **Client 端**: 增加 `connect_tls` 方法，验证服务器证书并建立加密连接。
3.  **证书生成**: 提供脚本生成自签名的 CA 和服务器证书用于测试。

## 3. 实现细节

### 依赖

我们需要 `tokio-rustls` (基于 Rustls 的 Tokio 绑定) 和 `rustls-pemfile` (用于解析 PEM 格式的证书)。

```toml
[dependencies]
tokio-rustls = "0.26"
rustls = "0.23"
rustls-pemfile = "2.1"
```

### Server 实现

我们修改了 `server::run`，使其接受一个可选的 `TlsAcceptor`。如果提供了 Acceptor，则对每个连接执行 TLS 握手。

为了使 `process` 函数能够同时处理 `TcpStream` 和 `TlsStream<TcpStream>`，我们将连接抽象为 generic trait `AsyncStream` (需满足 `AsyncRead + AsyncWrite + Unpin + Send`)。

### Client 实现

增加了 `Client::connect_tls` 方法，它需要一个 `TlsConnector` 和目标域名（用于验证证书上的 CN/SAN）。

## 4. 运行演示

首先，生成测试证书：

```bash
cd Day46-60/58.TLS
./gen_certs.sh
```

运行 TLS 演示：

```bash
cargo run --example tls_demo
```

你将看到客户端成功通过 TLS 连接到服务器，并执行 PING, SET, GET 命令。

```text
Certificates generated in certs/
>>> TLS Server started on 127.0.0.1:6379
>>> Client connecting via TLS...
>>> PING...
<<< PONG: b"PONG"
>>> SET hello = world
>>> GET hello
<<< GET: Some(b"world")
>>> TLS Communication Verified!
```
