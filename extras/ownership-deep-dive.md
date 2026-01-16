# 所有权系统深度剖析 (Deep Dive into Ownership)

所有权 (Ownership) 是 Rust 最独特、最核心的特性。它让 Rust 在无需垃圾回收 (GC) 的情况下保证内存安全。本文将深入探讨所有权背后的机制，特别是**栈与堆**、**移动语义 (Move Semantics)** 和 **Copy Trait**。

## 1. 内存模型：栈 (Stack) 与 堆 (Heap)

理解所有权，首先要理解 Rust 如何使用内存。

### 栈 (Stack)
- **特点**：后进先出 (LIFO)，速度极快。
- **存储内容**：编译时已知大小的数据（如 `i32`, `bool`, 固定大小数组）。
- **分配**：入栈 (Push) 和出栈 (Pop)，无需复杂的分配器查找空闲内存。

```
Stack:
| Value | Address |
|-------|---------|
|  x=5  |  0x100  | <- Top
|  y=10 |  0x104  |
```

### 堆 (Heap)
- **特点**：无序，通过指针访问，速度稍慢（需要分配和寻址）。
- **存储内容**：编译时大小未知或可变的数据（如 `String`, `Vec<T>`）。
- **分配**：需要在堆上找到一块足够大的空间，标记为已用，并返回指针。

```
Stack Pointer -> Heap Address -> Data
```

## 2. 所有权三原则

1. Rust 中的每一个值都有一个被称为其 **所有者 (Owner)** 的变量。
2. 值在任一时刻有且只有一个所有者。
3. 当所有者（变量）离开作用域，这个值将被丢弃 (Dropped)。

## 3. 移动语义 (Move Semantics)

这是 Rust 与 C++ 等语言的一个关键区别。在 C++ 中，赋值通常是拷贝；在 Rust 中，对于复杂类型，赋值是**移动**。

### 示例：String 的移动

```rust
let s1 = String::from("hello");
let s2 = s1;
// println!("{}", s1); // 错误！s1 的所有权已经移动给 s2
```

### 内存视角 (ASCII 图解)

**步骤 1: `let s1 = String::from("hello");`**

`String` 由三部分组成（存储在栈上）：
1. `ptr`: 指向堆内存的指针
2. `len`: 当前长度
3. `capacity`: 总容量

```
Stack (s1)             Heap
+---------+           +---+---+---+---+---+
| ptr     |---------->| h | e | l | l | o |
| len: 5  |           +---+---+---+---+---+
| cap: 5  |
+---------+
```

**步骤 2: `let s2 = s1;` (Move)**

Rust **不会**深度拷贝堆上的数据。它只是拷贝了栈上的指针、长度和容量。
**关键点**：为了保证内存安全（避免双重释放 Double Free），Rust 会将 `s1` 标记为无效。

```
Stack (s1) - INVALID   Stack (s2)             Heap
+---------+           +---------+           +---+---+---+---+---+
| ptr     |           | ptr     |---------->| h | e | l | l | o |
| len: 5  |           | len: 5  |           +---+---+---+---+---+
| cap: 5  |           | cap: 5  |
+---------+           +---------+
```

现在，`s2` 是堆数据的唯一所有者。当 `s2` 离开作用域时，Rust 会自动调用 `drop` 释放堆内存。

## 4. Copy Trait

你可能会问，为什么整数赋值不会发生移动？

```rust
let x = 5;
let y = x;
println!("x = {}, y = {}", x, y); // 有效！
```

这是因为像整数这样已知大小的简单类型，完全存储在栈上，拷贝它们非常快。
Rust 有一个特殊的 trait 叫做 `Copy`。
- 如果一个类型实现了 `Copy`，赋值操作会创建一个旧变量的副本。
- 只有完全存储在栈上的类型才能实现 `Copy`。

**常见的 Copy 类型**:
- 所有整数类型 (`u32`, `i32`, `usize` 等)
- 布尔类型 (`bool`)
- 浮点数类型 (`f64`, `f32`)
- 字符类型 (`char`)
- 元组（当且仅当其包含的类型也都实现了 `Copy`）

**不能 Copy 的类型**:
- `String`
- `Vec<T>`
- 任何需要分配堆内存或管理资源的类型

## 5. 函数传参中的所有权

函数传参的机制与赋值完全一样。

```rust
fn main() {
    let s = String::from("hello");
    take_ownership(s); // s 的所有权移动到函数内
    // println!("{}", s); // 错误！s 已失效

    let x = 5;
    makes_copy(x); // x 是 Copy 类型，传递的是副本
    println!("{}", x); // 有效
}

fn take_ownership(some_string: String) {
    println!("{}", some_string);
} // some_string 离开作用域，drop 被调用，内存释放

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
} // some_integer 离开作用域，无事发生（栈数据弹出）
```

## 6. 如何避免移动？

如果你确实需要保留原数据，同时创建一个新的副本，可以使用 `.clone()` 方法（深度拷贝）。

```rust
let s1 = String::from("hello");
let s2 = s1.clone();

println!("s1 = {}, s2 = {}", s1, s2);
```

**注意**：`clone` 通常意味着堆分配和数据复制，可能会影响性能。

## 总结

- **栈数据**：可以直接 Copy，速度快。
- **堆数据**：默认 Move，所有权转移，原变量失效。
- **所有权规则**：保证了没有悬垂指针，没有双重释放，且无需垃圾回收。

理解这一层，你就跨过了 Rust 学习曲线最陡峭的一步！
