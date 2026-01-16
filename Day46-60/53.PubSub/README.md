# Day 53: 发布订阅 (Pub/Sub)

Redis 的发布订阅 (Pub/Sub) 是一种消息通信模式：发送者 (Publisher) 发送消息，订阅者 (Subscriber) 接收消息，它们之间不需要直接建立联系。

## 1. 核心概念

*   **Channel (频道)**: 消息传递的载体。
*   **Publish (发布)**: 向指定频道发送消息。
*   **Subscribe (订阅)**: 订阅一个或多个频道，接收发往该频道的消息。

## 2. 实现思路

在 Mini-Redis 中，我们需要在 `Db` 中维护频道的订阅关系。

```rust
use tokio::sync::broadcast;

struct State {
    entries: HashMap<String, Bytes>,
    /// Map channel name -> broadcast sender
    pub_sub: HashMap<String, broadcast::Sender<Bytes>>,
}
```

我们使用 `tokio::sync::broadcast` 通道。这是一个多生产者、多消费者的通道，非常适合实现 Pub/Sub。当有客户端订阅某个频道时，我们如果该频道不存在，就创建一个新的 broadcast channel。

## 3. 命令处理

*   **PUBLISH channel message**:
    1.  查找 `pub_sub` Map 中是否存在该 `channel`。
    2.  如果有，向其 `Sender` 发送 `message`。
    3.  返回接收到消息的订阅者数量。

*   **SUBSCRIBE channel**:
    1.  查找或创建 `channel` 对应的 `Sender`。
    2.  调用 `sender.subscribe()` 获取 `Receiver`。
    3.  进入**订阅模式**：不再处理普通命令，而是循环读取 `Receiver` 中的消息，并将其封装为 Frame 发送给客户端。

## 4. 限制

为了简化，今天的实现中，一旦客户端进入订阅模式，就只能接收订阅的消息，无法再退出（Ctrl+C 断开连接除外），也无法动态增加订阅频道（标准的 Redis 是支持的）。
