# Day 09: 模块系统 (Modules)

Rust 拥有一个强大的模块系统，用于组织代码、控制私有性（公有/私有）。这包括：
*   **包 (Packages)**: Cargo 的一个功能，构建、测试和共享 crate。
*   **Crates**: 一个模块的树形结构，它产生一个库或可执行文件。
*   **模块 (Modules)**: 让你可以控制代码的组织、作用域和私有性路径。
*   **路径 (Paths)**: 为 struct、function 或 module 等项命名的方式。

## 1. 包与 Crates

*   **Crate Root**: 源代码文件，Rust 编译器从这里开始，组成你的 Crate 的根模块（通常是 `src/lib.rs` 或 `src/main.rs`）。
*   **Package**: 包含 `Cargo.toml` 文件的目录，描述了如何构建一个或多个 Crates。

## 2. 定义模块 (Modules)

使用 `mod` 关键字。模块可以嵌套。

```rust
// lib.rs

mod front_of_house {
    // 默认是私有的
    mod hosting {
        fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // 报错！hosting 模块是私有的
    // front_of_house::hosting::add_to_waitlist();
}
```

## 3. 路径与可见性 (Paths & Visibility)

默认情况下，Rust 中所有项（函数、方法、结构体、枚举、模块和常量）都是私有的。父模块无法使用子模块中的私有项，但子模块可以使用祖先模块中的项。

要使项公开，使用 `pub` 关键字。

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // 绝对路径 (从 Crate 根开始)
    crate::front_of_house::hosting::add_to_waitlist();

    // 相对路径 (从当前模块开始)
    front_of_house::hosting::add_to_waitlist();
}
```

### super 关键字

可以使用 `super` 开头来构建从父模块开始的相对路径（类似于文件系统中的 `..`）。

```rust
fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order(); // 调用父模块的函数
    }

    fn cook_order() {}
}
```

### 结构体与枚举的可见性

*   如果结构体是 `pub` 的，其字段默认仍是私有的。必须逐个标记为 `pub`。
*   如果枚举是 `pub` 的，其所有成员默认都是 `pub` 的。

## 4. use 关键字

我们可以使用 `use` 关键字创建一个短路径（引入作用域），就像文件系统中的符号链接。

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

### 习惯用法

*   **函数**: 引入函数的父模块。例如 `use std::collections::HashMap`。
*   **结构体/枚举**: 引入完整路径。例如 `use std::collections::HashMap`。

### as 关键字

使用 `as` 指定别名，解决名称冲突。

```rust
use std::fmt::Result;
use std::io::Result as IoResult;
```

### pub use 重导出

使用 `pub use` 将引入的项再次导出，让外部代码也可以使用它。

```rust
pub use crate::front_of_house::hosting;
```

## 5. 将模块拆分为多个文件

当模块变大时，可以将其移动到单独的文件中。

### 方式 1: 新风格 (推荐)

**src/lib.rs**:
```rust
mod front_of_house; // 声明模块，内容在 src/front_of_house.rs 中

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

**src/front_of_house.rs**:
```rust
pub mod hosting; // 声明子模块，内容在 src/front_of_house/hosting.rs 中
```

**src/front_of_house/hosting.rs**:
```rust
pub fn add_to_waitlist() {}
```

### 方式 2: 旧风格 (mod.rs)

**src/front_of_house/mod.rs**: 代替 `src/front_of_house.rs`。这在 Rust 2015 中是必须的，但在 Rust 2018+ 中仍然支持。
