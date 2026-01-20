# Day 19: 智能指针 - Box<T>

## 📝 学习目标

- 理解 **栈 (Stack)** 与 **堆 (Heap)** 的内存模型
- 掌握 **`Box<T>`** 的核心作用：所有权转移到堆
- 学会使用 `Box` 解决 **递归类型** (Infinite Size) 问题
- 理解 **`Deref`** (解引用) 与 **`Drop`** (资源清理) Trait
- 了解 **解引用强制转换 (Deref Coercion)** 的魔法

## 🎯 核心概念：指向堆的指针

在 Rust 中，所有值默认都是在 **栈** 上分配的。
`Box<T>` 是最简单的智能指针，它允许你把数据存储在 **堆** 上，并在栈上保留一个指向堆数据的指针。

### 内存布局可视化

```mermaid
graph TD
    subgraph Stack [栈 Stack (固定大小, 快)]
        Ptr[b: ptr] -->|指向| HeapData
    end
    
    subgraph Heap [堆 Heap (动态大小, 慢)]
        HeapData[5]
    end
    
    style Stack fill:#ccffcc
    style Heap fill:#ffcccc
```

- **栈上的 `b`**: 只有指针大小 (8 字节)，非常快。
- **堆上的数据**: 可以非常大 (比如 1GB 的数组)。

---

## 🛠️ 为什么需要 Box？

### 1. 递归类型 (Recursive Types)

Rust 必须在编译时知道类型的确切大小。如果不使用指针，递归类型的大小是无限的。

```rust
// ❌ 无法编译：大小无限！
// List = i32 + List = i32 + i32 + List = ... ∞
enum List {
    Cons(i32, List),
    Nil,
}
```

**解决方案**: 使用 `Box`。哪怕 `Box` 指向一头大象，`Box` 本身的大小永远只是一个指针的大小。

```mermaid
graph LR
    Cons1[Cons 1] -->|Box ptr| Cons2[Cons 2]
    Cons2 -->|Box ptr| Nil[Nil]
    
    style Cons1 fill:#fff9c4
    style Cons2 fill:#fff9c4
```

```rust
// ✅ 可以编译：大小固定
enum List {
    Cons(i32, Box<List>), // i32 + 指针大小
    Nil,
}
```

### 2. 大数据转移

当你拥有大量数据（如 `[i32; 10000]`）并希望转移所有权时，如果不使用 `Box`，数据会在栈上进行拷贝，效率低下。使用 `Box` 只会拷贝指针。

### 3. Trait 对象

当你需要在一个集合中存储不同类型，但它们实现相同 Trait 时 (Day 26 详解)。

---

## 🔮 这就是魔法：Deref 与 Drop

智能指针之所以叫“智能”，是因为它们实现了两个特殊的 Trait。

### 1. Deref (解引用)

允许我们将智能指针当做普通引用来用 (`*` 运算符)。

```rust
use std::ops::Deref;

struct MyBox<T>(T);

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0 // 返回内部数据的引用
    }
}

fn main() {
    let x = 5;
    let y = MyBox(x);

    assert_eq!(5, x);
    assert_eq!(5, *y); // 编译器实际上运行的是 *(y.deref())
}
```

**解引用强制转换 (Deref Coercion)**:
这是 Rust 的一个超级便利特性。如果你实现了 `Deref<Target=U>`，那么 `&T` 可以自动变成 `&U`。

- `&String` -> `&str`
- `&Box<String>` -> `&String` -> `&str`

> 这就是为什么接受 `&str` 的函数也能接受 `&Box<String>`。

### 2. Drop (清理)

当智能指针离开作用域时，`drop` 方法会被自动调用，用来释放堆内存或资源（如文件句柄、锁）。

```rust
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer { data: String::from("my stuff") };
    let d = CustomSmartPointer { data: String::from("other stuff") };
    println!("CustomSmartPointers created.");
} // 打印顺序：d 先 drop，c 后 drop (栈的后进先出)
```

---

## 💻 代码实战：构建链表

```rust
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    // 构造链表: 1 -> 2 -> 3 -> Nil
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    
    println!("List: {:?}", list);
}
```

---

## 🏋️ 练习题

👉 **[点击这里查看练习题](./exercises/README.md)**

1. **基础 Box**: 将一个栈上的大数据移动到堆上。
2. **递归求和**: 编写一个递归函数，计算 `List` 中所有整数的和。
3. **实现智能指针**: 手动实现一个类似 Box 的封装结构，并观察 `Drop` 的调用时机。

---

## 💡 最佳实践

1. **不要滥用 Box**: Rust 默认的栈分配非常高效。只有在确实需要堆分配（递归、大小未知、大数据）时才使用 Box。
2. **理解所有权**: `Box<T>` 拥有它指向的数据。当 Box 被销毁，数据也被销毁。

---

## ⏭️ 下一步

`Box<T>` 解决了“独占所有权”的堆分配问题。但如果我们想要 **多个指针同时指向同一块数据** 该怎么办？（比如图数据结构）。
Rust 提供了引用计数指针 `Rc<T>`。

下一节: [Day 20: 智能指针 - Rc 与 RefCell](../20.SmartPointers_RcRefCell/README.md)
