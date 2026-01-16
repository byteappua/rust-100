# Day 84: 原始套接字 (Raw Sockets)

详细内容请参考 [Stage 6 Overview](../STAGE6_OVERVIEW.md)。

## 核心内容
- TCP/UDP 底层操作
- `socket2` crate 的使用
- 原始套接字编程基础
- 网络协议实现

## 示例项目 `raw-sockets`

本项目演示了如何使用 `socket2` 进行更底层的套接字配置，以及标准的 `std::net` TCP 编程。

### 功能演示
1. **Socket2 Creation**: 演示使用 `socket2` crate 创建并配置（非阻塞、地址复用）一个 TCP 套接字。
2. **TCP Echo Server**: 一个运行在后台线程的简单 TCP 服务器。
3. **TCP Client**: 一个连接到服务器并发送/接收数据的客户端。

### 运行代码
```bash
cd Day81-90/84.RawSockets
cargo run
```

## 下一步
Day 85 将学习嵌入式 Rust 简介。
