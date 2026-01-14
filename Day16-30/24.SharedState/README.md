# Day 24: 并发编程 - 共享状态 (Shared State)

另一种并发模型是共享内存。在 Rust 中，这通常意味着多线程同时访问和修改同一块数据。

## 1. Mutex<T> (互斥器)

`Mutex` (Mutual Exclusion) 允许一次只允许一个线程访问数据。要访问数据，线程必须先获取锁 (`lock`)。

*   `Mutex<T>` 提供**内部可变性**（类似于 `RefCell<T>`）。
*   `lock()` 返回一个 `MutexGuard` 智能指针，当它离开作用域时，锁会自动释放。

```rust
use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    } // 锁在这里自动释放

    println!("m = {:?}", m);
}
```

## 2. Arc<T> (原子引用计数)

要在多个线程间共享 `Mutex<T>` 的所有权，我们需要类似 `Rc<T>` 的东西。但是 `Rc<T>` 并不是线程安全的。

Rust 提供了 `Arc<T>` (Atomic Reference Counting)，它是线程安全的引用计数智能指针。

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
```

注意：`Arc` 会带来一些性能损耗，所以如果不需要多线程共享，优先使用 `Rc`。
