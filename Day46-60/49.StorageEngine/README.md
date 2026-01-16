# Day 49: 存储引擎实现

今天我们将实现 Mini-Redis 的核心：存储数据的部分。

## 1. 并发与共享状态

由于我们的服务器是多线程（通过 Tokio 异步任务）并发处理请求的，因此存储层必须是**线程安全**的。

最简单的方式是使用 `std::collections::HashMap` 配合 `std::sync::Mutex`。为了在多个任务间共享，我们需要将其包裹在 `std::sync::Arc` 中。

## 2. 定义 Db 结构体

```rust
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use bytes::Bytes;

#[derive(Clone)]
pub struct Db {
    shared: Arc<Shared>,
}

struct Shared {
    state: Mutex<State>,
}

struct State {
    entries: HashMap<String, Bytes>,
}
```

这里我们多包裹了一层 `Shared` 和 `State`，虽然对于目前的需求看起来有点冗余，但这是为了后续扩展（例如添加 Pub/Sub 功能）做准备。

## 3. 实现操作方法

我们需要实现 `new`, `get`, `set` 等基本操作：

```rust
impl Db {
    pub fn new() -> Db {
        let shared = Arc::new(Shared {
            state: Mutex::new(State {
                entries: HashMap::new(),
            }),
        });
        Db { shared }
    }

    pub fn get(&self, key: &str) -> Option<Bytes> {
        let state = self.shared.state.lock().unwrap();
        state.entries.get(key).cloned()
    }

    pub fn set(&self, key: String, value: Bytes) {
        let mut state = self.shared.state.lock().unwrap();
        state.entries.insert(key, value);
    }
}
```

注意：在 Tokio 中使用 `std::sync::Mutex` 是可以的，只要临界区（锁持有的时间）非常短，且不跨越 `.await` 调用。对于简单的 HashMap 操作，这是满足要求的，且比 `tokio::sync::Mutex` 性能更好。
