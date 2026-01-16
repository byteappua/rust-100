# Day 57: 哨兵与高可用 (Sentinel)

在分布式系统中，**高可用 (High Availability, HA)** 是核心需求之一。Redis Sentinel 是官方的高可用解决方案，它负责监控、通知和自动故障转移。

## 1. Sentinel 核心概念

Sentinel 主要包含以下三个功能：

1.  **监控 (Monitoring)**: 不断检查 Master 和 Replica 是否正常运行。
2.  **通知 (Notification)**: 当被监控的 Redis 实例出问题时，通过 API 通知管理员或其他应用程序。
3.  **自动故障转移 (Automatic Failover)**: 如果 Master 挂了，Sentinel 会将一个 Replica 提升为新的 Master，并通知其他 Replica 和客户端使用新的 Master。

## 2. 本节目标

在本节中，我们实现了一个简化的 Sentinel 模拟器 (`SentinelService`)：

-   它作为一个独立服务运行。
-   它定期向 Master 发送 `PING` 命令进行健康检查。
-   如果 Master 无响应，它会自动更新内部状态，将 Master 指向备用的 Replica 地址（模拟 Failover）。

## 3. 实现细节

### PING 命令

为了支持健康检查，我们首先在 Server 端实现了 `PING` 命令。

```rust
// Client side
pub async fn ping(&mut self, msg: Option<Bytes>) -> Result<Bytes, Error>
```

### Sentinel 服务

`SentinelService` 维护当前 Master 的地址和副本列表。

```rust
pub struct SentinelService {
    master_addr: Arc<Mutex<String>>,
    replicas: Vec<String>,
}
```

它启动一个循环，每隔一段时间（例如 500ms）连接 Master 并发送 Ping。如果连接失败或 Ping 失败，触发 `failover()` 逻辑。

### 模拟故障转移

为了演示，我们的 `examples/sentinel_demo.rs` 使用 `tokio::sync::broadcast` 来编程控制 Server 的启动和关闭。

1.  启动 Master (6379) 和 Replica (6380)。
2.  Sentinel 开始监控 6379。
3.  Client 通过咨询 Sentinel 获取 Master 地址（6379），并写入数据。
4.  Demo 程序发送关闭信号给 Master。
5.  Sentinel 检测到 6379 不可达，将 Master 切换为 6380。
6.  Client 再次咨询 Sentinel，获取到新的 Master 地址（6380），并成功连接。

## 4. 运行演示

```bash
cargo run --example sentinel_demo
```

输出示例：

```text
>>> Servers started: Master(6379), Replica(6380)
>>> Sentinel started monitoring.
>>> Client querying Sentinel... Current Master: 127.0.0.1:6379
>>> Client wrote 'status'='ok' to 127.0.0.1:6379

>>> SIMULATING FAILURE: Shutting down Master(6379)...
>>> Client querying Sentinel... New Master: 127.0.0.1:6380
>>> SUCCESS: Failover occurred correctly.
>>> Ping new master: b"PONG"
```
