# Day 22: 并发编程 - 线程 (Threads)

Rust 中的并发编程非常安全，因为 Rust 的所有权和类型系统会在编译时捕获许多并发错误（如数据竞争）。

## 1. 创建线程

使用 `std::thread::spawn` 创建新线程。

```rust
use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}
```

注意：当主线程结束时，所有派生线程也会立即停止，不管它们是否执行完毕。

## 2. 等待线程结束 (join)

为了确保派生线程执行完毕，我们可以保存 `spawn` 返回的 `JoinHandle`，并调用 `.join()`。

```rust
let handle = thread::spawn(|| {
    // ...
});

handle.join().unwrap(); // 阻塞主线程直到 handle 代表的线程结束
```

## 3. 线程与 move 闭包

如果要在线程中使用主线程的数据，通常需要将数据的所有权**移动 (move)** 到线程中。

```rust
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    // 强行将 v 的所有权移入闭包
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
    // println!("{:?}", v); // 错误！v 已经被移动了
}
```
