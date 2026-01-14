# Day 20: 智能指针 - Rc<T> 与 RefCell<T>

## 1. Rc<T> (引用计数)

`Rc<T>` (Reference Counting) 允许一个值有多个所有者。当所有者数量为 0 时，值才会被清理。

*   **特点**：只能用于单线程场景。
*   **用途**：用于图结构等多个部分读取同一数据的情况。

```rust
use std::rc::Rc;

let a = Rc::new(5);
let b = Rc::clone(&a); // 增加引用计数，不会深拷贝
println!("count = {}", Rc::strong_count(&a));
```

## 2. RefCell<T> (内部可变性)

`RefCell<T>` 代表其数据的单一所有权。它与 `Box<T>` 的区别在于：
*   `Box<T>`: 编译时执行借用规则检查。
*   `RefCell<T>`: **运行时**执行借用规则检查。如果违反规则（如同时存在两个可变借用），程序会 panic。

### 内部可变性模式

允许你修改一个不可变引用背后的数据。

```rust
use std::cell::RefCell;

let x = RefCell::new(5);
*x.borrow_mut() += 1;
assert_eq!(*x.borrow(), 6);
```

## 3. 结合 Rc<T> 和 RefCell<T>

这是 Rust 中常见的模式：拥有多个所有者，且每个所有者都能修改数据。

```rust
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use std::rc::Rc;
use std::cell::RefCell;

let value = Rc::new(RefCell::new(5));
// ...
```
