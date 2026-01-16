# Day 56: 集群模式探索

随着数据量和并发量的增加，单机 Redis 很难满足需求。为了解决扩展性（Scalability）和高可用性（High Availability）问题，通常会采用**集群（Cluster）**架构。

## 1. 集群架构概述

Redis 集群主要有两种常见的实现方式：

1.  **客户端分片 (Client-side Sharding)**:
    -   客户端通过某种 Hash 算法（如 Consistent Hashing 或简单的 Modulo）决定将 Key 存储在哪个节点。
    -   优点：实现简单，服务端无状态，无中心组件。
    -   缺点：扩容缩容（Resharding）麻烦，客户端需要知道所有节点信息。

2.  **代理分片 (Proxy Sharding)**:
    -   引入中间件（如 Twemproxy, Codis），客户端只连接 Proxy，Proxy 负责转发。
    -   优点：客户端透明。
    -   缺点：多了一跳网络开销，Proxy 成为新的瓶颈或单点。

3.  **Redis Cluster (服务端分片)**:
    -   Redis 官方集群方案。节点之间通过 Gossip 协议通信，数据分片存储在 16384 个 Slot 中。
    -   支持自动故障转移和重分片。

## 2. 本节目标

在本节中，我们将探索最基础的**客户端分片**模式。我们将改造 `Client` SDK，使其能够管理多个 Redis 节点，并根据 Key 将请求路由到正确的节点。

## 3. 实现细节

### ClusterClient

我们需要一个新的客户端结构体 `ClusterClient`，它维护一个节点列表：

```rust
pub struct ClusterClient {
    nodes: Vec<String>,
    clients: HashMap<String, Arc<Mutex<Client>>>,
}
```

### 路由算法 (Routing)

为了演示，我们使用 Rust 标准库的 `DefaultHasher` 对 Key 进行哈希，然后对节点数量取模：

```rust
let index = (hash(key) as usize) % self.nodes.len();
let target_node = &self.nodes[index];
```

> **注意**: 在生产环境中，应该使用一致性哈希（Consistent Hashing）或 Redis Cluster 的 CRC16(key) % 16384 算法，以减少节点变动时的数据迁移量。这里为了演示方便，使用了简单的取模哈希。

## 4. 运行演示

我们提供了一个示例 `examples/cluster_demo.rs`，它会：
1.  启动 3 个 Mini-Redis 服务器实例（端口 9001, 9002, 9003）。
2.  初始化 `ClusterClient`。
3.  向集群写入多个 Key，并打印每个 Key 被路由到了哪个节点。
4.  读取这些 Key，验证数据的一致性。

运行命令：

```bash
cargo run --example cluster_demo
```

你将看到类似以下的输出，显示 Key 被均匀分布到了不同的节点上：

```text
>>> Cluster started with nodes: ["127.0.0.1:9001", "127.0.0.1:9002", "127.0.0.1:9003"]

>>> Writing keys...
SET apple -> value-apple (Target: 127.0.0.1:9003)
SET banana -> value-banana (Target: 127.0.0.1:9001)
SET cherry -> value-cherry (Target: 127.0.0.1:9002)
...
```
