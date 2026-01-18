# Day 20: 智能指针 - Rc<T> 与 RefCell<T>

## 📝 学习目标
- 理解 `Rc<T>` 如何实现多重所有权。
- 理解 `RefCell<T>` 的内部可变性（Interior Mutability）模式。
- 掌握编译时借用检查与运行时借用检查的区别。
- 能够组合使用 `Rc<RefCell<T>>` 来实现可变的共享数据。

## 🎯 为什么要学这个
在 Rust 中，所有权规则非常严格：一个值在同一时间只能有一个所有者，或者多个不可变借用，或者一个可变借用。
但在实际数据结构（如双向链表、图、GUI 控件树）中，一个节点可能被多个其他节点引用，或者我们需要在不可变引用的上下文中修改数据（例如缓存、模拟对象）。`Rc` 和 `RefCell` 提供了绕过严格静态检查的安全机制。

## 📖 核心概念

### 1. Rc<T> (引用计数)
`Rc` 代表 **Reference Counting**。它用于在堆上分配数据，供程序的多个部分读取。
*   **多重所有权**：通过 `Rc::clone(&a)` 创建新的指向同一数据的引用（增加计数），而不是深拷贝数据。
*   **不可变性**：`Rc<T>` 指向的数据默认是不可变的。
*   **单线程**：`Rc` 不是线程安全的。多线程请使用 `Arc`。

### 2. RefCell<T> (内部可变性)
`RefCell<T>` 允许你在持有不可变引用的前提下修改内部数据。
*   **运行时检查**：普通引用在编译时检查借用规则。`RefCell` 在**运行时**检查。
*   **Borrowing**：
    *   `borrow()`: 返回 `Ref<T>`（类似 `&T`）。
    *   `borrow_mut()`: 返回 `RefMut<T>`（类似 `&mut T`）。
*   **Panic**：如果在运行时违反借用规则（例如同时存在两个 `RefMut`），程序会 Panic。

### 3. 组合技: Rc<RefCell<T>>
如果你需要一个数据既有**多个所有者**，又能**被修改**，就可以结合使用：
*   `Rc` 负责让数据有多个所有者。
*   `RefCell` 负责让拥有者可以修改数据。

```rust
use std::rc::Rc;
use std::cell::RefCell;

let value = Rc::new(RefCell::new(5));
let a = Rc::clone(&value);
let b = Rc::clone(&value);

*a.borrow_mut() += 10;
println!("b sees: {}", b.borrow()); // 15
```

## 💻 代码示例

### 示例 1: Rc 共享列表

```rust
use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a)); // b 和 c 都共享 a
}
```

### 示例 2: RefCell 修改值

```rust
use std::cell::RefCell;

fn main() {
    let x = RefCell::new(42);

    {
        let mut m = x.borrow_mut();
        *m += 1;
    } // m 离开作用域，借用结束

    println!("x = {:?}", x.borrow());
}
```

## 🏋️ 练习题

### 练习 1: Rc 基础
创建两个列表 `b` 和 `c`，它们都共享列表 `a` 的尾部。打印引用计数的变化。
[查看参考答案](./exercises/solutions/exercise1_solution.rs)

### 练习 2: 内部可变性
编写一个 Mock Messenger，它记录发送给它的消息。使用 `RefCell` 来在不可变方法 `send` 中修改内部的 `sent_messages` 列表。
[查看参考答案](./exercises/solutions/exercise2_solution.rs)

### 练习 3: 共享可变树节点
定义一个树节点结构体，每个节点包含一个值和子节点列表。使用 `Rc<RefCell<Node>>` 使得父节点和子节点可以互相引用（或者简单的多所有权修改测试），并修改某个子节点的值，验证父节点也能看到修改。
[查看参考答案](./exercises/solutions/exercise3_solution.rs)

## 🤔 常见问题

### Q1: `Cell<T>` 和 `RefCell<T>` 有什么区别？
A:
- `Cell<T>`: 适用于实现了 `Copy` 的类型（如整数）。它通过**复制**值进出来工作 (`get`, `set`)，没有借用检查开销，永远不会 Panic。
- `RefCell<T>`: 适用于任何类型。它通过**引用**工作 (`borrow`, `borrow_mut`)，有运行时开销和 Panic 风险。

### Q2: 为什么不总是使用 `Rc<RefCell<T>>`？
A:
1.  **性能开销**：引用计数更新和运行时借用检查都有开销。
2.  **安全性**：运行时 Panic 比编译时错误更糟糕。
3.  **复杂性**：类型签名变得很长且难以阅读。
只在确实需要时使用。

## 🔗 扩展阅读
- [Rust Book: RefCell<T> and the Interior Mutability Pattern](https://doc.rust-lang.org/book/ch15-05-interior-mutability.html)
- [Rust Book: Reference Cycles](https://doc.rust-lang.org/book/ch15-06-reference-cycles.html)
