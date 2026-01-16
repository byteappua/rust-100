# Day 50: 异步网络层 (Tokio)

今天我们将完成 Mini-Redis 的最后一块拼图：网络层。我们将把之前的组件（协议解析、命令处理、存储引擎）整合到一个基于 Tokio 的异步服务器中。

## 1. Connection 抽象

直接操作 `TcpStream` 进行字节读写比较繁琐。我们封装一个 `Connection` 结构体，它负责：
1.  维护底层的 `TcpStream`。
2.  维护读写缓冲区 (`BytesMut` / `BufWriter`)。
3.  提供面向 Frame 的 API: `read_frame()` 和 `write_frame(frame)`。

## 2. Server 主循环

服务器的逻辑非常标准：
1.  `TcpListener` 监听端口。
2.  循环 `accept()` 获取连接。
3.  为每个连接 `tokio::spawn` 一个新的异步任务。

## 3. Handler 逻辑

在每个任务中：
1.  循环调用 `connection.read_frame()`。
2.  如果有数据，调用 `Command::from_frame()` 解析为命令。
3.  调用 `command.apply(&db)` 执行命令。
4.  将结果写入 `connection.write_frame()`。

## 4. 运行验证

完成后，我们就可以使用真正的 Redis 客户端（如 `redis-cli`）来测试我们的服务器了！
