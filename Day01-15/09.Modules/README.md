# Day 09: 模块系统 (Modules)

Rust 的模块系统包括：
*   **包 (Packages)**
*   **Crates**
*   **模块 (Modules)**
*   **路径 (Paths)**

## 1. 模块定义

```rust
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
    }
}
```

## 2. 使用 use

```rust
use crate::front_of_house::hosting;
```
