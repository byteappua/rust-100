# Day 93: 核心模块开发 (一)

本项目 (**DTask**) 的核心开发第一阶段。

## 开发目标

1.  **项目骨架搭建**: 建立基本的 Cargo 项目结构。
2.  **Raft 存储层实现**: 实现 `openraft` 所需的 `RaftStorage` trait。
3.  **核心数据模型**: 定义任务 (`Task`) 和状态机所需的类型。

## 完成内容

### 1. Raft 存储实现 (`MemStorage`)

由于 `openraft` 0.9+ 的 API 变更，本阶段首先实现了一个基于内存的存储层 `MemStorage`，用于快速验证共识逻辑。

-   实现了 `RaftLogReader`、`RaftStorage`、`RaftSnapshotBuilder` 等 Trait。
-   使用 `RwLock` 保护内存中的日志 (`log`) 和状态机 (`state_machine`)。
-   支持基本的日志追加、读取和快照操作。

### 2. 核心模型定义

在 `src/models.rs` 中定义了核心数据结构：

-   `Task`: 包含任务 ID、命令、状态等字段。
-   `Request`: Raft 状态机的写请求（如 `Set` 任务）。
-   `Response`: Raft 状态机的读响应。

### 3. 服务骨架

-   `scheduler.rs`: 定义了 `TaskScheduler` 结构体，作为系统的核心调度组件。
-   `dispatcher.rs`: 定义了 `Dispatcher`，负责后台任务的分发和执行。

## 运行方式

本阶段主要关注编译通过和单元测试，尚未提供完整的可运行 Server。

```bash
cd Day91-100/93.CoreDevelopment1/dtask
cargo build
cargo test
```

## 下一步

Day 94 将完善 `TaskScheduler` 的 gRPC 接口，并实现 `Dispatcher` 的实际轮询逻辑。
