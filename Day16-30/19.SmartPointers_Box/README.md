# Day 19: 智能指针 - Box<T>

## 📝 学习目标
- 理解栈 (Stack) 与堆 (Heap) 的区别。
- 掌握 `Box<T>` 的使用场景。
- 能够使用 `Box<T>` 定义递归数据结构（如链表）。
- 理解 `Deref` trait 和解引用强制转换 (Deref Coercion)。
- 理解 `Drop` trait 和资源清理。

## 🎯 为什么要学这个
在 Rust 中，默认情况下所有值都分配在栈上。栈非常快，但空间有限且要求数据大小固定。智能指针 `Box<T>` 是最基本的工具，用于将数据移动到堆上。掌握它对于处理大型数据结构、递归类型（如树、图）以及面向对象编程（Trait Objects）至关重要。

## 📖 核心概念

### 1. 栈 vs 堆
*   **栈**: 后进先出，速度极快。所有存储在栈上的数据必须有已知的大小。
*   **堆**: 可以在运行时请求任意大小的内存。访问速度稍慢（需要指针跳转），分配和释放需要开销。

`Box<T>` 是一个指针，它本身（指针地址）存储在栈上，而它指向的数据存储在堆上。

### 2. Box<T> 的使用场景
1.  **编译时大小未知**: 例如递归类型。
2.  **大数据转移**: 转移所有权时只拷贝指针，不拷贝数据。
3.  **Trait 对象**: 当你只关心类型实现了什么 Trait，而不关心具体类型时。

### 3. 递归类型 (Recursive Types)
Rust 需要在编译时知道每个类型的大小。如果一个结构体直接包含自身，大小就是无限的，Rust 无法编译。
`Box<T>` 的大小是固定的（指针大小），因此可以用来打破递归循环。

```rust
// 无法编译：Has infinite size
// enum List { Cons(i32, List), Nil }

// 可以编译
enum List {
    Cons(i32, Box<List>),
    Nil,
}
```

### 4. Deref Trait
实现了 `Deref` trait 的智能指针可以像普通引用一样使用 `*` 运算符。
Rust 提供**解引用强制转换 (Deref Coercion)**：如果 `T` 实现了 `Deref<Target=U>`，我们可以将 `&T` 传给需要 `&U` 的函数。
例如：`String` 实现 `Deref<Target=str>`，所以 `&String` 可以传给接受 `&str` 的函数。

### 5. Drop Trait
`Drop` trait 类似于其他语言的析构函数。当值离开作用域时，Rust 会自动调用 `drop` 方法。我们可以实现它来释放文件、网络连接等资源。

## 💻 代码示例

### 示例 1: 基本 Box 使用

```rust
fn main() {
    let b = Box::new(5);
    println!("b = {}", b);
}
```

### 示例 2: 递归数据结构

```rust
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Nil))));
    println!("{:?}", list);
}
```

## 🏋️ 练习题

### 练习 1: Box 基础
创建一个 `Box` 来存储一个非常大的数组（例如 1000 个 i32），并验证它确实是在堆上分配的（虽然验证堆分配比较困难，这里主要练习语法）。
[查看参考答案](./exercises/solutions/exercise1_solution.rs)

### 练习 2: 链表构建
使用 `Box` 定义一个 `Cons` 列表，并编写一个函数来求和。
[查看参考答案](./exercises/solutions/exercise2_solution.rs)

### 练习 3: 自定义智能指针
实现一个 `MyBox<T>` 结构体，并实现 `Deref` trait，使其可以被解引用。
[查看参考答案](./exercises/solutions/exercise3_solution.rs)

## 🤔 常见问题

### Q1: `Box::new` 会发生拷贝吗？
A: 在优化构建 (`--release`) 中，编译器通常会优化掉中间的栈拷贝，直接在堆上构建对象。但在 Debug 模式下，可能会先在栈上创建再移动到堆上。

### Q2: 既然 Rust 有 GC 吗？
A: Rust 没有垃圾回收器 (GC)。`Box` 利用所有权系统和 `Drop` trait 来管理内存。当 `Box` 离开作用域时，它拥有的堆内存会被立即释放。

## 🔗 扩展阅读
- [Rust Book: Using Box<T> to Point to Data on the Heap](https://doc.rust-lang.org/book/ch15-01-box.html)
- [Rust Book: Smart Pointers](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html)
