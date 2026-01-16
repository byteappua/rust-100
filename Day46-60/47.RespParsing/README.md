# Day 47: 协议解析 (Resp)

Redis 使用 RESP (Redis Serialization Protocol) 协议进行客户端与服务器之间的通信。它的设计目标是：
1.  易于实现。
2.  解析速度快。
3.  人类可读。

## 1. RESP 数据类型

RESP 是一个基于文本的协议（除了 Bulk Strings 中包含的二进制数据），通过第一个字节来区分数据类型：

*   `+`: 简单字符串 (Simple Strings)
    *   例如: `+OK\r\n`
*   `-`: 错误 (Errors)
    *   例如: `-Error message\r\n`
*   `:`: 整数 (Integers)
    *   例如: `:1000\r\n`
*   `$`: 批量字符串 (Bulk Strings)
    *   例如: `$6\r\nfoobar\r\n`
*   `*`: 数组 (Arrays)
    *   例如: `*2\r\n$3\r\nfoo\r\n$3\r\nbar\r\n`

## 2. 定义 Frame 枚举

我们使用枚举 `Frame` 来表示解析后的数据帧：

```rust
use bytes::Bytes;

#[derive(Clone, Debug)]
pub enum Frame {
    Simple(String),
    Error(String),
    Integer(i64),
    Bulk(Bytes),
    Null, // 特殊的 Bulk String ($-1\r\n)
    Array(Vec<Frame>),
}
```

## 3. 实现解析逻辑

我们将实现一个简单的解析器，能够将字节流解析为 `Frame`。由于这里只做演示，我们重点实现 `Frame` 结构的定义和基本的解析单元测试。

在实际的网络编程中，我们通常需要处理“半包”和“粘包”问题，这需要配合 `tokio_util::codec` 模块中的 `Decoder` trait 来完成。但在今天，我们先手动实现一个简单的解析函数来理解原理。

```rust
use bytes::Bytes;
use std::io::Cursor;

// 模拟解析一个字符串
pub fn parse_frame(src: &mut Cursor<&[u8]>) -> Result<Frame, String> {
    // ... 实现解析逻辑
    Err("Not implemented".to_string())
}
```
