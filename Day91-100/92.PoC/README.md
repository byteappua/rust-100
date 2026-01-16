# Day 92: 原型验证 (PoC)

本项目 (**DTask**) 的原型验证阶段。

## 验证目标

1.  **gRPC 通信**: 验证 Client -> Server 的 RPC 调用链路。
2.  **Raft 共识**: 验证 OpenRaft 库的集成和类型定义。

## 验证结果

### 1. gRPC 通信 (成功)

使用 `tonic` 实现了简单的 Echo 服务。
- Server 监听 `[::1]:50051`。
- Client 发送 "Hello Tonic"。
- 成功接收响应 "Echo: Hello Tonic"。

### 2. Raft 集成 (部分完成)

- 成功定义了 Raft 类型 (`TypeConfig`, `Request`, `Response`)。
- 成功搭建了 `RaftNetwork` 和 `RaftStorage` 的基础结构。
- **发现问题**: `openraft` 0.9 版本引入了较大的 API 变更 (V2 Storage)，实现完整的 `RaftStorage` trait 需要处理复杂的类型约束和宏 (`async_trait`) 兼容性问题。
- **决策**: 在 Day 93 (Core Development) 中集中解决 Storage 实现问题，或者回退到更稳定的 `openraft` 版本，或者使用 `openraft` 提供的默认 `MemStore` (如果适用)。

## 运行方式

```bash
cd Day91-100/92.PoC/dtask-poc

# 编译
cargo build

# 运行 gRPC Server
./target/debug/grpc_server &

# 运行 gRPC Client
./target/debug/grpc_client
```

## 项目结构

- `dtask-poc/`: Cargo 项目根目录
    - `proto/`: gRPC 协议定义
    - `src/bin/grpc_server.rs`: gRPC 服务端
    - `src/bin/grpc_client.rs`: gRPC 客户端
    - `src/lib.rs`: Raft 类型定义
    - `src/store.rs`: Raft 存储接口存根
    - `src/network.rs`: Raft 网络接口存根
