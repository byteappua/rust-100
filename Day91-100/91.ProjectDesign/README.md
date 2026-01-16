# Day 91: 选题与架构设计

本项目选择了 **Option 1: 分布式任务调度系统** 作为毕业设计课题。

项目名称：**DTask** (Distributed Task Scheduler)

## 完成内容

详细的设计文档已创建于 [DESIGN.md](./design.md)。

主要包含：
1.  **需求分析**：定义了任务提交、分布式执行、集群管理等核心功能。
2.  **架构设计**：采用 Leader-Follower 架构，利用 Raft 算法保证调度器的高可用，Worker 节点支持水平扩展。
3.  **技术选型**：Rust + Tokio + Tonic (gRPC) + OpenRaft + PostgreSQL。
4.  **数据模型**：设计了任务表和执行日志表的数据库 Schema。
5.  **接口设计**：初步定义了 gRPC 服务接口。

## 下一步计划

Day 92 将进行原型验证 (PoC)，主要验证 gRPC 通信链路和 Raft 节点的基本组网。
