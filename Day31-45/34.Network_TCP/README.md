# Day 34: 网络编程 (Network Programming - TCP)

Rust 标准库 `std::net` 提供了 TCP 网络编程的基础。

## 1. TCP Server (服务端)

*   `TcpListener::bind(addr)`: 绑定端口。
*   `listener.incoming()`: 迭代接收连接 (`TcpStream`)。
*   通常为每个连接创建一个新线程来处理。

```rust
use std::net::TcpListener;
use std::thread;

let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

for stream in listener.incoming() {
    let stream = stream.unwrap();
    thread::spawn(|| {
        handle_connection(stream);
    });
}
```

## 2. TCP Client (客户端)

*   `TcpStream::connect(addr)`: 连接到服务器。
*   `stream.write_all()`: 发送数据。
*   `stream.read()`: 接收数据。

`TcpStream` 实现了 `Read` 和 `Write` Trait。

## 3. 注意事项

*   网络操作可能会阻塞，可以使用非阻塞模式或异步 I/O (如 Tokio) 来处理高并发。
*   处理 `read` 返回 `0` 的情况，这通常表示连接被对方关闭。
