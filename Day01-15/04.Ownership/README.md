# Day 04: 所有权 (Ownership)

## 📝 学习目标
- 理解 Rust 内存管理的三种方式
- 掌握所有权的三大基本规则
- 理解 Stack 与 Heap 的区别
- 掌握移动 (Move) 和克隆 (Clone) 语义
- 理解 Copy Trait 及其适用场景

## 🎯 为什么要学这个
所有权是 Rust 最独特、最核心的特性。
- **无需 GC 的安全**：它让 Rust 在没有垃圾回收 (GC) 的情况下保证内存安全，避免了 Java/Python 的 GC 停顿问题。
- **避免常见错误**：它在编译期就杜绝了空指针、双重释放 (Double Free)、悬垂指针 (Dangling Pointer) 等 C/C++ 常见 bug。
- **并发安全**：所有权系统天然地限制了多线程间的数据竞争。

## 📖 核心概念

### 1. 内存管理模型
Rust 选择了一条不同的路：通过**所有权系统**管理内存，包含一组在**编译时**检查的规则。运行时没有额外开销。

### 2. 栈 (Stack) 与 堆 (Heap)
- **栈 (Stack)**: 后进先出，速度极快。存储固定大小数据 (如 `i32`, `bool`)。
- **堆 (Heap)**: 存储大小未知或可变的数据 (如 `String`, `Vec`)。访问需通过指针，速度稍慢。

### 3. 所有权三原则
1. **每个值都有一个所有者 (Owner)**：每个数据只能被一个变量拥有。
2. **同一时间只能有一个所有者**：避免了多重所有权的混乱。
3. **所有者离开作用域，值被丢弃**：自动释放资源，防止内存泄漏。

### 4. 移动语义 (Move Semantics)
对于复杂类型（如 `String`），赋值操作是 **移动 (Move)** 而非浅拷贝。

```rust
let s1 = String::from("hello");
let s2 = s1; // s1 的所有权移动给了 s2
// println!("{}", s1); // 错误！s1 已失效
```

### 5. 克隆 (Clone)
如果你确实需要深度拷贝堆上的数据，使用 `.clone()` 方法。

```rust
let s1 = String::from("hello");
let s2 = s1.clone(); // 堆数据被复制
println!("s1 = {}, s2 = {}", s1, s2); // 正常运行
```

### 6. Copy Trait
对于存储在栈上的简单类型（如整数），赋值是自动 **拷贝 (Copy)**。

```rust
let x = 5;
let y = x; // x 的值被拷贝给 y
println!("x = {}, y = {}", x, y); // x 依然有效
```
**常见的 Copy 类型**: 整数、浮点数、布尔值、字符、元素全是 Copy 的元组。

## 💻 代码示例

### 示例 1: 所有权转移
```rust
fn main() {
    let s = String::from("hello");
    takes_ownership(s); // s 被移动到函数内
    // println!("{}", s); // 错误！s 在这里无效了

    let x = 5;
    makes_copy(x); // x 是 Copy 类型，传递的是副本
    println!("{}", x); // x 依然有效
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} // some_string 离开作用域，内存被释放

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}
```

### 示例 2: 返回值转移所有权
```rust
fn main() {
    let s1 = gives_ownership();         //返回值移动给 s1
    let s2 = String::from("hello");     // s2 进入作用域
    let s3 = takes_and_gives_back(s2);  // s2 被移动到函数，函数返回值移动给 s3
} // s3 离开作用域被 drop。s2 已被移动无操作。s1 离开作用域被 drop。

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string // 返回 some_string 并移出给调用者
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string  // 原样返回
}
```

## 🏋️ 练习题

我们为你准备了练习题来深度体验所有权机制。

- **练习 1**: 修复所有权移动导致的错误
- **练习 2**: 使用 Clone 解决移动问题
- **练习 3**: 区分 Copy 和 Move 类型

👉 **[点击这里查看练习题](./exercises/README.md)**

## 🤔 常见问题 (FAQ)

### Q1: 为什么 Rust 不默认使用 Clone？
A: `Clone` 通常涉及堆内存分配和数据复制，开销较大。Rust 倾向于性能优先，默认行为（Move）是零成本的（只拷贝指针）。如果需要深拷贝，必须显式调用 `.clone()`。

### Q2: 什么是 RAII？
A: RAII (Resource Acquisition Is Initialization) 是一种编程模式，资源（内存、文件句柄等）的生命周期绑定到变量的作用域。Rust 的所有权系统就是 RAII 的完美体现：变量创建即获取资源，离开作用域即释放资源。

## 💡 最佳实践
- **理解 Move**: 习惯变量被"移动"而不是"拷贝"。
- **少用 Clone**: 除非真的需要两个独立的数据副本，否则尽量避免 `clone`，因为这会影响性能。考虑使用 **引用** (下一节的内容) 来共享数据。
- **利用作用域**: 利用 `{}` 手动控制变量的生命周期，及时释放内存。

## 🔗 扩展阅读
- [Rust 程序设计语言 - 什么是所有权](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)
- [Extra: 所有权深度剖析](../../extras/ownership-deep-dive.md) (强烈推荐阅读！)

## 📚 本节要点回顾
- 堆数据默认移动 (Move)，栈数据默认拷贝 (Copy)。
- 所有权规则保证了内存安全，防止双重释放。
- 变量离开作用域时自动调用 `drop` 释放资源。

## ⏭️ 下一步
所有权虽然强大，但如果我们每次使用数据都要转移所有权再还回来，那就太麻烦了。Rust 提供了 **引用 (References)** 来解决这个问题。

下一节: [Day 05: 引用与借用](../05.ReferencesAndBorrowing/README.md)
