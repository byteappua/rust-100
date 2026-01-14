# Day 23: 并发编程 - 消息传递 (Message Passing)

Go 语言有一句名言："不要通过共享内存来通信，而要通过通信来共享内存。" Rust 也支持这种方式，主要通过 **Channel (通道)** 来实现。

## 1. mpsc 通道

Rust 标准库提供了 `mpsc` (Multiple Producer, Single Consumer)，即**多生产者，单消费者**通道。

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    // tx: 发送端 (transmitter)
    // rx: 接收端 (receiver)
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        // send 会转移所有权，val 在这里之后不能再被使用
    });

    // recv() 会阻塞直到收到一个值
    // try_recv() 不会阻塞
    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
```

## 2. 发送多个值与遍历接收

接收端 `rx` 可以被当作迭代器使用。当通道关闭且没有剩余值时，迭代结束。

```rust
thread::spawn(move || {
    let vals = vec![
        String::from("hi"),
        String::from("from"),
        String::from("thread"),
    ];

    for val in vals {
        tx.send(val).unwrap();
        thread::sleep(Duration::from_secs(1));
    }
}); // 闭包结束，tx 离开作用域，通道被关闭

for received in rx {
    println!("Got: {}", received);
}
```

## 3. 多生产者

可以通过 `tx.clone()` 创建多个发送端，它们都发送到同一个接收端。

```rust
let (tx, rx) = mpsc::channel();
let tx1 = tx.clone();
thread::spawn(move || { tx1.send(...).unwrap(); });
thread::spawn(move || { tx.send(...).unwrap(); });
```
