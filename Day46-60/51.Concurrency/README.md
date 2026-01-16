# Day 51: 并发控制与锁

在 Day 50 中，我们使用了 `Mutex` 来保护 HashMap。这意味着在任何时刻，只能有一个任务访问数据库，无论是读还是写。对于读多写少的场景（如缓存），这会成为严重的性能瓶颈。

今天我们将优化并发控制，使用 **读写锁 (`RwLock`)** 来替代互斥锁。

## 1. Mutex vs RwLock

*   **Mutex (互斥锁)**: 一次只允许一个线程持有锁。
*   **RwLock (读写锁)**:
    *   允许多个**读者**同时持有读锁。
    *   只允许一个**写者**持有写锁。
    *   读锁与写锁互斥。

## 2. 选择合适的 RwLock

Rust 标准库提供了 `std::sync::RwLock`，Tokio 提供了 `tokio::sync::RwLock`。

*   如果临界区非常短且非异步（如单纯的 HashMap 查找），推荐使用 `std::sync::RwLock` (或者性能更好的 `parking_lot::RwLock`)。因为它开销小，且不会阻塞 Tokio 调度器太久。
*   如果临界区包含 `.await` 点（如 IO 操作），**必须**使用 `tokio::sync::RwLock`。

由于我们的 `Db` 操作（`get`/`set`）仅仅是内存操作，不包含 IO，因此我们选择同步的 `std::sync::RwLock`。

## 3. 实现

修改 `src/db.rs`:

```rust
use std::sync::{Arc, RwLock};

struct Shared {
    state: RwLock<State>,
}

// ...

impl Db {
    pub fn get(&self, key: &str) -> Option<Bytes> {
        // 获取读锁
        let state = self.shared.state.read().unwrap();
        state.entries.get(key).cloned()
    }

    pub fn set(&self, key: String, value: Bytes) {
        // 获取写锁
        let mut state = self.shared.state.write().unwrap();
        state.entries.insert(key, value);
    }
}
```

## 4. 进一步优化：分片 (Sharding)

虽然 `RwLock` 解决了读并发问题，但在高并发写入时，单一的锁仍然是热点。
一种常见的优化是**分片** (Sharding)：将数据分散到多个 HashMap 中，每个 HashMap 有自己的锁。通过 Hash(key) % N 来决定使用哪个分片。
Rust 社区有一个优秀的库 **`dashmap`** 实现了这种并发 Hash Map，我们在后续可以探索使用。
